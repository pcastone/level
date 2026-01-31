//! View API endpoints (Timeline, Kanban, Calendar)

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::Result;
use crate::models::Noun;

use super::state::AppState;

/// View routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/views/:sow_id/timeline", get(timeline_view))
        .route("/views/:sow_id/kanban", get(kanban_view))
        .route("/views/:sow_id/calendar", get(calendar_view))
}

#[derive(Debug, Deserialize)]
pub struct TimelineQuery {
    start: Option<String>,
    end: Option<String>,
    #[serde(default = "default_limit")]
    limit: i64,
}

fn default_limit() -> i64 {
    100
}

#[derive(Debug, Serialize)]
pub struct TimelineResponse {
    view: String,
    sow_id: String,
    items: Vec<TimelineItem>,
}

#[derive(Debug, Serialize)]
pub struct TimelineItem {
    id: String,
    short_name: String,
    title: String,
    #[serde(rename = "type")]
    noun_type: String,
    state: String,
    due_date: Option<DateTime<Utc>>,
    is_blocked: bool,
}

/// GET /views/:sow_id/timeline - Timeline view
async fn timeline_view(
    State(state): State<AppState>,
    Path(sow_id): Path<Uuid>,
    Query(query): Query<TimelineQuery>,
) -> Result<Json<TimelineResponse>> {
    let nouns: Vec<Noun> = if let (Some(start), Some(end)) = (&query.start, &query.end) {
        sqlx::query_as::<_, Noun>(
            r#"
            SELECT id, type as noun_type, sow_id, parent_id, short_name, title, description,
                   state, is_blocked, due_date, completed_at, closed_at, custom_fields, created_at, updated_at
            FROM nouns
            WHERE sow_id = $1 AND due_date >= $2 AND due_date <= $3
            ORDER BY due_date ASC
            LIMIT $4
            "#,
        )
        .bind(sow_id)
        .bind(start)
        .bind(end)
        .bind(query.limit)
        .fetch_all(&state.pool)
        .await?
    } else {
        sqlx::query_as::<_, Noun>(
            r#"
            SELECT id, type as noun_type, sow_id, parent_id, short_name, title, description,
                   state, is_blocked, due_date, completed_at, closed_at, custom_fields, created_at, updated_at
            FROM nouns
            WHERE sow_id = $1 AND due_date IS NOT NULL
            ORDER BY due_date ASC
            LIMIT $2
            "#,
        )
        .bind(sow_id)
        .bind(query.limit)
        .fetch_all(&state.pool)
        .await?
    };

    let items: Vec<TimelineItem> = nouns
        .into_iter()
        .map(|n| TimelineItem {
            id: n.id.to_string(),
            short_name: n.short_name,
            title: n.title,
            noun_type: n.noun_type,
            state: n.state,
            due_date: n.due_date,
            is_blocked: n.is_blocked,
        })
        .collect();

    Ok(Json(TimelineResponse {
        view: "timeline".to_string(),
        sow_id: sow_id.to_string(),
        items,
    }))
}

#[derive(Debug, Deserialize)]
pub struct KanbanQuery {
    container_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct KanbanResponse {
    view: String,
    sow_id: String,
    columns: Vec<KanbanColumn>,
}

#[derive(Debug, Serialize)]
pub struct KanbanColumn {
    state: String,
    items: Vec<KanbanItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KanbanItem {
    id: String,
    short_name: String,
    title: String,
    #[serde(rename = "type")]
    noun_type: String,
    is_blocked: bool,
    sort_order: i32,
}

/// GET /views/:sow_id/kanban - Kanban view
async fn kanban_view(
    State(state): State<AppState>,
    Path(sow_id): Path<Uuid>,
    Query(query): Query<KanbanQuery>,
) -> Result<Json<KanbanResponse>> {
    // Fetch nouns - either for a specific container or all
    let nouns: Vec<Noun> = if let Some(container_id) = query.container_id {
        sqlx::query_as::<_, Noun>(
            r#"
            SELECT n.id, n.type as noun_type, n.sow_id, n.parent_id, n.short_name, n.title, n.description,
                   n.state, n.is_blocked, n.due_date, n.completed_at, n.closed_at, n.custom_fields, n.created_at, n.updated_at
            FROM nouns n
            INNER JOIN noun_assignments na ON na.noun_id = n.id AND na.container_id = $2
            WHERE n.sow_id = $1
            ORDER BY na.sort_order ASC
            "#,
        )
        .bind(sow_id)
        .bind(container_id)
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default()
    } else {
        // Get all nouns grouped by state
        sqlx::query_as::<_, Noun>(
            r#"
            SELECT id, type as noun_type, sow_id, parent_id, short_name, title, description,
                   state, is_blocked, due_date, completed_at, closed_at, custom_fields, created_at, updated_at
            FROM nouns
            WHERE sow_id = $1 AND state NOT IN ('Closed', 'Archived')
            ORDER BY created_at ASC
            "#,
        )
        .bind(sow_id)
        .fetch_all(&state.pool)
        .await?
    };

    // Group by state
    let mut columns_map: std::collections::HashMap<String, Vec<KanbanItem>> = std::collections::HashMap::new();
    for st in ["Normal", "Escalated", "Completed", "Incompleted"] {
        columns_map.insert(st.to_string(), vec![]);
    }

    for (idx, noun) in nouns.into_iter().enumerate() {
        let item = KanbanItem {
            id: noun.id.to_string(),
            short_name: noun.short_name,
            title: noun.title,
            noun_type: noun.noun_type.clone(),
            is_blocked: noun.is_blocked,
            sort_order: idx as i32,
        };
        columns_map
            .entry(noun.state.clone())
            .or_default()
            .push(item);
    }

    let columns: Vec<KanbanColumn> = ["Normal", "Escalated", "Completed", "Incompleted"]
        .iter()
        .map(|st| KanbanColumn {
            state: st.to_string(),
            items: columns_map.get(*st).cloned().unwrap_or_default(),
        })
        .collect();

    Ok(Json(KanbanResponse {
        view: "kanban".to_string(),
        sow_id: sow_id.to_string(),
        columns,
    }))
}

#[derive(Debug, Deserialize)]
pub struct CalendarQuery {
    month: Option<String>, // YYYY-MM format
}

#[derive(Debug, Serialize)]
pub struct CalendarResponse {
    view: String,
    sow_id: String,
    month: String,
    days: Vec<CalendarDay>,
}

#[derive(Debug, Serialize)]
pub struct CalendarDay {
    date: String,
    items: Vec<CalendarItem>,
}

#[derive(Debug, Serialize)]
pub struct CalendarItem {
    id: String,
    short_name: String,
    title: String,
    #[serde(rename = "type")]
    noun_type: String,
    is_blocked: bool,
}

/// GET /views/:sow_id/calendar - Calendar view
async fn calendar_view(
    State(state): State<AppState>,
    Path(sow_id): Path<Uuid>,
    Query(query): Query<CalendarQuery>,
) -> Result<Json<CalendarResponse>> {
    let now = Utc::now();
    let month_str = query
        .month
        .unwrap_or_else(|| format!("{}-{:02}", now.year(), now.month()));

    // Parse month
    let parts: Vec<&str> = month_str.split('-').collect();
    let (year, month) = if parts.len() == 2 {
        (
            parts[0].parse::<i32>().unwrap_or(now.year()),
            parts[1].parse::<u32>().unwrap_or(now.month()),
        )
    } else {
        (now.year(), now.month())
    };

    let start_date = format!("{}-{:02}-01", year, month);
    let end_date = if month == 12 {
        format!("{}-01-01", year + 1)
    } else {
        format!("{}-{:02}-01", year, month + 1)
    };

    let nouns: Vec<Noun> = sqlx::query_as::<_, Noun>(
        r#"
        SELECT id, type as noun_type, sow_id, parent_id, short_name, title, description,
               state, is_blocked, due_date, completed_at, closed_at, custom_fields, created_at, updated_at
        FROM nouns
        WHERE sow_id = $1 AND due_date >= $2 AND due_date < $3
        ORDER BY due_date ASC
        "#,
    )
    .bind(sow_id)
    .bind(&start_date)
    .bind(&end_date)
    .fetch_all(&state.pool)
    .await?;

    // Group by day
    let mut days_map: std::collections::HashMap<String, Vec<CalendarItem>> =
        std::collections::HashMap::new();

    for noun in nouns {
        if let Some(due_date) = noun.due_date {
            let date_str = due_date.format("%Y-%m-%d").to_string();
            days_map.entry(date_str).or_default().push(CalendarItem {
                id: noun.id.to_string(),
                short_name: noun.short_name,
                title: noun.title,
                noun_type: noun.noun_type,
                is_blocked: noun.is_blocked,
            });
        }
    }

    let mut days: Vec<CalendarDay> = days_map
        .into_iter()
        .map(|(date, items)| CalendarDay { date, items })
        .collect();
    days.sort_by(|a, b| a.date.cmp(&b.date));

    Ok(Json(CalendarResponse {
        view: "calendar".to_string(),
        sow_id: sow_id.to_string(),
        month: month_str,
        days,
    }))
}

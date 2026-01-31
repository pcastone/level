//! Output formatting for CLI

use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

/// Output format options
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Table,
    Json,
    Minimal,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => Self::Json,
            "minimal" | "min" => Self::Minimal,
            _ => Self::Table,
        }
    }
}

/// Format a list of SOWs
pub fn format_sows(sows: &[serde_json::Value], format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&sows).unwrap_or_default());
        }
        OutputFormat::Minimal => {
            for sow in sows {
                let short_name = sow.get("short_name").and_then(|v| v.as_str()).unwrap_or("-");
                let title = sow.get("title").and_then(|v| v.as_str()).unwrap_or("-");
                println!("{}\t{}", short_name, title);
            }
        }
        OutputFormat::Table => {
            #[derive(Tabled)]
            struct SowRow {
                #[tabled(rename = "ID")]
                id: String,
                #[tabled(rename = "Short Name")]
                short_name: String,
                #[tabled(rename = "Title")]
                title: String,
                #[tabled(rename = "State")]
                state: String,
            }

            let rows: Vec<SowRow> = sows
                .iter()
                .map(|s| SowRow {
                    id: s.get("id").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                    short_name: s.get("short_name").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                    title: s.get("title").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                    state: s.get("state").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                })
                .collect();

            if rows.is_empty() {
                println!("{}", "No SOWs found".yellow());
            } else {
                let table = Table::new(rows).with(Style::rounded()).to_string();
                println!("{}", table);
            }
        }
    }
}

/// Format a list of nouns
pub fn format_nouns(nouns: &[serde_json::Value], format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&nouns).unwrap_or_default());
        }
        OutputFormat::Minimal => {
            for noun in nouns {
                let short_name = noun.get("short_name").and_then(|v| v.as_str()).unwrap_or("-");
                let title = noun.get("title").and_then(|v| v.as_str()).unwrap_or("-");
                println!("{}\t{}", short_name, title);
            }
        }
        OutputFormat::Table => {
            #[derive(Tabled)]
            struct NounRow {
                #[tabled(rename = "Short Name")]
                short_name: String,
                #[tabled(rename = "Type")]
                noun_type: String,
                #[tabled(rename = "Title")]
                title: String,
                #[tabled(rename = "State")]
                state: String,
                #[tabled(rename = "Blocked")]
                blocked: String,
            }

            let rows: Vec<NounRow> = nouns
                .iter()
                .map(|n| {
                    let blocked = n.get("is_blocked").and_then(|v| v.as_bool()).unwrap_or(false);
                    NounRow {
                        short_name: n.get("short_name").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                        noun_type: n.get("type").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                        title: truncate(n.get("title").and_then(|v| v.as_str()).unwrap_or("-"), 40),
                        state: n.get("state").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                        blocked: if blocked { "Yes".red().to_string() } else { "No".to_string() },
                    }
                })
                .collect();

            if rows.is_empty() {
                println!("{}", "No nouns found".yellow());
            } else {
                let table = Table::new(rows).with(Style::rounded()).to_string();
                println!("{}", table);
            }
        }
    }
}

/// Format a single noun detail
pub fn format_noun_detail(noun: &serde_json::Value, format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(noun).unwrap_or_default());
        }
        OutputFormat::Minimal => {
            let short_name = noun.get("short_name").and_then(|v| v.as_str()).unwrap_or("-");
            let title = noun.get("title").and_then(|v| v.as_str()).unwrap_or("-");
            println!("{}: {}", short_name, title);
        }
        OutputFormat::Table => {
            println!("{}", "─".repeat(60).dimmed());
            println!("{}: {}", "ID".bold(), noun.get("id").and_then(|v| v.as_str()).unwrap_or("-"));
            println!("{}: {}", "Short Name".bold(), noun.get("short_name").and_then(|v| v.as_str()).unwrap_or("-"));
            println!("{}: {}", "Type".bold(), noun.get("type").and_then(|v| v.as_str()).unwrap_or("-"));
            println!("{}: {}", "Title".bold(), noun.get("title").and_then(|v| v.as_str()).unwrap_or("-"));

            if let Some(desc) = noun.get("description").and_then(|v| v.as_str()) {
                println!("{}: {}", "Description".bold(), desc);
            }

            println!("{}: {}", "State".bold(), noun.get("state").and_then(|v| v.as_str()).unwrap_or("-"));

            let blocked = noun.get("is_blocked").and_then(|v| v.as_bool()).unwrap_or(false);
            if blocked {
                println!("{}: {}", "Blocked".bold(), "Yes".red());
            }

            if let Some(due) = noun.get("due_date").and_then(|v| v.as_str()) {
                println!("{}: {}", "Due Date".bold(), due);
            }

            if let Some(parent) = noun.get("parent_id").and_then(|v| v.as_str()) {
                println!("{}: {}", "Parent".bold(), parent);
            }

            println!("{}: {}", "Created".bold(), noun.get("created_at").and_then(|v| v.as_str()).unwrap_or("-"));
            println!("{}: {}", "Updated".bold(), noun.get("updated_at").and_then(|v| v.as_str()).unwrap_or("-"));
            println!("{}", "─".repeat(60).dimmed());
        }
    }
}

/// Format timeline view
pub fn format_timeline(data: &serde_json::Value, format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
        }
        _ => {
            if let Some(items) = data.get("items").and_then(|v| v.as_array()) {
                #[derive(Tabled)]
                struct TimelineRow {
                    #[tabled(rename = "Due Date")]
                    due_date: String,
                    #[tabled(rename = "Short Name")]
                    short_name: String,
                    #[tabled(rename = "Type")]
                    noun_type: String,
                    #[tabled(rename = "Title")]
                    title: String,
                    #[tabled(rename = "State")]
                    state: String,
                }

                let rows: Vec<TimelineRow> = items
                    .iter()
                    .map(|i| TimelineRow {
                        due_date: i.get("due_date").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                        short_name: i.get("short_name").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                        noun_type: i.get("type").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                        title: truncate(i.get("title").and_then(|v| v.as_str()).unwrap_or("-"), 40),
                        state: i.get("state").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                    })
                    .collect();

                if rows.is_empty() {
                    println!("{}", "No items with due dates".yellow());
                } else {
                    println!("{}", "Timeline View".bold());
                    let table = Table::new(rows).with(Style::rounded()).to_string();
                    println!("{}", table);
                }
            }
        }
    }
}

/// Format kanban view
pub fn format_kanban(data: &serde_json::Value, format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
        }
        _ => {
            if let Some(columns) = data.get("columns").and_then(|v| v.as_array()) {
                println!("{}", "Kanban View".bold());
                println!();

                for column in columns {
                    let state = column.get("state").and_then(|v| v.as_str()).unwrap_or("Unknown");
                    let items = column.get("items").and_then(|v| v.as_array());

                    let count = items.map(|i| i.len()).unwrap_or(0);
                    println!("{} ({})", state.bold().underline(), count);

                    if let Some(items) = items {
                        for item in items {
                            let short_name = item.get("short_name").and_then(|v| v.as_str()).unwrap_or("-");
                            let title = truncate(item.get("title").and_then(|v| v.as_str()).unwrap_or("-"), 50);
                            let blocked = item.get("is_blocked").and_then(|v| v.as_bool()).unwrap_or(false);

                            if blocked {
                                println!("  {} {} {}", "●".red(), short_name, title.dimmed());
                            } else {
                                println!("  {} {} {}", "●".green(), short_name, title.dimmed());
                            }
                        }
                    }
                    println!();
                }
            }
        }
    }
}

/// Format calendar view
pub fn format_calendar(data: &serde_json::Value, format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
        }
        _ => {
            let month = data.get("month").and_then(|v| v.as_str()).unwrap_or("Unknown");
            println!("{}: {}", "Calendar".bold(), month);
            println!();

            if let Some(days) = data.get("days").and_then(|v| v.as_array()) {
                for day in days {
                    let date = day.get("date").and_then(|v| v.as_str()).unwrap_or("-");
                    println!("{}", date.bold());

                    if let Some(items) = day.get("items").and_then(|v| v.as_array()) {
                        for item in items {
                            let short_name = item.get("short_name").and_then(|v| v.as_str()).unwrap_or("-");
                            let title = truncate(item.get("title").and_then(|v| v.as_str()).unwrap_or("-"), 50);
                            println!("  {} {}", short_name, title.dimmed());
                        }
                    }
                }
            }
        }
    }
}

/// Format aliases
pub fn format_aliases(aliases: &[serde_json::Value], format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&aliases).unwrap_or_default());
        }
        OutputFormat::Minimal => {
            for alias in aliases {
                let name = alias.get("name").and_then(|v| v.as_str()).unwrap_or("-");
                let noun_id = alias.get("noun_id").and_then(|v| v.as_str()).unwrap_or("-");
                println!("*{}\t{}", name, noun_id);
            }
        }
        OutputFormat::Table => {
            #[derive(Tabled)]
            struct AliasRow {
                #[tabled(rename = "Alias")]
                name: String,
                #[tabled(rename = "Noun ID")]
                noun_id: String,
            }

            let rows: Vec<AliasRow> = aliases
                .iter()
                .map(|a| AliasRow {
                    name: format!("*{}", a.get("name").and_then(|v| v.as_str()).unwrap_or("-")),
                    noun_id: a.get("noun_id").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                })
                .collect();

            if rows.is_empty() {
                println!("{}", "No aliases defined".yellow());
            } else {
                let table = Table::new(rows).with(Style::rounded()).to_string();
                println!("{}", table);
            }
        }
    }
}

/// Format success message
pub fn format_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

/// Format error message
pub fn format_error(message: &str) {
    eprintln!("{} {}", "✗".red(), message);
}

/// Format help output
pub fn format_help(data: &serde_json::Value, format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
        }
        _ => {
            // Generic help formatter
            if let Some(arr) = data.as_array() {
                for item in arr {
                    if let Some(name) = item.get("name").and_then(|v| v.as_str()) {
                        print!("{}", name.bold());
                    }
                    if let Some(abbrev) = item.get("abbreviation").and_then(|v| v.as_str()) {
                        print!(" ({})", abbrev.dimmed());
                    }
                    println!();

                    if let Some(desc) = item.get("description").and_then(|v| v.as_str()) {
                        println!("  {}", desc);
                    }
                    println!();
                }
            } else if let Some(obj) = data.as_object() {
                for (key, value) in obj {
                    println!("{}: {}", key.bold(), value);
                }
            }
        }
    }
}

/// Truncate string to max length
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}

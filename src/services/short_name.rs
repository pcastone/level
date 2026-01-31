//! Short name generation service

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Result;
use crate::models::NounType;

/// Short name generation service
pub struct ShortNameService;

impl ShortNameService {
    /// Generate short name: {SOW.short_name}-{TYPE_ABBREV}-{SEQUENCE}
    pub async fn generate(
        pool: &PgPool,
        sow_id: Uuid,
        sow_short_name: &str,
        noun_type: NounType,
    ) -> Result<String> {
        let type_abbrev = noun_type.abbreviation();
        let sequence = Self::next_sequence(pool, sow_id, noun_type).await?;

        Ok(format!("{}-{}-{:03}", sow_short_name, type_abbrev, sequence))
    }

    /// Get and increment sequence for SOW+type
    async fn next_sequence(pool: &PgPool, sow_id: Uuid, noun_type: NounType) -> Result<i32> {
        let type_str = noun_type.to_string();

        // Try to get existing sequence
        let existing: Option<(i32,)> = sqlx::query_as(
            r#"
            SELECT next_value FROM noun_sequences
            WHERE sow_id = $1 AND noun_type = $2
            FOR UPDATE
            "#,
        )
        .bind(sow_id)
        .bind(&type_str)
        .fetch_optional(pool)
        .await?;

        let sequence = if let Some((current,)) = existing {
            // Increment existing sequence
            sqlx::query(
                r#"
                UPDATE noun_sequences
                SET next_value = next_value + 1
                WHERE sow_id = $1 AND noun_type = $2
                "#,
            )
            .bind(sow_id)
            .bind(&type_str)
            .execute(pool)
            .await?;

            current
        } else {
            // Create new sequence starting at 1
            sqlx::query(
                r#"
                INSERT INTO noun_sequences (id, sow_id, noun_type, next_value)
                VALUES ($1, $2, $3, 2)
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(sow_id)
            .bind(&type_str)
            .execute(pool)
            .await?;

            1
        };

        Ok(sequence)
    }

    /// Validate SOW short_name
    pub fn validate_sow_short_name(short_name: &str) -> Result<()> {
        use crate::error::LevelError;

        if short_name.is_empty() {
            return Err(LevelError::ValidationError(
                "SOW short_name cannot be empty".to_string(),
            ));
        }

        if short_name.contains(' ') {
            return Err(LevelError::ValidationError(
                "SOW short_name cannot contain spaces".to_string(),
            ));
        }

        if !short_name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(LevelError::ValidationError(
                "SOW short_name can only contain alphanumeric characters, hyphens, and underscores".to_string(),
            ));
        }

        Ok(())
    }

    /// Check if SOW short_name is unique
    pub async fn is_sow_short_name_unique(pool: &PgPool, short_name: &str) -> Result<bool> {
        let exists: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM nouns
                WHERE type = 'sow' AND short_name = $1
            )
            "#,
        )
        .bind(short_name)
        .fetch_one(pool)
        .await?;

        Ok(!exists.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_abbreviations() {
        assert_eq!(NounType::Sow.abbreviation(), "SOW");
        assert_eq!(NounType::Task.abbreviation(), "TASK");
        assert_eq!(NounType::Request.abbreviation(), "REQ");
        assert_eq!(NounType::Meeting.abbreviation(), "MEET");
        assert_eq!(NounType::Deliverable.abbreviation(), "DELIV");
        assert_eq!(NounType::Event.abbreviation(), "EVT");
        assert_eq!(NounType::Blocker.abbreviation(), "BLOCK");
        assert_eq!(NounType::Artifact.abbreviation(), "ART");
        assert_eq!(NounType::Group.abbreviation(), "GRP");
        assert_eq!(NounType::Project.abbreviation(), "PROJ");
        assert_eq!(NounType::MileStone.abbreviation(), "MILE");
    }

    #[test]
    fn test_validate_sow_short_name() {
        assert!(ShortNameService::validate_sow_short_name("IT").is_ok());
        assert!(ShortNameService::validate_sow_short_name("HR-2024").is_ok());
        assert!(ShortNameService::validate_sow_short_name("Project_Alpha").is_ok());

        assert!(ShortNameService::validate_sow_short_name("").is_err());
        assert!(ShortNameService::validate_sow_short_name("IT Project").is_err());
        assert!(ShortNameService::validate_sow_short_name("IT@Project").is_err());
    }
}

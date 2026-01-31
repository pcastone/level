//! Reference resolution for CLI
//!
//! Resolves noun references in the following order:
//! 1. UUID - direct ID match
//! 2. ShortName - format: SOW-TYPE-NNN (e.g., IT-TASK-001)
//! 3. Alias - prefixed with * (e.g., *budget)
//! 4. Title - quoted string search

use anyhow::{Context, Result};
use uuid::Uuid;

use crate::client::LevelClient;

/// Reference type
#[derive(Debug, Clone)]
pub enum Reference {
    /// UUID reference
    Uuid(Uuid),
    /// Short name reference (e.g., IT-TASK-001)
    ShortName(String),
    /// Alias reference (e.g., *budget)
    Alias(String),
    /// Title search
    Title(String),
}

impl Reference {
    /// Parse a reference string
    pub fn parse(s: &str) -> Self {
        let s = s.trim();

        // Check for UUID
        if let Ok(uuid) = Uuid::parse_str(s) {
            return Self::Uuid(uuid);
        }

        // Check for alias (starts with *)
        if let Some(alias) = s.strip_prefix('*') {
            return Self::Alias(alias.to_string());
        }

        // Check for short name pattern: SOW-TYPE-NNN
        if is_short_name(s) {
            return Self::ShortName(s.to_string());
        }

        // Otherwise treat as title search
        Self::Title(s.to_string())
    }

    /// Resolve the reference to a noun ID
    pub async fn resolve(&self, client: &LevelClient, sow_id: &str) -> Result<String> {
        match self {
            Self::Uuid(uuid) => Ok(uuid.to_string()),
            Self::ShortName(short_name) => {
                resolve_short_name(client, sow_id, short_name).await
            }
            Self::Alias(name) => {
                resolve_alias(client, name).await
            }
            Self::Title(title) => {
                resolve_title(client, sow_id, title).await
            }
        }
    }
}

/// Check if string matches short name pattern
fn is_short_name(s: &str) -> bool {
    // Pattern: SOW-TYPE-NNN or similar
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() < 3 {
        return false;
    }

    // Last part should be numeric
    let last = parts.last().unwrap();
    if !last.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // All parts should be alphanumeric
    parts.iter().all(|p| p.chars().all(|c| c.is_alphanumeric()))
}

/// Resolve short name to noun ID
async fn resolve_short_name(client: &LevelClient, sow_id: &str, short_name: &str) -> Result<String> {
    // List nouns and find by short_name
    let response = client.list_nouns(sow_id, None, None, None).await?;

    if let Some(nouns) = response.get("nouns").and_then(|v| v.as_array()) {
        for noun in nouns {
            if let Some(sn) = noun.get("short_name").and_then(|v| v.as_str()) {
                if sn.eq_ignore_ascii_case(short_name) {
                    if let Some(id) = noun.get("id").and_then(|v| v.as_str()) {
                        return Ok(id.to_string());
                    }
                }
            }
        }
    }

    anyhow::bail!("Noun not found: {}", short_name)
}

/// Resolve alias to noun ID
async fn resolve_alias(client: &LevelClient, name: &str) -> Result<String> {
    let aliases = client.list_aliases().await?;

    for alias in aliases {
        if let Some(alias_name) = alias.get("name").and_then(|v| v.as_str()) {
            if alias_name.eq_ignore_ascii_case(name) {
                if let Some(noun_id) = alias.get("noun_id").and_then(|v| v.as_str()) {
                    return Ok(noun_id.to_string());
                }
            }
        }
    }

    anyhow::bail!("Alias not found: *{}", name)
}

/// Resolve title to noun ID
async fn resolve_title(client: &LevelClient, sow_id: &str, title: &str) -> Result<String> {
    // Remove quotes if present
    let title = title.trim_matches('"').trim_matches('\'');

    let response = client.list_nouns(sow_id, None, None, None).await?;

    if let Some(nouns) = response.get("nouns").and_then(|v| v.as_array()) {
        // Exact match first
        for noun in nouns {
            if let Some(t) = noun.get("title").and_then(|v| v.as_str()) {
                if t.eq_ignore_ascii_case(title) {
                    if let Some(id) = noun.get("id").and_then(|v| v.as_str()) {
                        return Ok(id.to_string());
                    }
                }
            }
        }

        // Partial match
        let title_lower = title.to_lowercase();
        for noun in nouns {
            if let Some(t) = noun.get("title").and_then(|v| v.as_str()) {
                if t.to_lowercase().contains(&title_lower) {
                    if let Some(id) = noun.get("id").and_then(|v| v.as_str()) {
                        return Ok(id.to_string());
                    }
                }
            }
        }
    }

    anyhow::bail!("Noun not found with title: {}", title)
}

/// Resolve multiple references
pub async fn resolve_many(
    client: &LevelClient,
    sow_id: &str,
    refs: &[String],
) -> Result<Vec<String>> {
    let mut ids = Vec::new();

    for r in refs {
        let reference = Reference::parse(r);
        let id = reference
            .resolve(client, sow_id)
            .await
            .with_context(|| format!("Failed to resolve: {}", r))?;
        ids.push(id);
    }

    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uuid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        match Reference::parse(uuid_str) {
            Reference::Uuid(uuid) => assert_eq!(uuid.to_string(), uuid_str),
            _ => panic!("Expected UUID reference"),
        }
    }

    #[test]
    fn test_parse_alias() {
        match Reference::parse("*budget") {
            Reference::Alias(name) => assert_eq!(name, "budget"),
            _ => panic!("Expected Alias reference"),
        }
    }

    #[test]
    fn test_parse_short_name() {
        match Reference::parse("IT-TASK-001") {
            Reference::ShortName(sn) => assert_eq!(sn, "IT-TASK-001"),
            _ => panic!("Expected ShortName reference"),
        }
    }

    #[test]
    fn test_parse_title() {
        match Reference::parse("My Task Title") {
            Reference::Title(title) => assert_eq!(title, "My Task Title"),
            _ => panic!("Expected Title reference"),
        }
    }

    #[test]
    fn test_is_short_name() {
        assert!(is_short_name("IT-TASK-001"));
        assert!(is_short_name("HR-REQ-123"));
        assert!(is_short_name("PROJ-DELIV-001"));

        assert!(!is_short_name("just-text"));
        assert!(!is_short_name("no-numbers"));
        assert!(!is_short_name("single"));
    }
}

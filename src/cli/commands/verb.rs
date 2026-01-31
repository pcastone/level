//! Verb command handlers

use anyhow::Result;
use colored::Colorize;

use crate::client::LevelClient;
use crate::config::CliConfig;
use crate::output::{self, OutputFormat};
use crate::reference::{self, Reference};

/// Complete noun(s)
pub async fn complete(
    client: &LevelClient,
    config: &CliConfig,
    references: Vec<String>,
    format: OutputFormat,
) -> Result<()> {
    apply_verb(client, config, references, "Complete", format).await
}

/// Incomplete noun(s)
pub async fn incomplete(
    client: &LevelClient,
    config: &CliConfig,
    references: Vec<String>,
    format: OutputFormat,
) -> Result<()> {
    apply_verb(client, config, references, "Incomplete", format).await
}

/// Escalate noun(s)
pub async fn escalate(
    client: &LevelClient,
    config: &CliConfig,
    references: Vec<String>,
    format: OutputFormat,
) -> Result<()> {
    apply_verb(client, config, references, "Escalate", format).await
}

/// Normal (de-escalate) noun(s)
pub async fn normal(
    client: &LevelClient,
    config: &CliConfig,
    references: Vec<String>,
    format: OutputFormat,
) -> Result<()> {
    apply_verb(client, config, references, "Normal", format).await
}

/// Close noun(s)
pub async fn close(
    client: &LevelClient,
    config: &CliConfig,
    references: Vec<String>,
    format: OutputFormat,
) -> Result<()> {
    apply_verb(client, config, references, "Close", format).await
}

/// Helper to apply verb to multiple nouns
async fn apply_verb(
    client: &LevelClient,
    config: &CliConfig,
    references: Vec<String>,
    verb: &str,
    format: OutputFormat,
) -> Result<()> {
    let sow_id = config.get_sow(None)?;

    if references.len() == 1 {
        // Single noun - use direct endpoint
        let ref_parsed = Reference::parse(&references[0]);
        let noun_id = ref_parsed.resolve(client, &sow_id).await?;

        let result = client.apply_verb(&sow_id, &noun_id, verb, None).await?;

        if let OutputFormat::Json = format {
            println!("{}", serde_json::to_string_pretty(&result)?);
        } else {
            output::format_success(&format!("{} applied to {}", verb, references[0]));
        }
    } else {
        // Multiple nouns - use batch endpoint
        let noun_ids = reference::resolve_many(client, &sow_id, &references).await?;

        let result = client.batch_verb(&sow_id, noun_ids, verb).await?;

        if let OutputFormat::Json = format {
            println!("{}", serde_json::to_string_pretty(&result)?);
        } else {
            output::format_success(&format!("{} applied to {} nouns", verb, references.len()));
        }
    }

    Ok(())
}

/// Create blocking relationship
pub async fn blocked(
    client: &LevelClient,
    config: &CliConfig,
    blocker: String,
    target: String,
    reason: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let sow_id = config.get_sow(None)?;

    let blocker_ref = Reference::parse(&blocker);
    let blocker_id = blocker_ref.resolve(client, &sow_id).await?;

    let target_ref = Reference::parse(&target);
    let target_id = target_ref.resolve(client, &sow_id).await?;

    let context = serde_json::json!({
        "target_id": target_id,
        "reason": reason,
    });

    let result = client.apply_verb(&sow_id, &blocker_id, "Blocked", Some(context)).await?;

    if let OutputFormat::Json = format {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        output::format_success(&format!("{} is now blocking {}", blocker, target));
    }

    Ok(())
}

/// Release blocking relationship
pub async fn release(
    client: &LevelClient,
    config: &CliConfig,
    blocker: String,
    target: String,
    format: OutputFormat,
) -> Result<()> {
    let sow_id = config.get_sow(None)?;

    let blocker_ref = Reference::parse(&blocker);
    let blocker_id = blocker_ref.resolve(client, &sow_id).await?;

    let target_ref = Reference::parse(&target);
    let target_id = target_ref.resolve(client, &sow_id).await?;

    let context = serde_json::json!({
        "target_id": target_id,
    });

    let result = client.apply_verb(&sow_id, &blocker_id, "Release", Some(context)).await?;

    if let OutputFormat::Json = format {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        output::format_success(&format!("{} released from blocking {}", blocker, target));
    }

    Ok(())
}

/// Assign noun to container
pub async fn assign(
    client: &LevelClient,
    config: &CliConfig,
    noun: String,
    container: String,
    order: Option<i32>,
    format: OutputFormat,
) -> Result<()> {
    let sow_id = config.get_sow(None)?;

    let noun_ref = Reference::parse(&noun);
    let noun_id = noun_ref.resolve(client, &sow_id).await?;

    let container_ref = Reference::parse(&container);
    let container_id = container_ref.resolve(client, &sow_id).await?;

    let context = serde_json::json!({
        "container_id": container_id,
        "sort_order": order.unwrap_or(0),
    });

    let result = client.apply_verb(&sow_id, &noun_id, "Assign", Some(context)).await?;

    if let OutputFormat::Json = format {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        output::format_success(&format!("{} assigned to {}", noun, container));
    }

    Ok(())
}

/// Unassign noun from container
pub async fn unassign(
    client: &LevelClient,
    config: &CliConfig,
    noun: String,
    container: String,
    format: OutputFormat,
) -> Result<()> {
    let sow_id = config.get_sow(None)?;

    let noun_ref = Reference::parse(&noun);
    let noun_id = noun_ref.resolve(client, &sow_id).await?;

    let container_ref = Reference::parse(&container);
    let container_id = container_ref.resolve(client, &sow_id).await?;

    let context = serde_json::json!({
        "container_id": container_id,
    });

    let result = client.apply_verb(&sow_id, &noun_id, "Unassign", Some(context)).await?;

    if let OutputFormat::Json = format {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        output::format_success(&format!("{} unassigned from {}", noun, container));
    }

    Ok(())
}

/// Reparent noun
pub async fn reparent(
    client: &LevelClient,
    config: &CliConfig,
    noun: String,
    parent: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let sow_id = config.get_sow(None)?;

    let noun_ref = Reference::parse(&noun);
    let noun_id = noun_ref.resolve(client, &sow_id).await?;

    let parent_id = if let Some(p) = &parent {
        let parent_ref = Reference::parse(p);
        Some(parent_ref.resolve(client, &sow_id).await?)
    } else {
        None
    };

    let context = serde_json::json!({
        "new_parent_id": parent_id,
    });

    let result = client.apply_verb(&sow_id, &noun_id, "Reparent", Some(context)).await?;

    if let OutputFormat::Json = format {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        match parent {
            Some(p) => output::format_success(&format!("{} moved under {}", noun, p)),
            None => output::format_success(&format!("{} moved to SOW root", noun)),
        }
    }

    Ok(())
}

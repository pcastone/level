//! Noun command handlers

use anyhow::Result;
use colored::Colorize;

use crate::client::LevelClient;
use crate::config::CliConfig;
use crate::output::{self, OutputFormat};
use crate::reference::Reference;
use crate::NounAction;

/// List nouns in SOW
pub async fn list_nouns(
    client: &LevelClient,
    config: &CliConfig,
    noun_type: Option<String>,
    state: Option<String>,
    blocked: bool,
    format: OutputFormat,
) -> Result<()> {
    let sow_id = config.get_sow(None)?;

    let response = client.list_nouns(
        &sow_id,
        noun_type.as_deref(),
        state.as_deref(),
        if blocked { Some(true) } else { None },
    ).await?;

    if let Some(nouns) = response.get("nouns").and_then(|v| v.as_array()) {
        output::format_nouns(nouns, format);
    }

    Ok(())
}

/// Handle noun subcommands
pub async fn handle_noun(
    client: &LevelClient,
    config: &CliConfig,
    action: NounAction,
    format: OutputFormat,
) -> Result<()> {
    let sow_id = config.get_sow(None)?;

    match action {
        NounAction::Create { noun_type, title, description, parent, due } => {
            let parent_id = if let Some(p) = parent {
                let reference = Reference::parse(&p);
                Some(reference.resolve(client, &sow_id).await?)
            } else {
                None
            };

            let noun = client.create_noun(
                &sow_id,
                &noun_type,
                &title,
                description.as_deref(),
                parent_id.as_deref(),
                due.as_deref(),
            ).await?;

            output::format_noun_detail(&noun, format);
            let short_name = noun.get("short_name").and_then(|v| v.as_str()).unwrap_or("unknown");
            output::format_success(&format!("Created noun: {}", short_name));
        }
        NounAction::Show { reference } => {
            let ref_parsed = Reference::parse(&reference);
            let noun_id = ref_parsed.resolve(client, &sow_id).await?;
            let noun = client.get_noun(&sow_id, &noun_id).await?;
            output::format_noun_detail(&noun, format);
        }
        NounAction::Update { reference, updates } => {
            let ref_parsed = Reference::parse(&reference);
            let noun_id = ref_parsed.resolve(client, &sow_id).await?;

            // Parse field=value pairs
            let mut update_obj = serde_json::Map::new();
            for update in updates {
                if let Some((key, value)) = update.split_once('=') {
                    update_obj.insert(key.to_string(), serde_json::Value::String(value.to_string()));
                }
            }

            let noun = client.update_noun(&sow_id, &noun_id, serde_json::Value::Object(update_obj)).await?;
            output::format_noun_detail(&noun, format);
            output::format_success("Noun updated");
        }
        NounAction::Delete { reference } => {
            let ref_parsed = Reference::parse(&reference);
            let noun_id = ref_parsed.resolve(client, &sow_id).await?;
            client.delete_noun(&sow_id, &noun_id).await?;
            output::format_success(&format!("Deleted noun: {}", reference));
        }
        NounAction::Children { reference } => {
            let ref_parsed = Reference::parse(&reference);
            let noun_id = ref_parsed.resolve(client, &sow_id).await?;
            let children = client.get_children(&sow_id, &noun_id).await?;
            output::format_nouns(&children, format);
        }
        NounAction::History { reference } => {
            let ref_parsed = Reference::parse(&reference);
            let noun_id = ref_parsed.resolve(client, &sow_id).await?;
            let transactions = client.get_transactions(&sow_id, &noun_id).await?;

            if let OutputFormat::Json = format {
                println!("{}", serde_json::to_string_pretty(&transactions)?);
            } else {
                println!("{}", "Transaction History".bold());
                if let Some(txns) = transactions.get("transactions").and_then(|v| v.as_array()) {
                    for txn in txns {
                        let verb = txn.get("verb").and_then(|v| v.as_str()).unwrap_or("-");
                        let actor = txn.get("actor").and_then(|v| v.as_str()).unwrap_or("-");
                        let created = txn.get("created_at").and_then(|v| v.as_str()).unwrap_or("-");
                        println!("  {} by {} at {}", verb.bold(), actor, created.dimmed());
                    }
                }
            }
        }
    }
    Ok(())
}

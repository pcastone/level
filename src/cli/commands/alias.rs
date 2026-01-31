//! Alias command handlers

use anyhow::Result;

use crate::client::LevelClient;
use crate::config::CliConfig;
use crate::output::{self, OutputFormat};
use crate::reference::Reference;
use crate::AliasAction;
use uuid;

/// List all aliases
pub async fn list_aliases(client: &LevelClient, format: OutputFormat) -> Result<()> {
    let aliases = client.list_aliases().await?;
    output::format_aliases(&aliases, format);
    Ok(())
}

/// Handle alias subcommands
pub async fn handle_alias(
    client: &LevelClient,
    action: AliasAction,
    format: OutputFormat,
) -> Result<()> {
    match action {
        AliasAction::Create { name, noun } => {
            // We need to resolve the noun reference, but we need a SOW ID for that
            // For aliases, we'll assume the noun reference is already a UUID or we use the default SOW
            let config = CliConfig::load()?;
            let sow_id = config.default_sow.as_deref().unwrap_or("");

            let noun_id = if let Ok(uuid) = uuid::Uuid::parse_str(&noun) {
                uuid.to_string()
            } else if !sow_id.is_empty() {
                let ref_parsed = Reference::parse(&noun);
                ref_parsed.resolve(client, sow_id).await?
            } else {
                anyhow::bail!("Cannot resolve noun reference without a default SOW set. Use 'level sow set-default <sow>' first.");
            };

            let alias = client.create_alias(&name, &noun_id).await?;

            if let OutputFormat::Json = format {
                println!("{}", serde_json::to_string_pretty(&alias)?);
            } else {
                output::format_success(&format!("Created alias: *{} -> {}", name, noun_id));
            }
        }
        AliasAction::Delete { name } => {
            client.delete_alias(&name).await?;
            output::format_success(&format!("Deleted alias: *{}", name));
        }
    }
    Ok(())
}

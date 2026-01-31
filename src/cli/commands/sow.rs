//! SOW command handlers

use anyhow::Result;

use crate::client::LevelClient;
use crate::output::{self, OutputFormat};
use crate::SowAction;

/// List all SOWs
pub async fn list_sows(client: &LevelClient, format: OutputFormat) -> Result<()> {
    let sows = client.list_sows().await?;
    output::format_sows(&sows, format);
    Ok(())
}

/// Handle SOW subcommands
pub async fn handle_sow(
    client: &LevelClient,
    action: SowAction,
    format: OutputFormat,
) -> Result<()> {
    match action {
        SowAction::Create { short_name, title, description } => {
            let sow = client.create_sow(&short_name, &title, description.as_deref()).await?;
            output::format_noun_detail(&sow, format);
            output::format_success(&format!("Created SOW: {}", short_name));
        }
        SowAction::Show { reference } => {
            let sow = client.get_sow(&reference).await?;
            output::format_noun_detail(&sow, format);
        }
        SowAction::SetDefault { reference } => {
            // Update local config
            let mut config = crate::config::CliConfig::load()?;
            config.set_default_sow(reference.clone())?;
            output::format_success(&format!("Default SOW set to: {}", reference));
        }
    }
    Ok(())
}

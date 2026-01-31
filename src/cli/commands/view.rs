//! View command handlers

use anyhow::Result;

use crate::client::LevelClient;
use crate::config::CliConfig;
use crate::output::{self, OutputFormat};
use crate::reference::Reference;

/// Timeline view
pub async fn timeline(
    client: &LevelClient,
    config: &CliConfig,
    start: Option<String>,
    end: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let sow_id = config.get_sow(None)?;

    let data = client.timeline(&sow_id, start.as_deref(), end.as_deref()).await?;
    output::format_timeline(&data, format);

    Ok(())
}

/// Kanban view
pub async fn kanban(
    client: &LevelClient,
    config: &CliConfig,
    container: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let sow_id = config.get_sow(None)?;

    let container_id = if let Some(c) = container {
        let ref_parsed = Reference::parse(&c);
        Some(ref_parsed.resolve(client, &sow_id).await?)
    } else {
        None
    };

    let data = client.kanban(&sow_id, container_id.as_deref()).await?;
    output::format_kanban(&data, format);

    Ok(())
}

/// Calendar view
pub async fn calendar(
    client: &LevelClient,
    config: &CliConfig,
    month: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let sow_id = config.get_sow(None)?;

    let data = client.calendar(&sow_id, month.as_deref()).await?;
    output::format_calendar(&data, format);

    Ok(())
}

//! User command handlers

use anyhow::Result;
use colored::Colorize;

use crate::client::LevelClient;
use crate::output::{self, OutputFormat};

/// Handle preferences command
pub async fn preferences(
    client: &LevelClient,
    default_sow: Option<String>,
    timezone: Option<String>,
    theme: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    // If no updates, just show current preferences
    if default_sow.is_none() && timezone.is_none() && theme.is_none() {
        let prefs = client.get_preferences().await?;

        if let OutputFormat::Json = format {
            println!("{}", serde_json::to_string_pretty(&prefs)?);
        } else {
            println!("{}", "User Preferences".bold().underline());
            println!();

            if let Some(sow) = prefs.get("default_sow").and_then(|v| v.as_str()) {
                println!("{}: {}", "Default SOW".bold(), sow);
            } else {
                println!("{}: {}", "Default SOW".bold(), "(not set)".dimmed());
            }

            println!(
                "{}: {}",
                "Timezone".bold(),
                prefs.get("timezone").and_then(|v| v.as_str()).unwrap_or("UTC")
            );
            println!(
                "{}: {}",
                "Theme".bold(),
                prefs.get("theme").and_then(|v| v.as_str()).unwrap_or("light")
            );
            println!(
                "{}: {}",
                "Date Format".bold(),
                prefs.get("date_format").and_then(|v| v.as_str()).unwrap_or("YYYY-MM-DD")
            );
        }

        return Ok(());
    }

    // Build update object
    let mut updates = serde_json::Map::new();

    if let Some(sow) = default_sow {
        updates.insert("default_sow".to_string(), serde_json::Value::String(sow));
    }
    if let Some(tz) = timezone {
        updates.insert("timezone".to_string(), serde_json::Value::String(tz));
    }
    if let Some(th) = theme {
        updates.insert("theme".to_string(), serde_json::Value::String(th));
    }

    let prefs = client
        .update_preferences(serde_json::Value::Object(updates))
        .await?;

    if let OutputFormat::Json = format {
        println!("{}", serde_json::to_string_pretty(&prefs)?);
    } else {
        output::format_success("Preferences updated");
    }

    Ok(())
}

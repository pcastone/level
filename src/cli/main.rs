//! Level CLI - Command-line interface for Level project management

mod client;
mod commands;
mod config;
mod output;
mod reference;

use clap::{Parser, Subcommand};
use colored::Colorize;

use client::LevelClient;
use config::CliConfig;

#[derive(Parser)]
#[command(name = "level")]
#[command(author = "Level Team")]
#[command(version = "0.1.0")]
#[command(about = "Project management CLI with Noun/Verb/Container grammar")]
#[command(propagate_version = true)]
struct Cli {
    /// API server URL (overrides config)
    #[arg(long, env = "LEVEL_API_URL")]
    api_url: Option<String>,

    /// API key for authentication
    #[arg(long, env = "LEVEL_API_KEY")]
    api_key: Option<String>,

    /// Output format (table, json, minimal)
    #[arg(long, short, default_value = "table")]
    format: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all SOWs
    Sows,

    /// SOW management commands
    Sow {
        #[command(subcommand)]
        action: SowAction,
    },

    /// List nouns in current SOW
    Nouns {
        /// Filter by noun type
        #[arg(long, short = 't')]
        noun_type: Option<String>,

        /// Filter by state
        #[arg(long, short)]
        state: Option<String>,

        /// Show only blocked items
        #[arg(long, short)]
        blocked: bool,
    },

    /// Noun management commands
    Noun {
        #[command(subcommand)]
        action: NounAction,
    },

    // Verb commands
    /// Mark noun(s) as completed
    Complete {
        /// Noun reference(s) to complete
        #[arg(required = true)]
        references: Vec<String>,
    },

    /// Mark noun(s) as incompleted
    Incomplete {
        /// Noun reference(s) to mark incomplete
        #[arg(required = true)]
        references: Vec<String>,
    },

    /// Escalate noun(s)
    Escalate {
        /// Noun reference(s) to escalate
        #[arg(required = true)]
        references: Vec<String>,
    },

    /// Reset noun(s) to normal state
    Normal {
        /// Noun reference(s) to normalize
        #[arg(required = true)]
        references: Vec<String>,
    },

    /// Close noun(s)
    Close {
        /// Noun reference(s) to close
        #[arg(required = true)]
        references: Vec<String>,
    },

    /// Create blocking relationship
    Blocked {
        /// Blocker noun reference
        blocker: String,
        /// Target noun reference
        target: String,
        /// Reason for blocking
        #[arg(long, short)]
        reason: Option<String>,
    },

    /// Release blocking relationship
    Release {
        /// Blocker noun reference
        blocker: String,
        /// Target noun reference
        target: String,
    },

    /// Assign noun to container
    Assign {
        /// Noun reference
        noun: String,
        /// Container reference
        container: String,
        /// Sort order
        #[arg(long, short)]
        order: Option<i32>,
    },

    /// Unassign noun from container
    Unassign {
        /// Noun reference
        noun: String,
        /// Container reference
        container: String,
    },

    /// Reparent noun
    Reparent {
        /// Noun reference
        noun: String,
        /// New parent reference (omit for SOW root)
        parent: Option<String>,
    },

    /// View commands
    Timeline {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start: Option<String>,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end: Option<String>,
    },

    /// Kanban view
    Kanban {
        /// Container reference
        #[arg(long, short)]
        container: Option<String>,
    },

    /// Calendar view
    Calendar {
        /// Month (YYYY-MM)
        #[arg(long, short)]
        month: Option<String>,
    },

    /// Alias management
    Aliases,

    /// Create or manage alias
    Alias {
        #[command(subcommand)]
        action: AliasAction,
    },

    /// Documentation for nouns, verbs, containers, roles
    #[command(name = "docs")]
    Docs {
        #[command(subcommand)]
        topic: HelpTopic,
    },

    /// Show or update user preferences
    Preferences {
        /// Set default SOW
        #[arg(long)]
        default_sow: Option<String>,
        /// Set timezone
        #[arg(long)]
        timezone: Option<String>,
        /// Set theme (light/dark)
        #[arg(long)]
        theme: Option<String>,
    },
}

#[derive(Subcommand)]
enum SowAction {
    /// Create a new SOW
    Create {
        /// Short name (unique identifier)
        short_name: String,
        /// Title
        title: String,
        /// Description
        #[arg(long, short)]
        description: Option<String>,
    },
    /// Show SOW details
    Show {
        /// SOW reference
        reference: String,
    },
    /// Set default SOW
    SetDefault {
        /// SOW reference
        reference: String,
    },
}

#[derive(Subcommand)]
enum NounAction {
    /// Create a new noun
    Create {
        /// Noun type (task, item, request, etc.)
        #[arg(long, short = 't')]
        noun_type: String,
        /// Title
        title: String,
        /// Description
        #[arg(long, short)]
        description: Option<String>,
        /// Parent noun reference
        #[arg(long, short)]
        parent: Option<String>,
        /// Due date (YYYY-MM-DD)
        #[arg(long)]
        due: Option<String>,
    },
    /// Show noun details
    Show {
        /// Noun reference
        reference: String,
    },
    /// Update noun fields
    Update {
        /// Noun reference
        reference: String,
        /// Field updates (field=value)
        #[arg(required = true)]
        updates: Vec<String>,
    },
    /// Delete noun
    Delete {
        /// Noun reference
        reference: String,
    },
    /// Show noun children
    Children {
        /// Noun reference
        reference: String,
    },
    /// Show noun history
    History {
        /// Noun reference
        reference: String,
    },
}

#[derive(Subcommand)]
enum AliasAction {
    /// Create an alias
    Create {
        /// Alias name
        name: String,
        /// Noun reference
        noun: String,
    },
    /// Delete an alias
    Delete {
        /// Alias name
        name: String,
    },
}

#[derive(Subcommand)]
enum HelpTopic {
    /// Help for noun types
    Nouns {
        /// Specific noun type
        noun_type: Option<String>,
    },
    /// Help for verbs
    Verbs {
        /// Specific verb
        verb: Option<String>,
    },
    /// Help for container types
    Containers,
    /// Help for actor roles
    Roles,
    /// Help for noun states
    States,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Load configuration
    let config = match CliConfig::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}: Failed to load config: {}", "Error".red(), e);
            std::process::exit(1);
        }
    };

    // Override config with CLI args
    let api_url = cli.api_url.unwrap_or(config.api_url.clone());
    let api_key = cli.api_key.or(config.api_key.clone());

    // Create client
    let client = match LevelClient::new(&api_url, api_key.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}: Failed to create client: {}", "Error".red(), e);
            std::process::exit(1);
        }
    };

    let output_format = output::OutputFormat::from_str(&cli.format);

    // Execute command
    let result = match cli.command {
        Commands::Sows => commands::sow::list_sows(&client, output_format).await,
        Commands::Sow { action } => commands::sow::handle_sow(&client, action, output_format).await,
        Commands::Nouns { noun_type, state, blocked } => {
            commands::noun::list_nouns(&client, &config, noun_type, state, blocked, output_format).await
        }
        Commands::Noun { action } => commands::noun::handle_noun(&client, &config, action, output_format).await,
        Commands::Complete { references } => {
            commands::verb::complete(&client, &config, references, output_format).await
        }
        Commands::Incomplete { references } => {
            commands::verb::incomplete(&client, &config, references, output_format).await
        }
        Commands::Escalate { references } => {
            commands::verb::escalate(&client, &config, references, output_format).await
        }
        Commands::Normal { references } => {
            commands::verb::normal(&client, &config, references, output_format).await
        }
        Commands::Close { references } => {
            commands::verb::close(&client, &config, references, output_format).await
        }
        Commands::Blocked { blocker, target, reason } => {
            commands::verb::blocked(&client, &config, blocker, target, reason, output_format).await
        }
        Commands::Release { blocker, target } => {
            commands::verb::release(&client, &config, blocker, target, output_format).await
        }
        Commands::Assign { noun, container, order } => {
            commands::verb::assign(&client, &config, noun, container, order, output_format).await
        }
        Commands::Unassign { noun, container } => {
            commands::verb::unassign(&client, &config, noun, container, output_format).await
        }
        Commands::Reparent { noun, parent } => {
            commands::verb::reparent(&client, &config, noun, parent, output_format).await
        }
        Commands::Timeline { start, end } => {
            commands::view::timeline(&client, &config, start, end, output_format).await
        }
        Commands::Kanban { container } => {
            commands::view::kanban(&client, &config, container, output_format).await
        }
        Commands::Calendar { month } => {
            commands::view::calendar(&client, &config, month, output_format).await
        }
        Commands::Aliases => commands::alias::list_aliases(&client, output_format).await,
        Commands::Alias { action } => commands::alias::handle_alias(&client, action, output_format).await,
        Commands::Docs { topic } => commands::help::handle_help(&client, topic, output_format).await,
        Commands::Preferences { default_sow, timezone, theme } => {
            commands::user::preferences(&client, default_sow, timezone, theme, output_format).await
        }
    };

    if let Err(e) = result {
        eprintln!("{}: {}", "Error".red(), e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parses() {
        // Verify CLI structure is valid
        Cli::command().debug_assert();
    }

    #[test]
    fn test_parse_sows_command() {
        let cli = Cli::try_parse_from(["level", "sows"]).unwrap();
        assert!(matches!(cli.command, Commands::Sows));
    }

    #[test]
    fn test_parse_sow_create() {
        let cli = Cli::try_parse_from(["level", "sow", "create", "IT", "IT Projects"]).unwrap();
        if let Commands::Sow { action: SowAction::Create { short_name, title, description } } = cli.command {
            assert_eq!(short_name, "IT");
            assert_eq!(title, "IT Projects");
            assert!(description.is_none());
        } else {
            panic!("Expected Sow Create command");
        }
    }

    #[test]
    fn test_parse_sow_show() {
        let cli = Cli::try_parse_from(["level", "sow", "show", "IT"]).unwrap();
        if let Commands::Sow { action: SowAction::Show { reference } } = cli.command {
            assert_eq!(reference, "IT");
        } else {
            panic!("Expected Sow Show command");
        }
    }

    #[test]
    fn test_parse_nouns_with_filters() {
        let cli = Cli::try_parse_from(["level", "nouns", "-t", "task", "-s", "normal", "-b"]).unwrap();
        if let Commands::Nouns { noun_type, state, blocked } = cli.command {
            assert_eq!(noun_type, Some("task".to_string()));
            assert_eq!(state, Some("normal".to_string()));
            assert!(blocked);
        } else {
            panic!("Expected Nouns command");
        }
    }

    #[test]
    fn test_parse_noun_create() {
        let cli = Cli::try_parse_from([
            "level", "noun", "create",
            "-t", "task",
            "Build feature",
            "-d", "Build the new feature",
            "--due", "2024-12-31"
        ]).unwrap();
        if let Commands::Noun { action: NounAction::Create { noun_type, title, description, parent, due } } = cli.command {
            assert_eq!(noun_type, "task");
            assert_eq!(title, "Build feature");
            assert_eq!(description, Some("Build the new feature".to_string()));
            assert!(parent.is_none());
            assert_eq!(due, Some("2024-12-31".to_string()));
        } else {
            panic!("Expected Noun Create command");
        }
    }

    #[test]
    fn test_parse_complete_multiple() {
        let cli = Cli::try_parse_from(["level", "complete", "IT-TASK-001", "IT-TASK-002", "IT-TASK-003"]).unwrap();
        if let Commands::Complete { references } = cli.command {
            assert_eq!(references.len(), 3);
            assert_eq!(references[0], "IT-TASK-001");
            assert_eq!(references[1], "IT-TASK-002");
            assert_eq!(references[2], "IT-TASK-003");
        } else {
            panic!("Expected Complete command");
        }
    }

    #[test]
    fn test_parse_blocked_command() {
        let cli = Cli::try_parse_from([
            "level", "blocked", "IT-BLOCK-001", "IT-TASK-001",
            "-r", "Waiting for approval"
        ]).unwrap();
        if let Commands::Blocked { blocker, target, reason } = cli.command {
            assert_eq!(blocker, "IT-BLOCK-001");
            assert_eq!(target, "IT-TASK-001");
            assert_eq!(reason, Some("Waiting for approval".to_string()));
        } else {
            panic!("Expected Blocked command");
        }
    }

    #[test]
    fn test_parse_assign_command() {
        let cli = Cli::try_parse_from(["level", "assign", "IT-TASK-001", "IT-PROJ-001", "-o", "5"]).unwrap();
        if let Commands::Assign { noun, container, order } = cli.command {
            assert_eq!(noun, "IT-TASK-001");
            assert_eq!(container, "IT-PROJ-001");
            assert_eq!(order, Some(5));
        } else {
            panic!("Expected Assign command");
        }
    }

    #[test]
    fn test_parse_reparent_with_parent() {
        let cli = Cli::try_parse_from(["level", "reparent", "IT-TASK-001", "IT-TASK-002"]).unwrap();
        if let Commands::Reparent { noun, parent } = cli.command {
            assert_eq!(noun, "IT-TASK-001");
            assert_eq!(parent, Some("IT-TASK-002".to_string()));
        } else {
            panic!("Expected Reparent command");
        }
    }

    #[test]
    fn test_parse_reparent_to_root() {
        let cli = Cli::try_parse_from(["level", "reparent", "IT-TASK-001"]).unwrap();
        if let Commands::Reparent { noun, parent } = cli.command {
            assert_eq!(noun, "IT-TASK-001");
            assert!(parent.is_none());
        } else {
            panic!("Expected Reparent command");
        }
    }

    #[test]
    fn test_parse_timeline_view() {
        let cli = Cli::try_parse_from(["level", "timeline", "--start", "2024-01-01", "--end", "2024-12-31"]).unwrap();
        if let Commands::Timeline { start, end } = cli.command {
            assert_eq!(start, Some("2024-01-01".to_string()));
            assert_eq!(end, Some("2024-12-31".to_string()));
        } else {
            panic!("Expected Timeline command");
        }
    }

    #[test]
    fn test_parse_alias_create() {
        let cli = Cli::try_parse_from(["level", "alias", "create", "budget", "IT-DELIV-001"]).unwrap();
        if let Commands::Alias { action: AliasAction::Create { name, noun } } = cli.command {
            assert_eq!(name, "budget");
            assert_eq!(noun, "IT-DELIV-001");
        } else {
            panic!("Expected Alias Create command");
        }
    }

    #[test]
    fn test_parse_docs_nouns() {
        let cli = Cli::try_parse_from(["level", "docs", "nouns", "task"]).unwrap();
        if let Commands::Docs { topic: HelpTopic::Nouns { noun_type } } = cli.command {
            assert_eq!(noun_type, Some("task".to_string()));
        } else {
            panic!("Expected Docs Nouns command");
        }
    }

    #[test]
    fn test_parse_with_global_options() {
        let cli = Cli::try_parse_from([
            "level",
            "--api-url", "http://custom:8080",
            "--api-key", "secret123",
            "--format", "json",
            "sows"
        ]).unwrap();
        assert_eq!(cli.api_url, Some("http://custom:8080".to_string()));
        assert_eq!(cli.api_key, Some("secret123".to_string()));
        assert_eq!(cli.format, "json");
    }

    #[test]
    fn test_parse_preferences() {
        let cli = Cli::try_parse_from([
            "level", "preferences",
            "--default-sow", "IT",
            "--timezone", "America/New_York",
            "--theme", "dark"
        ]).unwrap();
        if let Commands::Preferences { default_sow, timezone, theme } = cli.command {
            assert_eq!(default_sow, Some("IT".to_string()));
            assert_eq!(timezone, Some("America/New_York".to_string()));
            assert_eq!(theme, Some("dark".to_string()));
        } else {
            panic!("Expected Preferences command");
        }
    }
}

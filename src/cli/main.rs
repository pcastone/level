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

    /// Help for nouns, verbs, containers, roles
    Help {
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
        Commands::Help { topic } => commands::help::handle_help(&client, topic, output_format).await,
        Commands::Preferences { default_sow, timezone, theme } => {
            commands::user::preferences(&client, default_sow, timezone, theme, output_format).await
        }
    };

    if let Err(e) = result {
        eprintln!("{}: {}", "Error".red(), e);
        std::process::exit(1);
    }
}

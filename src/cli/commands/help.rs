//! Help command handlers

use anyhow::Result;
use colored::Colorize;

use crate::client::LevelClient;
use crate::output::{self, OutputFormat};
use crate::HelpTopic;

/// Handle help subcommands
pub async fn handle_help(
    client: &LevelClient,
    topic: HelpTopic,
    format: OutputFormat,
) -> Result<()> {
    match topic {
        HelpTopic::Nouns { noun_type } => {
            let data = client.help_nouns(noun_type.as_deref()).await?;

            if let OutputFormat::Json = format {
                println!("{}", serde_json::to_string_pretty(&data)?);
            } else if noun_type.is_some() {
                // Single noun type
                format_noun_help(&data);
            } else {
                // All noun types
                println!("{}", "Noun Types".bold().underline());
                println!();
                if let Some(nouns) = data.as_array() {
                    for noun in nouns {
                        format_noun_help(noun);
                    }
                }
            }
        }
        HelpTopic::Verbs { verb } => {
            let data = client.help_verbs(verb.as_deref()).await?;

            if let OutputFormat::Json = format {
                println!("{}", serde_json::to_string_pretty(&data)?);
            } else if verb.is_some() {
                format_verb_help(&data);
            } else {
                println!("{}", "Verbs".bold().underline());
                println!();
                if let Some(verbs) = data.as_array() {
                    for v in verbs {
                        format_verb_help(v);
                    }
                }
            }
        }
        HelpTopic::Containers => {
            let data = client.help_containers().await?;

            if let OutputFormat::Json = format {
                println!("{}", serde_json::to_string_pretty(&data)?);
            } else {
                println!("{}", "Container Types".bold().underline());
                println!();
                if let Some(containers) = data.as_array() {
                    for c in containers {
                        let ctype = c.get("type").and_then(|v| v.as_str()).unwrap_or("-");
                        let desc = c.get("description").and_then(|v| v.as_str()).unwrap_or("-");
                        println!("{}", ctype.bold());
                        println!("  {}", desc);
                        if let Some(features) = c.get("features").and_then(|v| v.as_array()) {
                            for f in features {
                                if let Some(s) = f.as_str() {
                                    println!("  {} {}", "•".dimmed(), s);
                                }
                            }
                        }
                        println!();
                    }
                }
            }
        }
        HelpTopic::Roles => {
            let data = client.help_roles().await?;

            if let OutputFormat::Json = format {
                println!("{}", serde_json::to_string_pretty(&data)?);
            } else {
                println!("{}", "Actor Roles".bold().underline());
                println!();
                if let Some(roles) = data.as_array() {
                    for r in roles {
                        let role = r.get("role").and_then(|v| v.as_str()).unwrap_or("-");
                        let desc = r.get("description").and_then(|v| v.as_str()).unwrap_or("-");
                        println!("{}", role.bold());
                        println!("  {}", desc);
                        if let Some(verbs) = r.get("default_verbs").and_then(|v| v.as_array()) {
                            let verb_list: Vec<&str> = verbs.iter().filter_map(|v| v.as_str()).collect();
                            println!("  {}: {}", "Verbs".dimmed(), verb_list.join(", "));
                        }
                        println!();
                    }
                }
            }
        }
        HelpTopic::States => {
            let data = client.help_states().await?;

            if let OutputFormat::Json = format {
                println!("{}", serde_json::to_string_pretty(&data)?);
            } else {
                println!("{}", "Noun States".bold().underline());
                println!();
                if let Some(states) = data.as_array() {
                    for s in states {
                        let state = s.get("state").and_then(|v| v.as_str()).unwrap_or("-");
                        let desc = s.get("description").and_then(|v| v.as_str()).unwrap_or("-");
                        let terminal = s.get("is_terminal").and_then(|v| v.as_bool()).unwrap_or(false);

                        print!("{}", state.bold());
                        if terminal {
                            print!(" {}", "(terminal)".dimmed());
                        }
                        println!();
                        println!("  {}", desc);
                        println!();
                    }
                }
            }
        }
    }
    Ok(())
}

fn format_noun_help(noun: &serde_json::Value) {
    let name = noun.get("name").and_then(|v| v.as_str()).unwrap_or("-");
    let abbrev = noun.get("abbreviation").and_then(|v| v.as_str()).unwrap_or("-");
    let desc = noun.get("description").and_then(|v| v.as_str()).unwrap_or("-");
    let is_container = noun.get("is_container").and_then(|v| v.as_bool()).unwrap_or(false);
    let is_leaf = noun.get("is_leaf_only").and_then(|v| v.as_bool()).unwrap_or(false);
    let can_be_goal = noun.get("can_be_goal").and_then(|v| v.as_bool()).unwrap_or(false);

    print!("{} ", name.bold());
    print!("{}", format!("({})", abbrev).dimmed());
    if is_container {
        print!(" {}", "[container]".cyan());
    }
    if is_leaf {
        print!(" {}", "[leaf-only]".yellow());
    }
    if can_be_goal {
        print!(" {}", "[goal]".green());
    }
    println!();
    println!("  {}", desc);
    println!();
}

fn format_verb_help(verb: &serde_json::Value) {
    let name = verb.get("name").and_then(|v| v.as_str()).unwrap_or("-");
    let desc = verb.get("description").and_then(|v| v.as_str()).unwrap_or("-");
    let changes_state = verb.get("changes_state").and_then(|v| v.as_bool()).unwrap_or(false);
    let modifies_rels = verb.get("modifies_relationships").and_then(|v| v.as_bool()).unwrap_or(false);

    print!("{}", name.bold());
    if changes_state {
        print!(" {}", "[changes state]".cyan());
    }
    if modifies_rels {
        print!(" {}", "[modifies relationships]".yellow());
    }
    println!();
    println!("  {}", desc);

    if let Some(states) = verb.get("required_states").and_then(|v| v.as_array()) {
        if !states.is_empty() {
            let state_list: Vec<&str> = states.iter().filter_map(|v| v.as_str()).collect();
            println!("  {}: {}", "Requires".dimmed(), state_list.join(", "));
        }
    }

    if let Some(effects) = verb.get("side_effects").and_then(|v| v.as_array()) {
        if !effects.is_empty() {
            println!("  {}:", "Side Effects".dimmed());
            for e in effects {
                if let Some(s) = e.as_str() {
                    println!("    {} {}", "•".dimmed(), s);
                }
            }
        }
    }

    println!();
}

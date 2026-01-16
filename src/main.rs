use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use hostctl::config::{Config, Environment, HostEntry};
use hostctl::hosts::HostsManager;
use hostctl::storage::ConfigStorage;
use std::net::IpAddr;

/// hostctl - A command-line tool for managing hosts files
///
/// Allows users to create different environment configurations and switch between them quickly.
#[derive(Parser)]
#[command(name = "hostctl")]
#[command(about = "Manage hosts file with different environments", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available commands
#[derive(Subcommand)]
enum Commands {
    /// List all environments
    List,
    /// Show current environment
    Current,
    /// Switch to specified environment
    Switch {
        /// Environment name
        name: String,
    },
    /// Show details of specified environment
    Show {
        /// Environment name
        name: String,
    },
    /// Create new environment
    Add {
        /// Environment name
        name: String,
        /// Environment description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Remove environment
    Remove {
        /// Environment name
        name: String,
    },
    /// Add hosts entry to environment
    AddEntry {
        /// Environment name
        environment: String,
        /// IP address
        ip: String,
        /// Hostname
        hostname: String,
        /// Comment
        #[arg(short, long)]
        comment: Option<String>,
    },
    /// Remove hosts entry from environment
    RemoveEntry {
        /// Environment name
        environment: String,
        /// Hostname
        hostname: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => list_environments(),
        Commands::Current => show_current_environment(),
        Commands::Switch { name } => switch_environment(&name),
        Commands::Show { name } => show_environment(&name),
        Commands::Add { name, description } => add_environment(&name, description),
        Commands::Remove { name } => remove_environment(&name),
        Commands::AddEntry {
            environment,
            ip,
            hostname,
            comment,
        } => add_entry(&environment, &ip, &hostname, comment),
        Commands::RemoveEntry {
            environment,
            hostname,
        } => remove_entry(&environment, &hostname),
    }
}

/// List all environments
fn list_environments() -> Result<()> {
    let config: Config = ConfigStorage::load_config()?;

    if config.environments.is_empty() {
        println!("No environments configured.");
        return Ok(());
    }

    println!("Environments:");
    for (name, env) in &config.environments {
        let current = if config.current_environment.as_ref() == Some(name) {
            " (current)"
        } else {
            ""
        };
        println!("  - {}{}: {} entries", name, current, env.entries.len());
    }

    Ok(())
}

/// Show current environment
fn show_current_environment() -> Result<()> {
    let config = ConfigStorage::load_config()?;

    match &config.current_environment {
        Some(name) => {
            if let Some(env) = config.get_environment(name) {
                println!("Current environment: {name}");
                if let Some(desc) = &env.description {
                    println!("Description: {desc}");
                }
                println!("Entries:");
                for entry in &env.entries {
                    println!("  {}", entry.to_line());
                }
            } else {
                println!("Current environment '{name}' not found.");
            }
        }
        None => {
            println!("No environment is currently active.");
        }
    }

    Ok(())
}

/// Switch to specified environment
fn switch_environment(name: &str) -> Result<()> {
    let mut config = ConfigStorage::load_config()?;

    if let Some(env) = config.get_environment(name) {
        // Verify all entries in the environment
        for entry in &env.entries {
            if !HostsManager::is_valid_hostname(&entry.hostname) {
                anyhow::bail!(
                    "Invalid hostname in environment '{name}': {}",
                    entry.hostname
                );
            }
        }

        // Apply environment
        HostsManager::apply_environment(env)?;
        config.current_environment = Some(name.to_string());
        ConfigStorage::save_config(&config)?;

        println!("Switched to environment: {name}");
    } else {
        anyhow::bail!("Environment '{name}' not found.");
    }

    Ok(())
}

/// Show details of specified environment
fn show_environment(name: &str) -> Result<()> {
    let config = ConfigStorage::load_config()?;

    if let Some(env) = config.get_environment(name) {
        println!("Environment: {name}");
        if let Some(desc) = &env.description {
            println!("Description: {desc}");
        }
        println!("Entries:");
        if env.entries.is_empty() {
            println!("  (no entries)");
        } else {
            for entry in &env.entries {
                println!("  {}", entry.to_line());
            }
        }
    } else {
        anyhow::bail!("Environment '{name}' not found.");
    }

    Ok(())
}

/// Create new environment
fn add_environment(name: &str, description: Option<String>) -> Result<()> {
    let mut config = ConfigStorage::load_config()?;

    // Validate environment name
    if !HostsManager::is_valid_hostname(name) {
        anyhow::bail!("Invalid environment name: {name}");
    }

    if config.get_environment(name).is_some() {
        anyhow::bail!("Environment '{name}' already exists.");
    }

    let mut env = Environment::new(name.to_string());
    if let Some(desc) = description {
        env = env.with_description(desc);
    }

    config.add_environment(env);
    ConfigStorage::save_config(&config)?;

    println!("Environment '{name}' created successfully.");
    Ok(())
}

/// Remove environment
fn remove_environment(name: &str) -> Result<()> {
    let mut config = ConfigStorage::load_config()?;

    if config.remove_environment(name) {
        ConfigStorage::save_config(&config)?;
        println!("Environment '{name}' removed successfully.");
    } else {
        anyhow::bail!("Environment '{name}' not found.");
    }

    Ok(())
}

/// Add hosts entry to environment
fn add_entry(environment: &str, ip: &str, hostname: &str, comment: Option<String>) -> Result<()> {
    let mut config = ConfigStorage::load_config()?;

    // Validate IP address
    let ip_addr: IpAddr = ip.parse().context("Invalid IP address")?;

    // Validate hostname
    if !HostsManager::is_valid_hostname(hostname) {
        anyhow::bail!("Invalid hostname: {hostname}");
    }

    if let Some(env) = config.get_environment_mut(environment) {
        let mut entry = HostEntry::new(ip_addr, hostname.to_string());
        if let Some(comment) = comment {
            entry = entry.with_comment(comment);
        }

        env.add_entry(entry);
        ConfigStorage::save_config(&config)?;

        println!("Entry added to environment '{environment}': {ip} {hostname}");
    } else {
        anyhow::bail!("Environment '{environment}' not found.");
    }

    Ok(())
}

/// Remove hosts entry from environment
fn remove_entry(environment: &str, hostname: &str) -> Result<()> {
    let mut config = ConfigStorage::load_config()?;

    if let Some(env) = config.get_environment_mut(environment) {
        if env.remove_entry(hostname) {
            ConfigStorage::save_config(&config)?;
            println!("Entry removed from environment '{environment}': {hostname}");
        } else {
            anyhow::bail!("Entry '{hostname}' not found in environment '{environment}'.");
        }
    } else {
        anyhow::bail!("Environment '{environment}' not found.");
    }

    Ok(())
}

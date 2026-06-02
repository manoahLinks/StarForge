use crate::utils::{config, print as p};
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Set a configuration parameter
    Set {
        /// Configuration key (e.g., telemetry.enabled)
        key: String,
        /// Configuration value (e.g., true/false)
        value: String,
    },
    /// Show current configuration
    Show,
}

pub fn handle(cmd: ConfigCommands) -> Result<()> {
    match cmd {
        ConfigCommands::Set { key, value } => set_config(key, value),
        ConfigCommands::Show => show_config(),
    }
}

fn set_config(key: String, value: String) -> Result<()> {
    let mut cfg = config::load()?;
    match key.as_str() {
        "telemetry.enabled" => {
            let enabled = value.parse::<bool>()
                .map_err(|_| anyhow::anyhow!("Invalid value '{}' for telemetry.enabled. Must be 'true' or 'false'.", value))?;
            cfg.telemetry_enabled = Some(enabled);
            config::save(&cfg)?;
            p::success(&format!("Configuration key 'telemetry.enabled' set to '{}'", enabled));
        }
        _ => anyhow::bail!("Unknown configuration key '{}'. Supported keys: telemetry.enabled", key),
    }
    Ok(())
}

fn show_config() -> Result<()> {
    let cfg = config::load()?;
    p::header("starforge Configuration");
    p::separator();
    p::kv("Config file", &config::config_path().display().to_string());
    p::kv("network", &cfg.network);
    p::kv("telemetry.enabled", &cfg.telemetry_enabled.unwrap_or(true).to_string());
    p::separator();
    Ok(())
}

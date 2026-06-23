use crate::commands::info;
use crate::utils::{config, horizon, print as p};
use anyhow::Result;
use colored::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

pub fn handle() -> Result<()> {
    let results = run_checks()?;
    print_report(&results);

    let failures: Vec<_> = results.iter().filter(|r| !r.passed).collect();
    if failures.is_empty() {
        p::success("All checks passed.");
        Ok(())
    } else {
        anyhow::bail!(
            "{} check(s) failed. Run `starforge config doctor` after fixing the issues above.",
            failures.len()
        )
    }
}

pub fn run_checks() -> Result<Vec<CheckResult>> {
    let mut results = Vec::new();

    results.push(check_config_file());
    results.extend(check_config_schema()?);
    results.extend(check_wallets()?);
    results.push(check_stellar_cli());
    results.extend(check_connectivity()?);

    Ok(results)
}

fn check_config_file() -> CheckResult {
    let path = config::config_path();
    if !path.exists() {
        return CheckResult {
            name: "Config file".to_string(),
            passed: true,
            detail: format!(
                "No config file at {} (using built-in defaults)",
                path.display()
            ),
        };
    }

    match std::fs::read_to_string(&path) {
        Ok(contents) => match toml::from_str::<toml::Value>(&contents) {
            Ok(_) => CheckResult {
                name: "Config file".to_string(),
                passed: true,
                detail: format!("Valid TOML at {}", path.display()),
            },
            Err(e) => CheckResult {
                name: "Config file".to_string(),
                passed: false,
                detail: format!("Invalid TOML: {}", e),
            },
        },
        Err(e) => CheckResult {
            name: "Config file".to_string(),
            passed: false,
            detail: format!("Cannot read {}: {}", path.display(), e),
        },
    }
}

fn check_config_schema() -> Result<Vec<CheckResult>> {
    let cfg = config::load()?;
    match config::validate_config(&cfg) {
        Ok(()) => Ok(vec![CheckResult {
            name: "Config schema".to_string(),
            passed: true,
            detail: format!(
                "version={}, network={}, {} wallet(s), {} network(s)",
                cfg.version,
                cfg.network,
                cfg.wallets.len(),
                cfg.networks.len()
            ),
        }]),
        Err(e) => Ok(vec![CheckResult {
            name: "Config schema".to_string(),
            passed: false,
            detail: e.to_string(),
        }]),
    }
}

fn check_wallets() -> Result<Vec<CheckResult>> {
    let cfg = config::load()?;
    let mut results = Vec::new();

    if cfg.wallets.is_empty() {
        results.push(CheckResult {
            name: "Wallets".to_string(),
            passed: true,
            detail: "No wallets configured".to_string(),
        });
        return Ok(results);
    }

    for wallet in &cfg.wallets {
        let label = format!("Wallet '{}'", wallet.name);
        if let Err(e) = config::validate_wallet_name(&wallet.name) {
            results.push(CheckResult {
                name: label,
                passed: false,
                detail: e.to_string(),
            });
            continue;
        }
        if let Err(e) = config::validate_public_key(&wallet.public_key) {
            results.push(CheckResult {
                name: label,
                passed: false,
                detail: format!("invalid public key: {}", e),
            });
            continue;
        }
        if let Some(ref secret) = wallet.secret_key {
            if let Err(e) = config::validate_secret_key(secret) {
                results.push(CheckResult {
                    name: label,
                    passed: false,
                    detail: format!("invalid secret key: {}", e),
                });
                continue;
            }
        }
        if let Err(e) = config::validate_network_exists(&cfg, &wallet.network) {
            results.push(CheckResult {
                name: label,
                passed: false,
                detail: format!("invalid network: {}", e),
            });
            continue;
        }
        results.push(CheckResult {
            name: label,
            passed: true,
            detail: format!("{} on {}", wallet.public_key, wallet.network),
        });
    }

    Ok(results)
}

fn check_stellar_cli() -> CheckResult {
    match info::detect_stellar_cli() {
        Some(path) => CheckResult {
            name: "Stellar CLI".to_string(),
            passed: true,
            detail: format!("found at {}", path.display()),
        },
        None => CheckResult {
            name: "Stellar CLI".to_string(),
            passed: false,
            detail: "not found on PATH (install stellar-cli for full functionality)".to_string(),
        },
    }
}

fn check_connectivity() -> Result<Vec<CheckResult>> {
    let cfg = config::load()?;
    let mut results = Vec::new();

    for (name, net_cfg) in &cfg.networks {
        if !should_check_network(name, &cfg.network, &net_cfg.horizon_url) {
            continue;
        }

        let horizon_label = format!("Horizon ({})", name);
        if horizon::check_horizon_endpoint(&net_cfg.horizon_url) {
            results.push(CheckResult {
                name: horizon_label,
                passed: true,
                detail: net_cfg.horizon_url.clone(),
            });
        } else {
            results.push(CheckResult {
                name: horizon_label,
                passed: false,
                detail: format!("{} is unreachable", net_cfg.horizon_url),
            });
        }

        if let Some(ref soroban_url) = net_cfg.soroban_rpc_url {
            let soroban_label = format!("Soroban RPC ({})", name);
            if horizon::check_soroban_rpc(soroban_url) {
                results.push(CheckResult {
                    name: soroban_label,
                    passed: true,
                    detail: soroban_url.clone(),
                });
            } else {
                results.push(CheckResult {
                    name: soroban_label,
                    passed: false,
                    detail: format!("{} is unreachable", soroban_url),
                });
            }
        }
    }

    Ok(results)
}

fn should_check_network(name: &str, active_network: &str, horizon_url: &str) -> bool {
    if name == active_network {
        return true;
    }

    let local = horizon_url.contains("localhost") || horizon_url.contains("127.0.0.1");
    if local {
        return false;
    }

    matches!(name, "testnet" | "mainnet")
}

fn print_report(results: &[CheckResult]) {
    p::header("starforge Config Doctor");
    p::separator();
    println!();

    for result in results {
        let status = if result.passed {
            "PASS".green().bold()
        } else {
            "FAIL".red().bold()
        };
        println!("  {}  {}", status, result.name.bright_white());
        println!("        {}", result.detail.dimmed());
    }

    println!();
    p::separator();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stellar_cli_check_returns_structured_result() {
        let result = check_stellar_cli();
        assert_eq!(result.name, "Stellar CLI");
        assert!(!result.detail.is_empty());
    }

    #[test]
    fn run_checks_returns_non_empty_results() {
        let results = run_checks().expect("run_checks should succeed");
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.name == "Config schema"));
    }

    #[test]
    fn should_skip_localhost_networks_unless_active() {
        assert!(!should_check_network(
            "docker-testnet",
            "testnet",
            "http://localhost:8000"
        ));
        assert!(should_check_network(
            "docker-testnet",
            "docker-testnet",
            "http://localhost:8000"
        ));
    }
}

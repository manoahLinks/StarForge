use crate::utils::{print as p, templates};
use anyhow::Result;
use clap::Subcommand;
use colored::*;
use dialoguer::{Confirm, Input};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum TemplateCommands {
    /// Search for templates in the marketplace
    Search {
        /// Search query (matches name, description, or tags)
        query: String,
        /// Filter by tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,
    },
    /// List all available templates
    List,
    /// Show details of a specific template
    Show {
        /// Template name
        name: String,
    },
    /// Publish a template to the local marketplace
    Publish {
        /// Path to the template directory
        path: PathBuf,
        /// Template name
        #[arg(long)]
        name: Option<String>,
        /// Template description
        #[arg(long)]
        description: Option<String>,
        /// Author name
        #[arg(long)]
        author: Option<String>,
        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,
        /// Version
        #[arg(long, default_value = "1.0.0")]
        version: String,
    },
    /// Remove a template from the local marketplace
    Remove {
        /// Template name
        name: String,
    },
    /// Initialize the template registry with example templates
    Init,
}

pub fn handle(cmd: TemplateCommands) -> Result<()> {
    match cmd {
        TemplateCommands::Publish { path } => publish(path),
        TemplateCommands::List => list(),
        TemplateCommands::Search { query } => search(query),
    }
}

fn publish(path: PathBuf) -> Result<()> {
    let template = templates::publish_template(&path)?;

    p::header("Template Publish");
    p::success("Template registered successfully");
    p::kv_accent("Name", &template.name);
    p::kv("Version", &template.version);
    p::kv("Source", &template.source);
    if !template.tags.is_empty() {
        p::kv("Tags", &template.tags.join(", "));
    }
    if let Some(path) = template.path.as_ref() {
        p::kv("Path", path);
    }

    Ok(())
}

fn list() -> Result<()> {
    let registry = templates::load_registry()?;
    p::header("Template Registry");
    if registry.templates.is_empty() {
        p::info("No templates found. Publish one with: starforge template publish <path>");
        return Ok(());
    }

    for (i, template) in registry.templates.iter().enumerate() {
        println!("  {:>2}. {}@{}", i + 1, template.name, template.version);
        p::kv("Description", &template.description);
        p::kv("Source", &template.source);
        if !template.tags.is_empty() {
            p::kv("Tags", &template.tags.join(", "));
        }
        if let Some(path) = template.path.as_ref() {
            p::kv("Path", path);
        }
        if i + 1 < registry.templates.len() {
            println!();
        }
    }

    Ok(())
}

fn search(query: String) -> Result<()> {
    let results = templates::search_templates(&query, None)?;
    p::header(&format!("Template search results for '{}'", query));
    if results.is_empty() {
        p::info("No templates matched that query.");
        return Ok(());
    }

    p::info(&format!("Found {} result(s), ranked by popularity:", results.len()));
    println!();

    for (i, template) in results.iter().enumerate() {
        let badge = p::verified_badge(template.verified);
        println!(
            "  {:>2}. {}@{}{}",
            i + 1,
            template.name.cyan().bold(),
            template.version,
            badge
        );
        p::kv("Description", &template.description);
        p::kv("Downloads", &template.downloads.to_string());
        p::kv("Source", &template.source);
        if !template.tags.is_empty() {
            p::kv("Tags", &template.tags.join(", "));
        }
        if i + 1 < results.len() {
            println!();
        }
    }

    Ok(())
}

use clap::Subcommand;
use anyhow::Result;
use colored::*;
use dialoguer::{Confirm, Select, Input};
use std::path::PathBuf;

use crate::config::{Config, get_config_dir};
use crate::project::{Project, ProjectManager};
use crate::templates::{TemplateManager, TemplateType};

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new CLI utility project
    Init {
        /// Name of the CLI utility
        name: String,
        /// Template to use (rust, python, node, go)
        #[arg(short, long)]
        template: Option<String>,
    },
    /// List all managed CLI utilities
    List,
    /// Build a CLI utility project
    Build {
        /// Name of the CLI utility to build
        name: Option<String>,
    },
    /// Remove a CLI utility project
    Remove {
        /// Name of the CLI utility to remove
        name: String,
    },
    /// Manage templates
    Template {
        #[command(subcommand)]
        action: TemplateAction,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
pub enum TemplateAction {
    /// List available templates
    List,
    /// Add a new template
    Add {
        /// Name of the template
        name: String,
        /// Path to template directory
        path: PathBuf,
    },
    /// Remove a template
    Remove {
        /// Name of the template to remove
        name: String,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    /// Reset configuration to defaults
    Reset,
}

pub fn init_project(name: String, template: Option<String>) -> Result<()> {
    println!("{} Initializing new CLI utility: {}", "✨".bright_green(), name.bright_blue());
    
    let template_manager = TemplateManager::new()?;
    let available_templates = template_manager.list_templates()?;
    
    let template_type = match template {
        Some(t) => {
            if available_templates.contains(&t) {
                t
            } else {
                println!("{} Template '{}' not found. Available templates:", "⚠️".bright_yellow(), t);
                for tmpl in &available_templates {
                    println!("  - {}", tmpl);
                }
                return Ok(());
            }
        }
        None => {
            if available_templates.is_empty() {
                println!("{} No templates available. Creating basic Rust template.", "⚠️".bright_yellow());
                "rust".to_string()
            } else {
                let selection = Select::new()
                    .with_prompt("Select a template")
                    .default(0)
                    .items(&available_templates)
                    .interact()?;
                available_templates[selection].clone()
            }
        }
    };
    
    let mut project_manager = ProjectManager::new()?;
    let project = project_manager.create_project(name.clone(), template_type)?;
    
    println!("{} Successfully created CLI utility: {}", "✅".bright_green(), name.bright_blue());
    println!("  📁 Location: {}", project.path.display());
    println!("  🔧 Template: {}", project.template);
    println!("");
    println!("Next steps:");
    println!("  1. cd {}", project.path.display());
    println!("  2. murex build {}", name);
    
    Ok(())
}

pub fn list_projects() -> Result<()> {
    let project_manager = ProjectManager::new()?;
    let projects = project_manager.list_projects()?;
    
    if projects.is_empty() {
        println!("{} No CLI utilities found.", "📋".bright_blue());
        println!("Use {} to create your first one!", "murex init <name>".bright_green());
        return Ok(());
    }
    
    println!("{} Your CLI utilities:", "📋".bright_blue());
    println!("");
    
    for project in projects {
        let status = if project.path.exists() {
            "✅ Ready".bright_green()
        } else {
            "❌ Missing".bright_red()
        };
        
        println!("  {} {}", project.name.bright_blue(), status);
        println!("    📁 {}", project.path.display().to_string().dimmed());
        println!("    🔧 Template: {}", project.template.dimmed());
        println!("");
    }
    
    Ok(())
}

pub fn build_project(name: Option<String>) -> Result<()> {
    let project_manager = ProjectManager::new()?;
    
    let project_name = match name {
        Some(n) => n,
        None => {
            // Try to detect project in current directory
            let current_dir = std::env::current_dir()?;
            if let Some(name) = current_dir.file_name().and_then(|s| s.to_str()) {
                name.to_string()
            } else {
                return Err(anyhow::anyhow!("Could not determine project name. Please specify with: murex build <name>"));
            }
        }
    };
    
    let project = project_manager.get_project(&project_name)?;
    println!("{} Building CLI utility: {}", "🔨".bright_yellow(), project_name.bright_blue());
    
    project.build()?;
    
    println!("{} Successfully built: {}", "✅".bright_green(), project_name.bright_blue());
    Ok(())
}

pub fn remove_project(name: String) -> Result<()> {
    let mut project_manager = ProjectManager::new()?;
    
    if !project_manager.project_exists(&name)? {
        println!("{} CLI utility '{}' not found.", "❌".bright_red(), name);
        return Ok(());
    }
    
    let confirm = Confirm::new()
        .with_prompt(&format!("Are you sure you want to remove '{}'?", name))
        .default(false)
        .interact()?;
        
    if !confirm {
        println!("Cancelled.");
        return Ok(());
    }
    
    project_manager.remove_project(&name)?;
    println!("{} Removed CLI utility: {}", "🗑️".bright_red(), name.bright_blue());
    
    Ok(())
}

pub fn handle_template_command(action: TemplateAction) -> Result<()> {
    let mut template_manager = TemplateManager::new()?;
    
    match action {
        TemplateAction::List => {
            let templates = template_manager.list_templates()?;
            println!("{} Available templates:", "📋".bright_blue());
            for template in templates {
                println!("  - {}", template.bright_green());
            }
        }
        TemplateAction::Add { name, path } => {
            template_manager.add_template(name.clone(), path)?;
            println!("{} Added template: {}", "✅".bright_green(), name.bright_blue());
        }
        TemplateAction::Remove { name } => {
            template_manager.remove_template(&name)?;
            println!("{} Removed template: {}", "🗑️".bright_red(), name.bright_blue());
        }
    }
    
    Ok(())
}

pub fn handle_config_command(action: ConfigAction) -> Result<()> {
    let mut config = Config::load()?;
    
    match action {
        ConfigAction::Show => {
            println!("{} Current configuration:", "⚙️".bright_blue());
            println!("  Default template: {}", config.default_template.bright_green());
            println!("  Projects directory: {}", config.projects_dir.display());
            println!("  Auto-build: {}", if config.auto_build { "enabled".bright_green() } else { "disabled".bright_red() });
        }
        ConfigAction::Set { key, value } => {
            match key.as_str() {
                "default_template" => config.default_template = value.clone(),
                "projects_dir" => config.projects_dir = PathBuf::from(&value),
                "auto_build" => config.auto_build = value.parse().unwrap_or(false),
                _ => {
                    println!("{} Unknown configuration key: {}", "❌".bright_red(), key);
                    return Ok(());
                }
            }
            config.save()?;
            println!("{} Set {} = {}", "✅".bright_green(), key.bright_blue(), value.bright_green());
        }
        ConfigAction::Reset => {
            let confirm = Confirm::new()
                .with_prompt("Reset configuration to defaults?")
                .default(false)
                .interact()?;
                
            if confirm {
                config = Config::default();
                config.save()?;
                println!("{} Configuration reset to defaults", "✅".bright_green());
            }
        }
    }
    
    Ok(())
}

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use colored::*;

use crate::config::Config;
use crate::project::Project;

pub struct PathManager {
    config: Config,
}

impl PathManager {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        Ok(Self { config })
    }
    
    /// Check if the bin directory is in the user's PATH
    pub fn is_bin_dir_in_path(&self) -> bool {
        if let Ok(path_var) = env::var("PATH") {
            let paths: Vec<&str> = path_var.split(':').collect();
            let bin_dir_str = self.config.bin_dir.to_string_lossy();
            paths.iter().any(|&p| p == bin_dir_str)
        } else {
            false
        }
    }
    
    /// Install a project's binary to the bin directory
    pub fn install_project(&self, project: &Project) -> Result<()> {
        let binary_path = self.find_project_binary(project)?;
        let bin_name = &project.name;
        let target_path = self.config.bin_dir.join(bin_name);
        
        // Remove existing link/file if it exists
        if target_path.exists() {
            fs::remove_file(&target_path)?;
        }
        
        // Create symlink on Unix systems, copy on Windows
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&binary_path, &target_path)?;
        }
        
        #[cfg(not(unix))]
        {
            fs::copy(&binary_path, &target_path)?;
        }
        
        println!("  📦 Installed {} to {}", bin_name.bright_blue(), target_path.display());
        
        // Warn if bin directory is not in PATH
        if !self.is_bin_dir_in_path() {
            self.show_path_warning();
        }
        
        Ok(())
    }
    
    /// Uninstall a project's binary from the bin directory
    pub fn uninstall_project(&self, project_name: &str) -> Result<()> {
        let target_path = self.config.bin_dir.join(project_name);
        
        if target_path.exists() {
            fs::remove_file(&target_path)?;
            println!("  🗑️  Uninstalled {} from bin directory", project_name.bright_blue());
        } else {
            println!("  ⚠️  {} not found in bin directory", project_name.bright_yellow());
        }
        
        Ok(())
    }
    
    /// List all installed binaries in the bin directory
    pub fn list_installed(&self) -> Result<Vec<String>> {
        let mut binaries = Vec::new();
        
        if !self.config.bin_dir.exists() {
            return Ok(binaries);
        }
        
        for entry in fs::read_dir(&self.config.bin_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() || entry.file_type()?.is_symlink() {
                if let Some(name) = entry.file_name().to_str() {
                    binaries.push(name.to_string());
                }
            }
        }
        
        binaries.sort();
        Ok(binaries)
    }
    
    /// Find the built binary for a project
    pub fn find_project_binary(&self, project: &Project) -> Result<PathBuf> {
        match project.template.as_str() {
            "rust" => {
                let release_path = project.path.join("target/release").join(&project.name);
                let debug_path = project.path.join("target/debug").join(&project.name);
                
                if release_path.exists() {
                    Ok(release_path)
                } else if debug_path.exists() {
                    Ok(debug_path)
                } else {
                    Err(anyhow::anyhow!("No built binary found for Rust project: {}", project.name))
                }
            }
            "go" => {
                let binary_path = project.path.join(&project.name);
                if binary_path.exists() {
                    Ok(binary_path)
                } else {
                    Err(anyhow::anyhow!("No built binary found for Go project: {}", project.name))
                }
            }
            "python" => {
                let script_path = project.path.join("main.py");
                if script_path.exists() {
                    Ok(script_path)
                } else {
                    Err(anyhow::anyhow!("No main.py found for Python project: {}", project.name))
                }
            }
            "node" => {
                let script_path = project.path.join("index.js");
                if script_path.exists() {
                    Ok(script_path)
                } else {
                    Err(anyhow::anyhow!("No index.js found for Node.js project: {}", project.name))
                }
            }
            "bash" => {
                let script_path = project.path.join("main.sh");
                if script_path.exists() {
                    Ok(script_path)
                } else {
                    Err(anyhow::anyhow!("No main.sh found for Bash project: {}", project.name))
                }
            }
            "zsh" => {
                let script_path = project.path.join("main.zsh");
                if script_path.exists() {
                    Ok(script_path)
                } else {
                    Err(anyhow::anyhow!("No main.zsh found for Zsh project: {}", project.name))
                }
            }
            "bun" => {
                let script_path = project.path.join("bun.js");
                if script_path.exists() {
                    Ok(script_path)
                } else {
                    Err(anyhow::anyhow!("No bun.js found for Bun project: {}", project.name))
                }
            }
            _ => Err(anyhow::anyhow!("Unknown template: {}", project.template)),
        }
    }
    
    /// Check if binary exists
    pub fn binary_exists(&self, project: &Project) -> bool {
        match self.find_project_binary(project) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    
    /// Show warning about PATH configuration
    pub fn show_path_warning(&self) {
        println!("");
        println!("{} The murex bin directory is not in your PATH!", "⚠️".bright_yellow());
        println!("To use your CLI utilities from anywhere, add this to your shell profile:");
        println!("");
        
        let bin_dir = self.config.bin_dir.display();
        let shell = env::var("SHELL").unwrap_or_else(|_| "bash".to_string());
        
        if shell.contains("fish") {
            println!("  {}", format!("fish_add_path {}", bin_dir).bright_green());
        } else if shell.contains("zsh") {
            println!("  {}", format!("echo 'export PATH=\"{}:$PATH\"' >> ~/.zshrc", bin_dir).bright_green());
        } else {
            println!("  {}", format!("echo 'export PATH=\"{}:$PATH\"' >> ~/.bashrc", bin_dir).bright_green());
        }
        
        println!("");
        println!("Then restart your terminal or run:");
        println!("  {}", "source ~/.bashrc  # or ~/.zshrc".bright_green());
        println!("");
    }
    
    /// Get PATH setup instructions
    pub fn get_path_instructions(&self) -> String {
        let bin_dir = self.config.bin_dir.display();
        let shell = env::var("SHELL").unwrap_or_else(|_| "bash".to_string());
        
        if shell.contains("fish") {
            format!("fish_add_path {}", bin_dir)
        } else if shell.contains("zsh") {
            format!("export PATH=\"{}:$PATH\"", bin_dir)
        } else {
            format!("export PATH=\"{}:$PATH\"", bin_dir)
        }
    }
    
    /// Check and setup PATH if needed
    pub fn check_path_setup(&self) -> Result<()> {
        if !self.is_bin_dir_in_path() {
            println!("{} Setting up PATH configuration...", "🔧".bright_blue());
            self.show_path_warning();
        } else {
            println!("{} PATH is correctly configured!", "✅".bright_green());
        }
        Ok(())
    }
}

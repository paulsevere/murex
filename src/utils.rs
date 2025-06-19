use anyhow::Result;
use std::path::Path;
use std::process::Command;
use std::env;
use colored::*;

use crate::config::Config;

/// Check if a command is available in the system PATH
pub fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Get the current working directory as a string
pub fn current_dir_string() -> Result<String> {
    let current_dir = std::env::current_dir()?;
    Ok(current_dir.display().to_string())
}

/// Check if a path is a valid project directory
pub fn is_valid_project_dir(path: &Path) -> bool {
    if !path.exists() || !path.is_dir() {
        return false;
    }
    
    // Check for common project indicators
    path.join("Cargo.toml").exists() ||
    path.join("package.json").exists() ||
    path.join("go.mod").exists() ||
    path.join("main.py").exists() ||
    path.join("pyproject.toml").exists()
}

/// Create a symbolic link or copy file based on platform
pub fn create_link_or_copy(src: &Path, dst: &Path) -> Result<()> {
    if dst.exists() {
        std::fs::remove_file(dst)?;
    }
    
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(src, dst)?;
    }
    
    #[cfg(not(unix))]
    {
        std::fs::copy(src, dst)?;
    }
    
    Ok(())
}

/// Format file size in human readable format
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Validate project name
pub fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Project name cannot be empty"));
    }
    
    if name.len() > 64 {
        return Err(anyhow::anyhow!("Project name cannot be longer than 64 characters"));
    }
    
    // Check for valid characters (alphanumeric, dash, underscore)
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(anyhow::anyhow!("Project name can only contain letters, numbers, dashes, and underscores"));
    }
    
    // Cannot start with dash or underscore
    if name.starts_with('-') || name.starts_with('_') {
        return Err(anyhow::anyhow!("Project name cannot start with dash or underscore"));
    }
    
    Ok(())
}

/// Open a project directory in the configured editor
pub fn open_project_in_editor(project_dir: &Path, config: &Config) -> Result<()> {
    // Try to determine the editor to use
    let editor = if let Some(ref configured_editor) = config.editor {
        configured_editor.clone()
    } else if let Ok(env_editor) = env::var("EDITOR") {
        env_editor
    } else if let Ok(env_visual) = env::var("VISUAL") {
        env_visual
    } else {
        // Try common editors
        if command_exists("code") {
            "code".to_string()
        } else if command_exists("vim") {
            "vim".to_string()
        } else if command_exists("nano") {
            "nano".to_string()
        } else {
            return Err(anyhow::anyhow!("No editor found. Please set EDITOR environment variable or configure with 'murex config set editor <your-editor>'"));
        }
    };
    
    println!("  📝 Opening project in {}", editor.bright_blue());
    
    // Special handling for VS Code and similar editors that can open directories
    if editor == "code" || editor == "subl" || editor == "atom" {
        Command::new(&editor)
            .arg(project_dir)
            .spawn()?;
    } else {
        // For terminal editors, open the main file instead of the directory
        let main_file = find_main_file(project_dir)?;
        Command::new(&editor)
            .arg(&main_file)
            .spawn()?;
    }
    
    Ok(())
}

/// Find the main file to open for a project
fn find_main_file(project_dir: &Path) -> Result<std::path::PathBuf> {
    // Try to find the main file based on common patterns
    let candidates = vec![
        "src/main.rs",
        "main.py", 
        "index.js",
        "bun.js",
        "main.go",
        "main.sh",
        "main.zsh",
        "README.md",
    ];
    
    for candidate in candidates {
        let file_path = project_dir.join(candidate);
        if file_path.exists() {
            return Ok(file_path);
        }
    }
    
    // If no main file found, just return the project directory
    Ok(project_dir.to_path_buf())
}

use anyhow::Result;
use std::path::Path;
use std::process::Command;

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

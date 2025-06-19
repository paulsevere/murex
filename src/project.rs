use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use colored::*;

use crate::config::{Config, get_projects_file_path};
use crate::templates::TemplateManager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub template: String,
    pub created_at: String,
    pub last_built: Option<String>,
}

impl Project {
    pub fn new(name: String, path: PathBuf, template: String) -> Self {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
        
        Self {
            name,
            path,
            template,
            created_at: now,
            last_built: None,
        }
    }
    
    pub fn build(&self) -> Result<()> {
        if !self.path.exists() {
            return Err(anyhow::anyhow!("Project directory does not exist: {}", self.path.display()));
        }
        
        match self.template.as_str() {
            "rust" => self.build_rust(),
            "python" => self.build_python(),
            "node" => self.build_node(),
            "go" => self.build_go(),
            "bash" => self.build_bash(),
            "zsh" => self.build_zsh(),
            "bun" => self.build_bun(),
            _ => Err(anyhow::anyhow!("Unknown template: {}", self.template)),
        }
    }
    
    fn build_rust(&self) -> Result<()> {
        println!("  🦀 Building Rust project...");
        let output = Command::new("cargo")
            .args(&["build", "--release"])
            .current_dir(&self.path)
            .output()?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Rust build failed:\n{}", stderr));
        }
        
        Ok(())
    }
    
    fn build_python(&self) -> Result<()> {
        println!("  🐍 Building Python project...");
        // Check for requirements.txt and install dependencies
        let requirements_path = self.path.join("requirements.txt");
        if requirements_path.exists() {
            let output = Command::new("pip")
                .args(&["install", "-r", "requirements.txt"])
                .current_dir(&self.path)
                .output()?;
                
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow::anyhow!("Python dependency installation failed:\n{}", stderr));
            }
        }
        
        // Make the main script executable
        let main_script = self.path.join("main.py");
        if main_script.exists() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&main_script)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&main_script, perms)?;
            }
        }
        
        Ok(())
    }
    
    fn build_node(&self) -> Result<()> {
        println!("  📦 Building Node.js project...");
        let package_json = self.path.join("package.json");
        if package_json.exists() {
            let output = Command::new("npm")
                .args(&["install"])
                .current_dir(&self.path)
                .output()?;
                
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow::anyhow!("npm install failed:\n{}", stderr));
            }
            
            // Try to run build script if it exists
            let output = Command::new("npm")
                .args(&["run", "build"])
                .current_dir(&self.path)
                .output();
                
            // It's okay if build script doesn't exist
            if let Ok(output) = output {
                if !output.status.success() {
                    println!("  ⚠️  Build script failed (this might be expected)");
                }
            }
        }
        
        Ok(())
    }
    
    fn build_go(&self) -> Result<()> {
        println!("  🐹 Building Go project...");
        let output = Command::new("go")
            .args(&["build", "-o", &self.name])
            .current_dir(&self.path)
            .output()?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Go build failed:\n{}", stderr));
        }
        
        Ok(())
    }
    
    fn build_bash(&self) -> Result<()> {
        println!("  🐚 Building Bash project...");
        let main_script = self.path.join("main.sh");
        if main_script.exists() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&main_script)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&main_script, perms)?;
            }
        }
        
        Ok(())
    }
    
    fn build_zsh(&self) -> Result<()> {
        println!("  🐚 Building Zsh project...");
        let main_script = self.path.join("main.zsh");
        if main_script.exists() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&main_script)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&main_script, perms)?;
            }
        }
        
        Ok(())
    }
    
    fn build_bun(&self) -> Result<()> {
        println!("  🐰 Building Bun project...");
        let output = Command::new("bun")
            .args(&["install"])
            .current_dir(&self.path)
            .output()?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Bun install failed:\n{}", stderr));
        }
        
        let output = Command::new("bun")
            .args(&["run", "start"])
            .current_dir(&self.path)
            .output()?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Bun start failed:\n{}", stderr));
        }
        
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectRegistry {
    pub projects: Vec<Project>,
}

impl ProjectRegistry {
    pub fn load() -> Result<Self> {
        let projects_file = get_projects_file_path()?;
        
        if !projects_file.exists() {
            return Ok(Self::default());
        }
        
        let content = fs::read_to_string(&projects_file)?;
        let registry: ProjectRegistry = serde_json::from_str(&content)?;
        Ok(registry)
    }
    
    pub fn save(&self) -> Result<()> {
        let projects_file = get_projects_file_path()?;
        let parent_dir = projects_file.parent().unwrap();
        
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&projects_file, content)?;
        Ok(())
    }
    
    pub fn add_project(&mut self, project: Project) {
        // Remove existing project with same name
        self.projects.retain(|p| p.name != project.name);
        self.projects.push(project);
    }
    
    pub fn remove_project(&mut self, name: &str) -> bool {
        let initial_len = self.projects.len();
        self.projects.retain(|p| p.name != name);
        self.projects.len() != initial_len
    }
    
    pub fn get_project(&self, name: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.name == name)
    }
    
    pub fn list_projects(&self) -> Vec<&Project> {
        self.projects.iter().collect()
    }
}

pub struct ProjectManager {
    registry: ProjectRegistry,
    config: Config,
}

impl ProjectManager {
    pub fn new() -> Result<Self> {
        let registry = ProjectRegistry::load()?;
        let config = Config::load()?;
        
        Ok(Self { registry, config })
    }
    
    pub fn create_project(&mut self, name: String, template: String) -> Result<Project> {
        let project_path = self.config.projects_dir.join(&name);
        
        if project_path.exists() {
            return Err(anyhow::anyhow!("Project directory already exists: {}", project_path.display()));
        }
        
        // Create project from template
        let template_manager = TemplateManager::new()?;
        template_manager.create_project_from_template(&template, &project_path, &name)?;
        
        let project = Project::new(name, project_path, template);
        self.registry.add_project(project.clone());
        self.registry.save()?;
        
        Ok(project)
    }
    
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        Ok(self.registry.list_projects().into_iter().cloned().collect())
    }
    
    pub fn get_project(&self, name: &str) -> Result<Project> {
        self.registry
            .get_project(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Project '{}' not found", name))
    }
    
    pub fn remove_project(&mut self, name: &str) -> Result<()> {
        if let Some(project) = self.registry.get_project(name) {
            if project.path.exists() {
                fs::remove_dir_all(&project.path)?;
            }
        }
        
        if !self.registry.remove_project(name) {
            return Err(anyhow::anyhow!("Project '{}' not found", name));
        }
        
        self.registry.save()?;
        Ok(())
    }
    
    pub fn project_exists(&self, name: &str) -> Result<bool> {
        Ok(self.registry.get_project(name).is_some())
    }
}

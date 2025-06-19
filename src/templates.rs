use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::config::get_config_dir;

#[derive(Debug, Clone)]
pub enum TemplateType {
    Rust,
    Python,
    Node,
    Go,
    Bash,
    Zsh,
    Bun,
    Custom(String),
}

impl From<&str> for TemplateType {
    fn from(s: &str) -> Self {
        match s {
            "rust" => TemplateType::Rust,
            "python" => TemplateType::Python,
            "node" => TemplateType::Node,
            "go" => TemplateType::Go,
            "bash" => TemplateType::Bash,
            "zsh" => TemplateType::Zsh,
            "bun" => TemplateType::Bun,
            _ => TemplateType::Custom(s.to_string()),
        }
    }
}

impl ToString for TemplateType {
    fn to_string(&self) -> String {
        match self {
            TemplateType::Rust => "rust".to_string(),
            TemplateType::Python => "python".to_string(),
            TemplateType::Node => "node".to_string(),
            TemplateType::Go => "go".to_string(),
            TemplateType::Bash => "bash".to_string(),
            TemplateType::Zsh => "zsh".to_string(),
            TemplateType::Bun => "bun".to_string(),
            TemplateType::Custom(name) => name.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomTemplate {
    pub name: String,
    pub path: PathBuf,
    pub description: Option<String>,
}

pub struct TemplateManager {
    custom_templates: HashMap<String, CustomTemplate>,
}

impl TemplateManager {
    pub fn new() -> Result<Self> {
        let mut manager = Self {
            custom_templates: HashMap::new(),
        };
        
        manager.load_custom_templates()?;
        Ok(manager)
    }
    
    pub fn list_templates(&self) -> Result<Vec<String>> {
        let mut templates = vec![
            "rust".to_string(),
            "python".to_string(),
            "node".to_string(),
            "go".to_string(),
            "bash".to_string(),
            "zsh".to_string(),
            "bun".to_string(),
        ];
        
        for name in self.custom_templates.keys() {
            templates.push(name.clone());
        }
        
        templates.sort();
        Ok(templates)
    }
    
    pub fn create_project_from_template(&self, template: &str, project_path: &Path, project_name: &str) -> Result<()> {
        fs::create_dir_all(project_path)?;
        
        match template {
            "rust" => self.create_rust_project(project_path, project_name),
            "python" => self.create_python_project(project_path, project_name),
            "node" => self.create_node_project(project_path, project_name),
            "go" => self.create_go_project(project_path, project_name),
            "bash" => self.create_bash_project(project_path, project_name),
            "zsh" => self.create_zsh_project(project_path, project_name),
            "bun" => self.create_bun_project(project_path, project_name),
            _ => {
                if let Some(custom_template) = self.custom_templates.get(template) {
                    self.create_from_custom_template(custom_template, project_path, project_name)
                } else {
                    Err(anyhow::anyhow!("Unknown template: {}", template))
                }
            }
        }
    }
    
    fn create_rust_project(&self, project_path: &Path, project_name: &str) -> Result<()> {
        // Create Cargo.toml
        let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = {{ version = "4.0", features = ["derive"] }}
anyhow = "1.0"
"#, project_name);
        
        fs::write(project_path.join("Cargo.toml"), cargo_toml)?;
        
        // Create src directory and main.rs
        let src_dir = project_path.join("src");
        fs::create_dir_all(&src_dir)?;
        
        let main_rs = format!(r#"use clap::{{Parser, Subcommand}};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "{}")]
#[command(about = "A CLI utility created with Murex")]
#[command(version = "0.1.0")]
struct Cli {{
    #[command(subcommand)]
    command: Option<Commands>,
}}

#[derive(Subcommand)]
enum Commands {{
    /// Say hello
    Hello {{
        /// Name to greet
        #[arg(short, long)]
        name: Option<String>,
    }},
}}

fn main() -> Result<()> {{
    let cli = Cli::parse();
    
    match cli.command {{
        Some(Commands::Hello {{ name }}) => {{
            let name = name.unwrap_or_else(|| "World".to_string());
            println!("Hello, {{}}!", name);
        }}
        None => {{
            println!("Welcome to {}! Use --help for more information.");
        }}
    }}
    
    Ok(())
}}
"#, project_name, project_name);
        
        fs::write(src_dir.join("main.rs"), main_rs)?;
        
        // Create README
        let readme = format!(r#"# {}

A CLI utility created with Murex.

## Installation

```bash
cargo build --release
```

## Usage

```bash
cargo run -- hello --name "Your Name"
```
"#, project_name);
        
        fs::write(project_path.join("README.md"), readme)?;
        
        Ok(())
    }
    
    fn create_python_project(&self, project_path: &Path, project_name: &str) -> Result<()> {
        // Create main.py
        let main_py = format!(r#"#!/usr/bin/env python3
"""
{} - A CLI utility created with Murex
"""

import argparse
import sys

def main():
    parser = argparse.ArgumentParser(description='A CLI utility created with Murex')
    parser.add_argument('--version', action='version', version='%(prog)s 0.1.0')
    
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # Hello command
    hello_parser = subparsers.add_parser('hello', help='Say hello')
    hello_parser.add_argument('-n', '--name', default='World', help='Name to greet')
    
    args = parser.parse_args()
    
    if args.command == 'hello':
        print(f"Hello, {{args.name}}!")
    else:
        print(f"Welcome to {}! Use --help for more information.")

if __name__ == '__main__':
    main()
"#, project_name, project_name);
        
        fs::write(project_path.join("main.py"), main_py)?;
        
        // Create requirements.txt
        let requirements = "# Add your dependencies here\n";
        fs::write(project_path.join("requirements.txt"), requirements)?;
        
        // Create README
        let readme = format!(r#"# {}

A CLI utility created with Murex.

## Installation

```bash
pip install -r requirements.txt
```

## Usage

```bash
python main.py hello --name "Your Name"
```
"#, project_name);
        
        fs::write(project_path.join("README.md"), readme)?;
        
        Ok(())
    }
    
    fn create_node_project(&self, project_path: &Path, project_name: &str) -> Result<()> {
        // Create package.json
        let package_json = format!(r#"{{
  "name": "{}",
  "version": "0.1.0",
  "description": "A CLI utility created with Murex",
  "main": "index.js",
  "bin": {{
    "{}": "./index.js"
  }},
  "scripts": {{
    "start": "node index.js"
  }},
  "dependencies": {{
    "commander": "^9.0.0"
  }},
  "keywords": ["cli"],
  "author": "",
  "license": "MIT"
}}
"#, project_name, project_name);
        
        fs::write(project_path.join("package.json"), package_json)?;
        
        // Create index.js
        let index_js = format!(r#"#!/usr/bin/env node

const {{ Command }} = require('commander');
const program = new Command();

program
  .name('{}')
  .description('A CLI utility created with Murex')
  .version('0.1.0');

program
  .command('hello')
  .description('Say hello')
  .option('-n, --name <name>', 'Name to greet', 'World')
  .action((options) => {{
    console.log(`Hello, ${{options.name}}!`);
  }});

if (process.argv.length === 2) {{
  console.log('Welcome to {}! Use --help for more information.');
}} else {{
  program.parse();
}}
"#, project_name, project_name);
        
        fs::write(project_path.join("index.js"), index_js)?;
        
        // Create README
        let readme = format!(r#"# {}

A CLI utility created with Murex.

## Installation

```bash
npm install
```

## Usage

```bash
node index.js hello --name "Your Name"
```
"#, project_name);
        
        fs::write(project_path.join("README.md"), readme)?;
        
        Ok(())
    }
    
    fn create_go_project(&self, project_path: &Path, project_name: &str) -> Result<()> {
        // Create go.mod
        let go_mod = format!(r#"module {}

go 1.19

require github.com/spf13/cobra v1.6.1

require (
	github.com/inconshreveable/mousetrap v1.0.1 // indirect
	github.com/spf13/pflag v1.0.5 // indirect
)
"#, project_name);
        
        fs::write(project_path.join("go.mod"), go_mod)?;
        
        // Create main.go
        let main_go = format!(r#"package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{{
	Use:   "{}",
	Short: "A CLI utility created with Murex",
	Long:  "A CLI utility created with Murex",
	Run: func(cmd *cobra.Command, args []string) {{
		fmt.Println("Welcome to {}! Use --help for more information.")
	}},
}}

var helloCmd = &cobra.Command{{
	Use:   "hello",
	Short: "Say hello",
	Run: func(cmd *cobra.Command, args []string) {{
		name, _ := cmd.Flags().GetString("name")
		fmt.Printf("Hello, %s!\\n", name)
	}},
}}

func init() {{
	helloCmd.Flags().StringP("name", "n", "World", "Name to greet")
	rootCmd.AddCommand(helloCmd)
}}

func main() {{
	if err := rootCmd.Execute(); err != nil {{
		fmt.Println(err)
		os.Exit(1)
	}}
}}
"#, project_name, project_name);
        
        fs::write(project_path.join("main.go"), main_go)?;
        
        // Create README
        let readme = format!(r#"# {}

A CLI utility created with Murex.

## Installation

```bash
go build -o {}
```

## Usage

```bash
./{} hello --name "Your Name"
```
"#, project_name, project_name, project_name);
        
        fs::write(project_path.join("README.md"), readme)?;
        
        Ok(())
    }
    
    fn create_bash_project(&self, project_path: &Path, project_name: &str) -> Result<()> {
        // Create main.sh
        let main_sh = format!(r#"#!/bin/bash

# {}
# A CLI utility created with Murex

# Hello command
hello() {{
    name=${{1:-World}}
    echo "Hello, $name!"
}}

# Main function
main() {{
    case $1 in
        hello)
            hello $2
            ;;
        --help|-h)
            echo "Usage: {} [command] [options]"
            echo "Commands:"
            echo "  hello [name]  Say hello to someone"
            echo "  --help        Show this help message"
            ;;
        *)
            echo "Welcome to {}! Use --help for more information."
            ;;
    esac
}}

main "$@"
"#, project_name, project_name, project_name);
        
        fs::write(project_path.join("main.sh"), main_sh)?;
        
        // Create README
        let readme = format!(r#"# {}

A CLI utility created with Murex.

## Installation

```bash
chmod +x main.sh
```

## Usage

```bash
./main.sh hello "Your Name"
```
"#, project_name);
        
        fs::write(project_path.join("README.md"), readme)?;
        
        Ok(())
    }
    
    fn create_zsh_project(&self, project_path: &Path, project_name: &str) -> Result<()> {
        // Create main.zsh
        let main_zsh = format!(r#"#!/bin/zsh

# {}
# A CLI utility created with Murex

# Hello command
hello() {{
    name=${{1:-World}}
    echo "Hello, $name!"
}}

# Main function
main() {{
    case $1 in
        hello)
            hello $2
            ;;
        --help|-h)
            echo "Usage: {} [command] [options]"
            echo "Commands:"
            echo "  hello [name]  Say hello to someone"
            echo "  --help        Show this help message"
            ;;
        *)
            echo "Welcome to {}! Use --help for more information."
            ;;
    esac
}}

main "$@"
"#, project_name, project_name, project_name);
        
        fs::write(project_path.join("main.zsh"), main_zsh)?;
        
        // Create README
        let readme = format!(r#"# {}

A CLI utility created with Murex.

## Installation

```bash
chmod +x main.zsh
```

## Usage

```bash
./main.zsh hello "Your Name"
```
"#, project_name);
        
        fs::write(project_path.join("README.md"), readme)?;
        
        Ok(())
    }
    
    fn create_bun_project(&self, project_path: &Path, project_name: &str) -> Result<()> {
        // Create bun.js
        let bun_js = format!(r#"#!/usr/bin/env bun

import {{ Command }} from 'bun';

const program = new Command();

program
  .name('{}')
  .description('A CLI utility created with Murex')
  .version('0.1.0');

program
  .command('hello')
  .description('Say hello')
  .option('-n, --name <n>', 'Name to greet', 'World')
  .action((options) => {{
    console.log(`Hello, ${{options.name}}!`);
  }});

if (process.argv.length === 2) {{
  console.log('Welcome to {}! Use --help for more information.');
}} else {{
  program.parse();
}}
"#, project_name, project_name);
        
        fs::write(project_path.join("bun.js"), bun_js)?;
        
        // Create README
        let readme = format!(r#"# {}

A CLI utility created with Murex.

## Installation

```bash
bun install
```

## Usage

```bash
bun run bun.js hello --name "Your Name"
```
"#, project_name);
        
        fs::write(project_path.join("README.md"), readme)?;
        
        Ok(())
    }
    
    fn create_from_custom_template(&self, template: &CustomTemplate, project_path: &Path, project_name: &str) -> Result<()> {
        // Copy template directory to project path
        self.copy_dir_recursive(&template.path, project_path)?;
        
        // Replace placeholders in files
        self.replace_placeholders_in_directory(project_path, project_name)?;
        
        Ok(())
    }
    
    fn copy_dir_recursive(&self, src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;
        
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            
            if src_path.is_dir() {
                self.copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        
        Ok(())
    }
    
    fn replace_placeholders_in_directory(&self, dir: &Path, project_name: &str) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.replace_placeholders_in_directory(&path, project_name)?;
            } else if let Some(extension) = path.extension() {
                // Only process text files
                if matches!(extension.to_str(), Some("rs") | Some("py") | Some("js") | Some("go") | Some("md") | Some("toml") | Some("json")) {
                    let content = fs::read_to_string(&path)?;
                    let updated_content = content.replace("{{PROJECT_NAME}}", project_name);
                    fs::write(&path, updated_content)?;
                }
            }
        }
        
        Ok(())
    }
    
    pub fn add_template(&mut self, name: String, path: PathBuf) -> Result<()> {
        if !path.exists() || !path.is_dir() {
            return Err(anyhow::anyhow!("Template path must be an existing directory"));
        }
        
        let template = CustomTemplate {
            name: name.clone(),
            path,
            description: None,
        };
        
        self.custom_templates.insert(name, template);
        self.save_custom_templates()?;
        
        Ok(())
    }
    
    pub fn remove_template(&mut self, name: &str) -> Result<()> {
        if self.custom_templates.remove(name).is_none() {
            return Err(anyhow::anyhow!("Template '{}' not found", name));
        }
        
        self.save_custom_templates()?;
        Ok(())
    }
    
    fn load_custom_templates(&mut self) -> Result<()> {
        let templates_file = get_config_dir()?.join("templates.json");
        
        if !templates_file.exists() {
            return Ok(());
        }
        
        let content = fs::read_to_string(&templates_file)?;
        let templates: HashMap<String, CustomTemplate> = serde_json::from_str(&content)?;
        self.custom_templates = templates;
        
        Ok(())
    }
    
    fn save_custom_templates(&self) -> Result<()> {
        let config_dir = get_config_dir()?;
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }
        
        let templates_file = config_dir.join("templates.json");
        let content = serde_json::to_string_pretty(&self.custom_templates)?;
        fs::write(&templates_file, content)?;
        
        Ok(())
    }
}

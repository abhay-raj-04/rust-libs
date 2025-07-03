mod cli;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, CargoSubcommands, Commands};
use include_dir::{include_dir, Dir};
use std::fs;
use std::path::Path;

// Embed the templates directory into the binary
static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CargoSubcommands::Forge(forge_cmd) => {
            match forge_cmd.command {
                Commands::New { name, template, interactive: _ } => {
                    handle_new_command(name, template)?;
                }
            }
        }
    }

    Ok(())
}

fn handle_new_command(name: String, template: Option<String>) -> Result<()> {
    println!("✅ Forging new project `{}`...", name);

    let project_path = Path::new(&name);
    
    // Create the project directory
    if project_path.exists() {
        anyhow::bail!("Directory `{}` already exists.", name);
    }
    fs::create_dir_all(project_path)
        .with_context(|| format!("Failed to create project directory: `{}`", name))?;

    // Determine template to use (default to "cli")
    let template_name = template.unwrap_or_else(|| "cli".to_string());

    // Check if template exists
    let template_dir = TEMPLATES_DIR
        .get_dir(&template_name)
        .with_context(|| format!("Template '{}' not found. Available templates: {}", 
            template_name, 
            get_available_templates().join(", ")))?;

    // Process template files recursively (simple string replacement for now)
    process_template_dir(&template_dir, project_path, &name)?;

    println!("✅ Done! Run `cd {} && cargo run` to get started.", name);
    Ok(())
}

fn get_available_templates() -> Vec<String> {
    TEMPLATES_DIR
        .dirs()
        .map(|dir| dir.path().file_name().unwrap().to_string_lossy().to_string())
        .collect()
}

fn process_template_dir(
    template_dir: &Dir,
    dest_path: &Path,
    project_name: &str,
) -> Result<()> {
    for entry in template_dir.entries() {
        match entry {
            include_dir::DirEntry::Dir(subdir) => {
                let sub_dest_path = dest_path.join(subdir.path().file_name().unwrap());
                fs::create_dir_all(&sub_dest_path)
                    .with_context(|| format!("Failed to create directory: {:?}", sub_dest_path))?;
                process_template_dir(subdir, &sub_dest_path, project_name)?;
            }
            include_dir::DirEntry::File(file) => {
                let file_name = file.path().file_name().unwrap().to_string_lossy();
                let dest_file_path = dest_path.join(&*file_name);
                
                let content = file.contents_utf8()
                    .with_context(|| format!("Failed to read template file: {:?}", file.path()))?;
                
                // Simple string replacement for now
                let rendered_content = content.replace("{{ project_name }}", project_name);
                
                fs::write(&dest_file_path, rendered_content)
                    .with_context(|| format!("Failed to write file: {:?}", dest_file_path))?;
            }
        }
    }
    Ok(())
}
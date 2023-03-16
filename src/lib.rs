mod colors;
mod project_root_detection;

use serde_json::{Map, Value};
use std::{error::Error, fs, io::Read};

pub fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(project_dir) = project_root_detection::get_project_dir_path() {
        let settings_file_path = project_dir.join(".vscode/settings.json");
        let mut settings_json = String::new();

        if settings_file_path.exists() {
            let mut settings_file = fs::File::open(settings_file_path.clone())?;
            settings_file.read_to_string(&mut settings_json)?;
        } else {
            settings_json = "{}".to_string();
        }

        let mut settings_json_object: Map<String, Value> = serde_json::from_str(&settings_json)?;

        let new_colors = colors::create_new_color_customizations_object();
        settings_json_object.insert(
            "workbench.colorCustomizations".to_string(),
            serde_json::json!(&new_colors),
        );

        let vscode_dir = project_dir.join(".vscode");
        if !vscode_dir.exists() {
            fs::create_dir(vscode_dir)?;
        }

        fs::write(
            settings_file_path,
            serde_json::to_string_pretty(&settings_json_object)?,
        )?;
    } else {
        println!("Failed to find project directory.");
    }

    Ok(())
}

mod colors;
mod project_root_detection;

use serde_json::{Map, Value};
use std::{error::Error, io::Read};

pub fn run() -> Result<(), Box<dyn Error>> {
    match project_root_detection::get_project_dir_path() {
        Ok(project_dir) => {
            let mut settings_json = String::new();
            let settings_file_path = project_dir.join(".vscode/settings.json");
            if settings_file_path.exists() {
                let mut settings_file = std::fs::File::open(settings_file_path.clone()).unwrap();
                settings_file.read_to_string(&mut settings_json).unwrap();
            } else {
                settings_json = "{}".to_string();
            }

            let mut settings_json_object: Map<String, Value> =
                serde_json::from_str(&settings_json).expect("Invalid json: settings.json");

            let new_colors = colors::create_new_color_customizations_object();

            // Act differently depending on if the key allready exists or not
            match settings_json_object.get("workbench.colorCustomizations") {
                Some(_) => {
                    // settings_json_object["workbench.colorCustomizations"].take();
                    settings_json_object["workbench.colorCustomizations"] =
                        serde_json::json!(&new_colors);
                }
                None => {
                    settings_json_object.insert(
                        "workbench.colorCustomizations".to_string(),
                        serde_json::json!(&new_colors),
                    );
                }
            };

            // Make sure the directory .vscode exists
            let vscode_dir = project_dir.join(".vscode");
            if !vscode_dir.exists() {
                std::fs::create_dir(vscode_dir)?;
            }

            // Write the json object back to the .vscode/settings.json file
            std::fs::write(
                settings_file_path,
                serde_json::to_string_pretty(&settings_json_object).unwrap(),
            )
            .unwrap();
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    Ok(())
}

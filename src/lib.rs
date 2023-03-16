mod colors;
mod project_root_detection;

use serde_json::{Map, Value};
use std::{ error::Error, io::Read};

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

            let new_colors = create_new_color_customizations_object();

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

fn rgb_to_hex(rgb: [u8; 3]) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])
}

fn calculate_relative_luminance(rgb: [u8; 3]) -> f32 {
    let r = f32::from(rgb[0]) / 255.0;
    let g = f32::from(rgb[1]) / 255.0;
    let b = f32::from(rgb[2]) / 255.0;

    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn create_new_color_customizations_object() -> Map<String, Value> {
    let new_color = colors::get_random_color();
    let new_color_hex = rgb_to_hex(new_color.1);
    let new_text_color_hex = match calculate_relative_luminance(new_color.1) {
        luminance if luminance > 0.5 => "#000000",
        _ => "#ffffff",
    };

    let new_active_background = serde_json::json!(&new_color_hex);
    let new_active_foreground = serde_json::json!(&new_text_color_hex);
    let new_inactive_background = serde_json::json!(format!("{}80", &new_color_hex));
    let new_inactive_foreground = serde_json::json!(format!("{}80", &new_text_color_hex));

    let mut new_colors = Map::new();
    new_colors.insert(
        "titleBar.activeBackground".to_string(),
        serde_json::json!(&new_active_background),
    );
    new_colors.insert(
        "titleBar.activeForeground".to_string(),
        serde_json::json!(&new_active_foreground),
    );
    new_colors.insert(
        "titleBar.inactiveBackground".to_string(),
        serde_json::json!(&new_inactive_background),
    );
    new_colors.insert(
        "titleBar.inactiveForeground".to_string(),
        serde_json::json!(&new_inactive_foreground),
    );

    println!("{}", new_color.0);

    new_colors
}

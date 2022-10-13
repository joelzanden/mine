extern crate dirs;
mod get_project_dir_path;
use get_project_dir_path::get_project_dir_path;
use std::{
    io::{Error, Read},
    path::PathBuf,
};

fn main() {
    let project_dir: Result<PathBuf, Error> = get_project_dir_path();
    match project_dir {
        Ok(project_dir) => {
            println!("project_dir: {}", project_dir.display());

            //  Read in the contents of the .vscode/settings.json file. If the file doesn't exist, create empty json object.
            let mut settings_json = String::new();
            let settings_file_path = project_dir.join(".vscode/settings.json");
            if settings_file_path.exists() {
                let mut settings_file = std::fs::File::open(settings_file_path.clone()).unwrap();
                settings_file.read_to_string(&mut settings_json).unwrap();
            } else {
                settings_json = "{}".to_string();
            }

            println!("settings_json: {}", settings_json);

            //  Parse the settings_json string into a json object with serde_json
            let mut settings_json_object: serde_json::Value =
                serde_json::from_str(&settings_json).unwrap();

            // Remove the "workbench.colorCustomizations" key if it allready exists.
            match settings_json_object.get("workbench.colorCustomizations") {
                Some(_) => {
                    settings_json_object["workbench.colorCustomizations"].take();
                }
                None => (),
            };

            // Add a new "workbench.colorCustomizations" key to the json object
            settings_json_object["workbench.colorCustomizations"] = serde_json::json!({
                "titleBar.activeBackground": "#00ffff",
                "titleBar.activeForeground": "#ff00ff",
                "titleBar.inactiveBackground": "#000000",
                "titleBar.inactiveForeground": "#ffffff",
            });

            println!(
                "settings_json_object outside of closure: {}",
                settings_json_object
            );

            // Write the json object back to the .vscode/settings.json file
            std::fs::write(
                settings_file_path.clone(),
                serde_json::to_string_pretty(&settings_json_object).unwrap(),
            )
            .unwrap();
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

// This struct gets serialized into this with serde_json:
// "workbench.colorCustomizations": {
//     "titleBar.activeForeground": "#000",
//     "titleBar.inactiveForeground": "#000000cc",
//     "titleBar.activeBackground": "#ffc600",
//     "titleBar.inactiveBackground": "#ffc600cc"
//   }:

// struct ColorCustomizations {
//     title_bar_active_foreground: String,
//     title_bar_inactive_foreground: String,
//     title_bar_active_background: String,
//     title_bar_inactive_background: String,
// }

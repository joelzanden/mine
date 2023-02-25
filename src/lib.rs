mod colors;

use colors::COLOR_NAMES;
use serde_json::{Map, Value};
use std::{
    error::Error,
    io::Read,
    path::{Path, PathBuf},
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let project_dir = get_project_dir_path();
    match project_dir {
        Ok(project_dir) => {
            println!("project_dir: {}", project_dir.display());

            // Read in the contents of the .vscode/settings.json file.
            // If the folder doesn't exist, create it.
            // If the file doesn't exist, create empty json object.
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
            let mut settings_json_object: Map<String, Value> =
                serde_json::from_str(&settings_json).expect("Invalid json");
            // Handle the error if the json is invalid

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

fn create_new_color_customizations_object() -> Map<String, Value> {
    let number_of_elements_in_array = COLOR_NAMES.len();
    let random_index = rand::random::<usize>() % number_of_elements_in_array;
    let base_color_name = COLOR_NAMES[random_index];
    println!("base_color_name: {:?}", base_color_name);

    let new_active_background = serde_json::json!("#00ffff");

    let mut new_colors = Map::new();
    new_colors.insert(
        "titleBar.activeBackground".to_string(),
        serde_json::json!(new_active_background),
    );
    new_colors.insert(
        "titleBar.activeForeground".to_string(),
        serde_json::json!("#ff00ff"),
    );
    new_colors.insert(
        "titleBar.inactiveBackground".to_string(),
        serde_json::json!("#000000"),
    );
    new_colors.insert(
        "titleBar.inactiveForeground".to_string(),
        serde_json::json!("#ffffff"),
    );

    new_colors
}

pub fn get_project_dir_path() -> Result<PathBuf, &'static str> {
    let mut path = std::env::current_dir().unwrap();
    let home = dirs::home_dir().unwrap();

    while path != Path::new("/") && path != home {
        match we_are_in_a_project_dir(&path) {
            true => {
                return Ok(path);
            }
            false => {
                path.pop();
            }
        }
    }

    Err("No project directory found")
}

pub fn we_are_in_a_project_dir(path: &Path) -> bool {
    let cargo_toml_path = path.join("Cargo.toml");
    let package_json_path = path.join("package.json");

    // Excluding .vscode/settings.json for now because I don't want to land in the home folder
    // That might include the user settings
    // let vscode_settings_path = path.join(".vscode/settings.json");

    cargo_toml_path.exists() || package_json_path.exists()
}

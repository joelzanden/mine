use std;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

pub(crate) fn get_project_dir_path() -> Result<PathBuf, Error> {
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

    Err(Error::new(
        ErrorKind::NotFound,
        "No project directory found",
    ))
}

pub(crate) fn we_are_in_a_project_dir(path: &PathBuf) -> bool {
    let cargo_toml_path = path.join("Cargo.toml");
    let package_json_path = path.join("package.json");

    // Excluding .vscode/settings.json for now because I don't want to land in the home folder
    // That might include the user settings
    // let vscode_settings_path = path.join(".vscode/settings.json");

    if cargo_toml_path.exists() || package_json_path.exists() {
        true
    } else {
        false
    }
}

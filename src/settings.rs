use std::{
    fs::File,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum LspVariant {
    Mesonlsp,
    Muon,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    lsp: LspVariant,
    muon_path: Option<String>,
    mesonlsp_path: Option<String>,
}

fn write_settings_to_file<P: AsRef<Path>>(path: P, settings: &Settings) -> std::io::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, settings)?;
    Ok(())
}

pub fn get_settings() -> Result<Settings, String> {
    let settings_file_path = PathBuf::from("settings.json");
    if !settings_file_path.exists() {
        let default_settings = Settings {
            lsp: LspVariant::Mesonlsp,
            mesonlsp_path: None,
            muon_path: None,
        };

        write_settings_to_file(&settings_file_path, &default_settings)
            .map_err(|e| format!("Failed to create settings file: {}", e))?;

        return Ok(default_settings);
    }

    let settings_file = File::open(&settings_file_path)
        .map_err(|e| format!("Failed to open settings file: {}", e))?;

    let settings: Settings = serde_json::from_reader(settings_file)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    return Ok(settings);
}

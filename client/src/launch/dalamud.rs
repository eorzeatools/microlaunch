use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{integrity::{Repository, RepositoryId}, auth::ClientLanguage};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="PascalCase")] // For consistency with Dalamud and C#
pub struct DalamudStartInfo {
    pub working_directory: String,
    pub configuration_path: String,

    pub plugin_directory: String,
    pub default_plugin_directory: String,
    pub asset_directory: String,
    pub client_language: i32, // see auth::ClientLanguage
    pub delay_initialize_ms: i32,

    pub game_version: String
}

use std::{io, fs};

// https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
fn copy_dir_all(src: impl AsRef<std::path::Path>, dst: impl AsRef<std::path::Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

impl DalamudStartInfo {
    pub fn get(wineprefix_path: &PathBuf, dalamud_path: &PathBuf) -> Self {
        let roaming_path_win = r#"C:\Users\steamuser\AppData\Roaming\dalamud"#.to_owned();
        let plugin_dir_path_win = roaming_path_win.clone() + r#"\installedPlugins"#;
        let default_plugin_path_win = roaming_path_win.clone() + r#"\devPlugins"#;
        let dalamud_config_path_win = roaming_path_win + r#"\dalamudConfig.json"#;

        // Create that directory
        let pfx = wineprefix_path
            .join("drive_c")
            .join("users")
            .join("steamuser")
            .join("AppData")
            .join("Roaming")
            .join("dalamud");
        std::fs::create_dir_all(&pfx).unwrap();
        std::fs::create_dir_all(pfx.join("installedPlugins")).unwrap();
        std::fs::create_dir_all(pfx.join("devPlugins")).unwrap();

        // Get working directory (dir of Dalamud.Injector.exe)
        let dalamud_system_dir = dalamud_path;

        // Slam dalamud (and dotnet) into the wineprefix because... I think we should have it in there
        let dalamud_wineprefix_dest = wineprefix_path
            .join("drive_c")
            .join("dalamud");
        if !dalamud_wineprefix_dest.exists() {

            // !! THIS SHIT DOESN'T WORK!
            // TODO: FIX THIS!!

            std::fs::create_dir_all(&dalamud_wineprefix_dest).unwrap();
            copy_dir_all(dalamud_system_dir, dalamud_wineprefix_dest)
                .expect("failed to copy dalamud to wineprefix");
            println!("COPIED DALAMUD TO WINEPREFIX");
        }

        Self {
            working_directory: r#"C:\dalamud\rel\"#.into(),
            configuration_path: dalamud_config_path_win,
            plugin_directory: plugin_dir_path_win,
            default_plugin_directory: default_plugin_path_win,
            asset_directory: r#"C:\dalamud\assets\"#.into(),
            client_language: ClientLanguage::English as i32,
            delay_initialize_ms: 0,
            game_version: Repository(RepositoryId::Ffxiv).get_version().unwrap()
        }
    }
}
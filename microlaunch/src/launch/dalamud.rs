use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::{integrity::{Repository, RepositoryId}, other::{get_client_language, to_windows_path}};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="PascalCase")] // For consistency with Dalamud and C#
pub struct DalamudStartInfo {
    pub working_directory: String,
    pub configuration_path: String,

    pub plugin_directory: String,
    pub default_plugin_directory: String,
    pub asset_directory: String,

    #[serde(rename="Language")]
    pub client_language: i32, // see auth::ClientLanguage
    pub delay_initialize_ms: i32,

    pub game_version: String
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
        let dalamud_working_dir = dalamud_path.join("rel").canonicalize().unwrap();
        let dalamud_assets_dir = dalamud_path.join("assets").canonicalize().unwrap();
        let dalamud_working_dir_win = to_windows_path(&dalamud_working_dir);
        let dalamud_assets_dir_win = to_windows_path(&dalamud_assets_dir);

        Self {
            working_directory: dalamud_working_dir_win,
            configuration_path: dalamud_config_path_win,
            plugin_directory: plugin_dir_path_win,
            default_plugin_directory: default_plugin_path_win,
            asset_directory: dalamud_assets_dir_win,
            client_language: get_client_language() as i32,
            delay_initialize_ms: 0,
            game_version: Repository(RepositoryId::Ffxiv).get_version().unwrap()
        }
    }
}
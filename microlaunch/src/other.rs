#![allow(dead_code)]

use std::path::Path;
use crate::auth::ClientLanguage;

static BASE_GAME_VERSION: &'static str = "2012.01.01.0000.0000";

pub fn get_patcher_useragent() -> String {
    "FFXIV PATCH CLIENT".into()
}

pub fn get_client_language() -> crate::auth::ClientLanguage {
    if let Some(e) = &crate::config::CONFIG.experimental {
        ClientLanguage::from(e.client_language.as_str())
    } else {
        crate::auth::ClientLanguage::English
    }
}

/// Converts a Unix path into a Wine Z:\ path
pub fn to_windows_path(path: &Path) -> String {
    "Z:".to_owned() + &path.to_string_lossy().replace("/", r#"\"#)
}
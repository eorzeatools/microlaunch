#![allow(dead_code)]

static BASE_GAME_VERSION: &'static str = "2012.01.01.0000.0000";

pub fn get_patcher_useragent() -> String {
    "FFXIV PATCH CLIENT".into()
}

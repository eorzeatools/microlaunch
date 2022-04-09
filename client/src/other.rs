#![allow(dead_code)]

static BASE_GAME_VERSION: &'static str = "2012.01.01.0000.0000";

static STEAM_APPID: u32 = 39210;
static STEAM_APPID_FTRIAL: u32 = 312060;

pub fn get_patcher_useragent() -> String {
    "FFXIV PATCH CLIENT".into()
}

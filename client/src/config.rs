// microlaunch.toml

use serde::Deserialize;

#[derive(Deserialize)]
pub struct MicrolaunchConfig {
    pub game: Option<GameConfig>,
    pub proton: Option<ProtonConfig>
}

#[derive(Deserialize)]
pub struct GameConfig {
    pub max_entitled_expansion_id: String
}

#[derive(Deserialize)]
pub struct ProtonConfig {
    // Compatibility data path (appid is 39210)
    // Probably ~/.steam/compatdata/39210
    pub compat_data_path: String,
    // Compat client install path (Proton wants this)
    // Probably ~/.steam/steam (absolute path)
    pub compat_client_install_path: String,
    // Proton root path (contains `proton` binary)
    pub proton_root_path: String,
    // Absolute path to ffxiv_dx11.exe
    pub game_binary_path: String,
}

lazy_static::lazy_static! {
    pub static ref CONFIG: MicrolaunchConfig = {
        let cfg_str = std::fs::read_to_string("microlaunch.toml").expect("could not open microlaunch.toml");
        toml::from_str::<MicrolaunchConfig>(&cfg_str).expect("bad config!")
    };
}
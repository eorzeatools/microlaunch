// microlaunch.toml

use serde::Deserialize;

#[derive(Deserialize)]
pub struct MicrolaunchConfig {
    pub launcher: LauncherConfig,
    pub proton: Option<ProtonConfig>,
    pub direct_launch: Option<DirectLaunchConfig>,
    pub experimental: Option<ExperimentalConfig>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum GameLaunchStrategy {
    Proton,
    DirectLaunch
}

#[derive(Deserialize)]
pub struct LauncherConfig {
    #[serde(rename="game_launch_strategy")]
    pub strategy: GameLaunchStrategy
}

#[derive(Deserialize)]
pub struct DirectLaunchConfig {
    // Absolute path to ffxiv_dx11.exe
    pub game_binary_path: String,
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

#[derive(Deserialize)]
pub struct ExperimentalConfig {
    // Experimental config contains
    // things that you probably shouldn't
    // touch unless you know what you're doing

    // Randomly tweaks the computer identifier
    // to try to bust state
    pub tweak_computer_id_randomly: bool
}

lazy_static::lazy_static! {
    pub static ref CONFIG: MicrolaunchConfig = {
        let cfg_str = std::fs::read_to_string("microlaunch.toml").expect("could not open microlaunch.toml");
        toml::from_str::<MicrolaunchConfig>(&cfg_str).expect("bad config!")
    };
}
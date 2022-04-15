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
    pub strategy: GameLaunchStrategy,

    // REQUIRED for "integrity" checking
    // (aka Square Enix is stupid again)
    pub game_path: String,

    // Command to run before the game
    // e.g. mangohud
    pub prefix_command: Option<String>
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
    #[serde(default = "bool::default")] // false
    pub tweak_computer_id_randomly: bool,

    // Path to the encrypted data file
    // Used to store your SEID and password
    // alongside login tokens and other stuff
    // By default, this is in ~/.config/microlaunch
    pub encrypted_data_path: Option<String>,
}

lazy_static::lazy_static! {
    pub static ref CONFIG: MicrolaunchConfig = {
        if cfg!(test) {
            let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.pop();
            d.push("microlaunch.toml");
            let cfg_str = std::fs::read_to_string(d).expect("could not open microlaunch.toml");
            toml::from_str::<MicrolaunchConfig>(&cfg_str).expect("bad config!")
        } else {
            let cfg_str = std::fs::read_to_string("microlaunch.toml").expect("could not open microlaunch.toml");
            toml::from_str::<MicrolaunchConfig>(&cfg_str).expect("bad config!")
        }
    };
}
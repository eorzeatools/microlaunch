// Steamworks utils

use steamworks::{Client, SingleClient};

pub enum SteamworksAppid {
    // FINAL FANTASY XIV Online - https://store.steampowered.com/app/39210/
    XivGame = 39210,

    // FINAL FANTASY XIV Online Free Trial - https://store.steampowered.com/app/312060/
    XivGameFtrial = 312060,

    // These will probably
    // never be used but I'm
    // leaving them in for
    // completeness' sake
    #[allow(dead_code)]
    XivDlcShadowbringers = 1016870,
    #[allow(dead_code)]
    XivDlcEndwalker = 1592500
}

impl From<SteamworksAppid> for steamworks::AppId {
    fn from(x: SteamworksAppid) -> Self {
        Self(x as u32)
    }
}

pub fn init_steamworks_for_app(app: SteamworksAppid) -> Result<(Client, SingleClient), steamworks::SteamError> {
    let sw_appid: steamworks::AppId = app.into();
    let (client, single) = Client::init_app(sw_appid.clone())?;

    Ok((client, single))
}

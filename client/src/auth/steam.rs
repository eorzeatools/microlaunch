use parking_lot::Mutex;
use steamworks::AuthTicket;
use crate::steamworks::SteamworksAppid;
use super::AccountType;

pub struct SteamTicket {
    pub text: String,
    pub length: usize,
    pub steamworks_ticket: AuthTicket
}

lazy_static::lazy_static! {
    pub static ref STEAM: Mutex<Box<Option<steamworks::Client>>> = Mutex::new(Box::new(None));
}

pub fn init(acc_type: &AccountType) -> Result<(), steamworks::SteamError> {
    if let Some(_) = STEAM.lock().as_ref() {
        println!("Steamworks is ALREADY INITIALISED - this should not happen!!! Panicking!");
        panic!("Steamworks duplicate init!");
    }

    let app_to_use = match acc_type {
        AccountType::Subscription => SteamworksAppid::XivGame,
        AccountType::FreeTrial => SteamworksAppid::XivGameFtrial,
    };

    let (client, _) = crate::steamworks::init_steamworks_for_app(app_to_use)?;

    *STEAM.lock() = Box::new(Some(client));

    Ok(())
}

pub fn get_ticket() -> SteamTicket {
    if let Some(steam) = STEAM.lock().as_ref() {
        let steam_raw = steam.user().authentication_session_ticket();
        let time = crate::steamworks::get_steam_server_time(steam);
        let (crypt, length) = sqexcrypt::enc::encrypt(steam_raw.1, time);
        SteamTicket {
            steamworks_ticket: steam_raw.0,
            text: crypt,
            length
        }
    } else {
        panic!("get_ticket:: Steamworks not initialised!");
    }
}
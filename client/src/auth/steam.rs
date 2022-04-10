use parking_lot::Mutex;

use crate::steamworks::SteamworksAppid;

use super::AccountType;

pub struct SteamTicket {
    pub text: String,
    pub length: i32
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
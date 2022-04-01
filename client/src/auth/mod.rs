// Authentication module for microlaunch
// Talks to Square Enix API

use self::steam::SteamTicket;

mod steam;

#[derive(Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum Platform {
    SqexStore = 0,
    Steam = 1
}

#[derive(Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum AccountType {
    Subscription = 0,
    FreeTrial = 1
}

#[derive(Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum GameRegion {
    Region0 = 0,
    Region1 = 1,
    Region2 = 2,
    UnitedStates = 3
}

pub enum ClientLanguage {
    Japanese,
    English,
    German,
    French
}

impl ClientLanguage {
    pub fn get_lang_code(&self) -> String {
        match self {
            ClientLanguage::Japanese => "ja".into(),
            ClientLanguage::English => "en-us".into(),
            ClientLanguage::German => "de".into(),
            ClientLanguage::French => "fr".into(),
        }
    }
}

pub struct LoginParams {
    pub region: String,
    pub platform: Platform,
    pub account_type: AccountType
}

pub fn get_oauth_page_url(region: i32, ftrial: bool, steam: bool, ticket: Option<SteamTicket>) -> String {
    let mut url =
        format!("https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/top?lng=en&rgn={}&isft={}&cssmode=1&isnew=1&launchver=3",
        region,
        if ftrial { "1" } else { "0" });
    
    if steam {
        if let Some(x) = ticket {
            // I can't merge this `if let` into the main `if`
            // because it's a nightly feature (~rust 1.59.0)
            // IGNORE THIS FOR NOW
            url.push_str("&issteam=1");
            url.push_str(&format!("&session_ticket={}", x.text));
            url.push_str(&format!("&ticket_size={}", x.length));
        } else {
            // this code path should never be hit
            // (steam=true ticket=None)
            unreachable!()
        }
    }

    url
}
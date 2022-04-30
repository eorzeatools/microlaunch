// Authentication module for microlaunch
// Talks to Square Enix API

use std::collections::HashMap;
use regex::Regex;
use serde::{Serialize, Deserialize};
use self::steam::SteamTicket;
use itertools::Itertools;

pub mod steam;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
#[repr(i32)]
pub enum Platform {
    SqexStore = 0,
    Steam = 1,
    Placeholder = 50
}

impl Default for Platform {
    fn default() -> Self {
        Self::SqexStore
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::SqexStore => "Square Enix Store",
            Self::Steam => "Steam",
            Self::Placeholder => "Platform...",
        })
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
#[repr(i32)]
pub enum AccountType {
    Subscription = 0,
    FreeTrial = 1,
    Placeholder = 50,
}

impl Default for AccountType {
    fn default() -> Self {
        Self::Subscription
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Subscription => "Full game (subscription)",
            Self::FreeTrial => "Free trial",
            Self::Placeholder => "Account type..."
        })
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum GameRegion {
    Japan = 1,
    America = 2,
    Europe = 3,
    Korea = 101
}

impl std::fmt::Debug for GameRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Japan => write!(f, "Japan"),
            Self::America => write!(f, "America"),
            Self::Europe => write!(f, "Europe"),
            Self::Korea => write!(f, "Korea"),
        }
    }
}

impl TryFrom<i32> for GameRegion {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == (GameRegion::Japan as i32) => Ok(GameRegion::Japan),
            x if x == (GameRegion::America as i32) => Ok(GameRegion::America),
            x if x == (GameRegion::Europe as i32) => Ok(GameRegion::Europe),
            x if x == (GameRegion::Korea as i32) => Ok(GameRegion::Korea),
            _ => Err(())
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum Expansion {
    ARealmReborn = 0,
    Heavensward = 1,
    Stormblood = 2,
    Shadowbringers = 3,
    Endwalker = 4
}

impl TryFrom<i32> for Expansion {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == (Expansion::ARealmReborn as i32) => Ok(Expansion::ARealmReborn),
            x if x == (Expansion::Heavensward as i32) => Ok(Expansion::Heavensward),
            x if x == (Expansion::Stormblood as i32) => Ok(Expansion::Stormblood),
            x if x == (Expansion::Shadowbringers as i32) => Ok(Expansion::Shadowbringers),
            x if x == (Expansion::Endwalker as i32) => Ok(Expansion::Endwalker),
            _ => Err(())
        }
    }
}

impl std::fmt::Debug for Expansion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ARealmReborn => write!(f, "Final Fantasy XIV: A Realm Reborn"),
            Self::Heavensward => write!(f, "Final Fantasy XIV: Heavensward"),
            Self::Stormblood => write!(f, "Final Fantasy XIV: Stormblood"),
            Self::Shadowbringers => write!(f, "Final Fantasy XIV: Shadowbringers"),
            Self::Endwalker => write!(f, "Final Fantasy XIV: Endwalker"),
        }
    }
}

// TODO: Implement the ability to choose a client language
// (The official launcher also doesn't do this!)
#[allow(dead_code)]
pub enum ClientLanguage {
    Japanese,
    English,
    German,
    French
}

// TODO: Wtf is this used for?
// Uncomment it if it's somehow important to anyone
/*
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
*/

#[derive(Debug, Clone)]
pub enum GameLoginResult {
    Successful(GameLoginData),
    SteamLinkRequired,
    WrongSteamAccount,
    NoMoreGameTime,
    TermsNotAccepted,
    NoServiceAccount,
    Error
}

#[derive(Debug, Clone)]
pub struct GameLoginData {
    pub sid: String,
    pub region: GameRegion,
    pub max_expansion: Expansion,
    pub playable: bool,
    pub steam_username: Option<String>
}

pub async fn login_oauth(
    username: String,
    password: String,
    otp: String,
    ftrial: bool,
    steam: bool,
    region: GameRegion,
    ticket: Option<SteamTicket>
) -> GameLoginResult {
    if matches!(ticket, None) && steam {
        panic!("what the Fuck ???????????");
    }

    let top_page_url = get_oauth_page_url(region as i32, ftrial, steam, ticket);

    println!("requesting oauth top page at url: ");
    println!("{}", top_page_url);

    // get Oauth top page
    let req = reqwest::Client::builder().user_agent(&format!("SQEXAuthor/2.0.0(Windows 6.2; ja-JP; {})", generate_computer_id()));
    let client = req.build().unwrap();

    let (csrf, steam_user) = {
        let a = {
            let request = client.request(reqwest::Method::GET, top_page_url.clone());
            let request = request.header("Accept",
                "image/gif, image/jpeg, image/pjpeg, application/x-ms-application, application/xaml+xml, application/x-ms-xbap, */*");
            let request = request.header("Accept-Encoding", "gzip, deflate");
            let request = request.header("Connection", "Keep-Alive");
            let request = request.header("Cookie", "_rsid= \"\"");
            request.send().await
        };
    
        let a = a.expect("Failed to make top-page http request. wtf");
    
        let top_text = a.text().await.unwrap();
    
        if top_text.contains("window.external.user(\"restartup\");") {
            if steam {
                println!("Steam link required - please link Square Enix acct to Steam");
                return GameLoginResult::SteamLinkRequired;
            }

            panic!("invalid resp - restartUp but not steam wtf");
        }

        let steam_username = if steam {
            let steam_re = Regex::new(r#"<input name="sqexid" type="hidden" value="(.*)"/>"#).unwrap();
            let steam_res = steam_re.captures(top_text.as_str().clone());

            if let None = steam_res {
                panic!("Steam = true but could not get Steam username. What?");
            }

            Some(steam_res.unwrap().get(1).unwrap().as_str().to_owned())
        } else {
            None
        };

        let csrf_re = Regex::new(r#"\t<\s*input .* name="_STORED_" value="(:?.*)">"#).unwrap();
        let csrf_res = csrf_re.captures(top_text.as_str());
        let token = csrf_res.expect("fuck square enix").get(1).unwrap();
        (token.clone().as_str().to_owned(), steam_username)
    };

    println!("got csrf token: {}...", &csrf[..20]);
    if steam {
        println!("got steam user: {}", &steam_user.as_ref().unwrap());
    }

    // Actually make the oauth login request

    let req =
        client.request(reqwest::Method::POST, "https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/login.send");

    let req = req.header("Accept", "image/gif, image/jpeg, image/pjpeg, application/x-ms-application, application/xaml+xml, application/x-ms-xbap, */*");
    let req = req.header("Referer", top_page_url);
    let req = req.header("Accept-Encoding", "gzip, deflate");
    let req = req.header("Host", "ffxiv-login.square-enix.com");
    let req = req.header("Connection", "Keep-Alive");
    let req = req.header("Cache-Control", "no-cache");
    let req = req.header("Cookie", r#"_rsid="""#);

    let true_sqexid = if steam {
        if username.to_lowercase() != steam_user.as_ref().unwrap().to_lowercase() {
            return GameLoginResult::WrongSteamAccount;
        }

        steam_user.clone().unwrap()
    } else {
        username.clone()
    };
    
    let mut body_map = HashMap::new();
    body_map.insert("_STORED_", csrf);
    body_map.insert("sqexid", true_sqexid.into());
    body_map.insert("password", password.into());
    body_map.insert("otppw", otp.into());
    
    let req = req.form(&body_map);

    let resp = req.send().await;

    let f = resp.unwrap().text().await.unwrap();
    println!("got login response");

    if f.contains("login=auth,ng,err,You could not log in because the service account in question is not registered.") {
        // No service account
        return GameLoginResult::NoServiceAccount;
    }

    let resp_regex = regex::Regex::new(r#"window.external.user\("login=auth,ok,(.*)"\);"#).unwrap();
    let re_match = resp_regex.captures(&f);
    if let None = re_match {
        return GameLoginResult::Error;
    }
    let re_match = re_match.unwrap();
    let login_args = re_match.get(1).expect("no match? (get(1) fail)").as_str();
    println!("login arguments are: {login_args}");

    let got_keys = login_args.split(",");
    let keys =
        got_keys.tuples().fold(HashMap::new(), |mut map, (k,v)| {
            map.insert(k, v);
            map
        });

    let data = GameLoginData {
        playable: keys.get("playable") == Some(&"1"),
        sid: (*keys.get("sid").unwrap()).into(),
        max_expansion: (*keys.get("maxex").unwrap()).parse::<i32>().unwrap().try_into().unwrap(),
        region: (*keys.get("region").unwrap()).parse::<i32>().unwrap().try_into().unwrap(),
        steam_username: steam_user
    };

    if keys.get("terms") != Some(&"1") {
        return GameLoginResult::TermsNotAccepted;
    }

    if !data.playable {
        return GameLoginResult::NoMoreGameTime;
    }

    GameLoginResult::Successful(data)
}

pub fn get_oauth_page_url(region: i32, ftrial: bool, steam: bool, ticket: Option<SteamTicket>) -> String {
    let mut url =
        format!("https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/top?lng=en&rgn={}&isft={}&cssmode=1&isnew=1&launchver=3",
        region,
        if ftrial { "1" } else { "0" });
    
    if steam {
        if let Some(x) = ticket {
            // I can't merge this `if let` into the main `if`
            // because it's a nightly feature (~rust 1.60.0)
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

/// Square Enix wants a unique computer ID
/// so this is how I generate one
pub fn generate_computer_id() -> String {
    use sha2::Digest;

    // This is dumb and bad code
    // but more importantly it's dumb and bad code that
    // doesn't make any sense here
    // BECAUSE ON LINUX, WE HAVE /etc/machine-id
    // IN M O S T CASES AND WE SHOULD ONLY BE RUNNING THIS WHEN NOT THAT
    // OH FUCKING WELL THIS CAN STAY RN
    let unique_bytes: [u8; 4] = if cfg!(target_os = "linux") {
        let cmd = std::process::Command::new("uname")
            .arg("-a")
            .output()
            .expect("failed to run uname -a? wtf?");
        let mut sha = sha2::Sha256::new();
        sha.update(cmd.stdout);
        let fin = sha.finalize();
        let mut bytes: [u8; 4] = fin[0..4].try_into().unwrap();

        if let Some(experimental) = &crate::config::CONFIG.experimental {
            if experimental.tweak_computer_id_randomly {
                for i in 0..4 {
                    bytes[i] = bytes[i].wrapping_add(rand::random::<u8>());
                }
            }
        }

        bytes
    } else {
        [0xde, 0xad, 0xc0, 0xde]
    };

    let actual_bytes = {
        let mut fuck: [u8; 5] = [0, 0, 0, 0, 0];
        fuck[1..].clone_from_slice(&unique_bytes);
        let checksum = (-((fuck[1].overflowing_add(fuck[2]).0.overflowing_add(fuck[3]).0.overflowing_add(fuck[4])).0 as i64)) as u8;
        fuck[0] = checksum;
        fuck
    };

    hex::encode(actual_bytes)
}
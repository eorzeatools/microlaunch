// Authentication module for microlaunch
// Talks to Square Enix API

#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

use regex::Regex;

use self::steam::SteamTicket;

mod steam;

#[derive(Hash, PartialEq, Eq, Clone)]
#[repr(i32)]
pub enum Platform {
    SqexStore = 0,
    Steam = 1
}

#[derive(Hash, PartialEq, Eq, Clone)]
#[repr(i32)]
pub enum AccountType {
    Subscription = 0,
    FreeTrial = 1
}

#[derive(Hash, PartialEq, Eq, Clone)]
#[repr(i32)]
pub enum GameRegion {
    Japan = 1,
    America = 2,
    Europe = 3,
    Korea = 101
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

pub async fn login_oauth(
    username: &str,
    password: &str,
    otp: &str,
    ftrial: bool,
    steam: bool,
    region: GameRegion,
    ticket: Option<SteamTicket>
) {
    if matches!(ticket, None) && steam {
        panic!("what the Fuck ???????????");
    }

    let top_page_url = get_oauth_page_url(region as i32, ftrial, steam, ticket);

    println!("requesting oauth top page at url: ");
    println!("{}", top_page_url);

    // get Oauth top page
    let req = reqwest::Client::builder().user_agent(&format!("SQEXAuthor/2.0.0(Windows 6.2; ja-JP; {})", generate_computer_id()));
    let client = req.build().unwrap();

    let csrf = {
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
                return;
            }

            panic!("invalid resp - restartUp but not steam wtf");
        }

        let csrf_re = Regex::new(r#"\t<\s*input .* name="_STORED_" value="(:?.*)">"#).unwrap();
        let csrf_res = csrf_re.captures(top_text.as_str());
        let token = csrf_res.expect("fuck square enix").get(1).unwrap();
        token.clone().as_str().to_owned()
    };

    println!("got csrf token: {}", csrf);

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
    
    let mut body_map = HashMap::new();
    body_map.insert("_STORED_", csrf);
    body_map.insert("sqexid", username.into());
    body_map.insert("password", password.into());
    body_map.insert("otppw", otp.into());
    
    let req = req.form(&body_map);

    let resp = req.send().await;

    let f = resp.unwrap().text().await.unwrap();
    println!("got login response:");
    println!("{}", f);
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

/// Square Enix wants a unique computer ID
/// so this is how I generate one
pub fn generate_computer_id() -> String {
    use sha2::Digest;

    // This is dumb and bad code
    let unique_bytes: [u8; 4] = if cfg!(target_os = "linux") {
        let cmd = std::process::Command::new("uname")
            .arg("-a")
            .output()
            .expect("failed to run uname -a? wtf?");
        let mut sha = sha2::Sha256::new();
        sha.update(cmd.stdout);
        let fin = sha.finalize();
        fin[0..4].try_into().unwrap()
    } else {
        [0xde, 0xad, 0xc0, 0xde]
    };

    let actual_bytes = {
        let mut fuck: [u8; 5] = [0, 0, 0, 0, 0];
        fuck[1..].clone_from_slice(&unique_bytes);
        let checksum = (-((fuck[1] + fuck[2] + fuck[3] + fuck[4]) as i32)) as u8;
        fuck[0] = checksum;
        fuck
    };

    hex::encode(actual_bytes)
}
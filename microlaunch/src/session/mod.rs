// Session module
// Responsible for making the RegisterSession call

// Yes this gets its own module it's my launcher fuck you

use std::{path::Path, fs};

use crate::{auth::GameLoginData, integrity::{RepositoryId, Repository}};
use data_encoding::HEXLOWER;
use reqwest::StatusCode;

lazy_static::lazy_static! {
    static ref FILES_TO_HASH: Vec<&'static str> = vec![
        "ffxivboot.exe",
        "ffxivboot64.exe",
        "ffxivlauncher.exe",
        "ffxivlauncher64.exe",
        "ffxivupdater.exe",
        "ffxivupdater64.exe"
    ];
}

fn get_boot_version_hash() -> String {
    let mut res = Repository(RepositoryId::Boot).get_version().unwrap() + "=";

    for i in 0..FILES_TO_HASH.len() {
        let filename: &str = FILES_TO_HASH[i];

        let fhash = {
            // calculate hash here

            let gamepath = Path::new(&crate::config::CONFIG.launcher.game_path);
            let path_to_file = gamepath.join("boot").join(filename);

            let file = fs::File::open(&path_to_file).unwrap();
            let hash = crate::integrity::hash::sha1_digest(file)
                .expect(&format!("failed to digest hash for file {}", filename));

            let size = fs::metadata(path_to_file).unwrap().len();
            format!("{}/{}", size, HEXLOWER.encode(hash.as_ref()))
        };

        res += &format!("{}/{}", filename, fhash);

        if i != FILES_TO_HASH.len() - 1 {
            res += ",";
        }
    }

    res
}

fn get_version_report(exlevel: i32) -> String {
    let mut report = get_boot_version_hash();

    if exlevel >= 1 {
        report += &format!("\nex1\t{}", Repository(RepositoryId::Ex1).get_version().unwrap());
    }

    if exlevel >= 2 {
        report += &format!("\nex2\t{}", Repository(RepositoryId::Ex2).get_version().unwrap());
    }

    if exlevel >= 3 {
        report += &format!("\nex3\t{}", Repository(RepositoryId::Ex3).get_version().unwrap());
    }

    if exlevel >= 4 {
        report += &format!("\nex4\t{}", Repository(RepositoryId::Ex4).get_version().unwrap());
    }

    report
}

#[derive(Clone)]
pub enum RegisterSessionResult {
    Ok(String), // String contains X-Patch-Unique-Id
    GamePatchNeeded,
    BootPatchNeeded
}

pub async fn register_session(logindata: &GameLoginData) -> RegisterSessionResult {
    let xiv_patch_ver = Repository(RepositoryId::Ffxiv).get_version().unwrap();

    let game_version_report = get_version_report(logindata.max_expansion as i32);
    println!("game version report follows:");
    println!("");
    println!("{}", game_version_report);
    println!("");

    let url = 
        format!("https://patch-gamever.ffxiv.com/http/win32/ffxivneo_release_game/{}/{}", xiv_patch_ver, logindata.sid);

    let clientbuilder = reqwest::Client::builder();
    let clientbuilder = clientbuilder.user_agent(crate::other::get_patcher_useragent());
    let client = clientbuilder.build().unwrap();

    let req = client.request(reqwest::Method::POST, url);
    let req = req.header("X-Hash-Check", "enabled");
    let req = req.body(game_version_report);
    let res = req.send().await.unwrap();

    if res.status() == StatusCode::CONFLICT {
        // boot needs to update
        println!("register_session:: FfxivBoot needs update. Cannot proceed! Aborting...");
        return RegisterSessionResult::BootPatchNeeded;
    }

    let hdrs = res.headers().clone();
    let upid = hdrs.get("X-Patch-Unique-ID");
    let text = res.text().await.unwrap();
    if let None = upid {
        // oh no
        println!("X-Unique-Patch-ID not found - aborting.");
    }

    if let Some(x) = upid {
        if text.is_empty() {
            // we're done!
            return RegisterSessionResult::Ok(x.to_str().unwrap().into());
        }
    }

    println!("register_session:: FfxivGame needs update. Cannot proceed! Text from request follows:");
    println!();
    println!("{}", text);

    return RegisterSessionResult::GamePatchNeeded;
}
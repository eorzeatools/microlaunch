use auth::{AccountType, GameRegion, Platform, ClientLanguage};
use clap::Parser;
use persist::{PERSISTENT, EncryptedPersistentData};

mod gui;
mod auth;
mod config;
mod launch;
mod integrity;
mod session;
mod other;
mod steamworks;
mod persist;

#[derive(clap::Parser, Debug)]
#[clap(author="Rin", 
    version=env!("CARGO_PKG_VERSION"),
    about="A native, versatile FINAL FANTASY XIV Online launcher made for Linux.",
    long_about=None,
    name="microlaunch")]
struct CommandLine {
    #[clap(long="--gui", help="Forces GUI mode.")]
    force_gui: bool,

    #[clap(long="--fake-login", help="Fake login. !! DO NOT TOUCH THIS OPTION IF YOU ARE NOT A DEVELOPER !!")]
    fake_login: bool,
}

fn run_gui() {
    println!("GUI mode starting...");
    let app = gui::MicrolaunchApp::default();
    let native_options = eframe::NativeOptions {
        decorated: true,
        transparent: false,
        min_window_size: Some(egui::vec2(320.0, 10.0)),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}

async fn do_full_login_process(data: EncryptedPersistentData) {
    let on_steam = data.platform == Platform::Steam;

    println!("Doing full login process for {}", data.sqex_id);

    if on_steam {
        let steam_res = auth::steam::init(&data.account_type);

        match steam_res {
            Ok(()) => {},
            Err(x) => {
                println!("-- ERROR: Failed to initialise Steamworks!");
                println!("-- ERROR: Ensure Steam is running on your computer, and you're");
                println!("-- ERROR: logged into the right account. Details follow:");
                println!("{:#?}", x);
                std::process::exit(1);
            },
        }
    }

    let mut steam_ticket = None;
    if data.platform == Platform::Steam {
        steam_ticket = Some(auth::steam::get_ticket());
    }

    let oauth_response =
        auth::login_oauth(&data.sqex_id,
            &data.password,
            "",
            data.account_type == AccountType::FreeTrial,
            on_steam,
            GameRegion::Europe,
            steam_ticket).await;
    
    match oauth_response {
        auth::GameLoginResult::Successful(ldata) => {
            let register_res = session::register_session(&ldata).await;

            match register_res {
                session::RegisterSessionResult::Ok(sid) => {
                    println!("Everything is OK - launching game!");
                    launch::launch_game(&ldata,
                            ClientLanguage::English,
                            &sid,
                            on_steam);
                },
                session::RegisterSessionResult::GamePatchNeeded => {
                    println!("-- ERROR: XIVGame patch required. Please patch your game through some method.");
                    std::process::exit(1);
                },
                session::RegisterSessionResult::BootPatchNeeded => {
                    println!("-- ERROR: XIVBoot patch required. Please patch your game through some method.");
                    std::process::exit(1);
                },
            }
        },
        auth::GameLoginResult::SteamLinkRequired => {
            println!("-- ERROR: Steam link is required to continue. Please link your Square Enix account.");
            std::process::exit(1);
        },
        auth::GameLoginResult::WrongSteamAccount => {
            println!("-- ERROR: Wrong Steam account. Make sure Steam is logged into the right account.");
            std::process::exit(1);
        },
        auth::GameLoginResult::Error => {
            println!("-- ERROR: Other. Please make sure your username and password is correct, but");
            println!("-- ERROR: that shouldn't be it because I only save it if it is.");
            println!("-- ERROR: Uh, report this? https://github.com/ry00001/microlaunch");
            std::process::exit(1);
        },
    }
}

fn do_autologin(data_ref: &EncryptedPersistentData) {
    let data = data_ref.clone();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .thread_name("microlaunch-login-worker-autologin")
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async move {
        do_full_login_process(data).await
    });
}

fn main() {
    let cli = CommandLine::parse();

    let fake_login_cfg = if let Some(e) = &config::CONFIG.experimental {
        e.fake_login
    } else {
        false
    };
    if cli.fake_login || fake_login_cfg {
        // Fake a login here
        println!("FAKE LOGIN - I trust you know what you're doing.");
        let ldata = auth::GameLoginData {
            sid: "0".into(),
            region: auth::GameRegion::Europe,
            max_expansion: auth::Expansion::Endwalker,
            playable: true,
            steam_username: None,
        };
        launch::launch_game(&ldata, auth::ClientLanguage::English, "0", false);

        return;
    }

    let persistent_ref = PERSISTENT.clone();
    let persistent = persistent_ref.lock();
    if persistent.autologin && !cli.force_gui {
        // do autologin here!
        println!("Autologin mode enabled");
        do_autologin(&persistent);
    }

    if cli.force_gui || !persistent.autologin {
        drop(persistent);
        run_gui();
    }
}
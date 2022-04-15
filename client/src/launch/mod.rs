// Launcher module
// This opens the game lol

use std::collections::HashMap;
use crate::{auth::{ClientLanguage, GameLoginData}, config::GameLaunchStrategy, integrity::{Repository, RepositoryId}};

fn build_cli_args_for_game(map: HashMap::<&str, &str>) -> String {
    let mut out: String = "".into();
    map.iter().for_each(|(k,v)| {
        out.push_str(&format!(" {k}={v}"))
    });
    out
}

pub fn launch_game(data: &GameLoginData, language: ClientLanguage, unique_patch_id: &str, is_steam: bool) {
    //let sid = &data.sid;
    let region = &data.region;

    let strategy = &crate::config::CONFIG.launcher.strategy;
    println!("== LAUNCHING THE GAME ==");
    println!("selected strategy: {:?}", strategy);

    let region_str = (*region as i32).to_string();
    let language_str = (language as i32).to_string();
    let max_exp = (data.max_expansion as i32).to_string();

    let mut argmap = HashMap::new();
    argmap.insert("DEV.DataPathType", "1");
    argmap.insert("DEV.MaxEntitledExpansionID", &max_exp);
    argmap.insert("DEV.TestSID", &unique_patch_id);
    argmap.insert("DEV.UseSqPack", "1");
    argmap.insert("SYS.Region", &region_str);
    argmap.insert("language", &language_str);

    if is_steam {
        argmap.insert("IsSteam", "1");
    }

    match strategy {
        GameLaunchStrategy::DirectLaunch => {
            if let Some(direct_config) = &crate::config::CONFIG.direct_launch {
                let game_binary_path =
                    std::path::Path::new(&direct_config.game_binary_path);

                let game_version = Repository(RepositoryId::Ffxiv).get_version().unwrap();
                println!("FFXIVGame version {game_version}");
                let game_args = build_cli_args_for_game(argmap);
                println!("game args: {game_args}");

                let mut command = std::process::Command::new(game_binary_path);
                let mut command = command.args(game_args.split(" "));
                if is_steam {
                    command = command.env("IS_FFXIV_LAUNCH_FROM_STEAM", "1");
                }
                println!("LAUNCHING DIRECTLY");
                let cmd = command.spawn().expect("failed to launch executable!");
                println!("Game PID = {}", cmd.id());
                std::thread::spawn(move || {
                    cmd.wait_with_output().unwrap();
                });
            } else {
                panic!("Direct launch strategy selected but no configuration found.");
            }
        },
        GameLaunchStrategy::Proton => {
            if let Some(proton_config) = &crate::config::CONFIG.proton {
                let proton_binary_path =
                    std::path::Path::new(&proton_config.proton_root_path).join("proton");

                let game_binary_path =
                    std::path::Path::new(&proton_config.game_binary_path);
        
                let game_version = Repository(RepositoryId::Ffxiv).get_version().unwrap();
                println!("FFXIVGame version {game_version}");
                let game_args = build_cli_args_for_game(argmap);
                println!("game args: {game_args}");
                
                // Oh this sucks
                let mut launch_cmd = if let None = &crate::config::CONFIG.launcher.prefix_command {
                    std::process::Command::new(&proton_binary_path)
                } else if let Some(pre_command) = &crate::config::CONFIG.launcher.prefix_command {
                    std::process::Command::new(pre_command)
                } else {
                    unreachable!()
                };

                let command = if let Some(_) = &crate::config::CONFIG.launcher.prefix_command {
                    launch_cmd.arg(proton_binary_path).arg("run")
                } else if let None = &crate::config::CONFIG.launcher.prefix_command {
                    launch_cmd.arg("run")
                } else {
                    unreachable!()
                };

                let command = command.arg(game_binary_path);
                let command = command.args(game_args.split(" "));
                let command = command.env("STEAM_COMPAT_DATA_PATH", &proton_config.compat_data_path);
                let mut command = command.env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &proton_config.compat_client_install_path);
                if is_steam {
                    command = command.env("IS_FFXIV_LAUNCH_FROM_STEAM", "1");
                }

                println!("LAUNCHING:");
                println!("{:?} {:?}", command.get_program(), command.get_args());
                let cmd = command
                    .spawn()
                    .expect("failed to launch executable!");
                println!("Proton PID = {}", cmd.id());
                std::thread::spawn(move || {
                    cmd.wait_with_output().unwrap();
                });
            } else {
                panic!("Proton selected as launch strategy, but configuration not found.");
            }
        }
    }

}
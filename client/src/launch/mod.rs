// Launcher module
// This opens the game lol

mod dalamud;

use std::{collections::HashMap, process::Stdio, borrow::Borrow, path::Path};
use crate::{auth::{ClientLanguage, GameLoginData}, config::GameLaunchStrategy, integrity::{Repository, RepositoryId}};

fn build_cli_args_for_game(map: HashMap::<&str, &str>) -> String {
    let mut out: String = "".into();
    map.iter().for_each(|(k,v)| {
        out.push_str(&format!(" {k}={v}"))
    });
    out.trim_start().to_string()
}

fn to_z_path(path: &std::path::PathBuf) -> String {
    "Z:".to_owned() + &path.to_string_lossy().replace("/", r#"\"#)
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

                let wine64_bin_path = proton_binary_path.parent().unwrap().join("files").join("bin").join("wine64");
        
                let game_version = Repository(RepositoryId::Ffxiv).get_version().unwrap();
                println!("FFXIVGame version {game_version}");
                let game_args = build_cli_args_for_game(argmap);
                println!("game args: {game_args}");

                let use_dalamud = if let Some(experimental) = &crate::config::CONFIG.experimental {
                    experimental.use_dalamud
                } else {
                    false
                };

                let wineprefix = std::path::Path::new(&proton_config.compat_data_path).join("pfx");
                
                // Oh this sucks
                // Oh this is about to suck even more
                let mut launch_cmd = if use_dalamud {
                    std::process::Command::new(&wine64_bin_path)
                } else if let None = &crate::config::CONFIG.launcher.prefix_command {
                    std::process::Command::new(&wine64_bin_path)
                } else if let Some(pre_command) = &crate::config::CONFIG.launcher.prefix_command {
                    std::process::Command::new(pre_command)
                } else {
                    unreachable!()
                };

                let mut command = if use_dalamud {
                    launch_cmd.arg(
                        "dalamud/DalamudWineHelper.exe"
                    )
                        //.arg(to_z_path(&Path::new(&proton_config.game_binary_path).to_path_buf()))
                } else if let Some(_) = &crate::config::CONFIG.launcher.prefix_command {
                    launch_cmd.arg(&wine64_bin_path)
                } else if let None = &crate::config::CONFIG.launcher.prefix_command {
                    &mut launch_cmd
                } else {
                    unreachable!()
                };

                if !use_dalamud {
                    command = command.arg(game_binary_path);
                }
                if use_dalamud {
                    let bin_path = &Path::new(&proton_config.game_binary_path).to_path_buf();
                    //let arg_string = r#"""#.to_owned() + &to_z_path(bin_path) + r#"""#;
                    let arg_string = to_z_path(bin_path);
                    command = command.arg(arg_string).args(game_args.split(" "));
                } else {
                    command = command.args(game_args.split(" "));
                }
                let command = command.env("STEAM_COMPAT_DATA_PATH", &proton_config.compat_data_path);
                let mut command = command.env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &proton_config.compat_client_install_path);
                if is_steam {
                    command = command.env("IS_FFXIV_LAUNCH_FROM_STEAM", "1");
                }

                command = command.env("WINEPREFIX", wineprefix.as_os_str());
                command = command.env("WINEDEBUG", "-all"); // Noisy!!!

                println!("LAUNCHING:");
                println!("{:?} {:?}", command.get_program(), command.get_args());
                let cmd = command
                    .spawn()
                    .expect("failed to launch executable!");
                println!("Proton PID = {}", cmd.id());

                if let Some(experimental) = &crate::config::CONFIG.experimental {
                    if experimental.use_dalamud {
                        println!("Beginning Dalamud injection process now.");
                        // Sleep to make sure the game is actually launched
                        std::thread::sleep(std::time::Duration::from_millis(2000));
                        // Invoke winedbg
                        let mut winedbg = std::process::Command::new(&wine64_bin_path);
                        let winedbg = winedbg.arg("winedbg");
                        let winedbg = winedbg.arg("--command");
                        let winedbg = winedbg.arg("info proc");
                        let winedbg = winedbg.env("WINEPREFIX", wineprefix.as_os_str());
                        let winedbg = winedbg.stdout(Stdio::piped());
                        let wdbg_cmd = winedbg.spawn()
                            .expect("failed to launch winedbg!");
                        let wdbg_out = wdbg_cmd.wait_with_output().unwrap();
                        let stdout_str = String::from_utf8(wdbg_out.stdout).unwrap();
                        // Now figure out what pid ffxiv_dx11.exe is
                        let binary_name_cow = game_binary_path.file_name().unwrap().to_string_lossy();
                        let binary_name: &str = binary_name_cow.borrow();
                        let splits_newline = stdout_str.split("\n");
                        let mut pid = 0;
                        for i in splits_newline {
                            if i.contains(binary_name) {
                                // We found it
                                let line = i.trim();
                                let mut line_split = line.split_whitespace();
                                let pid_hex = line_split.next().unwrap().trim();
                                pid = u32::from_str_radix(pid_hex, 16).unwrap();
                            }
                        }
                        println!("ffxiv_dx11.exe = {}", pid);
                        if pid == 0 {
                            panic!("pid = 0??? ffxiv_dx11.exe not found!! Maybe winedbg is being uncooperative!");
                        }
                        // We have the pid, let's go inject Dalamud
                        let dalamud_path = wineprefix.clone();
                        let dalamud_path = dalamud_path.join("drive_c");
                        let dalamud_path = dalamud_path.join("dalamud");
                        let start_info = dalamud::DalamudStartInfo::get(
                            &wineprefix,
                            &dalamud_path
                        );
                        println!("Dalamud start info established - injecting now.");
                        println!("{:#?}", start_info);
                        let start_info_json = serde_json::to_string(&start_info).unwrap();
                        let start_info_b64 = data_encoding::BASE64.encode(start_info_json.as_bytes());
                        //let dotnet_path = dalamud_path.join("dotnet").join("dotnet.exe");

                        let mut dalamud_injector = std::process::Command::new(&wine64_bin_path);
                        let dalamud_injector = dalamud_injector.arg(r#"C:\dalamud\rel\Dalamud.Injector.exe"#);
                        let dalamud_injector = dalamud_injector.arg(pid.to_string());
                        let dalamud_injector = dalamud_injector.arg(start_info_b64);
                        let dalamud_injector = dalamud_injector.env("WINEPREFIX", wineprefix.as_os_str());
                        let dalamud_injector = dalamud_injector.env("WINEDEBUG", "-all");
                        let dalamud_injector = dalamud_injector.env("DALAMUD_RUNTIME", r#"C:\dalamud\dotnet"#);
                        println!("{:?} {:?}", dalamud_injector.get_program(), dalamud_injector.get_args());
                        let injector_child = dalamud_injector
                            .spawn()
                            .expect("failed to run injector");
                        std::thread::spawn(move || {
                            injector_child.wait_with_output().unwrap();
                        });
                    }
                }

                std::thread::spawn(move || {
                    cmd.wait_with_output().unwrap();
                });
            } else {
                panic!("Proton selected as launch strategy, but configuration not found.");
            }
        }
    }

}
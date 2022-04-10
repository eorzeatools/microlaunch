use parking_lot::Mutex;

use eframe::{egui, epi};
use egui::*;

use crate::{auth::{AccountType, Platform, GameLoginResult, GameLoginData, ClientLanguage}, session::RegisterSessionResult};

pub struct MicrolaunchApp {
    title: Option<String>
}

impl Default for MicrolaunchApp {
    fn default() -> Self {
        Self {
            title: Some(format!("microlaunch ver.{} by rin", env!("CARGO_PKG_VERSION")))
        }
    }
}

#[derive(Clone)]
struct LoginPhaseData {
    username: String,
    password: String,
    otp: String,
    account_type: AccountType,
    platform: Platform,

    error_text: Option<String>
}

#[derive(Clone)]
enum Phase {
    Login(LoginPhaseData),
    ReadyToLaunch((GameLoginData, RegisterSessionResult, Platform)),
    #[allow(dead_code)]
    Launching
}

struct State {
    pub uuid: String
}

lazy_static::lazy_static! {
    static ref NEXT_PHASE: Mutex<Option<Box<Phase>>> = {
        Mutex::new(None)
    };

    static ref STATE: Mutex<Box<State>> = {
        let st = State {
            uuid: crate::auth::generate_computer_id()
        };
        Mutex::new(Box::new(st))
    };

    static ref PHASE: Mutex<Box<Phase>> = {
        let p = Phase::Login(LoginPhaseData {
            username: "".into(),
            password: "".into(),
            otp: "".into(),
            account_type: AccountType::Subscription,
            platform: Platform::SqexStore,
            error_text: None
        });
        Mutex::new(Box::new(p))
    };
}

impl epi::App for MicrolaunchApp {
    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        ctx.set_pixels_per_point(1.2);
        
        let mut style = (*ctx.style()).clone();

        let mut spc = egui::style::Spacing::default();
        spc.item_spacing = vec2(8.0, 6.0);
        style.spacing = spc;

        ctx.set_style(style);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        if let Some(x) = NEXT_PHASE.lock().as_ref() {
            *PHASE.lock().as_mut() = *x.clone();
        }

        match PHASE.lock().as_mut() {
            mut x @ Phase::Login { .. } => self.do_loginui(ctx, frame, &mut x),
            mut y @ Phase::ReadyToLaunch { .. } => self.do_readyui(ctx, frame, &mut y),
            _ => todo!()
        }

        Window::new("debug stuff")
            .id(Id::new("ul-debug-window"))
            .show(ctx, |ui|
        {
            let state = STATE.lock();

            ui.label(format!("computer unique SE-UUID: {}", state.uuid));
        });
    }

    fn name(&self) -> &str {
        self.title.as_deref().or(Some("microlaunch")).unwrap()
    }
}

impl MicrolaunchApp {
    fn do_readyui(&mut self, ctx: &egui::Context, _frame: &epi::Frame, phase: &mut Phase) {
        if let Phase::ReadyToLaunch((data, register, steam)) = phase {
            if let RegisterSessionResult::Ok(register_token) = register {
                CentralPanel::default()
                    .frame(Frame::none())
                .show(ctx, |ui| {
                    let offset = vec2(0.0, 0.0);

                    ui.allocate_ui_at_rect(
                        Rect::from_two_pos(pos2(20.0, 20.0), pos2(999.0, 999.0)),
                    |ui| {
                        ui.heading("microlaunch");
                        ui.label("by rin 2022");
                    });

                    let win = Window::new("ready to play")
                        .id(Id::new("ul-readytoplay-window"))
                        .title_bar(false)
                        .anchor(Align2::CENTER_CENTER, offset);

                    win.show(ctx, |ui| {
                        ui.label(format!("unique patch ID: {}", register_token));
                        ui.label(format!("game edition: {:?}", data.max_expansion));
                        ui.label(format!("region: {:?}", data.region));
                        if ui.button("launch the game!").clicked() {
                            // LAUNCH!
                            crate::launch::launch_game(data, ClientLanguage::English, register_token, *steam == Platform::Steam);
                        }
                    })
                });
            }

        }
    }

    fn do_loginui(&mut self, ctx: &egui::Context, _frame: &epi::Frame, phase: &mut Phase) {
        if let Phase::Login(data) = phase {
            CentralPanel::default()
                .frame(Frame::none())
            .show(ctx, |ui| {
                let offset = vec2(0.0, 0.0);

                ui.allocate_ui_at_rect(
                    Rect::from_two_pos(pos2(20.0, 20.0), pos2(999.0, 999.0)),
                |ui| {
                    ui.heading("microlaunch");
                    ui.label("by rin 2022");
                });


                let win = Window::new("log in")
                    .id(Id::new("ul-login-window"))
                    .title_bar(false)
                    .anchor(Align2::CENTER_CENTER, offset);

                win.show(ctx, |ui| {
                    ui.add(TextEdit::singleline(&mut data.username).hint_text("Square Enix ID..."));

                    ui.add(TextEdit::singleline(&mut data.password).password(true).hint_text("Password..."));

                    ui.add(TextEdit::singleline(&mut data.otp).hint_text("One-time password... (leave blank if n/a)"));

                    ComboBox::from_label("Platform")
                        .selected_text(
                            match data.platform {
                                Platform::SqexStore => "Square Enix",
                                Platform::Steam => "Steam"
                            }
                        )
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut data.platform, Platform::SqexStore,
                                "Square Enix");
                            ui.selectable_value(&mut data.platform, Platform::Steam,
                                "Steam");
                        });
                    
                    ComboBox::from_label("Account type")
                        .selected_text(
                            match data.account_type {
                                AccountType::Subscription => "Full game",
                                AccountType::FreeTrial => "Free trial"
                            }
                        )
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut data.account_type, AccountType::Subscription,
                                "Full game");
                            ui.selectable_value(&mut data.account_type, AccountType::FreeTrial,
                                "Free trial");
                        });
                    
                    if let Some(text) = &data.error_text {
                        ui.colored_label(egui::Color32::RED, text);
                    }

                    if ui.button("Log in").clicked() {
                        if data.platform == Platform::Steam {
                            // Steamworks time!
                            let auth_res = crate::auth::steam::init(&data.account_type);

                            if let Err(x) = auth_res {
                                // Oops
                                println!("Steamworks initialise error");
                                println!("{}", x);

                                data.error_text = 
                                    Some(format!(
                                        "{}\n{}{}",
                                        "Failed to initialise Steam!",
                                        "Please make sure your Steam account owns FINAL FANTASY XIV Online,",
                                        "you are logged into it, and Steam is running on your computer."
                                    ));
                                return;
                            }

                            return;
                        }

                        let fucker = data.clone();
                        let rt = tokio::runtime::Builder::new_multi_thread()
                            .thread_name("microlaunch-login-worker")
                            .enable_all()
                            .build()
                            .unwrap();
                        let a = rt.block_on(async move {
                            crate::auth::login_oauth(
                                &fucker.username.clone(),
                                &fucker.password.clone(),
                                &fucker.otp.clone(),
                                fucker.account_type.clone() == AccountType::FreeTrial,
                                fucker.platform.clone() == Platform::Steam,
                                crate::auth::GameRegion::Europe,
                                None
                            ).await
                        });
                        match a {
                            GameLoginResult::Successful(ldata) => {
                                let d2 = ldata.clone();
                                let register = rt.block_on(async move {
                                    crate::session::register_session(&d2).await
                                });
                                match register {
                                    RegisterSessionResult::Ok(_) => {},
                                    RegisterSessionResult::GamePatchNeeded => {
                                        data.error_text = Some("Game patch is required! microlaunch does not currently do this, sorry!".into());
                                    },
                                    RegisterSessionResult::BootPatchNeeded => {
                                        data.error_text = Some("Boot patch is required! microlaunch does not currently do this, sorry!".into());
                                    },
                                }
                                *NEXT_PHASE.lock() = Some(Box::new(Phase::ReadyToLaunch((ldata, register, data.platform))));
                            },
                            GameLoginResult::SteamLinkRequired => {
                                data.error_text = Some("Steam link required - please link your Square Enix account to Steam through Windows".into());
                            },
                            GameLoginResult::Error => {
                                data.error_text = Some("An error has occurred - username/password invalid?".into());
                            }
                        }
                    }
                });
            });

        } else {
            unreachable!()
        }
    }
}
use std::borrow::BorrowMut;
use parking_lot::Mutex;

use eframe::{egui, epi};
use egui::*;

use crate::auth::{AccountType, Platform};

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
    platform: Platform
}

enum Phase {
    Login(LoginPhaseData),
    #[allow(dead_code)]
    ReadyToLaunch,
    #[allow(dead_code)]
    Launching
}

struct State {
    pub uuid: String
}

lazy_static::lazy_static! {
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
            platform: Platform::SqexStore
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
        match PHASE.lock().borrow_mut().as_mut() {
            x @ Phase::Login { .. } => self.do_loginui(ctx, frame, x),
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

                    if ui.button("Log in").clicked() {
                        let fucker = data.clone();
                        let rt = tokio::runtime::Builder::new_multi_thread()
                            .thread_name("microlaunch-login-worker")
                            .enable_all()
                            .build()
                            .unwrap();
                        rt.block_on(async move {
                            crate::auth::login_oauth(
                                &fucker.username.clone(),
                                &fucker.password.clone(),
                                &fucker.otp.clone(),
                                fucker.account_type.clone() == AccountType::FreeTrial,
                                fucker.platform.clone() == Platform::Steam,
                                crate::auth::GameRegion::Europe,
                                None
                            ).await;
                        });
                    }
                });
            });

        } else {
            unreachable!()
        }
    }
}
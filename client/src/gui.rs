use std::sync::Mutex;

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

lazy_static::lazy_static! {
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
        match PHASE.lock().unwrap().as_mut() {
            x @ Phase::Login { .. } => self.do_loginui(ctx, frame, x),
            _ => todo!()
        }  
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
                        // do something
                    }
                });
            });

        } else {
            unreachable!()
        }
    }
}

/*
fn custom_frame(ctx: &egui::Context, frame: &epi::Frame, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::*;

    let text_color = ctx.style().visuals.text_color();

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui|
    {
        let rect = ui.min_rect().shrink2(vec2(0.0, 250.0));
        let painter = ui.painter();
        let height = 30.0;

        painter.rect(
            rect.shrink(1.0),
            10.0,
            ctx.style().visuals.window_fill(),
            Stroke::new(1.0, text_color),
        );

        painter.text(
            rect.left_top() + vec2(10.0, 10.0),
            Align2::LEFT_TOP,
            "microlaunch - by rin (2022)",
            FontId::proportional(14.0),
            text_color
        );

        painter.text(
            rect.center_top() + vec2(0.0, height / 2.0),
            Align2::CENTER_CENTER,
            title,
            FontId::proportional(height - 2.0),
            text_color,
        );

        let title_bar_rect = {
            let mut rect = rect;
            rect.max.y = rect.min.y + height + 5.0;
            rect
        };
        let title_bar_response =
            ui.interact(title_bar_rect, egui::Id::new("title_bar"), egui::Sense::drag());
        if title_bar_response.drag_started() {
            frame.drag_window();
        }

        let content_rect = {
            let mut rect = rect;
            rect.min.y = title_bar_rect.max.y;
            rect.set_left(8.0);
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}
*/
use iced::Application;
use iced::Checkbox;
use iced::Column;
use iced::Command;
use iced::Row;
use iced::Text;
use iced::TextInput;
use iced::text_input;

fn get_version() -> String {
    let hash = &env!("GIT_HASH")[1..8];
    format!("{}@{}/{}", env!("CARGO_PKG_VERSION"), env!("GIT_BRANCH"), hash)
}

macro_rules! color {
    ($nm:ident ($r:literal $g:literal $b:literal)) => {
        lazy_static::lazy_static! {
            static ref $nm: iced::Color = iced::Color::from_rgb8($r, $g, $b);
        }
    }
}

color!(WHITE (232 232 232));
color!(GRAY (64 64 64));
color!(LIGHTGRAY (107 107 107));
color!(DARKGRAY (20 20 20));
color!(SLIGHTLYLIGHTERGRAY (26 26 26));

struct UlTextInputStylesheet;
impl text_input::StyleSheet for UlTextInputStylesheet {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: iced::Background::Color(*DARKGRAY),
            ..Default::default()
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: iced::Background::Color(*SLIGHTLYLIGHTERGRAY),
            ..Default::default()
        }
    }

    fn placeholder_color(&self) -> iced::Color {
        *LIGHTGRAY
    }

    fn value_color(&self) -> iced::Color {
        *WHITE
    }

    fn selection_color(&self) -> iced::Color {
        Default::default()
    }
}

#[derive(Default)]
pub struct LoginState {
    username: String,
    password: String,
    otp: String,
    save_info: bool,

    username_state: text_input::State,
    pw_state: text_input::State,
    otp_state: text_input::State,
}

#[derive(Clone, Debug)]
pub enum Message {
    // Login
    UsernameChanged(String),
    PasswordChanged(String),
    OnetimeChanged(String),
    SaveInfoToggled(bool),
}

pub enum MicrolaunchApplication {
    Login(LoginState),
    ReadyToPlay,
}

impl Application for MicrolaunchApplication {
    type Executor = iced_futures::executor::Tokio;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (MicrolaunchApplication::Login(LoginState::default()), Command::none())
    }

    fn title(&self) -> String {
        format!("microlaunch ver. {} - by rin, 2022", get_version())
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Self::Message> {

        match self {
            MicrolaunchApplication::Login(state) => {
                match message {
                    Message::UsernameChanged(x) => {
                        state.username = x;
                        Command::none()
                    },
                    Message::PasswordChanged(x) => {
                        state.password = x;
                        Command::none()
                    },
                    Message::OnetimeChanged(x) => {
                        state.otp = x;
                        Command::none()
                    },
                    Message::SaveInfoToggled(x) => {
                        state.save_info = x;
                        Command::none()
                    }
                }
            },
            MicrolaunchApplication::ReadyToPlay => todo!(),
        }
    
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::from_rgb8(41, 41, 41)
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        match self {
            MicrolaunchApplication::Login(state) => {
                Column::new()
                .padding(32)
                .spacing(10)
                    .push(
                        TextInput::new(
                            &mut state.username_state,
                            "Square Enix ID...",
                            &state.username,
                            Message::UsernameChanged
                        )
                        .padding(10)
                        .style(UlTextInputStylesheet)
                    )
                    .push(
                        TextInput::new(
                            &mut state.pw_state,
                            "Password...",
                            &state.password,
                            Message::PasswordChanged
                        )
                        .padding(10)
                        .password()
                        .style(UlTextInputStylesheet)
                    )
                    .push(
                        TextInput::new(
                            &mut state.otp_state,
                            "One-time code (optional)...",
                            &state.otp,
                            Message::OnetimeChanged
                        )
                        .padding(10)
                        .style(UlTextInputStylesheet)
                    )
                    .push(
                        Row::new()
                            .padding(10)
                            .push(
                                Checkbox::new(
                                    state.save_info,
                                    "",
                                    Message::SaveInfoToggled
                                )
                            )
                            .push(
                                // WORKAROUND: Checkbox does not provide text colour
                                Text::new("Save information").color(*WHITE)
                            )
                    )
                    .into()
            },
            MicrolaunchApplication::ReadyToPlay => {
                todo!()
            },
        }
    }
}
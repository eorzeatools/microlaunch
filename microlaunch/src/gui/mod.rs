mod style;

use iced::Application;
use iced::Checkbox;
use iced::Column;
use iced::Command;
use iced::PickList;
use iced::Row;
use iced::Text;
use iced::TextInput;
use iced::VerticalAlignment;
use iced::pick_list;
use iced::text_input;
use crate::auth::Platform;
use crate::auth::AccountType;

fn get_version() -> String {
    let hash = &env!("GIT_HASH")[1..8];
    format!("{}@{}/{}", env!("CARGO_PKG_VERSION"), env!("GIT_BRANCH"), hash)
}

pub struct LoginState {
    username: String,
    password: String,
    otp: String,
    save_info: bool,
    platform: Option<Platform>,
    account_type: Option<AccountType>,

    username_state: text_input::State,
    pw_state: text_input::State,
    otp_state: text_input::State,
    platform_state: pick_list::State<Platform>,
    acct_type_state: pick_list::State<AccountType>,
}

impl Default for LoginState {
    fn default() -> Self {
        Self {
            username: Default::default(),
            password: Default::default(),
            otp: Default::default(),
            save_info: Default::default(),
            platform: Some(Platform::Placeholder),
            account_type: Some(AccountType::Placeholder),

            username_state: Default::default(),
            pw_state: Default::default(),
            otp_state: Default::default(),
            platform_state: Default::default(),
            acct_type_state: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    // Login
    UsernameChanged(String),
    PasswordChanged(String),
    OnetimeChanged(String),
    SaveInfoToggled(bool),
    PlatformChanged(Platform),
    AccountTypeChanged(AccountType),
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
                    },
                    Message::PlatformChanged(x) => {
                        state.platform = Some(x);
                        Command::none()
                    },
                    Message::AccountTypeChanged(x) => {
                        state.account_type = Some(x);
                        Command::none()
                    },
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
                        .style(style::UlTextInputStylesheet)
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
                        .style(style::UlTextInputStylesheet)
                    )
                    .push(
                        TextInput::new(
                            &mut state.otp_state,
                            "One-time code (optional)...",
                            &state.otp,
                            Message::OnetimeChanged
                        )
                        .padding(10)
                        .style(style::UlTextInputStylesheet)
                    )
                    .push(
                        Row::new()
                        .spacing(10)
                        .push(
                            PickList::new(
                                &mut state.platform_state,
                                vec![
                                    Platform::SqexStore,
                                    Platform::Steam
                                ],
                                state.platform,
                                Message::PlatformChanged
                            )
                            .style(style::UlPickListStylesheet)
                        )
                        .push(
                            PickList::new(
                                &mut state.acct_type_state,
                                vec![
                                    AccountType::Subscription,
                                    AccountType::FreeTrial
                                ],
                                state.account_type,
                                Message::AccountTypeChanged
                            )
                            .style(style::UlPickListStylesheet)
                        )
                        .push(
                            Row::new()
                            .padding(5) // WORKAROUND: What the fuck, iced?
                            .push(
                                Row::new()
                                .spacing(5)
                                .push(
                                    Checkbox::new(
                                        state.save_info,
                                        "",
                                        Message::SaveInfoToggled
                                    )
                                    .style(style::UlCheckboxStylesheet)
                                    .spacing(0)
                                )
                                .push(
                                    // WORKAROUND: Checkbox does not provide text colour
                                    Text::new("Save information").color(*style::WHITE)
                                )
                            )
                        )
                    )
                    .push(
                        Row::new()
                            .padding(2)

                    )
                    .into()
            },
            MicrolaunchApplication::ReadyToPlay => {
                todo!()
            },
        }
    }
}
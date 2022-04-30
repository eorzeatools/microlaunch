mod style;

use iced::Application;
use iced::Button;
use iced::Checkbox;
use iced::Column;
use iced::Command;
use iced::PickList;
use iced::Row;
use iced::Text;
use iced::TextInput;
use iced::button;
use iced::pick_list;
use iced::text_input;
use crate::auth::ClientLanguage;
use crate::auth::GameLoginData;
use crate::auth::GameLoginResult;
use crate::auth::GameRegion;
use crate::auth::Platform;
use crate::auth::AccountType;
use crate::integrity::Repository;
use crate::integrity::RepositoryId;
use crate::session::RegisterSessionResult;

fn get_version() -> String {
    let hash = &env!("GIT_HASH")[0..8];
    format!("{}@{}/{}", env!("CARGO_PKG_VERSION"), env!("GIT_BRANCH"), hash)
}

pub struct LoginState {
    username: String,
    password: String,
    otp: String,
    save_info: bool,
    platform: Option<Platform>,
    account_type: Option<AccountType>,

    error_text: Option<String>,
    disable_button: bool,

    username_state: text_input::State,
    pw_state: text_input::State,
    otp_state: text_input::State,
    platform_state: pick_list::State<Platform>,
    acct_type_state: pick_list::State<AccountType>,
    button_state: button::State,
}

impl Default for LoginState {
    fn default() -> Self {
        let mut t = Self {
            username: Default::default(),
            password: Default::default(),
            otp: Default::default(),
            save_info: Default::default(),
            platform: Some(Platform::SqexStore),
            account_type: Some(AccountType::Subscription),

            error_text: None,
            disable_button: false,

            username_state: Default::default(),
            pw_state: Default::default(),
            otp_state: Default::default(),
            platform_state: Default::default(),
            acct_type_state: Default::default(),
            button_state: Default::default(),
        };

        let persist = crate::persist::PERSISTENT.lock();
        if persist.autologin {
            t.username = persist.sqex_id.clone();
            t.password = persist.password.clone();
            t.platform = Some(persist.platform);
            t.account_type = Some(persist.account_type);
            t.save_info = true;
        }
        drop(persist);

        t
    }
}

#[derive(Default)]
pub struct ReadyState {
    play_button_state: button::State,
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
    LoginClicked,
    OauthLoginComplete(GameLoginResult),
    RegisterSessionComplete(GameLoginData, RegisterSessionResult),

    // Launch
    LaunchGameButtonPressed,
}

pub enum MicrolaunchApplication {
    Login(LoginState),
    ReadyToPlay {
        ldata: GameLoginData,
        sid: String,
        steam: bool,
        state: ReadyState,
    },
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
                    Message::LoginClicked => {
                        // Let's log in
                        let ticket = if state.platform == Some(Platform::Steam) {
                            let sworks = crate::auth::steam::init(&state.account_type.unwrap());
                            if let Err(_x) = sworks {
                                // error!
                                state.error_text = Some(
                                    "Error initialising Steam. Please make sure you have the Steam client running, and logged into an account that owns Final Fantasy XIV.".into()
                                );
                                return Command::none();
                            };
                            let tick = crate::auth::steam::get_ticket();
                            Some(tick)
                        } else {
                            None
                        };

                        let fut = {
                            let un = state.username.clone();
                            let pw = state.password.clone();
                            let otp = state.otp.clone();
                            crate::auth::login_oauth(
                                un,
                                pw,
                                otp,
                                state.account_type == Some(AccountType::FreeTrial),
                                state.platform == Some(Platform::Steam),
                                GameRegion::Europe,
                                ticket,
                            )
                        };

                        state.disable_button = true;

                        Command::perform(fut, move |x| {
                            Message::OauthLoginComplete(x)
                        })
                    },
                    Message::OauthLoginComplete(x) => {
                        match x {
                            GameLoginResult::Successful(ldata) => {
                                let ld2 = ldata.clone();
                                let fut = crate::session::register_session(ld2);
                                Command::perform(fut, move |v| {
                                    Message::RegisterSessionComplete(ldata.clone(), v)
                                })
                            },
                            GameLoginResult::SteamLinkRequired => {
                                state.error_text = Some("Steam link is required - please use the official launcher on a Windows system to link this SEID to your Steam account.".into());
                                state.disable_button = false;
                                Command::none()
                            },
                            GameLoginResult::WrongSteamAccount => {
                                state.error_text = Some("This Square Enix ID is linked to a different Steam account than the one you are logged into.".into());
                                state.disable_button = false;
                                Command::none()
                            },
                            GameLoginResult::NoMoreGameTime => {
                                state.error_text = Some("Your subscription has expired. Please add game time at the Mog Station. (https://sqex.to/Msp)".into());
                                state.disable_button = false;
                                Command::none()
                            },
                            GameLoginResult::TermsNotAccepted => {
                                state.error_text = Some("You have not accepted Square Enix's terms and conditions. Please use the official launcher to do this.".into());
                                state.disable_button = false;
                                Command::none()
                            },
                            GameLoginResult::NoServiceAccount => {
                                state.error_text = Some("This Square Enix ID does not have any Final Fantasy XIV service accounts registered. Please add one at the Mog Station. (https://sqex.to/Msp)".into());
                                state.disable_button = false;
                                Command::none()
                            },
                            GameLoginResult::Error => {
                                state.error_text = Some("An error has occurred. Please make sure your username and password are correct, and if you use a one-time code, enter it properly.".into());
                                state.disable_button = false;
                                Command::none()
                            },
                        }
                    },
                    Message::RegisterSessionComplete(ldata, x) => {
                        match x {
                            RegisterSessionResult::Ok(uid) => {
                                if state.save_info {
                                    // Save info
                                    let mut persistent = crate::persist::PERSISTENT.lock();
                                    persistent.sqex_id = state.username.clone();
                                    persistent.password = state.password.clone();
                                    persistent.platform = state.platform.unwrap().clone();
                                    persistent.account_type = state.account_type.unwrap().clone();
                                    persistent.autologin = true;
                                    drop(persistent);
                                    crate::persist::write_persistent_data();
                                } else {
                                    // Erase saved info, disable autologin
                                    let mut persistent = crate::persist::PERSISTENT.lock();
                                    persistent.sqex_id = "".into();
                                    persistent.password = "".into();
                                    persistent.platform = Platform::SqexStore;
                                    persistent.account_type = AccountType::Subscription;
                                    persistent.autologin = false;
                                    drop(persistent);
                                    crate::persist::write_persistent_data();
                                }

                                *self = Self::ReadyToPlay {
                                    ldata,
                                    sid: uid,
                                    steam: state.platform == Some(Platform::Steam),
                                    state: ReadyState::default()
                                };
                                Command::none()
                            },
                            RegisterSessionResult::GamePatchNeeded => {
                                state.error_text = Some("Final Fantasy XIV needs to update. Please update your game through any method necessary. (Game patch)".into());
                                state.disable_button = false;
                                Command::none()
                            },
                            RegisterSessionResult::BootPatchNeeded => {
                                state.error_text = Some("Final Fantasy XIV needs to update. Please update your game through any method necessary. (Boot patch)".into());
                                state.disable_button = false;
                                Command::none()
                            },
                        }
                    },
                    Message::LaunchGameButtonPressed => unreachable!(),
                }
            },
            MicrolaunchApplication::ReadyToPlay { state: _, ldata, sid, steam } => {
                match message {
                    Message::UsernameChanged(_) => unreachable!(),
                    Message::PasswordChanged(_) => unreachable!(),
                    Message::OnetimeChanged(_) => unreachable!(),
                    Message::SaveInfoToggled(_) => unreachable!(),
                    Message::PlatformChanged(_) => unreachable!(),
                    Message::AccountTypeChanged(_) => unreachable!(),
                    Message::LoginClicked => unreachable!(),
                    Message::OauthLoginComplete(_) => unreachable!(),
                    Message::RegisterSessionComplete(_, _) => unreachable!(),

                    Message::LaunchGameButtonPressed => {
                        // launch the game
                        crate::launch::launch_game(
                            ldata,
                            ClientLanguage::English,
                            sid,
                            *steam
                        );
                        Command::none()
                    },
                }
            },
        }
    
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::from_rgb8(41, 41, 41)
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        match self {
            MicrolaunchApplication::Login(state) => {
                let mut col = Column::new()
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
                            .padding(7)
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
                            .padding(7)
                        )
                        .push(
                            // If anyone can tell me a better way to get this fucking panel
                            // to lay out properly without using 3 nested Rows, please do
                            // @lostkagamine twitter
                            Row::new()
                            // WORKAROUND: This .padding needs to be equal
                            // to the padding of the PickLists
                            // otherwise this lays out wrong
                            // I love GUI programming
                            .padding(7)
                            .push(
                                Row::new()
                                .spacing(5)
                                .push(
                                    Checkbox::new(
                                        state.save_info,
                                        // WORKAROUND: The Checkbox label does not
                                        // allow you to set the text colour
                                        // so we don't use it and use our own label instead
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
                        .push(
                            iced::Space::new(iced::Length::FillPortion(3), iced::Length::Fill)
                        )
                        .push(
                            if state.disable_button {
                                iced::Button::new(
                                    &mut state.button_state,
                                    Text::new("logging in...")
                                        .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .padding(7)
                                .width(iced::Length::FillPortion(2))
                                .style(style::UlButtonStylesheet)
                            } else {
                                iced::Button::new(
                                    &mut state.button_state,
                                    Text::new("log in!")
                                        .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .on_press(Message::LoginClicked)
                                .padding(7)
                                .width(iced::Length::FillPortion(2))
                                .style(style::UlButtonStylesheet)
                            }
                        )
                    );
                if let Some(x) = &state.error_text {
                    col = col.push(
                        iced::Space::new(iced::Length::Fill, iced::Length::Units(100))
                    );
                    col = col.push(
                        Text::new(x).color(*style::RED)
                    );
                }
                col.into()
            },
            MicrolaunchApplication::ReadyToPlay { state, ldata, .. } => {
                let mut row = Row::new()
                .padding(32)
                .spacing(10);

                let mut col = Column::new()
                .push(
                    Text::new(&format!("Game edition: {:?}", &ldata.max_expansion))
                        .color(*style::WHITE)
                );
                col = col.push(
                    Text::new(&format!("Version (ffxiv): {}", Repository(RepositoryId::Ffxiv).get_version().unwrap()))
                        .color(*style::WHITE)
                );
                col = col.push(
                    iced::Space::with_width(iced::Length::FillPortion(3))
                );

                if ldata.max_expansion as i32 > 0 {
                    for i in 1..=(ldata.max_expansion as i32) {
                        col = col.push(
                            Text::new(
                                &format!("Version (ex{i}): {}",
                                    Repository(i.try_into().unwrap()).get_version().unwrap())
                            )
                            .color(*style::WHITE)
                        )
                    }
                }

                col = col.push(
                    Button::new(
                        &mut state.play_button_state,
                        Text::new("launch the game!")
                            .horizontal_alignment(iced::HorizontalAlignment::Center)
                    )
                    .on_press(
                        Message::LaunchGameButtonPressed
                    )
                    .padding(7)
                    .style(style::UlButtonStylesheet)
                );

                row = row.push(col);
                
                row.into()
            },
        }
    }
}
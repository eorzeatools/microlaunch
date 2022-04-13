use std::collections::HashMap;

pub struct GameArguments(HashMap<String, String>);

impl GameArguments {
    pub fn new(from: HashMap<String, String>) -> Self {
        Self(from)
    }

    pub fn into_args_for_game(self) -> Vec<String> {
        let mut out: String = "".into();
        self.0.iter().for_each(|(k,v)| {
            out.push_str(&format!(" {k}={v}"))
        });
        out.split(" ").into_iter().map(|x| x.to_owned()).collect::<Vec<String>>()
    }
}

pub trait LaunchStrategyImpl {
    fn launch(game_directory_path: String, args: GameArguments, env: HashMap<String, String>);
}
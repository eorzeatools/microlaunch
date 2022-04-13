use std::collections::HashMap;
use super::strategy::{LaunchStrategyImpl, self};

pub struct ProtonLaunchStrategy;

impl LaunchStrategyImpl for ProtonLaunchStrategy {
    fn launch(game_directory_path: String, args: strategy::GameArguments, env: HashMap<String, String>) {
        
    }
}
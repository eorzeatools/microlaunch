use std::path::{PathBuf, Path};

pub mod hash;

pub enum RepositoryId {
    Boot, // launcher?
    Ffxiv, // ffxiv game
    Ex1, // heavensward
    Ex2, // stormblood
    Ex3, // shadowbringers
    Ex4 // endwalker
}

pub struct Repository(pub RepositoryId);

fn get_path_for_exid(base: &Path, exid: u8) -> PathBuf {
    base.join("game").join("sqpack").join(format!("ex{}", exid)).join(format!("ex{}.ver", exid))
}

impl Repository {
    pub fn get_version_file_path(&self) -> PathBuf {
        let base_path = std::path::Path::new(&crate::config::CONFIG.launcher.game_path);

        match self.0 {
            RepositoryId::Boot => base_path.join("boot").join("ffxivboot.ver"),
            RepositoryId::Ffxiv => base_path.join("game").join("ffxivgame.ver"),
            RepositoryId::Ex1 => get_path_for_exid(base_path, 1),
            RepositoryId::Ex2 => get_path_for_exid(base_path, 2),
            RepositoryId::Ex3 => get_path_for_exid(base_path, 3),
            RepositoryId::Ex4 => get_path_for_exid(base_path, 4),
        }
    }

    pub fn get_version(&self) -> Result<String, std::io::Error> {
        std::fs::read_to_string(self.get_version_file_path())
    }
}
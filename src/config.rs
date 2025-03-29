use std::io::Read;
use std::{
    fs::{self, File},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::view::Error;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub video_count: u32,
}

impl Config {
    pub fn load_or_default() -> Result<Config, Error> {
        let Some(root) = dirs::config_local_dir() else {
            return Err(Error::FileBadAccess);
        };

        let root = root.join("yt-feeds/");

        if !Path::exists(&root) {
            fs::create_dir_all(&root).map_err(|_| Error::FileBadAccess)?;
        }

        let file = root.join("config.toml");

        if !Path::exists(&file) {
            let default_config = Config { video_count: 30 };
            let toml = toml::to_string(&default_config).map_err(|_| Error::TomlError)?;
            fs::write(file, toml).map_err(|_| Error::TomlError)?;

            return Ok(default_config);
        }

        match File::open(&file) {
            Ok(mut file) => {
                let mut raw = String::new();
                file.read_to_string(&mut raw)
                    .map_err(|_| Error::FileBadAccess)?;

                return toml::from_str(&raw).map_err(|_| Error::TomlError);
            }
            Err(_) => {
                return Err(Error::FileBadAccess);
            }
        }
    }
}

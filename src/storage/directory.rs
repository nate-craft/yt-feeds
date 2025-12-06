use std::{
    fs::{self, create_dir_all},
    path::{PathBuf, MAIN_SEPARATOR},
};

use anyhow::anyhow;

pub struct Directory {
    path: PathBuf,
}

pub enum Dirs {
    Config,
    Data,
}

impl Dirs {
    pub fn create(self) -> anyhow::Result<()> {
        match self {
            Dirs::Config => dirs::data_local_dir()
                .or(dirs::data_dir())
                .map(|dir| dir.join("yt-feeds"))
                .ok_or(anyhow!("Could not find data directory"))
                .map(|dir| {
                    create_dir_all(dir)
                        .map_err(|err| anyhow!(err).context("Could not generate data directory"))
                })?,
            Dirs::Data => dirs::config_local_dir()
                .or(dirs::config_dir())
                .map(|dir| dir.join("yt-feeds"))
                .ok_or(anyhow!("Could not find data directory"))
                .map(|dir| {
                    create_dir_all(dir).map_err(|err| {
                        anyhow!(err).context("Could not generate configuration directory")
                    })
                })?,
        }
    }
}

impl Directory {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn subdir(&self, name: &str) -> anyhow::Result<Directory> {
        let joined = self.path.join(format!("{}{}", name, MAIN_SEPARATOR));

        if !joined.is_dir() {
            fs::create_dir_all(&joined).map_err(|err| {
                anyhow!(err).context(format!(
                    "Could not create subdirectory: {}",
                    joined.to_string_lossy()
                ))
            })?;
        }

        Ok(Directory::new(joined))
    }

    pub fn file(&self, name: &str) -> anyhow::Result<PathBuf> {
        let parent = self
            .path
            .parent()
            .ok_or(anyhow!("Could not find parent directory!"))?;

        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }

        Ok(self.path.join(name))
    }
}

impl TryFrom<Dirs> for Directory {
    type Error = anyhow::Error;

    fn try_from(value: Dirs) -> anyhow::Result<Self> {
        match value {
            Dirs::Data => dirs::data_local_dir()
                .or(dirs::data_dir())
                .map(|dir| dir.join("yt-feeds"))
                .ok_or(anyhow!("Could not access data directory"))
                .map(|dir| Directory::new(dir)),
            Dirs::Config => dirs::config_local_dir()
                .or(dirs::config_dir())
                .map(|dir| dir.join("yt-feeds"))
                .ok_or(anyhow!("Could not access config directory"))
                .map(|dir| Directory::new(dir)),
        }
    }
}

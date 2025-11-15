use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::utilities::seconds_formatted;

pub const MPV_SOCKET: &'static str = "/tmp/yt-feeds-socket";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct WatchProgress {
    pub current: u32,
    pub duration: u32,
}

enum MpvCommand {
    GetProgress,
}

enum MpvResult {
    WatchProgress(WatchProgress),
}

impl WatchProgress {
    pub fn new(current: u32, duration: u32) -> WatchProgress {
        WatchProgress { current, duration }
    }

    pub fn playing() -> Option<WatchProgress> {
        if cfg!(target_os = "windows") {
            return None;
        } else if let Ok(result) = MpvCommand::GetProgress.run() {
            let MpvResult::WatchProgress(progress) = result;
            return Some(progress);
        }

        return None;
    }

    pub fn formatted(&self) -> String {
        format!(
            "{} / {}",
            seconds_formatted(self.current),
            seconds_formatted(self.duration)
        )
    }
}

impl MpvCommand {
    fn run(&self) -> Result<MpvResult, std::io::Error> {
        match self {
            MpvCommand::GetProgress => {
                let cmd_current =
                    json!({"command" : ["get_property", "playback-time"]}).to_string();
                let cmd_duration = json!({"command" : ["get_property", "duration"]}).to_string();

                let json_now = serde_json::from_str::<HashMap<String, Value>>(
                    &Self::read_from_socket(&cmd_current)?,
                )?;
                let json_total = serde_json::from_str::<HashMap<String, Value>>(
                    &Self::read_from_socket(&cmd_duration)?,
                )?;

                let now = json_now
                    .get("data")
                    .ok_or(std::io::Error::other("Couldn't parse json data from IPC"))?
                    .as_f64()
                    .ok_or(std::io::Error::other("Couldn't parse json float from IPC"))?
                    as u32;

                let total = json_total
                    .get("data")
                    .ok_or(std::io::Error::other("Couldn't parse json data from IPC"))?
                    .as_f64()
                    .ok_or(std::io::Error::other("Couldn't parse json float from IPC"))?
                    as u32;

                Ok(MpvResult::WatchProgress(WatchProgress::new(now, total)))
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn read_from_socket(command: &str) -> Result<String, std::io::Error> {
        use std::{
            io::{BufRead, BufReader, Write},
            os::unix::net::UnixStream,
        };

        let mut stream = UnixStream::connect(MPV_SOCKET)?;
        stream.write_all(command.as_bytes())?;
        stream.write_all(b"\n")?;

        let mut input = String::new();

        let mut reader = BufReader::new(&stream);
        reader.read_line(&mut input)?;
        Ok(input)
    }

    #[cfg(target_os = "windows")]
    fn read_from_socket(_: &str) -> Result<String, std::io::Error> {
        Ok("".to_owned())
    }
}

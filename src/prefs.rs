use crate::game::{Difficulty, GameMode};
use crate::i18n::{I18n, Lang};
use crate::prelude::*;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserPrefs {
    pub version: u8,
    pub lang: Lang,
    pub difficulty: Difficulty,
    pub mode: GameMode,
}

impl Default for UserPrefs {
    fn default() -> Self {
        Self {
            version: 1,
            lang: Lang::En,
            difficulty: Difficulty::Easy,
            mode: GameMode::Classic,
        }
    }
}

fn prefs_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Some(p) = std::env::var_os("USERPROFILE") {
            return PathBuf::from(p).join(".guessing_game.toml");
        }
        if let (Ok(drive), Ok(path)) = (std::env::var("HOMEDRIVE"), std::env::var("HOMEPATH")) {
            return PathBuf::from(format!("{}{}", drive, path)).join(".guessing_game.toml");
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home).join(".guessing_game.toml");
    }
    PathBuf::from(".guessing_game.toml")
}

pub fn load_prefs() -> Option<UserPrefs> {
    let path = prefs_path();
    let text = fs::read_to_string(path).ok()?;
    toml::from_str(&text).ok()
}

pub fn save_prefs(p: &UserPrefs, i18n: &I18n) {
    let path = prefs_path();
    let _ = fs::write(
        &path,
        toml::to_string_pretty(p).unwrap_or_else(|_| String::new()),
    );
    println!("{}", i18n.t("saving_prefs").as_ref());
}

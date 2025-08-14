use crate::i18n::{I18n, Lang};
use std::io;

/// Prompt simple con trim
pub fn read_line_trimmed(prompt: &str) -> String {
    println!("{}", prompt);
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("stdin failed");
    s.trim().to_string()
}

/// Tokens “canónicos” que aceptamos desde la consola.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Yes,
    No,
    Easy,
    Hard,
    ModeClassic,
    ModeUnlimited,
    ModeTimed,
    ModeHotCold,
    Invalid,
}

/// Normaliza la entrada del usuario a un Token, considerando idioma.
pub fn normalize_input(lang: Lang, input: &str) -> Token {
    let s = input.trim().to_lowercase();
    let is_any = |cands: &[&str]| cands.iter().any(|c| *c == s);

    match lang {
        Lang::En => {
            if is_any(&["y", "yes"]) {
                return Token::Yes;
            }
            if is_any(&["n", "no"]) {
                return Token::No;
            }
            if is_any(&["easy"]) {
                return Token::Easy;
            }
            if is_any(&["hard"]) {
                return Token::Hard;
            }
            if is_any(&["classic"]) {
                return Token::ModeClassic;
            }
            if is_any(&["unlimited"]) {
                return Token::ModeUnlimited;
            }
            if is_any(&["timed", "time", "timer"]) {
                return Token::ModeTimed;
            }
            if is_any(&["hotcold", "hot/cold", "hot-cold"]) {
                return Token::ModeHotCold;
            }
        }
        Lang::Es => {
            if is_any(&["s", "si", "sí", "y", "yes"]) {
                return Token::Yes;
            }
            if is_any(&["n", "no"]) {
                return Token::No;
            }
            if is_any(&["facil", "fácil", "easy"]) {
                return Token::Easy;
            }
            if is_any(&["dificil", "difícil", "hard"]) {
                return Token::Hard;
            }
            if is_any(&["clasico", "clásico", "classic"]) {
                return Token::ModeClassic;
            }
            if is_any(&["ilimitado", "unlimited"]) {
                return Token::ModeUnlimited;
            }
            if is_any(&["cronometrado", "temporizado", "timed"]) {
                return Token::ModeTimed;
            }
            if is_any(&[
                "caliente/frio",
                "caliente-frio",
                "caliente frio",
                "frio/caliente",
                "frio-caliente",
                "frio caliente",
                "hotcold",
                "hot/cold",
                "hot-cold",
            ]) {
                return Token::ModeHotCold;
            }
        }
        Lang::PtBr => {
            if is_any(&["s", "sim", "y", "yes"]) {
                return Token::Yes;
            }
            if is_any(&["n", "nao", "não", "no"]) {
                return Token::No;
            }
            if is_any(&["facil", "fácil", "easy"]) {
                return Token::Easy;
            }
            if is_any(&["dificil", "difícil", "hard"]) {
                return Token::Hard;
            }
            if is_any(&["classico", "clássico", "classic"]) {
                return Token::ModeClassic;
            }
            if is_any(&["ilimitado", "unlimited"]) {
                return Token::ModeUnlimited;
            }
            if is_any(&["cronometrado", "temporizado", "timed"]) {
                return Token::ModeTimed;
            }
            if is_any(&[
                "quente/frio",
                "quente-frio",
                "quente frio",
                "hotcold",
                "hot/cold",
                "hot-cold",
            ]) {
                return Token::ModeHotCold;
            }
        }
    }
    Token::Invalid
}

/// Selecciona locale con fallback a en
pub fn choose_locale() -> I18n {
    let input = read_line_trimmed("Choose your locale (en, es, pt-BR). Default is 'en':");
    let lang = if let Some(l) = Lang::from_input(&input) {
        l
    } else if input.is_empty() {
        Lang::En
    } else {
        println!("Invalid locale. Falling back to English (en).");
        Lang::En
    };
    I18n::new(lang)
}

/// Selección de dificultad
pub fn choose_difficulty(i18n: &I18n) -> crate::game::Difficulty {
    loop {
        let choice = read_line_trimmed(i18n.t("choose_difficulty").as_ref());
        match normalize_input(i18n.lang, &choice) {
            Token::Easy => return crate::game::Difficulty::Easy,
            Token::Hard => return crate::game::Difficulty::Hard,
            _ => println!("{}", i18n.t("invalid_option").as_ref()),
        }
    }
}

/// Etiquetas localizadas
pub fn mode_labels(i18n: &I18n) -> String {
    format!(
        "{}, {}, {}, {}",
        i18n.t("mode_classic_label").as_ref(),
        i18n.t("mode_unlimited_label").as_ref(),
        i18n.t("mode_timed_label").as_ref(),
        i18n.t("mode_hotcold_label").as_ref()
    )
}

/// Selección de modo
pub fn choose_mode(i18n: &I18n, prev_mode: Option<crate::game::GameMode>) -> crate::game::GameMode {
    if let Some(m) = prev_mode {
        println!("{}: {}", "Last mode", m.label(i18n));
    }
    loop {
        println!("{} ({})", i18n.t("choose_mode").as_ref(), mode_labels(i18n));
        let mode_input = read_line_trimmed("⮕ ");
        match normalize_input(i18n.lang, &mode_input) {
            Token::ModeClassic => return crate::game::GameMode::Classic,
            Token::ModeUnlimited => return crate::game::GameMode::Unlimited,
            Token::ModeTimed => {
                let secs_str = read_line_trimmed(i18n.t("timed_seconds").as_ref());
                if let Ok(secs) = secs_str.parse::<u64>() {
                    return crate::game::GameMode::Timed {
                        seconds: secs.max(5),
                    };
                }
                println!("{}", i18n.t("invalid_option").as_ref());
            }
            Token::ModeHotCold => return crate::game::GameMode::HotCold,
            _ => println!("{}", i18n.t("invalid_option").as_ref()),
        }
    }
}

/// ¿Jugar otra vez?
pub fn ask_play_again(i18n: &I18n) -> bool {
    loop {
        let ans = read_line_trimmed(i18n.t("play_again").as_ref());
        match normalize_input(i18n.lang, &ans) {
            Token::Yes => return true,
            Token::No => return false,
            _ => println!("{}", i18n.t("invalid_option").as_ref()),
        }
    }
}

mod game;
mod i18n;
mod input;
mod prefs;
mod prelude;

use crate::i18n::{I18n, Lang};
use crate::prefs::{load_prefs, save_prefs};
use crate::{game as g, input as ui};

fn main() {
    // Cargar prefs una sola vez
    let loaded = load_prefs();
    let mut prefs = loaded.clone().unwrap_or_default();

    // Selección/uso de locale
    let i18n = if let Some(ref p) = loaded {
        let chosen = I18n::new(p.lang);
        println!("{}", chosen.t("welcome").as_ref());
        println!(
            "{}",
            chosen.fmt(
                "loaded_prefs",
                &[
                    ("lang", p.lang.code().to_string()),
                    (
                        "diff",
                        match p.difficulty {
                            g::Difficulty::Easy => chosen.t("easy").into_owned(),
                            g::Difficulty::Hard => chosen.t("hard").into_owned(),
                        }
                    ),
                    (
                        "mode",
                        match p.mode {
                            g::GameMode::Classic => chosen.t("mode_classic_label").into_owned(),
                            g::GameMode::Unlimited => chosen.t("mode_unlimited_label").into_owned(),
                            g::GameMode::Timed { .. } => chosen.t("mode_timed_label").into_owned(),
                            g::GameMode::HotCold => chosen.t("mode_hotcold_label").into_owned(),
                        }
                    )
                ]
            )
        );
        chosen
    } else {
        println!("{}", I18n { lang: Lang::En }.t("welcome").as_ref());
        let chosen = ui::choose_locale();
        println!("{}", chosen.t("welcome").as_ref());
        prefs.lang = chosen.lang;
        save_prefs(&prefs, &chosen);
        chosen
    };

    // Bucle principal
    loop {
        let difficulty = ui::choose_difficulty(&i18n);

        let prev_mode = Some(prefs.mode);
        let mode = ui::choose_mode(&i18n, prev_mode);

        // Persistimos selección actual
        prefs.difficulty = difficulty;
        prefs.mode = mode;
        save_prefs(&prefs, &i18n);

        // Jugar
        let cfg = g::GameConfig::from(difficulty, mode);
        let game = g::Game::new(cfg, I18n::new(i18n.lang));
        game.play();

        if !ui::ask_play_again(&i18n) {
            println!("{}", i18n.t("bye").as_ref());
            break;
        }
    }
}

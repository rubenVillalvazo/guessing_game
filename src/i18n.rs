use crate::prelude::*;

/// Idiomas soportados
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Lang {
    En,
    Es,
    PtBr,
}

impl Lang {
    pub fn from_input(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "en" | "en-us" | "en-gb" => Some(Lang::En),
            "es" | "es-es" | "es-mx" | "es-ar" => Some(Lang::Es),
            "pt" | "pt-br" => Some(Lang::PtBr),
            _ => None,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Es => "es",
            Lang::PtBr => "pt-BR",
        }
    }
}

pub struct I18n {
    pub lang: Lang,
}

impl I18n {
    pub fn new(lang: Lang) -> Self {
        Self { lang }
    }

    /// Traducción con fallback (Cow para evitar problemas de lifetime).
    pub fn t(&self, key: &str) -> Cow<'static, str> {
        match self.lang {
            Lang::En => match key {
                "welcome" => Cow::Borrowed("🎉 WELCOME TO GUESS THE NUMBER! 🎉"),
                "choose_locale" => Cow::Borrowed("Choose your locale (en, es, pt-BR). Default is 'en':"),
                "invalid_locale" => Cow::Borrowed("Invalid locale. Falling back to English (en)."),
                "choose_difficulty" => Cow::Borrowed("Select difficulty (easy/hard):"),
                "invalid_option" => Cow::Borrowed("❓ Invalid option."),
                "easy" => Cow::Borrowed("easy"),
                "hard" => Cow::Borrowed("hard"),
                "choose_mode" => Cow::Borrowed("Select game mode: classic, unlimited, timed, hot/cold:"),
                "timed_seconds" => Cow::Borrowed("Enter total seconds for the timed game (e.g., 30):"),
                "number_think" => Cow::Borrowed("🤖 I'm thinking of a number between {min} and {max}."),
                "attempts_info" => Cow::Borrowed("You have {attempts} attempts."),
                "time_info" => Cow::Borrowed("You have {secs} seconds total."),
                "ask_guess" => Cow::Borrowed("🤔 Your guess?"),
                "not_number" => Cow::Borrowed("🚫 Not a valid number. Try again."),
                "out_of_range" => Cow::Borrowed("🚧 It must be between {min} and {max}."),
                "too_low" => Cow::Borrowed("🔻 Too low!"),
                "too_high" => Cow::Borrowed("🔺 Too high!"),
                "you_win" => Cow::Borrowed("🎉 Congrats! You guessed {num} in {attempt} attempt(s)."),
                "you_lose_attempts" => Cow::Borrowed("❌ Out of attempts. The number was {num}."),
                "you_lose_time" => Cow::Borrowed("⏰ Time's up! The number was {num}."),
                "play_again" => Cow::Borrowed("Play again? (y/n)"),
                "bye" => Cow::Borrowed("👋 Thanks for playing! See you next time!"),
                // Etiquetas de modos
                "mode_classic_label" => Cow::Borrowed("classic"),
                "mode_unlimited_label" => Cow::Borrowed("unlimited"),
                "mode_timed_label" => Cow::Borrowed("timed"),
                "mode_hotcold_label" => Cow::Borrowed("hot/cold"),
                // Pistas Hot/Cold
                "hot" => Cow::Borrowed("🔥 Hot!"),
                "very_hot" => Cow::Borrowed("♨️ Very hot!"),
                "warm" => Cow::Borrowed("🌤️ Warm!"),
                "cold" => Cow::Borrowed("🧊 Cold!"),
                // Persistencia
                "loaded_prefs" => Cow::Borrowed("Loaded preferences from ~/.guessing_game.toml (locale: {lang}, difficulty: {diff}, mode: {mode})."),
                "saving_prefs" => Cow::Borrowed("Saving preferences..."),
                _ => Cow::Owned(key.to_string()),
            },
            Lang::Es => match key {
                "welcome" => Cow::Borrowed("🎉 ¡BIENVENIDO A ADIVINA EL NÚMERO! 🎉"),
                "choose_locale" => Cow::Borrowed("Elige tu locale (en, es, pt-BR). Por defecto 'en':"),
                "invalid_locale" => Cow::Borrowed("Locale inválido. Usaremos inglés (en) por defecto."),
                "choose_difficulty" => Cow::Borrowed("Selecciona dificultad (fácil/difícil):"),
                "invalid_option" => Cow::Borrowed("❓ Opción inválida."),
                "easy" => Cow::Borrowed("fácil"),
                "hard" => Cow::Borrowed("difícil"),
                "choose_mode" => Cow::Borrowed("Selecciona el modo: clásico, ilimitado, cronometrado, caliente/frío:"),
                "timed_seconds" => Cow::Borrowed("Ingresa los segundos totales para el modo cronometrado (ej: 30):"),
                "number_think" => Cow::Borrowed("🤖 Estoy pensando un número entre {min} y {max}."),
                "attempts_info" => Cow::Borrowed("Tienes {attempts} intentos."),
                "time_info" => Cow::Borrowed("Tienes {secs} segundos en total."),
                "ask_guess" => Cow::Borrowed("🤔 ¿Tu conjetura?"),
                "not_number" => Cow::Borrowed("🚫 No es un número válido. Intenta de nuevo."),
                "out_of_range" => Cow::Borrowed("🚧 Debe estar entre {min} y {max}."),
                "too_low" => Cow::Borrowed("🔻 ¡Demasiado bajo!"),
                "too_high" => Cow::Borrowed("🔺 ¡Demasiado alto!"),
                "you_win" => Cow::Borrowed("🎉 ¡Felicidades! Adivinaste {num} en {attempt} intento(s)."),
                "you_lose_attempts" => Cow::Borrowed("❌ Sin intentos. El número era {num}."),
                "you_lose_time" => Cow::Borrowed("⏰ ¡Se acabó el tiempo! El número era {num}."),
                "play_again" => Cow::Borrowed("¿Quieres jugar otra vez? (s/n)"),
                "bye" => Cow::Borrowed("👋 ¡Gracias por jugar! ¡Hasta la próxima!"),
                "mode_classic_label" => Cow::Borrowed("clásico"),
                "mode_unlimited_label" => Cow::Borrowed("ilimitado"),
                "mode_timed_label" => Cow::Borrowed("cronometrado"),
                "mode_hotcold_label" => Cow::Borrowed("caliente/frío"),
                "hot" => Cow::Borrowed("🔥 ¡Caliente!"),
                "very_hot" => Cow::Borrowed("♨️ ¡Muy caliente!"),
                "warm" => Cow::Borrowed("🌤️ Tibio"),
                "cold" => Cow::Borrowed("🧊 Frío"),
                "loaded_prefs" => Cow::Borrowed("Preferencias cargadas de ~/.guessing_game.toml (locale: {lang}, dificultad: {diff}, modo: {mode})."),
                "saving_prefs" => Cow::Borrowed("Guardando preferencias..."),
                _ => Cow::Owned(key.to_string()),
            },
            Lang::PtBr => match key {
                "welcome" => Cow::Borrowed("🎉 BEM-VINDO AO ADIVINHE O NÚMERO! 🎉"),
                "choose_locale" => Cow::Borrowed("Escolha seu locale (en, es, pt-BR). Padrão: 'en':"),
                "invalid_locale" => Cow::Borrowed("Locale inválido. Usaremos inglês (en) por padrão."),
                "choose_difficulty" => Cow::Borrowed("Selecione a dificuldade (fácil/difícil):"),
                "invalid_option" => Cow::Borrowed("❓ Opção inválida."),
                "easy" => Cow::Borrowed("fácil"),
                "hard" => Cow::Borrowed("difícil"),
                "choose_mode" => Cow::Borrowed("Selecione o modo: clássico, ilimitado, cronometrado, quente/frio:"),
                "timed_seconds" => Cow::Borrowed("Informe os segundos totais para o modo cronometrado (ex: 30):"),
                "number_think" => Cow::Borrowed("🤖 Estou pensando em um número entre {min} e {max}."),
                "attempts_info" => Cow::Borrowed("Você tem {attempts} tentativas."),
                "time_info" => Cow::Borrowed("Você tem {secs} segundos no total."),
                "ask_guess" => Cow::Borrowed("🤔 Seu palpite?"),
                "not_number" => Cow::Borrowed("🚫 Não é um número válido. Tente novamente."),
                "out_of_range" => Cow::Borrowed("🚧 Deve estar entre {min} e {max}."),
                "too_low" => Cow::Borrowed("🔻 Muito baixo!"),
                "too_high" => Cow::Borrowed("🔺 Muito alto!"),
                "you_win" => Cow::Borrowed("🎉 Parabéns! Você acertou {num} em {attempt} tentativa(s)."),
                "you_lose_attempts" => Cow::Borrowed("❌ Acabaram as tentativas. O número era {num}."),
                "you_lose_time" => Cow::Borrowed("⏰ O tempo acabou! O número era {num}."),
                "play_again" => Cow::Borrowed("Jogar novamente? (s/n)"),
                "bye" => Cow::Borrowed("👋 Obrigado por jogar! Até a próxima!"),
                "mode_classic_label" => Cow::Borrowed("clássico"),
                "mode_unlimited_label" => Cow::Borrowed("ilimitado"),
                "mode_timed_label" => Cow::Borrowed("cronometrado"),
                "mode_hotcold_label" => Cow::Borrowed("quente/frio"),
                "hot" => Cow::Borrowed("🔥 Quente!"),
                "very_hot" => Cow::Borrowed("♨️ Muito quente!"),
                "warm" => Cow::Borrowed("🌤️ Morno!"),
                "cold" => Cow::Borrowed("🧊 Frio!"),
                "loaded_prefs" => Cow::Borrowed("Preferências carregadas de ~/.guessing_game.toml (locale: {lang}, dificuldade: {diff}, modo: {mode})."),
                "saving_prefs" => Cow::Borrowed("Salvando preferências..."),
                _ => Cow::Owned(key.to_string()),
            },
        }
    }

    pub fn fmt(&self, key: &str, args: &[(&str, String)]) -> String {
        let mut s: String = self.t(key).into_owned();
        for (k, v) in args {
            s = s.replace(&format!("{{{}}}", k), v);
        }
        s
    }
}

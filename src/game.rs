use crate::i18n::I18n;
use crate::prelude::*;

/// Dificultad
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Hard,
}

/// Modos de juego
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum GameMode {
    Classic,
    Unlimited,
    Timed { seconds: u64 },
    HotCold,
}

impl GameMode {
    pub fn label(self, i18n: &I18n) -> Cow<'static, str> {
        match self {
            GameMode::Classic => i18n.t("mode_classic_label"),
            GameMode::Unlimited => i18n.t("mode_unlimited_label"),
            GameMode::Timed { .. } => i18n.t("mode_timed_label"),
            GameMode::HotCold => i18n.t("mode_hotcold_label"),
        }
    }
}

/// Configuración derivada de (dificultad, modo)
pub struct GameConfig {
    pub min: i32,
    pub max: i32,
    pub max_attempts: Option<i32>, // None = ilimitado
    pub time_limit: Option<Duration>,
    pub hot_cold: bool,
}

impl GameConfig {
    pub fn from(difficulty: Difficulty, mode: GameMode) -> Self {
        let (min, max, attempts) = match difficulty {
            Difficulty::Easy => (1, 50, 12),
            Difficulty::Hard => (1, 100, 10),
        };
        let (max_attempts, time_limit, hot_cold) = match mode {
            GameMode::Classic => (Some(attempts), None, false),
            GameMode::Unlimited => (None, None, false),
            GameMode::Timed { seconds } => {
                (Some(attempts), Some(Duration::from_secs(seconds)), false)
            }
            GameMode::HotCold => (Some(attempts), None, true),
        };
        Self {
            min,
            max,
            max_attempts,
            time_limit,
            hot_cold,
        }
    }
}

/// Estado del juego
pub struct Game {
    secret: i32,
    cfg: GameConfig,
    i18n: I18n,
}

impl Game {
    pub fn new(cfg: GameConfig, i18n: I18n) -> Self {
        let secret = rand::rng().random_range(cfg.min..=cfg.max); // rand 0.9
        Self { secret, cfg, i18n }
    }

    fn prompt(msg: &str) -> String {
        println!("{}", msg);
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("stdin failed");
        buf.trim().to_string()
    }

    fn read_int_in_range(&self, min: i32, max: i32) -> i32 {
        loop {
            let input = Self::prompt(self.i18n.t("ask_guess").as_ref());
            match input.parse::<i32>() {
                Ok(n) if n >= min && n <= max => return n,
                Ok(_) => println!(
                    "{}",
                    self.i18n.fmt(
                        "out_of_range",
                        &[("min", min.to_string()), ("max", max.to_string())]
                    )
                ),
                Err(_) => println!("{}", self.i18n.t("not_number").as_ref()),
            }
        }
    }

    fn hot_cold_hint(&self, guess: i32) {
        let diff = (guess - self.secret).abs();
        let range = (self.cfg.max - self.cfg.min).max(1);
        let very_hot = (range as f32 * 0.05).ceil() as i32; // 5%
        let hot = (range as f32 * 0.10).ceil() as i32; // 10%
        let warm = (range as f32 * 0.20).ceil() as i32; // 20%

        if diff == 0 {
            return;
        } else if diff <= very_hot {
            println!("{}", self.i18n.t("very_hot").as_ref());
        } else if diff <= hot {
            println!("{}", self.i18n.t("hot").as_ref());
        } else if diff <= warm {
            println!("{}", self.i18n.t("warm").as_ref());
        } else {
            println!("{}", self.i18n.t("cold").as_ref());
        }
    }

    pub fn play(&self) {
        println!(
            "{}",
            self.i18n.fmt(
                "number_think",
                &[
                    ("min", self.cfg.min.to_string()),
                    ("max", self.cfg.max.to_string())
                ]
            )
        );

        if let Some(a) = self.cfg.max_attempts {
            println!(
                "{}",
                self.i18n
                    .fmt("attempts_info", &[("attempts", a.to_string())])
            );
        }
        if let Some(t) = self.cfg.time_limit {
            println!(
                "{}",
                self.i18n
                    .fmt("time_info", &[("secs", t.as_secs().to_string())])
            );
        }

        let start = Instant::now();
        let mut attempt = 0;

        loop {
            if let Some(limit) = self.cfg.time_limit {
                if start.elapsed() >= limit {
                    println!(
                        "{}",
                        self.i18n
                            .fmt("you_lose_time", &[("num", self.secret.to_string())])
                    );
                    return;
                }
            }
            if let Some(max_attempts) = self.cfg.max_attempts {
                if attempt >= max_attempts {
                    println!(
                        "{}",
                        self.i18n
                            .fmt("you_lose_attempts", &[("num", self.secret.to_string())])
                    );
                    return;
                }
            }

            attempt += 1;
            let guess = self.read_int_in_range(self.cfg.min, self.cfg.max);

            match guess.cmp(&self.secret) {
                Ordering::Less => {
                    println!("{}", self.i18n.t("too_low").as_ref());
                    if self.cfg.hot_cold {
                        self.hot_cold_hint(guess);
                    }
                }
                Ordering::Greater => {
                    println!("{}", self.i18n.t("too_high").as_ref());
                    if self.cfg.hot_cold {
                        self.hot_cold_hint(guess);
                    }
                }
                Ordering::Equal => {
                    println!(
                        "{}",
                        self.i18n.fmt(
                            "you_win",
                            &[
                                ("num", self.secret.to_string()),
                                ("attempt", attempt.to_string())
                            ]
                        )
                    );
                    return;
                }
            }
        }
    }
}

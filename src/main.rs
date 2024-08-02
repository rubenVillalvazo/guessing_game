use rand::Rng;
use std::cmp::Ordering;
use std::io;

enum Difficulty {
    Easy,
    Hard,
}

struct Game {
    secret_number: i32,
    min_number: i32,
    max_number: i32,
    max_attempts: i32,
}

impl Game {
    fn new(difficulty: Difficulty) -> Self {
        match difficulty {
            Difficulty::Easy => Game {
                secret_number: rand::thread_rng().gen_range(1..=50),
                min_number: 1,
                max_number: 50,
                max_attempts: 12,
            },
            Difficulty::Hard => Game {
                secret_number: rand::thread_rng().gen_range(1..=100),
                min_number: 1,
                max_number: 100,
                max_attempts: 10,
            },
        }
    }
}

enum InputType {
    Guess,
    PlayAgain,
    SelectDifficulty,
}

fn user_input(input_type: InputType) -> String {
    match input_type {
        InputType::Guess => {
            println!("🤔 ¿Cuál es tu conjetura?");
        }
        InputType::PlayAgain => {
            println!("¿Quieres jugar otra vez? (Y/N)");
        }
        InputType::SelectDifficulty => {
            println!("Selecciona la dificultad: fácil o difícil (easy/hard): ");
        }
    }
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("¡Falló al leer la línea!");
    return user_input;
}

fn select_difficulty() -> Difficulty {
    loop {
        let difficulty = user_input(InputType::SelectDifficulty)
            .trim()
            .to_lowercase();
        match difficulty.as_str() {
            "easy" => return Difficulty::Easy,
            "hard" => return Difficulty::Hard,
            _ => println!("Por favor, elige una dificultad válida: fácil o difícil (easy/hard)."),
        }
    }
}

fn game() {
    'game_loop: loop {
        println!("🎉 ¡BIENVENIDO A ADIVINA EL NÚMERO! 🎉");

        let difficulty = select_difficulty();
        let game = Game::new(difficulty);
        let mut attempts = 0;

        println!(
            "🤖 Estoy pensando en un número entre {} y {}. ¿Puedes adivinar cuál es? ¡Tienes {} intentos!",
            game.min_number, game.max_number, game.max_attempts
        );

        'guess_loop: loop {
            if attempts >= game.max_attempts {
                println!("❌ ¡Oh no! Has alcanzado el máximo de intentos. El número era {}. ¡Mejor suerte la próxima vez!", game.secret_number);
                break 'guess_loop;
            }

            let user_guess: i32 = match user_input(InputType::Guess).trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("🚫 ¡Eso no es un número válido! Inténtalo de nuevo.");
                    continue 'guess_loop;
                }
            };

            if user_guess < game.min_number || user_guess > game.max_number {
                println!(
                    "🚧 Recuerda, el número está entre {} y {}.",
                    game.min_number, game.max_number
                );
                continue 'guess_loop;
            }

            match user_guess.cmp(&game.secret_number) {
                Ordering::Less => println!("🔻 ¡Demasiado bajo! Intenta un número mayor."),
                Ordering::Greater => println!("🔺 ¡Demasiado alto! Intenta un número menor."),
                Ordering::Equal => {
                    println!(
                        "🎉 ¡Felicidades! ¡Has adivinado el número {}!",
                        game.secret_number
                    );
                    break 'guess_loop;
                }
            }
            attempts += 1;
        }

        'play_again_loop: loop {
            let user_answer = user_input(InputType::PlayAgain).trim().to_lowercase();
            if user_answer == "n" || user_answer == "no" {
                println!("👋 ¡Gracias por jugar! ¡Hasta la próxima!");
                break 'game_loop;
            } else if user_answer == "y" || user_answer == "yes" {
                println!("🔄 ¡Genial! Preparando un nuevo juego...");
                break 'play_again_loop;
            } else {
                println!("❓ Respuesta no válida. Por favor, responde con 'Y' (sí) o 'N' (no).");
            }
        }
    }
}

fn main() {
    game();
}

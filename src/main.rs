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
            println!("Input your guess: ");
        }
        InputType::PlayAgain => {
            println!("Would you like to play again? (Y/N)");
        }
        InputType::SelectDifficulty => {
            println!("Select Difficulty (easy/hard): ");
        }
    }
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
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
            _ => println!("Please enter a valid difficulty (easy/hard)."),
        }
    }
}

fn game() {
    'game_loop: loop {
        println!("GUESS THE NUMBER!");

        let difficulty = select_difficulty();
        let game = Game::new(difficulty);
        let mut attempts = 0;

        'guess_loop: loop {
            if attempts >= game.max_attempts {
                println!(
                    "You've reached the maximum number of attempts! The number was {}.",
                    game.secret_number
                );
                break 'guess_loop;
            }

            let user_guess: i32 = match user_input(InputType::Guess).trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a number!");
                    continue 'guess_loop;
                }
            };

            if user_guess < game.min_number || user_guess > game.max_number {
                println!(
                    "The secret number will be between {} and {}.",
                    game.min_number, game.max_number
                );
                continue 'guess_loop;
            }

            match user_guess.cmp(&game.secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break 'guess_loop;
                }
            }
            attempts += 1;
        }

        'play_again_loop: loop {
            let user_answer = user_input(InputType::PlayAgain).trim().to_lowercase();
            if user_answer == "n" || user_answer == "no" {
                break 'game_loop;
            } else if user_answer == "y" || user_answer == "yes" {
                break 'play_again_loop;
            } else {
                println!("Please enter a valid answer.");
            }
        }
    }
}

fn main() {
    game();
}

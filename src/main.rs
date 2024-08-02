use rand::Rng;
use std::cmp::Ordering;
use std::io;

enum InputType {
    Guess,
    PlayAgain,
}

fn user_input(input_type: InputType) -> String {
    match input_type {
        InputType::Guess => {
            println!("Input your guess: ");
        }
        InputType::PlayAgain => {
            println!("Would you like to play again? (Y/N)");
        }
    }
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    return user_input;
}

fn game() {
    'game_loop: loop {
        println!("GUESS THE NUMBER!");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        'guess_loop: loop {
            let user_guess: i32 = match user_input(InputType::Guess).trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a number!");
                    continue 'guess_loop;
                }
            };

            if user_guess < 1 || user_guess > 100 {
                println!("The secret number will be between 1 and 100.");
                continue 'guess_loop;
            }

            match user_guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break 'guess_loop;
                }
            }
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

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn user_input() -> String {
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    return user_input;
}

fn game() {
    println!("GUESS THE NUMBER!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    'game_loop: loop {
        println!("Input your guess: ");
        let user_guess: i32 = match user_input().trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        if user_guess < 1 || user_guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Would you like to play again? (Y/N)");

                loop {
                    let user_answer = user_input().trim().to_lowercase();
                    if user_answer == "n" || user_answer == "no" {
                        break 'game_loop;
                    } else if user_answer == "y" || user_answer == "yes" {
                        game();
                        break 'game_loop;
                    } else {
                        println!("Please enter a valid answer.");
                        println!("Would you like to play again? (Y/N)");
                        continue;
                    }
                }
            }
        }
    }
}

fn main() {
    game();
}

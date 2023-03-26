use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn game() {
    println!("GUESS THE NUMBER!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{secret_number}");

    'game_loop: loop {
        println!("Input your guess: ");
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");
        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Would you like to play again? (Y/N)");

                loop {
                    let mut user_answer = String::new();
                    io::stdin()
                        .read_line(&mut user_answer)
                        .expect("Failed to read line");
                    let user_answer = user_answer.trim().to_lowercase();
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

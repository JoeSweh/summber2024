use std::io::{self, Write};

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 42; // Secret number
    let mut attempts = 0;

    println!("Name: Jose F. Gonzalez Jr.");
    
    loop {
        // Simulate user input
        print!("Enter your guess: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        attempts += 1;

        match check_guess(guess, secret) {
            0 => {
                println!("Correct! You guessed the number in {} attempts.", attempts);
                break;
            }
            1 => println!("Too high!"),
            -1 => println!("Too low!"),
            _ => unreachable!(),
        }
    }
}
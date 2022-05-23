use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
struct Guess {
    guess: u32,
    high: u32,
    low: u32,
    tries: u32,
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

fn computer_guess(secret_number: u32, max: u32) -> u32 {
    let mut guess = max / 2;
    let mut high_bound = max;
    let mut low_bound = 0;
    let mut tries = 1;
    loop {
        dbg!(guess, low_bound, high_bound, tries);
        guess = if guess > secret_number {
            println!("==> Too high");
            high_bound = guess;
            low_bound + ((guess - low_bound) / 2)
        } else if guess < secret_number {
            println!("==> Too low");
            low_bound = guess;
            high_bound - ((high_bound - guess) / 2)
        } else {
            0 // Success
        };

        if tries > 10 || guess == 0 {
            break;
        }
        tries += 1;
    }

    tries
}

fn human_guess(secret_number: u32, user_name: &str) -> u32 {
    let mut human = Guess {
        guess: 0,
        high: 100,
        low: 0,
        tries: 0,
    };
    loop {
        human.tries += 1;
        println!("Please input your guess, Captain {}.", user_name);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect(format!("failed to read the line, Captain {}.", user_name).as_str());

        human.guess = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number: {}", guess);
                continue;
            }
        };

        println!("Captain {}, you guessed: {}", user_name, guess);
        match human.guess.cmp(&secret_number) {
            Ordering::Less => {
                human.low = human.guess;
                println!("Too small!");
            }
            Ordering::Greater => {
                human.guess = human.guess;
                println!("Too big!");
            }
            Ordering::Equal => break,
        }
    }
    human.tries
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut user_name = String::new();
    println!("Please input your name (or 'skip' to let the computer play).");
    io::stdin()
        .read_line(&mut user_name)
        .expect("failed to read your name");
    user_name = strip_trailing_newline(&user_name).to_string();

    let human_tries: u32;
    human_tries = if user_name != "skip" && user_name != "" {
        human_guess(secret_number, &user_name)
    } else {
        0
    };

    let computer_tries = computer_guess(secret_number, 100);
    println!("I (computer) solved it in {} tries.", computer_tries);

    if human_tries > 0 {
        println!(
            "You (human named {}) solved it in {} tries.",
            user_name, human_tries
        );
    }

    if computer_tries < human_tries {
        println!("Clearly I, (computer) have the superior intellect.");
    } else if human_tries > 0 && human_tries < computer_tries {
        println!(
            "You (human named {}) have the superior intellect.",
            user_name
        );
    }
}

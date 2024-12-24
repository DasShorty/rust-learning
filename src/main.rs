use rand::random_range;
use std::io::stdin;

fn main() {
    println!("Game: Guess the Number!");

    let random_number = generate_number(1, 100);

    let mut guessed = false;

    while !guessed {

        let guess = let_guess();

        match guess.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                guessed = true;
                break;
            }
        }

    }

}

fn let_guess() -> i32 {
    let mut guess = String::new();

    println!("Please input your guess: ");

    match stdin().read_line(&mut guess) {
        Ok(_) => {
            println!("You guessed: {}", guess);
        }
        Err(error) => {
            println!("Error: {}", error);
            let_guess();
        }
    }

    guess.trim().parse().unwrap_or_else(|_| {
        println!("Please input a number!");
        let_guess()
    })
}

fn generate_number(lowest: i32, highest: i32) -> i32 {
    println!("Generating random number between {lowest} and {highest}...");
    let random_number = random_range(lowest..highest);
    println!("Generated a random number");

    random_number
}

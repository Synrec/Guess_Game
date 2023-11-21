use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    loop {
        let mut guess_num = String::new();
        println!("What is your guess?: ");
        io::stdin()
            .read_line(&mut guess_num)
            .expect("Failed to read user input.");
        let guess: u32 = match guess_num.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("You have failed samurai.");
                break;
            },
        };
        println!("Your guess is {guess_num}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The guessed number is too small."),
            Ordering::Greater => println!("The guessed number is too big."),
            Ordering::Equal => {
                println!("The secret number is {secret_number}");
                println!("Match!");
                println!("Thanks for playing!");
                break;
            },
        }
    }
}

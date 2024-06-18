use rand::Rng; // crate rand
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // "" -> address 0xsasfdkhjb1238

        // data types - simple -> int, float, bool, char -> implement copy trait -> stored in the stack -> FIFO
        // data types - complex -> String, Vec, HashMap, etc. -> stored in the heap

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // trim input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // guess.trim().parse() returns type Result -> Ok or Err

        // Result<int, String>
        // Ok(int)
        // Error(String)

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

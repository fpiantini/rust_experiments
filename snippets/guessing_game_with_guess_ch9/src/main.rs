extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

// See chapter 9 ------------------------------------------------------------
// TODO ---- FIXME: replace panic with error handling...
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
// --------------------------------------------------------------------------

fn main() {
    println!("Guess the number! (From 1 to 100)");

    // rand::thread_rng() ritorna un rand::ThreadRng che è un:
    //    "random number generator local to the current thread
    //     of execution and seeded by the operating system"
    // Questo oggetto rand::ThreadRng ha dei metodi definiti,
    // uno dei quali e' gen_range(...) che ritorna un numero
    // nel range specificato. Per usare quest'ultimo occorre
    // inserire la linea:
    //   use rand::Rng;
    // vista sopra. Il motivo della necessita' di questa
    // dichiarazione e' il seguente:
    //    "Rng is a trait that defines methods that random number
    //    generators implement, and this trait must be in scope
    //    for us to use those methods."
    //
    // da notare che secret_number è un immutabile...
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // --- I want to manage negative numbers too
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        // --- Using the Guess structure (see chapter 9.3)
        let guess = Guess::new(guess);

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}


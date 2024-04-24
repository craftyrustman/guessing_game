use std::{cmp::Ordering, io::stdin};

use rand::Rng;

fn main() {
    println!("guessing game");
    let number = rand::thread_rng().gen_range(1..=100);
    println!("the number is {}", number);

    let mut count = 0;
    let mut isRight = false;

    while count<3 && isRight == false {
        println!("please enter a number between 1 and 100 : ");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("reading line failed");
        let guess: i32 = guess.trim().parse().expect("invalid number");
        let guess = Guess::new(guess);
        match number.cmp(guess.value()) {
        Ordering::Greater => println!("your number is too small"),
        Ordering::Less => println!("your number is too big"),
        Ordering::Equal => {
            println!("you are right! you win");
            isRight = true
        }
        }

        count += 1;
    }

    if count == 3 && isRight == false {
        println!("wrong again, you lose my friend")
    }
}

struct Guess {
    value: i32
}

impl Guess {
    fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess number should be between 1 and 100!")
        };
        Self { value }
    }

    fn value(&self) -> &i32{
        &self.value
    }
}
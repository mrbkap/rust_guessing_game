#![feature(rand)]
#![feature(old_io)]
use std::old_io;
use std::rand;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let goal = (rand::random::<u32>() % 10) + 1;

    loop {
        println!("Please input your guess.");
        let input = old_io::stdin().read_line()
                        .ok()
                        .expect("Failed to read line");

        let input_num: Result<u32, _>  = input.trim().parse();
        let num = match input_num {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                continue;
            }
        };

        match cmp(num, goal) {
            Ordering::Less => println!("You guessed too low"),
            Ordering::Greater => println!("You guessed too high"),
            Ordering::Equal => {
                println!("You got it!");
                return;
            },
        }
    }

}

fn cmp(l: u32, r: u32) -> Ordering {
    if l < r { Ordering::Less }
    else if r < l { Ordering::Greater }
    else { Ordering::Equal }
}

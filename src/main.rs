use std::io;
use rand::Rng;
use std::cmp::Ordering;

extern crate rand;

fn main() {
    let max_goal = rand::random::<u64>() + 1;
    println!("Guess the number!\n{}", max_goal);

    let mut rng = rand::thread_rng();
    let goal = (rng.gen::<u64>() % max_goal) + 1;

    loop {
        println!("Please input your guess.");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
                   .ok()
                   .expect("Failed to read line");

        let input_num: Result<u64, _>  = input.trim().parse();
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

fn cmp(l: u64, r: u64) -> Ordering {
    if l < r { Ordering::Less }
    else if r < l { Ordering::Greater }
    else { Ordering::Equal }
}

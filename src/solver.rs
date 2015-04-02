#![feature(old_io)]
use std::old_io;

pub fn solve(stdin : &mut old_io::stdio::StdinReader) {
    let mut line = stdin.read_line()
                   .ok()
                   .expect("Failed to read first line");
    assert_eq!(line.trim(), "Guess the number!");
    line = stdin.read_line()
                .ok()
                .expect("Failed to read max number");

    let mut min : u32 = 1;
    let mut max_goal : u32 =
            line.trim().parse()
                       .ok()
                       .expect("max goal wasn't a number");
    let mut tries = 1u32;

    loop {
        line = stdin.read_line()
                    .ok()
                    .expect("Not expecting an input?");
        assert_eq!(line.trim(), "Please input your guess.");

        let guess : u32 = ((max_goal - min) / 2) + min;
        println!("{} ({} rem)", guess, max_goal - min);

        line = stdin.read_line()
                    .ok()
                    .expect("No response?");
        match line.trim() {
            "You guessed too low" => min = guess + 1,
            "You guessed too high" => max_goal = guess - 1,
            "You got it!" => {
                println!("Done in {} tries!", tries);
                return;
            },
            _ => {
                println!("Unexpected input!");
                return;
            }
        }

        tries = tries + 1;
    }
}

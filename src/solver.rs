use std::io;
use std::io::{Read,Write};
use std::io::BufRead;

pub fn solve<R, W>(stdin: &mut io::BufReader<R>,
                   stdout: &mut io::BufWriter<W>)
   where R: io::Read, W: io::Write {
    let mut line: String = String::new();
    stdin.read_line(&mut line).unwrap();
    assert_eq!(line.trim(), "Guess the number!");

    line.truncate(0);
    stdin.read_line(&mut line).unwrap();

    let mut min: u32 = 1;
    let mut max: u32 = line.trim().parse().unwrap();
    let mut tries: u32 = 1;

    println!("Maximum is {}", max);

    loop {
        line.truncate(0);
        stdin.read_line(&mut line).unwrap();
        assert_eq!(line.trim(), "Please input your guess.");

        let guess: u32 = ((max - min) / 2) + min;
        println!("{} ({} remaining)", guess, max - min);
        writeln!(stdout, "{}", guess).unwrap();
        stdout.flush().unwrap();

        line.truncate(0);
        stdin.read_line(&mut line).unwrap();
        match line.trim() {
            "You guessed too low" => min = guess + 1,
            "You guessed too high" => max = guess - 1,
            "You got it!" => {
                println!("Done in {} tries (answer: {})!", tries, guess);
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

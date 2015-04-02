use std::old_io;

pub fn solve<R, W>(stdin : &mut old_io::BufferedReader<R>,
                   stdout : &mut old_io::BufferedWriter<W>)
  where R: old_io::Reader, W: old_io::Writer {
    let mut line = stdin.read_line().unwrap();
    assert_eq!(line.trim(), "Guess the number!");
    line = stdin.read_line().unwrap();

    let mut min : u32 = 1;
    let mut max_goal : u32 =
            line.trim().parse()
                       .ok()
                       .expect("max goal wasn't a number");
    let mut tries = 1u32;

    loop {
        line = stdin.read_line().unwrap();
        assert_eq!(line.trim(), "Please input your guess.");

        let guess : u32 = ((max_goal - min) / 2) + min;
        writeln!(stdout, "{} ({} rem)", guess, max_goal - min).unwrap();

        line = stdin.read_line().unwrap();
        match line.trim() {
            "You guessed too low" => min = guess + 1,
            "You guessed too high" => max_goal = guess - 1,
            "You got it!" => {
                writeln!(stdout, "Done in {} tries!", tries).unwrap();
                return;
            },
            _ => {
                stdout.write_line("Unexpected input!").unwrap();
                return;
            }
        }

        tries = tries + 1;
    }
}

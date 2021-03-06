use std::process::{Command, Stdio};
use std::env;

mod solver;

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        panic!("Usage: {} <path to guessing game>",
               env::args().nth(0).unwrap());
    }

    let path = args.nth(1).unwrap();

    println!("Running {}", path);
    match Command::new(path).stderr(Stdio::inherit())
                            .stdout(Stdio::piped())
                            .stdin(Stdio::piped())
                            .spawn() {
        Ok(mut child) => {
            {
                let mut stdin = child.stdin.as_mut().unwrap();
                let mut stdout = child.stdout.as_mut().unwrap();

                solver::solve(&mut std::io::BufReader::new(&mut stdout),
                              &mut std::io::BufWriter::new(&mut stdin));
            }
            child.wait().unwrap();
        }
        Err(e) => {
            panic!("Error running game: {}", e);
        }
    }
}

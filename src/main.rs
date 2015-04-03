#![feature(env)]
#![feature(io)]
#![feature(process)]
#![feature(core)]
use std::process::{Command, Stdio};
use std::env;

mod solver;

fn main() {
    if env::args().len() != 2 {
        panic!("Expected path to guessing_game");
    }

    let mut args = env::args();
    args.next();

    let path = args.next().unwrap();

    println!("Running {}", path);
    let mut child = Command::new(path.as_slice()).stderr(Stdio::inherit())
                                                 .stdout(Stdio::capture())
                                                 .stdin(Stdio::capture())
                                                 .spawn()
                                                 .unwrap();

    {
        let mut stdin = child.stdin.as_mut().unwrap();
        let mut stdout = child.stdout.as_mut().unwrap();

        solver::solve(&mut std::io::BufReader::new(&mut stdout),
                      &mut std::io::BufWriter::new(&mut stdin));
    }

    child.wait().unwrap();
}

use std::process::{Command, Stdio};
use std::env;

mod solver;

fn main() {
    if env::args().len() != 2 {
        panic!("Usage: {} <path to guessing game>",
               env::args().nth(0).unwrap());
    }

    let mut args = env::args();
    args.next();

    let path = args.next().unwrap();

    println!("Running {}", path);
    let mut child = Command::new(path).stderr(Stdio::inherit())
                                      .stdout(Stdio::piped())
                                      .stdin(Stdio::piped())
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

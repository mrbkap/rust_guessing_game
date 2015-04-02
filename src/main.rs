#![feature(old_io)]
#![feature(env)]
use std::old_io;
use std::old_io::process;
use std::old_io::Command;

use std::env;

mod solver;

fn main() {
    if env::args().len() != 2 {
        panic!("Expected path to guessing_game {}", env::args().len());
    }

    let mut args = env::args();
    args.next();

    let path = args.next().unwrap();

    println!("Running {}", path);
    let mut child = Command::new(path).stderr(std::old_io::process::InheritFd(2))
                                      .stdout(old_io::process::CreatePipe(false, true))
                                      .stdin(old_io::process::CreatePipe(true, false))
                                      .spawn()
                                      .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    let stdout = child.stdout.as_mut().unwrap();

    solver::solve(&mut old_io::BufferedReader::new(stdin),
                  &mut old_io::BufferedWriter::new(stdout));
}

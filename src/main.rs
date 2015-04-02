#![feature(old_io)]
use std::old_io;

mod solver;

fn main() {
    solver::solve(&mut old_io::stdin(), &mut old_io::stdout());
}

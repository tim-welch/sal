use crate::runner::run;

pub mod ast;
pub mod interpreter;
pub mod runner;
pub mod scanner;

fn main() {
    run()
}

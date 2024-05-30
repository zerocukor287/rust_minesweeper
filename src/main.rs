mod minesweeper;
use minesweeper::*;

fn main() {
    println!("Hello, minesweeper!");
    println!("{}", generate_map(255,21));
}

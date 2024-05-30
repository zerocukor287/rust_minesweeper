mod minesweeper;
use minesweeper::*;

fn main() {
    println!("Hello, minesweeper!");
    println!("{}", generate_map(29,21));
}

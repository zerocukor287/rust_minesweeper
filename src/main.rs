mod minesweeper;
use minesweeper::*;

fn main() {
    println!("Hello, minesweeper!");
    let mines = generate_map(8,6);
    println!("{}", visualize_map(mines));
}

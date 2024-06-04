mod minesweeper;
use minesweeper::*;

fn main() {
    println!("Hello, minesweeper!");
    let mines = generate_map(29,21);
    println!("{}", visualize_map(mines));
}

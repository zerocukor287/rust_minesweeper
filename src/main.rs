mod minesweeper;
use minesweeper::*;

fn main() {
    println!("Hello, minesweeper!");
    let mut mines = generate_map(8,6);
    println!("{}", visualize_map(&mines));
    fill_neighbours(&mut mines);
    println!("{}", visualize_map(&mines));
}

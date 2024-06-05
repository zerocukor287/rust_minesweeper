mod minesweeper;
use std::io;

use minesweeper::*;

fn main() {
    print_welcome();
    
    let mut mines = generate_map(8,6);
    fill_neighbours(&mut mines);

    loop {
        // show map
        println!("{}", visualize_map(&mines));

        // get input
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read.");

        // process input
        match translate_move(&guess) {
            MoveType::Unknown => println!("I don't understand this."),
            MoveType::Reveal { row, column } => {
                let result = reveal_tile(row as usize, column as usize, &mut mines);
                match result {
                    MoveResult::Explosion => {
                        println!("That was a mine. Game over."); break;
                    },
                    MoveResult::SafeMove => continue,
                    MoveResult::MakesNoSense => {
                        println!("I don't understand this.");
                    },
                }
            },
            MoveType::Defuse { row, column } => todo!(),
        };
    }
}

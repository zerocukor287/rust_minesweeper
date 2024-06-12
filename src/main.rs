mod minesweeper;
use std::io;

use minesweeper::*;

fn main() {
    print_welcome();
    
    let mut mines = generate_map(8,6);
    fill_neighbours(&mut mines);
    let mut first_guess = true;

    loop {
        // show map
        let (remaining, all) = get_progress(&mines);
        let all = all + remaining;
        println!("Progress: {remaining}/{all}");
        if remaining == all {
            println!("Success! All mines defused!");
            println!("{}", visualize_map(&mines, 'X'));
            break;
        }
        println!("{}", visualize_map(&mines, ' '));
        // get input
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read.");

        // process input
        if first_guess {
            while !process_input(&guess, &mut mines) {
                mines = generate_map(8,6);
            }
            first_guess = false;
        } else {
            if !process_input(&guess, &mut mines) {
                break;
            }
        }   
    }
}

fn process_input(guess: &str, mines: &mut Vec<Vec<TileState>>) -> bool{
    match translate_move(&guess) {
        MoveType::Unknown => println!("I don't understand this."),
        MoveType::Reveal { row, column } => {
            if row as usize >= mines.len() || column as usize >= mines[0].len() {
                println!("That tile is not existing."); return true;
            }
            let result = reveal_tile(row as usize, column as usize, mines);
            match result {
                MoveResult::Explosion => {
                    println!("That was a mine. Game over."); return false;
                },
                MoveResult::SafeMove => (),
                MoveResult::AlreadyRevealed => println!("Already revealed..."),
                MoveResult::MakesNoSense => {
                    println!("I don't understand this.");
                },
            }
        },
        MoveType::Defuse { row, column } => todo!(),
    };
    true
}

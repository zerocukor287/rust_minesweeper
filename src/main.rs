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
        let (visible, remaing) = get_progress(&mines);
        let all = remaing + visible;
        println!("Progress: {visible}/{all}");
        if visible == all {
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
                fill_neighbours(&mut mines);
            }
            first_guess = false;
        } else {
            if !process_input(&guess, &mut mines) {
                println!("That was a mine. Game over.");
                break;
            }
        }   
    }
}

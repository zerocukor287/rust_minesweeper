mod minesweeper;
use std::io;

use minesweeper::*;

fn main() {
    print_welcome();

    let mut still_playing = true;
    while still_playing {
        let (width, height) = get_size();
        let mut mines = generate_map(width,height);
        fill_neighbours(&mut mines);
        let mut first_guess = true;

        let (mut visible, mut remaing) = get_progress(&mines);
        let mut all = remaing + visible;
        while visible != all {
            // show map
            (visible, remaing) = get_progress(&mines);
            all = remaing + visible;
    
            println!("Progress: {visible}/{all}");
            println!("{}", visualize_map(&mines, ' '));
            // get input
            let mut guess = String::new();
            io::stdin().read_line(&mut guess)
                .expect("Failed to read.");

            // process input
            if want_to_quit(&guess) {
                still_playing = false;
                break;
            } else if restart(&guess) {
                break;
            } else if first_guess {
                while !process_input(&guess, &mut mines) {
                    mines = generate_map(8,6);
                    fill_neighbours(&mut mines);
                }
                first_guess = false;
            } else {
                if !process_input(&guess, &mut mines) {
                    println!("That was a mine. Game over.");
                    still_playing = start_again();
                    break;
                }
            }

        }
        if visible == all {
            println!("Success! All mines defused!");
            println!("{}", visualize_map(&mines, 'X'));
            still_playing = start_again();
        }
    }
}

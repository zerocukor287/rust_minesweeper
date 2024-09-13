mod minesweeper;
use std::io;

use minesweeper::*;

fn main() {
    print_welcome();

    let mut still_playing = true;
    while still_playing {
        print_stats(&get_stats());
        println!("Here is the mine field:");
        let (width, height) = get_size();
        let mut mines = generate_map(width,height);
        let mut first_guess = true;

        let (mut visible, mut remaing) = get_progress(&mines);
        let mut all = remaing + visible;
        while visible != all {
            // show map    
            println!("Progress: {visible}/{all}");
            println!("{}", visualize_map(&mines, ' ', true));
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
                    mines = generate_map(width,height);
                }
                first_guess = false;
            } else {
                if !process_input(&guess, &mut mines) {
                    println!("That was a mine. Game over.");
                    println!("{}", visualize_map(&mines, '*', true));
                    // defused so far
                    let mut defused: usize = 0;
                    for row in &mines {
                        defused += row.iter().filter(|tile| match tile {
                            TileState::Marked(num) => *num < 0,
                            _ => false
                        }).count();
                    }
                    save_stats(defused, visible, true);
                    still_playing = start_again();
                    break;
                }
            }

            (visible, remaing) = get_progress(&mines);
            all = remaing + visible;
        }
        if visible == all {
            println!("Success! All mines defused!");
            println!("{}", visualize_map(&mines, 'X', false));
            let total = mines.len() * mines[0].len();
            save_stats(total - visible, visible, false);
            still_playing = start_again();
        }
    }
    print_stats(&get_stats());
}

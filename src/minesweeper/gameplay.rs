use std::io::{self, ErrorKind};
use std::thread::sleep;
use std::time::Duration;

use rand::Rng;
use regex::Regex;

use super::map_generator::TileState;
use super::map_draw::*;

static QUIT_COMMANDS: [&str; 3] = ["q", "quit", "exit"];
static CREDITS_COMMANDS: [&str; 2] = ["credits", "credit"];
static RESTART_COMMANDS: [&str; 1] = ["restart"];
static HINT_COMMANDS: [&str; 1] = ["hint"];
static MAP_SIZE: [&str; 4] = ["s", "m", "l", "xl"];

pub fn print_welcome() {
    println!("Hello, minesweeper!\n");

    println!("Your task is to defuse all the mines.");
    print_help();
}

pub fn print_error_with_help() {
    println!("I don't understand this.\n");
    print_help()
}

pub fn print_help() {
    println!("To reveal a tile, type the column and row - like \"A1\" or \"28BC\"");
    println!("To mark as a potential mine, type \"mark\" with the position - like \"mark A1\" or \"mark 28BC\"");
    println!("To defuse a mine, type \"def\" with the position - like \"def A1\" or \"def 28BC\"\n");
    println!("Type \"def\" with the position again to remove the defuser.\n");
    println!("You can use a hint, type {} to reveal a random tile\n", join_tokens(HINT_COMMANDS));
    println!("If you want to restart the game, type {}\n", join_tokens(RESTART_COMMANDS));
    println!("If you want to close the game, type {}", join_tokens(QUIT_COMMANDS));
}

pub fn print_credits() {
    println!("");
    println!("   Chromatic Carrot");
    println!("www.chromaticcarrot.com\n");
    println!("  Graphics designer:");
    println!("(nobody)");
    println!("  Programmer:");
    println!("Balazs Erseki ~ zerocukor\n");
    sleep(Duration::from_secs(2));
}

pub fn want_to_quit(input: &str) -> bool {
    QUIT_COMMANDS.contains(&&input.trim().to_lowercase()[..])
}

pub fn credits(input: &str) -> bool {
    CREDITS_COMMANDS.contains(&&input.trim().to_lowercase()[..])
}

pub fn restart(input: &str) -> bool {
    RESTART_COMMANDS.contains(&&input.trim().to_lowercase()[..])
}

fn join_tokens<const L: usize>(array: [&str; L]) -> String {
    let mut ret = String::new();
    for (i, element) in array.iter().enumerate() {
        if i > 0 {
            if i == L-1 {
                // last element
                ret.push_str(", or ")
            } else {
                ret.push_str(", ")
            }
        }
        ret.push_str(&format!("'{}'", element));
    }
    ret
}

#[test]
fn join_tokens_test() {
    assert_eq!(join_tokens([]), "");
    assert_eq!(join_tokens(["General"]), "'General'");
    assert_eq!(join_tokens(["General", "Kenobi"]), "'General', or 'Kenobi'");
    assert_eq!(join_tokens(QUIT_COMMANDS), "'q', 'quit', or 'exit'");
}

pub fn get_size() -> (u8, u8) {
    println!("How big map would you like? s, m, l, xl");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read.");

        let input = input.trim().to_lowercase();
        if want_to_quit(&input) {
            return (0, 0);
        } else if credits(&input) {
            print_credits();
            println!("How big map would you like? s, m, l, xl");
        } else if input == "s" {
            return (6, 5)
        } else if input == "m" {
            return (10, 8);
        } else if input == "l" {
            return (15, 13);
        } else if input == "xl" {
            return (35, 30);
        } else {
            println!("I don't understand this: {}. Type {} to set map size or {} to quit",
                input, join_tokens(MAP_SIZE), join_tokens(QUIT_COMMANDS));
        }
    }
}

pub fn start_again() -> bool {
    let possible_inputs_no: [&str; 2] = ["n", "no"];
    let possible_inputs_yes: [&str; 2] = ["y", "yes"];
    let mut input = String::new();
    while !want_to_quit(&input) && !possible_inputs_no.contains(&input.trim()) && !possible_inputs_yes.contains(&input.trim()) {
        input.clear();
        println!("Do you want to start again? (y/n)");
        io::stdin().read_line(&mut input)
            .expect("Failed to read.");
    }
    possible_inputs_yes.contains(&input.trim())
}

pub enum MoveResult {
    Explosion,
    SafeMove,
    MakesNoSense,
    AlreadyRevealed,
}

pub fn process_input(guess: &str, mines: &mut Vec<Vec<TileState>>) -> bool{
    match translate_move(&guess) {
        MoveType::Unknown =>print_error_with_help(),
        MoveType::Reveal { row, column } => {
            if row as usize >= mines.len() || column as usize >= mines[0].len() {
                println!("That tile is not existing."); return true;
            }
            match reveal_tile(row as usize, column as usize, mines, false) {
                MoveResult::Explosion => {
                    return false;
                },
                MoveResult::SafeMove => (),
                MoveResult::AlreadyRevealed => println!("Already revealed..."),
                MoveResult::MakesNoSense => {
                    print_error_with_help();
                },
            }
        },
        MoveType::Defuse { row, column } => {
            if row as usize >= mines.len() || column as usize >= mines[0].len() {
                println!("That tile is not existing.");
                return true;
            }
            match defuse_tile(row as usize, column as usize, mines) {
                MoveResult::Explosion => {
                    return false;
                },
                MoveResult::SafeMove => (),
                MoveResult::AlreadyRevealed => println!("Already revealed..."),
                MoveResult::MakesNoSense => {
                    println!("Type 'def' with position to remove the defuser.");
                },
            }
        },
        MoveType::Mark { row, column } => {
            if row as usize >= mines.len() || column as usize >= mines[0].len() {
                println!("That tile is not existing.");
                return true;
            }
            match mark_tile(row as usize, column as usize, mines) {
                MoveResult::Explosion => {
                    return false;
                },
                MoveResult::SafeMove => (),
                MoveResult::AlreadyRevealed => println!("Already revealed..."),
                MoveResult::MakesNoSense => {
                    println!("Type 'def' with position to remove the defuser.");
                },
            }
        },
        MoveType::Hint => {
            show_hint(mines);
        }
    };
    true
}

fn best_hint(mines: &Vec<Vec<TileState>>) -> i16 {
    let mut best_hidden: i16 = 9;    // 8 is max
    for line in mines {
        for tile in line {
            match tile {
                TileState::Marked(num) => if *num > 0 && *num < best_hidden {best_hidden = *num;},
                TileState::HiddenEmpty(num ) => if i16::from(*num) < best_hidden {best_hidden = (*num) as i16;},
                TileState::Question(num) => if *num > 0 && *num < best_hidden {best_hidden = *num;},
                _ => {},
            }
        }
    }
    best_hidden
}

pub fn show_hint( mines: &mut Vec<Vec<TileState>>) -> MoveResult {
    let best_hidden = best_hint(mines);

    let rand_column = rand::thread_rng().gen_range(0..=mines[0].len()-1);
    let rand_row = rand::thread_rng().gen_range(0..=mines.len()-1);
    match mines[rand_row][rand_column] {
        TileState::Mine => show_hint(mines),
        TileState::Marked(num) => if num < 0 || num != best_hidden {show_hint(mines)} else {defuse_tile(rand_row, rand_column, mines);reveal_tile(rand_row, rand_column, mines, true)},
        TileState::HiddenEmpty(num) => if i16::from(num) == best_hidden { reveal_tile(rand_row, rand_column, mines, true) } else { show_hint(mines) },
        TileState::VisibleEmpty(_) => show_hint(mines),
        TileState::Question(num) => if num < 0 || num != best_hidden {show_hint(mines)} else {reveal_tile(rand_row, rand_column, mines, true)},
    }
}

pub fn reveal_tile(row: usize, column: usize, mine_map: &mut Vec<Vec<TileState>>, force: bool) -> MoveResult {
    mine_map[row][column] = match mine_map[row][column]{
        TileState::Mine => return MoveResult::Explosion,
        TileState::Marked(num) =>
            if !force || num < 0 {
                return MoveResult::MakesNoSense
            } else {
                TileState::VisibleEmpty(num as u8)
            }
        TileState::HiddenEmpty(x) => TileState::VisibleEmpty(x),
        TileState::VisibleEmpty(_) => return MoveResult::AlreadyRevealed,
        TileState::Question(x) => if x < 0 { return MoveResult::Explosion } else { TileState::VisibleEmpty(x as u8) },
    };

    // reveal neighbors
    if mine_map[row][column] == TileState::VisibleEmpty(0) {
        if row > 0 {
            if column > 0 {
                reveal_tile(row-1, column-1, mine_map, true);
            }
            reveal_tile(row-1, column, mine_map, true);
            if column + 1 < mine_map[0].len() {
                reveal_tile(row-1, column+1, mine_map, true);
            }
        }
        if column > 0 {
            reveal_tile(row, column-1, mine_map, true);
        }
        if column + 1 < mine_map[0].len() {
            reveal_tile(row, column+1, mine_map, true);
        }
        if row + 1 < mine_map.len() {
            if column > 0 {
                reveal_tile(row+1, column-1, mine_map, true);
            }
            reveal_tile(row+1, column, mine_map, true);
            if column + 1 < mine_map[0].len() {
                reveal_tile(row+1, column+1, mine_map, true);
            }
        }
    }
    // still any move left
    MoveResult::SafeMove
}

pub fn defuse_tile(row: usize, column: usize, mine_map: &mut Vec<Vec<TileState>>) -> MoveResult {
    mine_map[row][column] = match mine_map[row][column]{
        TileState::Mine => TileState::Marked(-1),
        TileState::Marked(num) => if num < 0 {TileState::Mine} else {TileState::HiddenEmpty(num as u8)},
        TileState::HiddenEmpty(num) => TileState::Marked(num as i16),
        TileState::VisibleEmpty(_) => return MoveResult::AlreadyRevealed,
        TileState::Question(x) => TileState::Marked(x),
    };

    MoveResult::SafeMove
}

pub fn mark_tile(row: usize, column: usize, mine_map: &mut Vec<Vec<TileState>>) -> MoveResult {
    mine_map[row][column] = match mine_map[row][column]{
        TileState::Mine => TileState::Question(-1),
        TileState::Marked(num) => TileState::Question(num),
        TileState::HiddenEmpty(num) => TileState::Question(num as i16),
        TileState::VisibleEmpty(_) => return MoveResult::AlreadyRevealed,
        TileState::Question(x) => if x < 0 { TileState::Mine } else { TileState::HiddenEmpty(x as u8) },
    };

    MoveResult::SafeMove
}

#[derive(PartialEq, Debug)]
pub enum MoveType {
    Unknown,
    Hint,
    Reveal{row: u8, column: u8},
    Defuse{row: u8, column: u8},
    Mark{row: u8, column: u8}
}

pub fn translate_move(input: &str) -> MoveType {
    let move_regex = Regex::new("^[0-9]+[a-zA-Z]+$|^[a-zA-Z]+[0-9]+$").unwrap();
    if input.starts_with("def ") {
        let index = parse_index(&input.trim()[4..]);
        match index {
            Ok((row, column)) => MoveType::Defuse { row, column },
            Err(_) => MoveType::Unknown,
        }
    } else if input.starts_with("mark ") {
        let index = parse_index(&input.trim()[5..]);
        match index {
            Ok((row, column)) => MoveType::Mark { row, column },
            Err(_) => MoveType::Unknown,
        }
    } else if HINT_COMMANDS.contains(&&input.trim().to_lowercase()[..]) {
        MoveType::Hint
    } else if move_regex.is_match(input.trim()) {
        let index = parse_index(&input);
        match index {
            Ok((row, column)) => MoveType::Reveal { row, column },
            Err(_) => MoveType::Unknown,
        }
    } else {
        MoveType::Unknown
    }
}

fn parse_index(input: &str) -> Result<(u8, u8), ErrorKind> {
    let row = get_row_number(&input.chars().filter(|c| c.is_alphabetic()).collect::<String>());
    let column = get_column_number(&input.chars().filter(|c| c.is_numeric()).collect::<String>());

    match (row, column) {
        (Ok(row), Ok(column)) => Ok((row, column)),
        _ => Err(ErrorKind::InvalidInput)
    }
}

#[test]
fn translate_move_test() {
    assert_eq!(MoveType::Reveal{row: 0, column: 0}, translate_move("A1"));
    assert_eq!(MoveType::Reveal{row: 0, column: 0}, translate_move("A1\n"));
    assert_eq!(MoveType::Defuse{row: 1, column: 1}, translate_move("def B2"));
    assert_eq!(MoveType::Defuse{row: 6, column: 4}, translate_move("def 5g"));
    assert_eq!(MoveType::Mark{row: 6, column: 1}, translate_move("mark 2g"));
    assert_eq!(MoveType::Mark{row: 2, column: 13}, translate_move("mark   c14   "));
    assert_eq!(MoveType::Unknown, translate_move("help"));
}

#[test]
fn parse_index_test() {
    assert_eq!(Ok((0,0)), parse_index("A1"));
    assert_eq!(Err(ErrorKind::InvalidInput), parse_index("A0"));

    assert_eq!(Ok((1,0)), parse_index("B1"));
    assert_eq!(Ok((1,0)), parse_index("1b"));
}

#[test]
fn double_defuse() {
    let row_1 = vec![TileState::HiddenEmpty(0)];
    let mut test_map = vec![row_1];

    defuse_tile(0, 0, &mut test_map);
    defuse_tile(0, 0, &mut test_map);

    assert_eq!(test_map[0][0], TileState::HiddenEmpty(0));
}
use std::io::ErrorKind;

use super::map_generator::TileState;
use super::map_draw::*;

pub fn print_welcome() {
    println!("Hello, minesweeper!\n");

    println!("Your task is to defuse all the mines.");
    println!("To reveal a tile, type the column and row - like \"A1\" or \"28BC\"");
    //println!("To mark as a potential mine, type \"mark\" with the position - like \"mark A1\" or \"mark 28BC\"");
    //println!("To defuse a mine, type \"def\" with the position - like \"def A1\" or \"def 28BC\"\n");

    println!("Here is the mine field:");
}

pub enum MoveResult {
    Explosion,
    SafeMove,
    MakesNoSense,
    AlreadyRevealed,
}

pub fn reveal_tile(row: usize, column: usize, mine_map: &mut Vec<Vec<TileState>>) -> MoveResult {
    mine_map[row][column] = match mine_map[row][column]{
        TileState::Mine => return MoveResult::Explosion,
        TileState::MineDefused => return MoveResult::MakesNoSense,
        TileState::HiddenEmpty(x) => TileState::VisibleEmpty(x),
        TileState::VisibleEmpty(_) => return MoveResult::AlreadyRevealed
    };

    // still any move left
    MoveResult::SafeMove
}

#[derive(PartialEq, Debug)]
pub enum MoveType {
    Unknown,
    Reveal{row: u8, column: u8},
    Defuse{row: u8, column: u8},
}

pub fn translate_move(input: &str) -> MoveType {
    if input.starts_with("def ") {
        let index = parse_index(&input.trim()[4..]);
        match index {
            Ok((row, column)) => MoveType::Defuse { row, column },
            Err(_) => MoveType::Unknown,
        }
    } else {
        let index = parse_index(&input);
        match index {
            Ok((row, column)) => MoveType::Reveal { row, column },
            Err(_) => MoveType::Unknown,
        }
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
    assert_eq!(MoveType::Defuse{row: 1, column: 1}, translate_move("def B2"));
    assert_eq!(MoveType::Defuse{row: 6, column: 4}, translate_move("def 5g"));
}

#[test]
fn parse_index_test() {
    assert_eq!(Ok((0,0)), parse_index("A1"));
    assert_eq!(Err(ErrorKind::InvalidInput), parse_index("A0"));

    assert_eq!(Ok((1,0)), parse_index("B1"));
    assert_eq!(Ok((1,0)), parse_index("1b"));
}
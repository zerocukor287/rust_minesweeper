use std::io::ErrorKind;

use super::map_generator::TileState;

/// Generates a 2D map for minesweeper
pub fn visualize_map(mine_map: &Vec<Vec<TileState>>) -> String {
    let mut map = String::new();
    map.push_str(add_first_line(mine_map[0].len() as u8).as_str());
    map.push('\n');
    for row in 0..(mine_map.len() as u8) {
        map.push_str(generate_line(&mine_map[row as usize]).as_str());
        map.push(' ');
        map.push_str(add_row_number(row).as_str());
        map.push('\n');
    }
    map
}

const MAX_ASCII_CHARACTERS: u8 = 26;    // chars A-Z
const FIRST_ASCII_CHARACTERS: u8 = 65;  // letter A

/// Adds a header at the end of the row as characters
/// 
/// It starts from 'A' till 'Z' then adds and extra 'A', like 'AA' for row 26, 'AB' for row 27 etc.
fn add_row_number(row: u8) -> String {
    let mut ret = String::new();
    if row >= MAX_ASCII_CHARACTERS {
        ret.push_str(&add_row_number(row / MAX_ASCII_CHARACTERS - 1));
    }

    let mut characters: Vec<u8> = Vec::new();
    let ascii_char = (row % MAX_ASCII_CHARACTERS) + FIRST_ASCII_CHARACTERS;
    characters.push(ascii_char);
    ret.push_str(std::str::from_utf8(&characters[..]).expect("Ohh boi! I cannot convert numbers to characters"));
    ret
}

pub fn get_row_number(input: &str) -> Result<u8, ErrorKind> {
    if input.is_empty() {
        return Err(ErrorKind::InvalidInput);
    }
    let mut sum = 0;
    let mut first = true;
    for ch in input.chars() {
        if !ch.is_ascii_alphabetic() {
            return Err(ErrorKind::InvalidInput);
        }
        if !first {
            sum = (1 + sum) * MAX_ASCII_CHARACTERS;
        }
        sum += ch.to_ascii_uppercase() as u8 - FIRST_ASCII_CHARACTERS;
        first = false;
    }
    Ok(sum)
}

#[test]
fn add_row_number_test() {
    assert_eq!("A", add_row_number(0));
    assert_eq!("E", add_row_number(4));
    assert_eq!("Z", add_row_number(25));
    assert_eq!("AA", add_row_number(26));
    assert_eq!("BA", add_row_number(52));
    assert_eq!("IV", add_row_number(255));
}

#[test]
fn get_row_number_test() {
    assert_eq!(Ok(0), get_row_number("A"));
    assert_eq!(Ok(0), get_row_number("a"));
    assert_eq!(Ok(1), get_row_number("B"));
    assert_eq!(Ok(8), get_row_number("I"));
    assert_eq!(Ok(26), get_row_number("AA"));
    assert_eq!(Ok(52), get_row_number("BA"));
    assert_eq!(Ok(52), get_row_number("Ba"));
    assert_eq!(Ok(52), get_row_number("bA"));
    assert_eq!(Ok(52), get_row_number("ba"));
    assert_eq!(Ok(255), get_row_number("IV"));
}

#[test]
fn get_row_number_test_invalid() {
    assert_eq!(Err(ErrorKind::InvalidInput), get_row_number(""));
    assert_eq!(Err(ErrorKind::InvalidInput), get_row_number("7"));
    assert_eq!(Err(ErrorKind::InvalidInput), get_row_number("A7c"));
}

fn number_of_spaces(width: u8) -> u8 {
    let log: u8 = u8::ilog10(width).try_into().unwrap();
    log
}

/// Generates the first row, aka header to the map
/// 
/// Empty string in case of 0, and then numbers separated by spaces.
/// The number of the spaces depends on the maximum column number.
fn add_first_line(width: u8) -> String {
    let mut line = String::new();
    if width == 0 {
        return line;
    }
    let spaces = number_of_spaces (width) + 1;
    line.push(' ');
    for number in 0..width {
        line.push_str((number + 1).to_string().as_str());
        let digits = number_of_spaces(number + 1);
        for _ in 0..(spaces-digits) {
            line.push(' ');
        }
    }
    line
}

#[test]
fn add_first_line_test() {
    assert_eq!("", add_first_line(0));
    assert_eq!(" 1 2 3 4 5 ", add_first_line(5));
    assert_eq!(" 1  2  3  4  5  6  7  8  9  10 ", add_first_line(10));
    assert_eq!(" 1  2  3  4  5  6  7  8  9  10 11 12 ", add_first_line(12));
}

fn generate_line(mine_line: &Vec<TileState>) -> String {
    let mut line = String::new();
    let spaces = number_of_spaces (mine_line.len() as u8);
    line.push('|');
    for tile in mine_line.iter() {
        match tile {
            TileState::Mine => line.push('o'),
            TileState::MineDefused => line.push('X'),
            TileState::HiddenEmpty(_) => line.push('o'),
            TileState::VisibleEmpty(num) => line.push_str(num.to_string().as_str()), 
        }
        line.push('|');
        for _ in 0..spaces {
            line.push(' ');
        }
    }
    line
}

pub fn get_column_number(input: &str) -> Result<u8, ErrorKind> {
    match input.parse::<u8>() {
        Ok(num) => Ok(num-1),
        _ => Err(ErrorKind::InvalidInput),
    }
}

#[test]
fn column_number_test() {
    assert_eq!(Ok(0), get_column_number("1"));
    assert_eq!(Ok(13), get_column_number("14"));
}
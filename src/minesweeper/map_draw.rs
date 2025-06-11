use std::io::ErrorKind;

use crossterm::style::{StyledContent, Stylize};

use super::map_generator::TileState;

/// Generates a 2D map for minesweeper
pub fn visualize_map(
    mine_map: &Vec<Vec<TileState>>,
    mine_char: char,
    show_revealed: bool,
) {
    let mut map: Vec<StyledContent<String>> = Vec::new();
    map.push(add_first_line(mine_map[0].len() as u8).stylize());
    map.push("\n".to_string().stylize());
    for row in 0..(mine_map.len() as u8) {
        map.append(&mut generate_line(
            &mine_map[row as usize],
            mine_char,
            show_revealed,
        ));
        map.push(" ".to_string().stylize());
        map.push(add_row_number(row).stylize());
        map.push("\n".to_string().stylize());
    }

    for token in map {
        print!("{}", token);
    }
}

pub fn get_progress(mine_map: &Vec<Vec<TileState>>) -> (usize, usize) {
    let mut visible_tiles = 0;
    let mut remaining_tiles = 0;
    for row in mine_map {
        visible_tiles += row
            .iter()
            .filter(|tile| match tile {
                TileState::VisibleEmpty(_) => true,
                _ => false,
            })
            .count();
        remaining_tiles += row
            .iter()
            .filter(|tile| match tile {
                TileState::HiddenEmpty(_) => true,
                TileState::Question(num) => *num >= 0,
                TileState::Marked(num) => *num >= 0,
                _ => false,
            })
            .count();
    }
    (visible_tiles, remaining_tiles)
}

const MAX_ASCII_CHARACTERS: u8 = 26; // chars A-Z
const FIRST_ASCII_CHARACTERS: u8 = 65; // letter A

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
    ret.push_str(
        std::str::from_utf8(&characters[..])
            .expect("Ohh boi! I cannot convert numbers to characters"),
    );
    ret
}

pub fn get_row_number(input: &str) -> Result<u8, ErrorKind> {
    if input.is_empty() || input.len() > 2 {
        return Err(ErrorKind::InvalidInput);
    }
    // max can be IV
    let input = input.to_uppercase();
    if input.len() == 2 {
        if input.chars().nth(0).unwrap() > 'I' {
            return Err(ErrorKind::InvalidInput);
        } else if input.chars().nth(0).unwrap() == 'I' && input.chars().nth(1).unwrap() > 'V' {
            return Err(ErrorKind::InvalidInput);
        }
    }
    // calculate numeric value
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
    assert_eq!(Ok(255), get_row_number("iv"));
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
    let spaces = number_of_spaces(width) + 1;
    line.push(' ');
    for number in 0..width {
        line.push_str((number + 1).to_string().as_str());
        let digits = number_of_spaces(number + 1);
        for _ in 0..(spaces - digits) {
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

fn generate_line(
    mine_line: &Vec<TileState>,
    mine_char: char,
    show_revealed: bool,
) -> Vec<StyledContent<String>> {
    let mut styled_line: Vec<StyledContent<String>> = Vec::new();

    //styled_line.push("blue".blue());

    let spaces = number_of_spaces(mine_line.len() as u8);
    styled_line.push("|".to_string().stylize());
    // add spaces
    let mut spaces_text = String::new();
    for _ in 0..spaces {
        spaces_text.push(' ');
    }
    styled_line.push(spaces_text.clone().stylize());
    for tile in mine_line.iter() {
        match tile {
            TileState::Mine => styled_line.push(mine_char.to_string().stylize()),
            TileState::Marked(num) => styled_line.push(
                if mine_char == ' ' {
                    ".".to_string().stylize()
                } else {
                    if *num < 0 {
                        mine_char.to_string().stylize()
                    } else {
                        "M".to_string().stylize()
                    }
                }),
            TileState::HiddenEmpty(_) => styled_line.push(" ".to_string().stylize()),
            TileState::VisibleEmpty(num) => {
                if show_revealed {
                    if *num == 0 || mine_char != ' '{
                        styled_line.push(num.to_string().stylize())
                    } else if *num == 1 {
                        styled_line.push(num.to_string().blue())
                    } else if *num == 2 {
                        styled_line.push(num.to_string().green())
                    } else if *num == 3 {
                        styled_line.push(num.to_string().yellow())
                    } else if *num == 4 {
                        styled_line.push(num.to_string().red())
                    } else if *num == 5 {
                        styled_line.push(num.to_string().magenta())
                    } else if *num == 6 {
                        styled_line.push(num.to_string().cyan())
                    } else if *num == 7 {
                        styled_line.push(num.to_string().dark_yellow())
                    } else if *num == 8 {
                        styled_line.push(num.to_string().dark_red())
                    }
                } else {
                    styled_line.push(" ".to_string().stylize())
                }
            }
            TileState::Question(num) => styled_line.push(
                if mine_char == ' ' {
                    "?".to_string().stylize()
                } else {
                    if *num < 0 {
                        mine_char.to_string().stylize()
                    } else {
                        "?".to_string().stylize()
                    }
                }),
        }
        styled_line.push("|".to_string().stylize());
        styled_line.push(spaces_text.clone().stylize());
    }
    styled_line
}

pub fn get_column_number(input: &str) -> Result<u8, ErrorKind> {
    match input.parse::<u8>() {
        Ok(num) => {
            if num > 0 {
                Ok(num - 1)
            } else {
                Err(ErrorKind::InvalidInput)
            }
        }
        _ => Err(ErrorKind::InvalidInput),
    }
}

#[test]
fn column_number_test() {
    assert_eq!(Ok(0), get_column_number("1"));
    assert_eq!(Ok(13), get_column_number("14"));
}

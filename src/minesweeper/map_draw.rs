
/// Generates a 2D map for minesweeper
pub fn generate_map(rows: u8, columns: u8) -> String {
    let mut map = String::new();
    map.push_str(add_first_line(columns).as_str());
    map.push('\n');
    for row in 0..rows {
        map.push_str(generate_line(columns).as_str());
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

#[test]
fn add_row_number_test() {
    assert_eq!("A", add_row_number(0));
    assert_eq!("E", add_row_number(4));
    assert_eq!("Z", add_row_number(25));
    assert_eq!("AA", add_row_number(26));
    assert_eq!("BA", add_row_number(52));
    assert_eq!("IV", add_row_number(255));
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

fn generate_line(width: u8) -> String {
    let mut line = String::new();
    let spaces = number_of_spaces (width);
    line.push('|');
    for _ in 0..width {
        line.push_str("o|");
        for _ in 0..spaces {
            line.push(' ');
        }
    }
    line
}


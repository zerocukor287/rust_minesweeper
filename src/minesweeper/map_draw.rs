
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
}

fn add_first_line(width: u8) -> String {
    let mut line = String::new();
    line.push(' ');
    for number in 0..width {
        line.push_str((number + 1).to_string().as_str());
        line.push(' ');
    }
    line
}

fn generate_line(width: u8) -> String {
    let mut line = String::new();
    line.push('|');
    for _ in 0..width {
        line.push_str("o|");
    }
    line
}


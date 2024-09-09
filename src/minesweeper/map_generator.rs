use rand::Rng;

#[cfg(test)]
use crate::reveal_tile;

#[derive(Clone, PartialEq, Debug)]
pub enum TileState {
    Mine,
    Marked(i16),
    HiddenEmpty(u8),
    VisibleEmpty(u8),
    Question(i16),
}

pub fn generate_map(width: u8, height: u8) -> Vec<Vec<TileState>> {
    // Fill with mines
    let generator = || match rand::thread_rng().gen_range(1..=100) {
        0..=20 => TileState::Mine,
        _ => TileState::HiddenEmpty(0),
    };
    
    let mut ret: Vec<Vec<TileState>> = Vec::new();
    ret.reserve(height as usize);

    // generate an array with mines
    for _ in 0..height {
        let mut row: Vec<TileState> = Vec::new();
        row.reserve(width as usize);
        for _ in 0..width {
            row.push(generator());
        }
        ret.push(row);
    }
    fill_neighbours(&mut ret);
    ret
}

fn fill_neighbours(mines: &mut Vec<Vec<TileState>>) {

    let height = mines.len();
    if height > 0 {
        let width = mines[0].len();

        for row in 0..height {
            for column in 0..width {
                if mines[row][column] == TileState::Mine {
                    continue;   // no calculation, it is a mine
                }

                mines[row][column] = count_neigbour_mines(row, column, mines, height, width);
            }
        }
    }
}

fn add_one(tile: &TileState) -> TileState {
 // fill the numbers for the neighbours of the mines
    match tile {
        TileState::Mine => TileState::HiddenEmpty(1),
        TileState::HiddenEmpty(x) => TileState::HiddenEmpty(1 + x),
        _ => panic!("Visible or defused tile"),
    }
}

pub fn count_neigbour_mines(row: usize, column: usize, mines: &mut Vec<Vec<TileState>>, height: usize, width: usize) -> TileState {    
    let mut tile = TileState::HiddenEmpty(0);
    
    // top row
    if row > 0 {
        let row_local = row-1;
        // left
        if column > 0 {
            if mines[row_local][column-1] == TileState::Mine {
                tile = add_one(&tile);
            }
        }
        // middle
        if mines[row_local][column] == TileState::Mine {
            tile = add_one(&tile);
        }
        // right
        if column+1 < width as usize {
            if mines[row_local][column+1] == TileState::Mine {
                tile = add_one(&tile);
            }
        }
    }

    // check this row
    // left
    if column > 0 {
        if mines[row][column-1] == TileState::Mine {
            tile = add_one(&tile);
        }
    }
    // right
    if column+1 < width as usize {
        if mines[row][column+1] == TileState::Mine {
            tile = add_one(&tile);
        }
    }

    // bottom row
    if row+1 < height as usize {
        let row_local = row+1;
        // left
        if column > 0 {
            if mines[row_local][column-1] == TileState::Mine {
                tile = add_one(&tile);
            }
        }
        // middle
        if mines[row_local][column] == TileState::Mine {
            tile = add_one(&tile);
        }
        // right
        if column+1 < width as usize {
            if mines[row_local][column+1] == TileState::Mine {
                tile = add_one(&tile);
            }
        }
    }
    tile
}

#[test]
fn generate_small_map() {
    assert_eq!(0, generate_map(0,0).len());
    assert_eq!(1, generate_map(0,1).len());
    assert_eq!(0, generate_map(0,1)[0].len());
    assert_eq!(2, generate_map(2,1)[0].len());
}

#[test]
fn fill_neighbours_test() {
    let row_1 = vec![TileState::HiddenEmpty(0), TileState::Mine];
    let row_2 = vec![TileState::HiddenEmpty(0), TileState::HiddenEmpty(0)];
    let mut test_map = vec![row_1, row_2];

    assert_eq!(TileState::HiddenEmpty(0), test_map[0][0]);

    fill_neighbours(&mut test_map);

    assert_eq!(TileState::HiddenEmpty(1), test_map[0][0]);
    assert_eq!(TileState::HiddenEmpty(1), test_map[1][0]);
    assert_eq!(TileState::HiddenEmpty(1), test_map[1][1]);
}

#[test]
fn bug_1_fill_neighbours_test() {
    let row_1 = vec![TileState::HiddenEmpty(0), TileState::Mine, TileState::HiddenEmpty(0)];
    let row_2 = vec![TileState::Mine, TileState::HiddenEmpty(0), TileState::HiddenEmpty(0)];
    let row_3 = vec![TileState::Mine, TileState::Mine, TileState::Mine];
    let mut test_map = vec![row_1, row_2, row_3];

    assert_eq!(test_map.len(), 3);

    fill_neighbours(&mut test_map);

    assert_eq!(test_map[1][1], TileState::HiddenEmpty(5));
    // ----------
    let mut mine_map = parse_map(
" |  |  |  |  |  |  | *|  |  
*|  | *|  |  |  | *| *|  |  
 |  |  |  |  |  |  |  | *|  
 |  |  |  | *|  |  |  |  |  
 | *|  | *|  |  |  | *|  |  
 |  |  | *| *| *|  |  | *|  
 |  | *|  | *| *|  |  |  | *
*|  |  |  |  | *|  | *|  |  ");
    fill_neighbours(&mut mine_map);

    assert_eq!(mine_map[4][4], TileState::HiddenEmpty(5));

    reveal_tile(4, 4, &mut mine_map, false);

    assert_eq!(mine_map[4][4], TileState::VisibleEmpty(5));
}

#[cfg(test)]
fn parse_map(input: &str) -> Vec<Vec<TileState>> {
    let mut mine_map:Vec<Vec<TileState>> = Vec::new();
    for input_line in input.lines() {
        let mut mine_line: Vec<TileState>= Vec::new();
        for token in input_line.split('|') {
            if token.trim() == "*" {
                mine_line.push(TileState::Mine);
            } else {
                mine_line.push(TileState::HiddenEmpty(0));
            }
        }
        mine_map.push(mine_line);
    }

    mine_map
}
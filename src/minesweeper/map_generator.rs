use rand::Rng;

#[derive(Clone, PartialEq, Debug)]
pub enum TileState {
    Mine,
    MineDefused,
    HiddenEmpty(u8),
    VisibleEmpty(u8),
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

    ret
}

pub fn fill_neighbours(mines: &mut Vec<Vec<TileState>>) {
    // fill the numbers for the neighbours of the mines
    let add_one = |tile: &TileState| {
        match tile {
            TileState::Mine => TileState::HiddenEmpty(1),
            TileState::HiddenEmpty(x) => TileState::HiddenEmpty(1 + x),
            _ => panic!("Visible or defused tile"),
        }
    };

    let height = mines.len();
    let width = mines[0].len();

    for row in 0..height {
        for column in 0..width {
            if mines[row][column] == TileState::Mine {
                continue;   // no calculation, it is a mine
            }

            // top row
            if row > 0 {
                let row_local = row-1;
                // left
                if column > 0 {
                    if mines[row_local][column-1] == TileState::Mine {
                        mines[row][column] = add_one(&mines[row][column]);
                    }
                }
                // middle
                if mines[row_local][column] == TileState::Mine {
                    mines[row][column] = add_one(&mines[row][column]);
                }
                // right
                if column+1 < width as usize {
                    if mines[row_local][column+1] == TileState::Mine {
                        mines[row][column] = add_one(&mines[row][column]);
                    }
                }
            }

            // check this row
            // left
            if column > 0 {
                if mines[row][column-1] == TileState::Mine {
                    mines[row][column] = add_one(&mines[row][column]);
                }
            }
            // right
            if column+1 < width as usize {
                if mines[row][column+1] == TileState::Mine {
                    mines[row][column] = add_one(&mines[row][column]);
                }
            }

            // bottom row
            if row+1 < height as usize {
                let row_local = row+1;
                // left
                if column > 0 {
                    if mines[row_local][column-1] == TileState::Mine {
                        mines[row][column] = add_one(&mines[row][column]);
                    }
                }
                // middle
                if mines[row_local][column] == TileState::Mine {
                    mines[row][column] = add_one(&mines[row][column]);
                }
                // right
                if column+1 < width as usize {
                    if mines[row_local][column+1] == TileState::Mine {
                        mines[row][column] = add_one(&mines[row][column]);
                    }
                }
            }
        }
    }
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
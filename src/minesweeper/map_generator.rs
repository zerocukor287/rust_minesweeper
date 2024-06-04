use rand::Rng;

#[derive(Clone, PartialEq)]
pub enum TileState {
    Mine,
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

    // fill the numbers for the neighbours of the mines
    let add_one = |tile: &TileState| {
        match tile {
            TileState::Mine => TileState::HiddenEmpty(1),
            TileState::HiddenEmpty(x) => TileState::HiddenEmpty(1 + x),
            TileState::VisibleEmpty(_) => panic!("Generated a visible tile!"),
        }
    };

    for row in 0..height as usize {
        for column in 0..width as usize {
            // top row
            if row > 1 {
                let row_local = row-1;
                // left
                if column > 1 {
                    if ret[row_local][column-1] == TileState::Mine {
                        ret[row][column] = add_one(&ret[row][column]);
                    }
                }
                // middle
                if ret[row_local][column] == TileState::Mine {
                    ret[row][column] = add_one(&ret[row][column]);
                }
                // right
                if column+1 < width as usize {
                    if ret[row_local][column+1] == TileState::Mine {
                        ret[row][column] = add_one(&ret[row][column]);
                    }
                }
            }

            // check this row
            // left
            if column > 1 {
                if ret[row][column-1] == TileState::Mine {
                    ret[row][column] = add_one(&ret[row][column]);
                }
            }
            // right
            if column+1 < width as usize {
                if ret[row][column+1] == TileState::Mine {
                    ret[row][column] = add_one(&ret[row][column]);
                }
            }

            // bottom row
            if row+1 < height as usize {
                let row_local = row+1;
                // left
                if column > 1 {
                    if ret[row_local][column-1] == TileState::Mine {
                        ret[row][column] = add_one(&ret[row][column]);
                    }
                }
                // middle
                if ret[row_local][column] == TileState::Mine {
                    ret[row][column] = add_one(&ret[row][column]);
                }
                // right
                if column+1 < width as usize {
                    if ret[row_local][column+1] == TileState::Mine {
                        ret[row][column] = add_one(&ret[row][column]);
                    }
                }
            }
        }
    }

    ret
}

#[test]
fn generate_small_map() {
    assert_eq!(0, generate_map(0,0).len());
    assert_eq!(1, generate_map(0,1).len());
    assert_eq!(0, generate_map(0,1)[0].len());
    assert_eq!(2, generate_map(2,1)[0].len());
}
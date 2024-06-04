use rand::{Rng};

#[derive(Clone)]
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

#[test]
fn generate_small_map() {
    assert_eq!(0, generate_map(0,0).len());
    assert_eq!(1, generate_map(0,1).len());
    assert_eq!(0, generate_map(0,1)[0].len());
    assert_eq!(2, generate_map(2,1)[0].len());
}
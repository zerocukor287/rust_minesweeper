#[derive(Clone)]
pub enum TileState {
    Mine,
    HiddenEmpty(u8),
    VisibleEmpty(u8),
}

pub fn generate_map(width: u8, height: u8) -> Vec<Vec<TileState>> {
    // Base 1d array
    let mut grid_raw:Vec<TileState> = vec![TileState::HiddenEmpty(0); height as usize];

    // Vector of 'width' elements slices
    let mut grid_base: Vec<Vec<TileState>> = vec![grid_raw; width as usize];

    // Final 2d array `&mut [&mut [_]]`
    //let mut grid = grid_base.as_mut_slice();

    // Accessing data
    //grid[0][0] = 4;
    grid_base
}
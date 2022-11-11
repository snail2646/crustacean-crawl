use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![TileType::Floor, NUM_TILES],
        }
    }
}

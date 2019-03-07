extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use crate::tile::Block;

#[wasm_bindgen]
pub struct Map {
    width: u32,
    height: u32,
    tiles: Vec<Block>,
}

#[wasm_bindgen]
impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let size: usize = width as usize * height as usize;
        let tiles = vec![Block::Empty; size];
        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn add_tile_at_position(&mut self, blocks: &[u8], row: u32, column: u32) {
        let start_index = self.get_index(row, column);
        for (i, block) in blocks.iter().enumerate() {
            let index = start_index + i;
            self.tiles[index] = Block::from(*block);
        }
    }

    pub fn tiles(&self) -> *const Block {
        self.tiles.as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}
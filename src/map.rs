extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use crate::block::Block;

#[wasm_bindgen]
pub struct Map {
    width: u32,
    height: u32,
    tiles: Vec<Block>,
}

#[wasm_bindgen]
impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let size: usize = (width * height) as usize;
        let tiles = vec![Block::Empty; size];
        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn add_tile_at_position(&mut self, blocks: Box<[u8]>, row: u32, column: u32) {
        let block_size = (blocks.len() as f64).sqrt() as u32;

        let mut row = row * &block_size;
        let mut column = column * &block_size;
        let mut index = self.get_index(row, column);

        for (i, block) in blocks.iter().enumerate() {
            if i != 0 && i as u32 % block_size == 0 {
                row = row + 1;
                column = column - block_size;
                index = self.get_index(row, column);
            }
            self.tiles[index + i] = Block::from(*block);
        }
    }

    pub fn tiles(&self) -> *const Block {
        self.tiles.as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}
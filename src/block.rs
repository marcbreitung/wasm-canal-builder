extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    Empty = 0,
    Path = 1,
    Ground = 2,
}

impl Block {
    pub fn from(i: u8) -> Block {
        match i {
            1 => Block::Path,
            2 => Block::Ground,
            _ => Block::Empty,
        }
    }

    pub fn from_array(i: Vec<u8>) -> Vec<Block> {
        let mut blocks: Vec<Block> = vec![];
        for block in i.iter().cloned() {
            blocks.push(Block::from(block))
        }
        blocks
    }
}
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
}
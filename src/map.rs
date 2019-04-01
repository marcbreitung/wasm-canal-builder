extern crate wasm_bindgen;
extern crate rust_problem_search as search;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Map {
    width: u32,
    height: u32,
    tiles: Vec<u8>,
    path: Vec<u8>,
}

#[wasm_bindgen]
impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let size: usize = (width * height) as usize;
        let tiles = vec![0; size];
        let path = vec![0; size];
        Map {
            width,
            height,
            tiles,
            path,
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
            self.tiles[index + i] = *block;
        }
    }

    pub fn tiles(&self) -> *const u8 {
        self.tiles.as_ptr()
    }

    pub fn path(&mut self) -> *const u8 {
        let nodes = self.tiles.clone();
        let start = search::state::State::new(1, 1);
        let goal = search::state::State::new(2, 2);
        let graph = search::graph::Graph::new(nodes, self.width.clone(), self.height.clone());
        let problem = search::problem::Problem::new(start, goal, graph);
        let mut breath_first_search = search::breath_first_search::BreathFirstSearch::new();
        self.path = breath_first_search.search_vec(&problem);
        self.path.as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}
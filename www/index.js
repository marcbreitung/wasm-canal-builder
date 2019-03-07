import {Map, Tile, Block} from "wasm-canal-builder";
import {memory} from "wasm-canal-builder/wasm_canal_builder_bg";

const width = 10;
const height = 10;

const map = Map.new(width, height);
const tilesPtr = map.tiles();
const tiles = new Uint8Array(memory.buffer, tilesPtr, width * height);

console.log(tiles);
map.add_tile_at_position(new Uint8Array([1, 2, 1, 1, 2, 1, 1, 2, 1]), 0, 0);
console.log(tiles);
import {Map, Block} from "wasm-canal-builder";
import {memory} from "wasm-canal-builder/wasm_canal_builder_bg";

export class Game {
    constructor(parameters) {
        let {id, width, height} = parameters;
        this.config = {
            id: id,
            width: width,
            height: height,
            cellSize: 20,
            color: {
                grid: "#373737",
                empty: "#ffffff",
                path: "#3651aa",
                ground: "#458d4c",
            }
        };
        this.ctx = null;

        this.initMap();
        this.initCanvas();
        this.initEvents();
        this.draw();
    }

    draw() {
        this.drawGrid();
        this.drawBlocks();
    }

    initMap() {
        this.map = Map.new(this.config.width, this.config.height);
        this.tiles = new Uint8Array(memory.buffer, this.map.tiles(), this.config.width * this.config.height);
    }

    initCanvas() {
        let canvas = document.getElementById(this.config.id);
        canvas.height = (this.config.cellSize + 1) * this.config.height + 1;
        canvas.width = (this.config.cellSize + 1) * this.config.width + 1;
        this.ctx = canvas.getContext('2d');
    }

    initEvents() {
        let add = document.getElementById('add');
        add.addEventListener('click', (event) => {
            event.preventDefault();
            let blocks = new Uint8Array(9);
            Array.from(document.querySelectorAll('[data-tile]')).forEach((element, index) => {
                blocks[index] = element.checked ? 1 : 2;
            });
            let row = document.getElementById('row').value;
            let col = document.getElementById('col').value;
            this.map.add_tile_at_position(blocks, row, col);
            this.draw();
            this.getPath();
        })
    }

    getPath() {
        let path = new Uint32Array(memory.buffer, this.map.path(), 20);
        console.log(path);
    }

    drawGrid() {
        this.ctx.beginPath();
        this.ctx.strokeStyle = this.config.color.grid;

        for (let i = 0; i <= this.config.width; i++) {
            this.ctx.moveTo(i * (this.config.cellSize + 1) + 1, 0);
            this.ctx.lineTo(i * (this.config.cellSize + 1) + 1, (this.config.cellSize + 1) * this.config.height + 1);
        }

        for (let j = 0; j <= this.config.height; j++) {
            this.ctx.moveTo(0, j * (this.config.cellSize + 1) + 1);
            this.ctx.lineTo((this.config.cellSize + 1) * this.config.width + 1, j * (this.config.cellSize + 1) + 1);
        }

        this.ctx.stroke();
    }

    drawBlocks() {
        this.drawBlockOfTypeWithColor(Block.Empty, this.config.color.empty);
        this.drawBlockOfTypeWithColor(Block.Path, this.config.color.path);
        this.drawBlockOfTypeWithColor(Block.Ground, this.config.color.ground);
    }

    drawBlockOfTypeWithColor(type, color) {
        this.ctx.beginPath();
        this.ctx.fillStyle = color;
        for (let row = 0; row < this.config.height; row++) {
            for (let col = 0; col < this.config.width; col++) {
                const idx = this.getIndex(row, col);
                if (this.tiles[idx] !== type) {
                    continue;
                }
                this.ctx.fillRect(
                    col * (this.config.cellSize + 1) + 1,
                    row * (this.config.cellSize + 1) + 1,
                    this.config.cellSize,
                    this.config.cellSize
                );
            }
        }
        this.ctx.stroke();
    }

    getIndex(row, column) {
        return row * this.config.width + column;
    }

}
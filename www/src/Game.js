import {Map} from "wasm-canal-builder";
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

        this.start = null;
        this.goal = null;

        this.col = 0;
        this.row = 0;

        this.initMap();
        this.initCanvas();
        this.initEvents();
        this.addStart();
        this.addGoal();
        this.draw();
    }

    draw() {
        this.drawGrid();
        this.drawBlocks();
        this.drawOutline();
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
        canvas.addEventListener('click', (event) => {
            let column = Math.floor((event.clientX - event.target.offsetLeft) / (this.config.cellSize + 1));
            let row = Math.floor((event.clientY - event.target.offsetTop) / (this.config.cellSize + 1));
            if (this.start === null && this.goal === null) {
                this.start = {column: Math.max(0, column), row: Math.max(0, row)};
            } else {
                this.goal = {column: Math.max(0, column), row: Math.max(0, row)};
                this.getPath();
            }
        });
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
        });

        document.getElementById('row').addEventListener('change', (event) => {
            this.row = event.target.value;
            this.draw();
        });

        document.getElementById('col').addEventListener('change', (event) => {
            this.col = event.target.value;
            this.draw();
        });
    }

    addStart() {
        this.map.add_tile_at_position([2, 2, 2, 2, 1, 2, 2, 1, 2], 0, 0);
        this.start = {column: 1, row: 1};
    }

    addGoal() {
        this.map.add_tile_at_position([2, 1, 2, 2, 1, 2, 2, 2, 2], 5, 5);
        this.goal = {column: 16, row: 16};
    }

    getPath() {
        console.log(this.start, this.goal);
        let path = new Uint8Array(memory.buffer, this.map.path(this.start.row, this.start.column, this.goal.row, this.goal.column), this.config.width * this.config.height);
        this.drawBlocks();
        this.drawPath(path, "#0071ff");
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
        this.drawBlockOfTypeWithColor(0, this.config.color.empty);
        this.drawBlockOfTypeWithColor(1, this.config.color.path);
        this.drawBlockOfTypeWithColor(2, this.config.color.ground);
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

    drawPath(path, color) {
        this.ctx.beginPath();
        this.ctx.fillStyle = color;
        for (let row = 0; row < this.config.height; row++) {
            for (let col = 0; col < this.config.width; col++) {
                const idx = this.getIndex(row, col);
                if (path[idx] !== 1) {
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

    drawOutline() {
        let rectSize = (this.config.cellSize * 3) + 3;
        this.ctx.beginPath();
        this.ctx.globalAlpha = 0.5;
        this.ctx.fillStyle = "#b63443";
        this.ctx.fillRect(rectSize * this.col, rectSize * this.row, rectSize, rectSize);
        this.ctx.globalAlpha = 1.0;
    }

    getIndex(row, column) {
        return row * this.config.width + column;
    }

}
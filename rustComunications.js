
import init, { JsBind } from "./pkg/hello_wasm.js";
import { start } from "./gamePacer.js";
let bind = null;

export let board = [];

init().then(() => {
    bind = JsBind.new(boardWidth, boardHeight);
    start();
});

export function rust_ended() {
    bind.js_ended();
}

const type_to_code = {
    "P": 0,
    "Q": 1,
    "K": 2,
    "B": 3,
    "H": 4,
    "T": 5,
}

export function rust_placePiece(pieceType, x, y) {
    bind.js_place(type_to_code[pieceType[0]], x, y, pieceType[1] == "w" ? 0 : 1);
}

function getKeyByValue(object, value) {
    return Object.keys(object).find(key => object[key] === value);
}

function pieceTypeBuilder(type_code, owner) {
    return getKeyByValue(type_to_code, type_code) + ((owner == 0) ? "w" : "b")
}

export function rust_loadBoard() {
    board = [];
    bind.js_reset();
    let piece_count = bind.js_piece_count();
    for (let i = 0; i < piece_count; i++) {
        board.push({
            "pieceType": pieceTypeBuilder(bind.js_piece_type(), bind.js_piece_owner()),
            "pos": {
                "x": bind.js_piece_x(),
                "y": bind.js_piece_y(),
            },
            "last_pos": {
                "x": bind.js_piece_last_x(),
                "y": bind.js_piece_last_y()
            },
            "being_eatten": bind.js_piece_being_eaten()

        })
        bind.js_next_piece();
    }
}

export function rust_move() {
    bind.js_move()
}

export function rust_freeSpace(x, y){
    return bind.js_free_space(x,y)
}
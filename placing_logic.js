import { ScreenToWorld } from "./utils.js"
import { rust_placePiece, rust_freeSpace } from "./rustComunications.js"
export let current_placing_piece = "Kw";
let last_placed_piece = "";
let resolve = () => { };

function random_piece_type() {
    return "Q"
    return ["P", "Q", "K", "B", "H", "T"][Math.floor(Math.random() * 5.9)]
}

export function place_player(player, piece) {
    console.log(piece)
    let type = piece || (player == "w") ? random_piece_type() : last_placed_piece[0]
    console.log(type)

    current_placing_piece = type + player;
    console.log("new piece: ", current_placing_piece)
    return new Promise((r) => {
        resolve = r;
    })

}

export function pos_owner(x, y) {
    return (y < (boardHeight / 2) ? "w" : "b")
}

function alowed_pos(x, y) {
    let owner = current_placing_piece[1];
    return owner == pos_owner(x, y) && rust_freeSpace(x, y)
}


(() => {
    document.getElementById("canvas").addEventListener("click", (ev) => {
        let { x, y } = ScreenToWorld({
            "x": ev.clientX,
            "y": ev.clientY
        })
        if (x >= 0 && x < boardWidth && y >= 0 && y < boardHeight && alowed_pos(x, y)) {
            rust_placePiece(current_placing_piece, x, y)
            suscessfull_placement();
        }
        else {
            console.log("OOB")
        }
    })
})()

function suscessfull_placement() {
    last_placed_piece = current_placing_piece;
    current_placing_piece = "";
    resolve()
    resolve = () => { };
}


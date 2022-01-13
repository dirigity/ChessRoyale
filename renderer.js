import { WorldToScreen, tileSize, rgb3 } from "./utils.js";
import { board } from "./rustComunications.js";
import { pos_owner, current_placing_piece } from "./placing_logic.js"
let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

function tick() {
    UItick();
    let colorsChange = ColorPaleteTick()
    BoardTick(!colorsChange);

    requestAnimationFrame(tick);
}

function UItick() {

}

let moveStartT = 0
const moveTime = 100;
function BoardTick(optimal) {
    if (!optimal) {
        RedrawBoard();
    }
    let t = (new Date()).getTime();
    for (let piece of board) {
        if (!optimal || moving(piece)) {
            clearPosition(piece.pos);
            clearPosition(piece.last_pos);
            drawPiece(piece, (t - moveStartT) / moveTime);
        }
    }
}

function moving(p){
    return p.pos != p.last_pos
}

const transition_speed = 30; // inversed
function ColorPaleteTick() { // returns if the palete was changed
    let oldBtBp = blackTileBlackPiece
    let oldWtBp = whiteTileBlackPiece
    let oldBtWp = blackTileWhitePiece
    let oldWtWp = whiteTileWhitePiece

    let placing_player = current_placing_piece[1];


    if (placing_player == "w") {
        blackTileBlackPiece += (BlackMin - blackTileBlackPiece) / transition_speed;
        whiteTileBlackPiece += (WhiteMin - whiteTileBlackPiece) / transition_speed;
        blackTileWhitePiece += (BlackMax - blackTileWhitePiece) / transition_speed;
        whiteTileWhitePiece += (WhiteMax - whiteTileWhitePiece) / transition_speed;
    }

    if (placing_player == "b") {
        blackTileWhitePiece += (BlackMin - blackTileWhitePiece) / transition_speed;
        whiteTileWhitePiece += (WhiteMin - whiteTileWhitePiece) / transition_speed;
        blackTileBlackPiece += (BlackMax - blackTileBlackPiece) / transition_speed;
        whiteTileBlackPiece += (WhiteMax - whiteTileBlackPiece) / transition_speed;
    }

    if (placing_player != "w" && placing_player != "b") {
        blackTileBlackPiece += (BlackMax - blackTileBlackPiece) / transition_speed;
        whiteTileBlackPiece += (WhiteMax - whiteTileBlackPiece) / transition_speed;
        blackTileWhitePiece += (BlackMax - blackTileWhitePiece) / transition_speed;
        whiteTileWhitePiece += (WhiteMax - whiteTileWhitePiece) / transition_speed;
    }


    let diff =
        Math.abs(oldBtBp - blackTileBlackPiece) +
        Math.abs(oldWtBp - whiteTileBlackPiece) +
        Math.abs(oldBtWp - blackTileWhitePiece) +
        Math.abs(oldWtWp - whiteTileWhitePiece);

    // console.log(diff, diff > 0.1)
    return diff > 0.1;
}

const BlackMax = 0;
const BlackMin = 40;

const WhiteMax = 255;
const WhiteMin = 100

let blackTileBlackPiece = BlackMax
let whiteTileBlackPiece = WhiteMax
let blackTileWhitePiece = BlackMax
let whiteTileWhitePiece = WhiteMax



function clearPosition(p) {
    let { x, y } = WorldToScreen(p);
    let patern = p.x % 2 == 0 ^ p.y % 2 == 0
    if (pos_owner(p.x, p.y) == "w") {
        ctx.fillStyle = (patern ? rgb3(whiteTileWhitePiece) : rgb3(blackTileWhitePiece));

    } else {
        ctx.fillStyle = (patern ? rgb3(whiteTileBlackPiece) : rgb3(blackTileBlackPiece));

    }
    ctx.fillRect(x, y, tileSize, tileSize);
}

function fullClear() {
    canvas.width = window.innerWidth
    canvas.height = window.innerHeight
    RedrawBoard()
    BoardTick(false);

}

function RedrawBoard() {
    for (let y = 0; y < boardHeight; y++) {
        for (let x = 0; x < boardWidth; x++) {
            clearPosition({
                "x": x,
                "y": y
            });
        }
    }
}

window.onresize = fullClear;

fullClear();

export function resetAnimationTime() {
    moveStartT = (new Date()).getTime();
    fullClear();
}

function smooth(t) { // t in [-1,1]
    return 1 / (1 + Math.exp(-t / 0.185))
}

function interpolate(pos0, posF, t) { // t in [0,1]
    return {
        "x": pos0.x + (posF.x - pos0.x) * smooth(-1 + 2 * t),
        "y": pos0.y + (posF.y - pos0.y) * smooth(-1 + 2 * t)
    }
}

function drawPiece(piece, t) {

    let pos = WorldToScreen(interpolate(piece.last_pos, piece.pos, t));
    let [sx, sy, swidth, sheight] = {
        "Qw": [0 * 2000 / 6, 0, 333, 333],
        "Kw": [1 * 2000 / 6, 0, 333, 333],
        "Bw": [2 * 2000 / 6, 0, 333, 333],
        "Hw": [3 * 2000 / 6, 0, 333, 333],
        "Tw": [4 * 2000 / 6, 0, 333, 333],
        "Pw": [5 * 2000 / 6, 0, 333, 333],

        "Qb": [0 * 2000 / 6, 333, 333, 333],
        "Kb": [1 * 2000 / 6, 333, 333, 333],
        "Bb": [2 * 2000 / 6, 333, 333, 333],
        "Hb": [3 * 2000 / 6, 333, 333, 333],
        "Tb": [4 * 2000 / 6, 333, 333, 333],
        "Pb": [5 * 2000 / 6, 333, 333, 333]

    }[piece.pieceType];
    ctx.drawImage(document.getElementById("spriteSheet"), sx, sy, swidth, sheight, pos.x, pos.y, tileSize, tileSize);
}

requestAnimationFrame(tick);
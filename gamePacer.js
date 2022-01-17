import { resetAnimationTime } from "./renderer.js";
import { rust_loadBoard, rust_move, rust_ended } from "./rustComunications.js";
import { place_player } from "./placing_logic.js";

let turn = 0;

export async function loop() {
    console.log("loop")

    turn++;
    if (!rust_ended()) {
        await place()
        await move(stepsPerTurn)
        loop()
    }
    else end();

}

export async function start() {
    console.log("start")
    await place("K");
    loop()
}

async function move(iterations) {
    console.log("-- move --")

    const stepTime = moveTime + moveWait;

    return new Promise(resolve => {
        rust_move();
        rust_loadBoard();
        resetAnimationTime();

        setTimeout(() => {
            rust_move();
            rust_loadBoard();
            resetAnimationTime();

            setTimeout(() => {
                if (iterations == 1) {
                    resolve();
                } else {
                    move(iterations - 1).then(resolve);
                }
            }, stepTime);

        }, stepTime)
    });
}

async function place(piece) {
    console.log("-- place -- ", piece)

    return new Promise(resolve => {
        place_player("w", piece).then(() => {
            rust_loadBoard();
            place_player("b", piece).then(() => {
                rust_loadBoard();
                resolve();
            })
        });
    });
}
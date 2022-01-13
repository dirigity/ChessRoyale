import { resetAnimationTime } from "./renderer.js";
import { rust_loadBoard, rust_move, rust_ended } from "./rustComunications.js";
import { place_player } from "./placing_logic.js";

let turn = 0;

export function loop() {
    turn++;
    if (!rust_ended())
        place(rust_ended()).then(() => {
            move(stepsPerTurn).then(loop)
        });
    else end();

}

async function move(iterations) {
    console.log("-- move --")

    const stepTime = moveTime + moveWait;

    return new Promise(resolve => {
        rust_move(0);
        rust_loadBoard();
        resetAnimationTime();

        setTimeout(() => {
            rust_move(1);
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

async function place() {
    console.log("-- place --")

    return new Promise(resolve => {
        place_player("w").then(() => {
            rust_loadBoard();
            place_player("b").then(() => {
                rust_loadBoard();
                resolve();
            })
        });
    });
}
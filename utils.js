let canvas = document.getElementById("canvas");

export function rerange(src_0, src_f, dst_0, dst_f, x) {
    return (x - src_0) * (dst_f - dst_0) / (src_f - src_0) + dst_0
}

export let tileSize = 10;

export function WorldToScreen({ x: Wx, y: Wy }) {
    tileSize = Math.min(canvas.width / boardWidth, canvas.height / boardHeight)

    return {
        "x": (canvas.width - boardWidth * tileSize) / 2 + Wx * tileSize,
        "y": (canvas.height - boardHeight * tileSize) / 2 + Wy * tileSize
    }
}

export function ScreenToWorld({ x: Sx, y: Sy }) {
    let Tx = Sx - (canvas.width - boardWidth * tileSize) / 2
    let Ty = Sy - (canvas.height - boardHeight * tileSize) / 2
    return {
        "x": Math.floor(Tx / tileSize),
        "y": Math.floor(Ty / tileSize)
    }
}

export function rgb3(a) {
    return rgb(a, a, a)
}
export function rgb(r, g, b) {
    var r_ = check(decToHex(r));
    var g_ = check(decToHex(g));
    var b_ = check(decToHex(b));
    return "#" + r_ + g_ + b_;
}
function decToHex(n) {
    if (n < 0) {
        n = 0xFFFFFFFF + n + 1;
    }
    return Math.round(n).toString(16).toUpperCase();
}
function check(n) {
    //console.log(n)
    if (n.length > 2) {
        return "FF";
    } else if (n.length < 2) {
        return "0" + n;

    } else return n
}
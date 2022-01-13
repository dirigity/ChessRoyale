// mod point_mod;
// use point_mod::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f64(s: f64);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_usize(s: usize);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

}
#[derive(Clone, Copy)]
enum PieceType {
    Pawn,
    Queen,
    King,
    Bishop,
    Horse,
    Tower,
}

impl PieceType {
    fn code_to_type(code: usize) -> Self {
        match code {
            0 => Self::Pawn,
            1 => Self::Queen,
            2 => Self::King,
            3 => Self::Bishop,
            4 => Self::Horse,
            5 => Self::Tower,
            _ => Self::Tower,
        }
    }
    fn type_to_code(t: Self) -> usize {
        match t {
            PieceType::Pawn => 0,
            PieceType::Queen => 1,
            PieceType::King => 2,
            PieceType::Bishop => 3,
            PieceType::Horse => 4,
            PieceType::Tower => 5,
        }
    }
}

#[derive(Clone, Copy)]
struct Piece {
    piece_type: PieceType,
    owner: usize,
    x: isize,
    y: isize,
    last_x: isize,
    last_y: isize,
    being_eaten: bool,
}

#[derive(Clone, Copy)]
struct MoveDelta {
    d_x: isize,
    d_y: isize,
}

// "P": 0,
// "Q": 1,
// "K": 2,
// "B": 3,
// "H": 4,
// "T": 5,
impl Piece {
    fn value(&self) -> f64 {
        match self.piece_type {
            PieceType::Pawn => 0.,
            PieceType::Queen => 9.,
            PieceType::King => 10000.,
            PieceType::Bishop => 3.5,
            PieceType::Horse => 3.,
            PieceType::Tower => 5.,
        }
    }
    fn move_deltae(&self, b: &Board) -> Vec<MoveDelta> {
        match self.piece_type {
            PieceType::Pawn => {
                let dir = if self.owner == 1 { 1 } else { -1 };

                let tmp0 = vec![MoveDelta { d_x: 0, d_y: dir }];
                let tmp1: Vec<MoveDelta> = vec![
                    MoveDelta { d_x: 1, d_y: dir },
                    MoveDelta { d_x: -1, d_y: dir },
                ]
                .iter()
                .map(|e| e.clone())
                .filter(|e| {
                    let pos_x = e.d_x + self.x;
                    let pos_y = e.d_y + self.y;
                    let victim = b.pieces.iter().find(|e| e.x == pos_x && e.y == pos_y);
                    victim.is_some() && victim.unwrap().owner != self.owner
                })
                .collect();

                tmp0.iter().map(|e| *e).chain(tmp1).collect()
            }
            PieceType::Queen => {
                let mut tmp = self.clone();
                tmp.piece_type = PieceType::Bishop;
                let tmp0 = tmp.move_deltae(b);
                tmp.piece_type = PieceType::Tower;
                let tmp1 = tmp0
                    .iter()
                    .map(|e| *e)
                    .chain(tmp.move_deltae(b).iter().map(|e| *e))
                    .collect();
                tmp1
            }
            PieceType::King => {
                vec![
                    MoveDelta { d_x: 0, d_y: -1 },
                    MoveDelta { d_x: 0, d_y: 1 },
                    MoveDelta { d_x: -1, d_y: 0 },
                    MoveDelta { d_x: 1, d_y: 0 },
                ]
            }
            PieceType::Bishop => {
                let size = b.height.min(b.width);
                let tmp0 = (0..size).map(|e| MoveDelta { d_x: e, d_y: -e });
                let tmp1 = (0..size).map(|e| MoveDelta { d_x: e, d_y: e });
                let tmp2 = (0..size).map(|e| MoveDelta { d_x: -e, d_y: -e });
                let tmp3 = (0..size).map(|e| MoveDelta { d_x: -e, d_y: e });

                tmp0.chain(tmp1).chain(tmp2).chain(tmp3).collect()
            }
            PieceType::Horse => {
                vec![
                    MoveDelta { d_x: 1, d_y: 2 },
                    MoveDelta { d_x: 1, d_y: -2 },
                    MoveDelta { d_x: -1, d_y: 2 },
                    MoveDelta { d_x: -1, d_y: -2 },
                    MoveDelta { d_x: 2, d_y: 1 },
                    MoveDelta { d_x: 2, d_y: -1 },
                    MoveDelta { d_x: -2, d_y: 1 },
                    MoveDelta { d_x: -2, d_y: -1 },
                ]
            }
            PieceType::Tower => {
                let tmp0 = (0..b.height).map(|e| MoveDelta { d_x: 0, d_y: -e });
                let tmp1 = (0..b.height).map(|e| MoveDelta { d_x: 0, d_y: e });
                let tmp2 = (0..b.width).map(|e| MoveDelta { d_x: -e, d_y: 0 });
                let tmp3 = (0..b.width).map(|e| MoveDelta { d_x: e, d_y: 0 });

                tmp0.chain(tmp1).chain(tmp2).chain(tmp3).collect()
            }
        }
    }
}

#[derive(Clone)]
struct Board {
    pieces: Vec<Piece>,
    width: isize,
    height: isize,
}

impl Board {
    fn push(&mut self, p: Piece) {
        self.pieces.push(p);
    }

    fn len(&self) -> usize {
        self.pieces.len()
    }
}

fn inbounds(x: isize, y: isize, height: isize, width: isize) -> bool {
    x >= 0 && y >= 0 && x < width && y < height
}

fn valid_board(test_board: &Board) -> bool {
    test_board
        .pieces
        .iter()
        .find(|e| inbounds(e.x, e.y, test_board.height, test_board.width))
        .is_none()
}

fn next_boards(old_board: &Board, player: usize) -> Vec<Board> {
    fn get_board_from_delta(
        move_delta: &MoveDelta,
        board: &Board,
        piece_i: usize,
    ) -> Option<Board> {
        let mut wip_board = board.clone();
        wip_board.pieces[piece_i].x += move_delta.d_x;
        wip_board.pieces[piece_i].y += move_delta.d_y;
        let piece = wip_board.pieces[piece_i].clone();

        let friendly_overlaping = match wip_board
            .pieces
            .iter_mut()
            .enumerate()
            .find(|(i, p)| p.x == piece.x && p.y == piece.y && piece_i != *i)
        {
            Some((i, p)) => {
                if p.owner == piece.owner {
                    true
                } else {
                    wip_board.pieces[i].being_eaten = true;
                    false
                }
            }
            None => false,
        };

        if valid_board(&wip_board) && !friendly_overlaping {
            Some(wip_board.clone())
        } else {
            None
        }
    }

    let mut base_board = old_board.clone();

    base_board.pieces = base_board
        .pieces
        .iter_mut()
        .filter(|p| !p.being_eaten)
        .map(|mut p| {
            p.last_x = p.x;
            p.last_y = p.y;
            *p
        })
        .collect();

    base_board
        .pieces
        .iter()
        .filter(|e| e.owner == player)
        .enumerate()
        .map(|(i, piece)| -> Vec<Board> {
            piece
                .move_deltae(&base_board)
                .iter()
                .map(|move_delta| get_board_from_delta(move_delta, &base_board, i))
                .filter(|op| op.is_some())
                .map(|e| e.unwrap())
                .collect()
        })
        .flatten()
        .collect()
}

fn get_next_board(old_board: &Board, player: usize, depth: usize) -> Board {
    let mut best_board_score = Option::None;
    let mut best_board = Option::None;
    for board in next_boards(old_board, player) {
        let s = score(&board, player, depth);
        if best_board_score.is_none() || best_board_score < Some(s) {
            best_board_score = Some(s);
            best_board = Some(board.clone());
        }
    }
    match best_board {
        Some(x) => x,
        None => old_board.clone(),
    }
}

fn score(old_board: &Board, player: usize, depth: usize) -> f64 {
    if depth == 1 {
        base_score(old_board, player)
    } else {
        let mut worse_next_board_score = Option::None;
        for board in next_boards(old_board, 1 - player) {
            let s = score(&board, 1 - player, depth - 1);
            if worse_next_board_score.is_none() || worse_next_board_score > Some(s) {
                worse_next_board_score = Some(s);
            }
        }
        match worse_next_board_score {
            Some(x) => -x,
            None => 0.,
        }
    }
}

fn base_score(old_board: &Board, player: usize) -> f64 {
    old_board.pieces.iter().fold(0., |mut acc, p| {
        acc += if !p.being_eaten {
            p.value() * if p.owner == player { 1. } else { -1. }
        } else {
            0.
        };
        acc
    })
}

#[wasm_bindgen]
pub struct JsBind {
    board: Board,
    comunication_i: usize,
}
#[wasm_bindgen]
impl JsBind {
    pub fn new(width: isize, height: isize) -> JsBind {
        JsBind {
            board: Board {
                pieces: Vec::new(),
                width: width,
                height: height,
            },
            comunication_i: 0,
        }
    }

    pub fn js_place(&mut self, piece_type_code: usize, x: isize, y: isize, player: usize) {
        self.board.push(Piece {
            piece_type: PieceType::code_to_type(piece_type_code),
            owner: player,
            x: x,
            y: y,
            last_x: x,
            last_y: y,
            being_eaten: false,
        });
    }

    pub fn js_reset(&mut self) {
        self.comunication_i = 0;
    }

    pub fn js_piece_count(&self) -> usize {
        self.board.len()
    }

    pub fn js_piece_type(&self) -> usize {
        PieceType::type_to_code(self.board.pieces[self.comunication_i].piece_type)
    }

    pub fn js_piece_x(&self) -> isize {
        self.board.pieces[self.comunication_i].x
    }

    pub fn js_piece_y(&self) -> isize {
        self.board.pieces[self.comunication_i].y
    }

    pub fn js_piece_last_x(&self) -> isize {
        self.board.pieces[self.comunication_i].last_x
    }

    pub fn js_piece_last_y(&self) -> isize {
        self.board.pieces[self.comunication_i].last_y
    }
    pub fn js_piece_being_eaten(&mut self) -> bool {
        self.board.pieces[self.comunication_i].being_eaten
    }

    pub fn js_next_piece(&mut self) {
        self.comunication_i += 1;
    }

    pub fn js_piece_owner(&mut self) -> usize {
        self.board.pieces[self.comunication_i].owner
    }

    pub fn js_move(&mut self, player: usize) {
        self.board = get_next_board(&self.board, player, 3);
    }

    pub fn js_ended(&self) -> bool {
        false
    }
}

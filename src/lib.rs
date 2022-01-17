mod engine;
use engine::board::*;
use engine::piece::*;

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
                next_player_to_move: Player::White,
            },
            comunication_i: 0,
        }
    }

    pub fn js_place(&mut self, piece_type_code: usize, x: isize, y: isize, player: usize) {
        self.board.push(Piece {
            piece_type: PieceType::code_to_type(piece_type_code),
            owner: Player::from_usize(player),
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
        self.board.pieces[self.comunication_i].owner.to_uzise()
    }

    pub fn js_move(&mut self) {
        // log(&format!("moving: {:?}", self.board) as &str);
        let next_board_state = self.board.get_next_board(1);
        self.board = next_board_state;
    }

    pub fn js_ended(&self) -> bool {
        false
    }

    pub fn js_free_space(&self, x: isize, y: isize) -> bool {
        self.board
            .pieces
            .iter()
            .find(|e| e.x == x && e.y == y)
            .is_none()
    }
}

use crate::engine::piece::*;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f64(s: f64);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_usize(s: usize);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f(a: String);

}

#[derive(Clone, Debug)]
pub struct Board {
    pub pieces: Vec<Piece>,
    pub width: isize,
    pub height: isize,
    pub next_player_to_move: Player,
}

impl Board {
    pub fn push(&mut self, p: Piece) {
        self.pieces.push(p);
    }

    pub fn len(&self) -> usize {
        self.pieces.len()
    }

    pub fn score(&self, depth: usize) -> f64 {
        if depth == 1 {
            self.base_score()
        } else {
            let mut next_board_score = Option::None;
            for board in self.next_boards() {
                let s = board.score(depth - 1);
                if next_board_score.is_none() || next_board_score < Some(s) {
                    next_board_score = Some(s);
                }
            }
            match next_board_score {
                Some(x) => -x,
                None => 0.,
            }
        }
    }

    fn base_score(&self) -> f64 {
        let ret = self.pieces.iter().fold(0., |mut acc, p| {
            acc += if !p.being_eaten {
                // log(&format!("piece added: {}", p.worth()) as &str);
                p.worth()
                    * if p.owner == self.next_player_to_move {
                        -1.
                    } else {
                        1.
                    }
            } else {
                log("being eaten ignored");
                0.
            };
            acc
        });

        // log(&format!("score of: {:?} is {}", self, ret) as &str);
        ret
    }

    fn get_board_from_delta(&self, move_delta: &MoveDelta, piece_i: usize) -> Option<Board> {
        let mut wip_board = self.clone();
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

        if wip_board.valid_board() && !friendly_overlaping {
            Some(wip_board.clone())
        } else {
            None
        }
    }

    fn next_boards(&self) -> Vec<Board> {
        // log(&format!("original: {:?}", self) as &str);

        let mut base_board = self.clone();

        base_board.next_player_to_move = base_board.next_player_to_move.oposite();
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
            .enumerate()
            .filter(|(_, e)| e.owner == self.next_player_to_move)
            .map(|(i, piece)| -> Vec<Board> {
                piece
                    .move_deltae(&base_board)
                    .iter()
                    .map(|move_delta| base_board.get_board_from_delta(move_delta, i))
                    .filter(|op| op.is_some())
                    .map(|e| e.unwrap())
                    .collect()
            })
            .flat_map(|e| e.clone())
            .collect()
    }

    pub fn get_next_board(&self, depth: usize) -> Board {
        let mut best_board_score = Option::None;
        let mut best_boards = vec![];

        log("log1");
        log_f(format!("log1.5: {}", self.next_boards().len()));

        for board in self.next_boards() {
            // log(&format!("considering: {:?}", board) as &str);

            let s = board.score(depth);
            if best_board_score.is_none() || best_board_score < Some(s) {
                best_board_score = Some(s);
                best_boards = vec![board.clone()];
            }
            if best_board_score == Some(s) {
                best_boards.push(board.clone());
            }
        }
        log_f(format!("log2: {}", best_boards.len()));

        match best_boards.len() {
            0 => {
                let mut ret = self.clone();
                ret.pieces = ret
                    .pieces
                    .iter_mut()
                    .filter(|p| !p.being_eaten)
                    .map(|mut p| {
                        p.last_x = p.x;
                        p.last_y = p.y;
                        *p
                    })
                    .collect();
                ret
            }
            _ => best_boards[(random() * 1000.) as usize % best_boards.len()].clone(),
        }
    }

    fn valid_board(&self) -> bool {
        self.pieces
            .iter()
            .find(|e| !inbounds(e.x, e.y, self.height, self.width))
            .is_none()
        // TODO: cant let the other one eat my king
    }
}

fn inbounds(x: isize, y: isize, height: isize, width: isize) -> bool {
    x >= 0 && y >= 0 && x < width && y < height
}

use std::ops::Mul;

use crate::engine::board::*;
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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn to_uzise(&self) -> usize {
        match self {
            Player::White => 0,
            Player::Black => 1,
        }
    }

    pub fn from_usize(i: usize) -> Self {
        match i {
            0 => Player::White,
            _ => Player::Black,
        }
    }

    pub fn oposite(&self) -> Self {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    Pawn,
    Queen,
    King,
    Bishop,
    Horse,
    Tower,
}

impl PieceType {
    pub fn code_to_type(code: usize) -> Self {
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
    pub fn type_to_code(t: Self) -> usize {
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

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub owner: Player,
    pub x: isize,
    pub y: isize,
    pub last_x: isize,
    pub last_y: isize,
    pub being_eaten: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct MoveDelta {
    pub d_x: isize,
    pub d_y: isize,
}

impl Mul<isize> for &MoveDelta {
    type Output = MoveDelta;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::Output {
            d_x: self.d_x * rhs,
            d_y: self.d_y * rhs,
        }
    }
}

impl Piece {
    pub fn worth(&self) -> f64 {
        match self.piece_type {
            PieceType::Pawn => 1.,
            PieceType::Queen => 9.,
            PieceType::King => 10000.,
            PieceType::Bishop => 3.5,
            PieceType::Horse => 3.,
            PieceType::Tower => 5.,
        }
    }

    fn slide_piece(&self, dir_vec: Vec<MoveDelta>, b: &Board) -> Vec<MoveDelta> {
        dir_vec
            .iter()
            .map(|delta| self.slide(delta, b))
            .flatten()
            .collect()
    }

    fn slide(&self, dir: &MoveDelta, b: &Board) -> Vec<MoveDelta> {
        let size = b.height.max(b.width);
        let mut take_one_more = true;

        (1..size)
            .map(|i| dir * i)
            .take_while(|delta| {
                let new_x = self.x + delta.d_x;
                let new_y = self.y + delta.d_y;
                let last_result = take_one_more;
                take_one_more = b // asi es posible caer encima de la pieza, pero no detrás
                    .pieces
                    .iter()
                    .find(|e| new_x == e.x && new_y == e.y)
                    .is_none();
                last_result
            })
            .collect()
    }

    pub fn move_deltae(&self, b: &Board) -> Vec<MoveDelta> {
        log(&format!("pieza de la que pido los delta: {:?}", self) as &str);

        let ret = match self.piece_type {
            PieceType::Pawn => {
                let dir = if self.owner == Player::White { 1 } else { -1 };

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

                vec![MoveDelta { d_x: 0, d_y: dir }]
                    .iter()
                    .map(|e| *e)
                    .chain(tmp1)
                    .collect()
            }
            PieceType::Queen => self.slide_piece(
                vec![
                    MoveDelta { d_x: 1, d_y: 1 },
                    MoveDelta { d_x: -1, d_y: 1 },
                    MoveDelta { d_x: -1, d_y: -1 },
                    MoveDelta { d_x: 1, d_y: -1 },
                    MoveDelta { d_x: 1, d_y: 0 },
                    MoveDelta { d_x: 0, d_y: 1 },
                    MoveDelta { d_x: -1, d_y: 0 },
                    MoveDelta { d_x: 0, d_y: -1 },
                ],
                b,
            ),
            PieceType::King => {
                vec![
                    MoveDelta { d_x: 0, d_y: -1 },
                    MoveDelta { d_x: 0, d_y: 1 },
                    MoveDelta { d_x: -1, d_y: 0 },
                    MoveDelta { d_x: 1, d_y: 0 },
                ]
            }
            PieceType::Bishop => self.slide_piece(
                vec![
                    MoveDelta { d_x: 1, d_y: 1 },
                    MoveDelta { d_x: -1, d_y: 1 },
                    MoveDelta { d_x: -1, d_y: -1 },
                    MoveDelta { d_x: 1, d_y: -1 },
                ],
                b,
            ),
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
            PieceType::Tower => self.slide_piece(
                vec![
                    MoveDelta { d_x: 1, d_y: 0 },
                    MoveDelta { d_x: 0, d_y: 1 },
                    MoveDelta { d_x: -1, d_y: 0 },
                    MoveDelta { d_x: 0, d_y: -1 },
                ],
                b,
            ),
        };

        log(&format!("delta resultantes: {:?}", ret) as &str);

        ret
    }
}

use ze_rook::{A1, H1, A8, H8};
use crate::board::{BoardState, Directions};

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub enum Square {
    NotOnTheBoard = 20,
    Empty = 0,
    MyKing = 1,
    MyQueen = 2,
    MyRook = 3,
    MyBishop = 4,
    MyKnight = 5,
    MyPawn = 6,
    OpponentKing = 11,
    OpponentQueen = 12,
    OpponentRook = 13,
    OpponentBishop = 14,
    OpponentKnight = 15,
    OpponentPawn = 16,
}
impl Square {
    pub fn is_my_piece(self) -> bool {
        matches!(
            self,
            Square::MyPawn
                | Square::MyKing
                | Square::MyRook
                | Square::MyKnight
                | Square::MyBishop
                | Square::MyQueen
        )
    }
    pub fn is_opponent_piece(self) -> bool {
        matches!(
            self,
            Square::OpponentPawn
                | Square::OpponentKing
                | Square::OpponentRook
                | Square::OpponentKnight
                | Square::OpponentBishop
                | Square::OpponentQueen
        )
    }
    pub fn swap_color(self) -> Square {
        match self {
            Square::Empty => Square::Empty,
            Square::NotOnTheBoard => Square::NotOnTheBoard,
            Square::MyPawn => Square::OpponentPawn,
            Square::MyKing => Square::OpponentKing,
            Square::MyRook => Square::OpponentRook,
            Square::MyKnight => Square::OpponentKnight,
            Square::MyBishop => Square::OpponentBishop,
            Square::MyQueen => Square::OpponentQueen,
            Square::OpponentPawn => Square::MyPawn,
            Square::OpponentKing => Square::MyKing,
            Square::OpponentRook => Square::MyRook,
            Square::OpponentKnight => Square::MyKnight,
            Square::OpponentBishop => Square::MyBishop,
            Square::OpponentQueen => Square::MyQueen,
        }
    }
    pub fn directions(self) -> &'static [i32] {
        match self {
            Square::MyPawn => &[
                Directions::NORTH,
                Directions::NORTH + Directions::NORTH,
                Directions::NORTH + Directions::WEST,
                Directions::NORTH + Directions::EAST,
            ],
            Square::MyKnight => &[
                Directions::NORTH + Directions::NORTH + Directions::EAST,
                Directions::NORTH + Directions::NORTH + Directions::WEST,
                Directions::WEST + Directions::WEST + Directions::NORTH,
                Directions::WEST + Directions::WEST + Directions::SOUTH,
                Directions::SOUTH + Directions::SOUTH + Directions::WEST,
                Directions::SOUTH + Directions::SOUTH + Directions::EAST,
                Directions::EAST + Directions::EAST + Directions::SOUTH,
                Directions::EAST + Directions::EAST + Directions::NORTH,
            ],
            Square::MyBishop => &[
                Directions::NORTH + Directions::EAST,
                Directions::NORTH + Directions::WEST,
                Directions::WEST + Directions::SOUTH,
                Directions::SOUTH + Directions::EAST,
            ],
            Square::MyRook => &[
                Directions::NORTH,
                Directions::WEST,
                Directions::SOUTH,
                Directions::EAST,
            ],
            Square::MyQueen | Square::MyKing => &[
                Directions::NORTH,
                Directions::WEST,
                Directions::SOUTH,
                Directions::EAST,
                Directions::NORTH + Directions::EAST,
                Directions::NORTH + Directions::WEST,
                Directions::WEST + Directions::SOUTH,
                Directions::SOUTH + Directions::EAST,
            ],
            _ => panic!(),
        }
    }
    pub fn value(self) -> i32 {
        let abs_val = match self {
            Square::MyKing | Square::OpponentKing => 60000,
            Square::MyQueen | Square::OpponentQueen => 929,
            Square::MyRook | Square::OpponentRook => 479,
            Square::MyBishop | Square::OpponentBishop => 320,
            Square::MyKnight | Square::OpponentKnight => 280,
            Square::MyPawn | Square::OpponentPawn => 100,
            _ => 0
        };
        if self.is_opponent_piece() {
            abs_val * -1
        } else {
            abs_val
        }
    }
    pub fn pst(self) -> [i32; 120] {
        match self {
            Square::MyPawn => [0,   0,   0,   0,   0,   0,   0,   0,   0,   0, 
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,  78,  83,  86,  73, 102,  82,  85,  90,   0,
            0,   7,  29,  21,  44,  40,  31,  44,   7,   0,
            0, -17,  16,  -2,  15,  14,   0,  15, -13,   0,
            0, -26,   3,  10,   9,   6,   1,   0, -23,   0,
            0, -22,   9,   5, -11, -10,  -2,   3, -19,   0,
            0, -31,   8,  -7, -37, -36, -14,   3, -31,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0],
            Square::MyKnight => [0,   0,   0,   0,   0,   0,   0,   0,   0,   0, 
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0, -66, -53, -75, -75, -10, -55, -58, -70,   0,
            0,  -3,  -6, 100, -36,   4,  62,  -4, -14,   0,
            0,  10,  67,   1,  74,  73,  27,  62,  -2,   0,
            0,  24,  24,  45,  37,  33,  41,  25,  17,   0,
            0,  -1,   5,  31,  21,  22,  35,   2,   0,   0,
            0, -18,  10,  13,  22,  18,  15,  11, -14,   0,
            0, -23, -15,   2,   0,   2,   0, -23, -20,   0,
            0, -74, -23, -26, -24, -19, -35, -22, -69,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0],
            Square::MyBishop => [0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,  -59, -78, -82, -76, -23,-107, -37, -50,  0,
            0,  -11,  20,  35, -42, -39,  31,   2, -22,  0,
            0,   -9,  39, -32,  41,  52, -10,  28, -14,  0,
            0,   25,  17,  20,  34,  26,  25,  15,  10,  0,
            0,   13,  10,  17,  23,  17,  16,   0,   7,  0,
            0,   14,  25,  24,  15,   8,  25,  20,  15,  0,
            0,   19,  20,  11,   6,   7,   6,  20,  16,  0,
            0,   -7,   2, -15, -12, -14, -15, -10, -10,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0],
            Square::MyRook => [0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,   35,  29,  33,   4,  37,  33,  56,  50,  0,
            0,   55,  29,  56,  67,  55,  62,  34,  60,  0,
            0,   19,  35,  28,  33,  45,  27,  25,  15,  0,
            0,    0,   5,  16,  13,  18,  -4,  -9,  -6,  0,
            0,  -28, -35, -16, -21, -13, -29, -46, -30,  0,
            0,  -42, -28, -42, -25, -25, -35, -26, -46,  0,
            0,  -53, -38, -31, -26, -29, -43, -44, -53,  0,
            0,  -30, -24, -18,   5,  -2, -18, -31, -32,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0],
            Square::MyQueen => [0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    6,   1,  -8,-104,  69,  24,  88,  26,  0,
            0,   14,  32,  60, -10,  20,  76,  57,  24,  0,
            0,   -2,  43,  32,  60,  72,  63,  43,   2,  0,
            0,    1, -16,  22,  17,  25,  20, -13,  -6,  0,
            0,  -14, -15,  -2,  -5,  -1, -10, -20, -22,  0,
            0,  -30,  -6, -13, -11, -16, -11, -16, -27,  0,
            0,  -36, -18,   0, -19, -15, -15, -21, -38,  0,
            0,  -39, -30, -31, -13, -31, -36, -34, -42,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0],
            Square::MyKing => [0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    4,  54,  47, -99, -99,  60,  83, -62,  0,
            0,  -32,  10,  55,  56,  56,  55,  10,   3,  0,
            0,  -62,  12, -57,  44, -67,  28,  37, -31,  0,
            0,  -55,  50,  11,  -4, -19,  13,   0, -49,  0,
            0,  -55, -43, -52, -28, -51, -47,  -8, -50,  0,
            0,  -47, -42, -43, -79, -64, -32, -29, -32,  0,
            0,   -4,   3, -14, -50, -57, -18,  13,   4,  0,
            0,   17,  30,  -3, -14,   6,  -1,  40,  18,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0],
            _ => [0; 120]
        }
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub prom: Option<Prom>,
}
impl Move {
    pub fn evaluate_mv(self, boardstate: &BoardState) -> i32 {
        let (i, j, prom) = (self.from, self.to, self.prom);
        let (piece, destination) = (boardstate.board[i], boardstate.board[j]);
        let mut score = piece.pst()[j] - piece.pst()[i];
        if destination.is_opponent_piece() {
            score += destination.swap_color().pst()[119-i];            
        }
        if piece == Square::MyKing && (i as i32 - j as i32 == 2 || j as i32 - i as i32 == 2) {
            score += Square::MyRook.pst()[(i+j)/2];
            score -= Square::MyRook.pst()[if j < i {
                A1
            } else {
                H1
            }]
        }
        if piece == Square::MyPawn {
            if A8 <= j || j <= H8 {
                score += match prom {
                    Some(Prom::N) => Square::MyKnight.pst()[j] - Square::MyPawn.pst()[j],
                    Some(Prom::B) => Square::MyBishop.pst()[j] - Square::MyPawn.pst()[j],
                    Some(Prom::R) => Square::MyRook.pst()[j] - Square::MyPawn.pst()[j],
                    Some(Prom::Q) => Square::MyQueen.pst()[j] - Square::MyPawn.pst()[j],
                    _ => 0,
                };
            }
            if j as i32 + Directions::SOUTH == boardstate.ep as i32 {
                score += Square::MyPawn.pst()[119-(j as i32 + Directions::SOUTH) as usize];
            }
        }
        score
    }
}

#[derive(Copy, Clone)]
pub enum Prom {
    Q,
    R,
    B,
    N,
}

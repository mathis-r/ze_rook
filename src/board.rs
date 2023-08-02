use crate::piece::Square;
use ze_rook::{A1, H1, A8, H8};
use crate::piece::{Move, Prom};
pub type Board = [Square; 120];
pub struct Directions {}
impl Directions {
    pub const NORTH: i32 = -10;
    pub const EAST: i32 = 1;
    pub const SOUTH: i32 = 10;
    pub const WEST: i32 = -1;
}
pub const INITIAL_BOARD: Board = [
// Padding
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
// Eigth rank
Square::NotOnTheBoard,
Square::OpponentRook,
Square::OpponentKnight,
Square::OpponentBishop,
Square::OpponentQueen,
Square::OpponentKing,
Square::OpponentBishop,
Square::OpponentKnight,
Square::OpponentRook,
Square::NotOnTheBoard,
// Seventh rank
Square::NotOnTheBoard,
Square::OpponentPawn,
Square::OpponentPawn,
Square::OpponentPawn,
Square::OpponentPawn,
Square::OpponentPawn,
Square::OpponentPawn,
Square::OpponentPawn,
Square::OpponentPawn,
Square::NotOnTheBoard,
// Sixth rank
Square::NotOnTheBoard,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::NotOnTheBoard,
// Fifth rank
Square::NotOnTheBoard,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::NotOnTheBoard,
// Fourth rank
Square::NotOnTheBoard,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::NotOnTheBoard,
// Third rank
Square::NotOnTheBoard,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::Empty,
Square::NotOnTheBoard,
// Second rank
Square::NotOnTheBoard,
Square::MyPawn,
Square::MyPawn,
Square::MyPawn,
Square::MyPawn,
Square::MyPawn,
Square::MyPawn,
Square::MyPawn,
Square::MyPawn,
Square::NotOnTheBoard,
// First rank
Square::NotOnTheBoard,
Square::MyRook,
Square::MyKnight,
Square::MyBishop,
Square::MyQueen,
Square::MyKing,
Square::MyBishop,
Square::MyKnight,
Square::MyRook,
Square::NotOnTheBoard,
// Padding
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
Square::NotOnTheBoard,
];


pub struct BoardState {
    pub board: Board,
    pub myc: (bool, bool),
    pub oppc: (bool, bool),
    pub ep: usize,
    pub kp: usize,
    pub color: char,
}

impl BoardState {
    pub fn new() -> BoardState {
        BoardState {board: INITIAL_BOARD, myc: (true, true), oppc: (true, true), ep: 0, kp: 0, color: 'w' }
    }
    pub fn print(&mut self) {
        let mut change = false;
        if self.color == 'b' {
            change = true;
            self.rotate();
        }
        let mut boardtoprint = String::new();
        for square in 0..120 {
            if square % 10 == 9 {
                boardtoprint.push('\n');
            } else {
                boardtoprint.push(' ');
                match self.board[square] {
                    Square::MyKing => boardtoprint.push('K'),
                    Square::MyQueen => boardtoprint.push('Q'),
                    Square::MyRook => boardtoprint.push('R'),
                    Square::MyBishop => boardtoprint.push('B'),
                    Square::MyKnight => boardtoprint.push('N'),
                    Square::MyPawn => boardtoprint.push('P'),
                    Square::OpponentKing => boardtoprint.push('k'),
                    Square::OpponentQueen => boardtoprint.push('q'),
                    Square::OpponentRook => boardtoprint.push('r'),
                    Square::OpponentBishop => boardtoprint.push('b'),
                    Square::OpponentKnight => boardtoprint.push('n'),
                    Square::OpponentPawn => boardtoprint.push('p'),
                    Square::Empty => boardtoprint.push('.'),
                    _ => boardtoprint.push(' '),
                }
            }
        }
        println!("{}", boardtoprint);
        println!("white castling : {:?} | black castling : {:?} | ep : {} | kp : {}", self.myc, self.oppc, self.ep, self.kp);
        if change {
            self.rotate();
        }
        println!("Side to move : {}", self.color);
    }
    pub fn rotate(&mut self) {
        self.ep = 119 - self.ep;
        self.kp = 119 - self.kp;
        (self.myc, self.oppc) = (self.oppc, self.myc);
        for i in 0..60 {
            (self.board[119-i], self.board[i]) = (self.board[i].swap_color(), self.board[119-i].swap_color());
        }
        self.color = match self.color {
            'w' => 'b',
            'b' => 'w',
            _ => 'w',
        }
    }
    pub fn apply_move(&mut self, mv: &Move) {
        let piece = self.board[mv.from];
        self.kp = 0;
        if mv.from == A1 {
            self.myc.0 = false;
        }
        if mv.from == H1 {
            self.myc.1 = false;
        }
        if mv.to == A8 {
            self.oppc.1 = false;
        }
        if mv.to == H8 {
            self.oppc.0 = false;
        }
        if piece == Square::MyKing {
            self.myc = (false, false);
            if mv.from as i32 - mv.to as i32 == 2{
                self.kp = (mv.to+mv.from)/2;
                (self.board[A1], self.board[self.kp]) = (self.board[self.kp], self.board[A1]);
            }
            if mv.to as i32 - mv.from as i32 == 2{
                self.kp = (mv.to+mv.from)/2;
                (self.board[H1], self.board[self.kp]) = (self.board[self.kp], self.board[H1]);
            }
        }
        if piece == Square::MyPawn {
            if mv.to as i32 + Directions::SOUTH == self.ep as i32 {
                self.board[(mv.to as i32 + Directions::SOUTH) as usize] = Square::Empty;
                self.ep = 0;
            }
            self.ep = 0;
            if A8 <= mv.to && mv.to <= H8 {
                self.board[mv.from] = promotion(&mv.prom);
                self.ep = 0;
            }
            if mv.to as i32 - mv.from as i32 == Directions::NORTH * 2 {
                self.ep = mv.to;
            }
        } else {
            self.ep = 0;
        }
        (self.board[mv.from], self.board[mv.to]) = (Square::Empty, self.board[mv.from]);
    }
    pub fn unmake(&mut self, mv: &Move, dest: &Square, ori_myc: &(bool, bool), ori_oppc: &(bool, bool), ep: &usize, kp: &usize) {
        if self.board[mv.to] == Square::MyKing {
            if mv.from as i32 - mv.to as i32 == 2{
                self.kp = (mv.to+mv.from)/2;
                (self.board[A1], self.board[self.kp]) = (self.board[self.kp], self.board[A1]);
            }
            if mv.to as i32 - mv.from as i32 == 2{
                self.kp = (mv.to+mv.from)/2;
                (self.board[H1], self.board[self.kp]) = (self.board[self.kp], self.board[H1]);
            }
        }
        if self.board[mv.to] == Square::MyPawn {
            if mv.to as i32 + Directions::SOUTH == *ep as i32 {
                self.board[(mv.to as i32 + Directions::SOUTH) as usize] = Square::OpponentPawn;
            }
        }
        if A8 <= mv.to && mv.to <= H8 {
            match mv.prom {
                Some(_) => self.board[mv.to] = Square::MyPawn,
                None => (),
            }
        }
        (self.board[mv.to], self.board[mv.from]) = (*dest, self.board[mv.to]);
        self.myc = *ori_myc;
        self.oppc = *ori_oppc;
        self.ep = *ep;
        self.kp = *kp;
    }
    pub fn search_check(&self) -> bool {
        let mut king = 1;
        for i in 0..120 {
            if self.board[i] == Square::OpponentKing {
                king = i;
            }
        }
        if king == 1 {
            return true;
        }
        let castling;
        let king_ori = kingori(&self.kp);
        if king_ori == 0 {
            castling = false;
        } else {
            castling = true;
        }
        let move_list = self.gen_move();
        for i in 0..move_list.len() {
            if move_list[i].to == king || move_list[i].to == self.kp {
                return true;
            }
            if castling {
                if move_list[i].to == king_ori {
                    return true;
                }
            }
        }
        false
    }
    pub fn evaluate_pos(&mut self) -> i32 {
        let mut score = 0;
        for i in 0..120 {
            let mut piece = self.board[i];
            if piece.is_my_piece() {
                score += piece.value() + piece.pst()[i];
            } else if piece.is_opponent_piece() {
                score += piece.value();
                self.rotate();
                piece = self.board[i];
                self.rotate();
                score = score - piece.pst()[119-i];
            }
        }
        score
    }
    pub fn gen_move(&self) -> Vec<Move> {
        let mut move_list: Vec<Move> = Vec::new();
        let mut piece;
        for i in 0..120 {
            if self.board[i].is_my_piece() {
                piece = self.board[i];
                for d in piece.directions() {
                    let mut j = (i as i32 + *d) as usize;
                    loop {
                        let destination = self.board[j];
                        if destination == Square::NotOnTheBoard || destination.is_my_piece() {
                            break;
                        }
                        if piece == Square::MyPawn {
                            if *d == Directions::NORTH || *d == Directions::NORTH * 2 {
                                if destination != Square::Empty {
                                    break;
                                }
                            }
                            if *d == Directions::NORTH*2 && ((i as i32) < (A1 as i32 +Directions::NORTH) || self.board[(i as i32 +Directions::NORTH) as usize] != Square::Empty) {
                                break;
                            }
                            if *d == Directions::NORTH+Directions::WEST || *d == Directions::NORTH+Directions::EAST {
                                if destination == Square::Empty && (j as i32 +Directions::SOUTH) as usize != self.ep && j != kingori(&self.kp) {
                                    break;
                                }
                            }
                            if A8 <= j && j <= H8 {
                                let prom = [Prom::Q, Prom::R, Prom::B, Prom::N];
                                for p in 0..prom.len() {
                                    move_list.push(Move {from: i, to: j, prom: Some(prom[p])});
                                }
                                break;
                            }
                        }
                        move_list.push(Move {from: i, to: j, prom: None});
                        match piece {
                            Square::MyPawn => break,
                            Square::MyKnight => break,
                            Square::MyKing => break,
                            _ => {
                                if destination.is_opponent_piece() {
                                    break;
                                }
                                if i == A1 && self.board[(j as i32 + Directions::EAST) as usize] == Square::MyKing && self.myc.0 == true {
                                    move_list.push(Move {from: (j as i32 + Directions::EAST) as usize, to: (j as i32 + Directions::WEST) as usize, prom: None});
                                }
                                if i == H1 && self.board[(j as i32 + Directions::WEST) as usize] == Square::MyKing && self.myc.1 == true {
                                    move_list.push(Move {from: (j as i32 + Directions::WEST) as usize, to: (j as i32 + Directions::EAST) as usize, prom: None});
                                }
                            }
                        };
                        j = (j as i32 + *d) as usize;
                    }
                }
            }
        }
        move_list
    }
}

fn promotion(prom: &Option<Prom>) -> Square {
    match *prom {
        Some(Prom::Q) => Square::MyQueen,
        Some(Prom::R) => Square::MyRook,
        Some(Prom::B) => Square::MyBishop,
        Some(Prom::N) => Square::MyKnight,
        _ => Square::Empty,
    }
}

fn kingori(kp: &usize) -> usize {
    let king_ori;
    match *kp {
        23 | 25 => {
            king_ori = 24;
        },
        24 | 26 => {
            king_ori = 25;
        },
        93 | 95 => {
            king_ori = 94;
        },
        94 | 96 => {
            king_ori = 95;
        },
        _ => {
            king_ori = 0;
        },
    };
    king_ori
}

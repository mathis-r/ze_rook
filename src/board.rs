use crate::render;
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
    pub w_pos_table: Vec<usize>,
    pub b_pos_table: Vec<usize>,
    pub my_attack_table: [bool;120],
    pub opp_attack_table: [bool;120],
    pub my_king: usize,
    pub opp_king: usize,
    pub pinned: [(usize, i32) ;8],
    pub checkers: (usize, usize),
}

impl BoardState {
    pub fn new() -> BoardState {
        let mut w_pos_table: Vec<usize> = Vec::new();
        let mut b_pos_table: Vec<usize> = Vec::new();
        let mut my_attack_table = [false; 120];
        let mut opp_attack_table = [false; 120];
        let checkers = (0, 0);
        let my_king = 95;
        let opp_king = 25;
        let pinned = [(0, 0);8];

        // white pos/attack table :
        for i in 81..89 {
            w_pos_table.push(i); // we keep this index to loop over our pieces

            my_attack_table[i] = true;
        }
        for i in 91..99 {
            w_pos_table.push(i);
            if i != 91 && i != 98 {
                my_attack_table[i] = true;
            }
        }
        for i in 71..79 {
            my_attack_table[i] = true;
        }

        // black pos/attack table :
        for i in 21..29 {
            b_pos_table.push(119 - i); // we reverse the index to make it faster to loop over our
                                       // own pieces when the position is reversed (black to move)

            if i != 21 && i != 28 {
                opp_attack_table[i] = true;
            }
        }
        for i in 31..39 {
            b_pos_table.push(119 - i);
            opp_attack_table[i] = true;
        }
        for i in 41..49 {
            opp_attack_table[i] = true;
        }

        let mut board = BoardState {board: INITIAL_BOARD, myc: (true, true), oppc: (true, true), ep: 0, kp: 0, color: 'w', w_pos_table, b_pos_table, my_attack_table, opp_attack_table, my_king, opp_king, pinned, checkers };
        board.rotate();
        board.gen_captures();
        board.rotate();
        board
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
            if self.color == 'b' {
                println!("Side to move : w");
            }
            else {
                println!("Side to move : b");
            }
        }
        println!("Checkers : {} {}", render(self.checkers.0 as i32), render(self.checkers.1 as i32));
        println!("{:?}", self.pinned);
        let mut pos_tab = String::new();
        let mut att_tab = String::new();
        let mut fgd;
        for square in 0..120 {
            fgd = false;
            if square % 10 == 9 {
                pos_tab.push(' ');
                pos_tab.push('0');
                pos_tab.push('\n');
                att_tab.push(' ');
                att_tab.push('0');
                att_tab.push('\n');
            } else {
                pos_tab.push(' ');
                att_tab.push(' ');
                if self.my_attack_table[square] == true {
                    att_tab.push('1');
                } else {
                    att_tab.push('0');
                }
                for q in 0..self.w_pos_table.len() {
                    if square == self.w_pos_table[q] {
                        pos_tab.push('1');
                        fgd = true;
                        break;
                    }
                }
                if fgd {
                   continue; 
                }
                pos_tab.push('0');
            }
        }
        println!("{pos_tab}");
        println!("{att_tab}");
        self.rotate();
        let mut pos_tab = String::new();
        let mut att_tab = String::new();
        let mut fgd;
        for square in 0..120 {
            fgd = false;
            if square % 10 == 9 {
                pos_tab.push(' ');
                pos_tab.push('0');
                pos_tab.push('\n');
                att_tab.push(' ');
                att_tab.push('0');
                att_tab.push('\n');
            } else {
                pos_tab.push(' ');
                att_tab.push(' ');
                if self.my_attack_table[square] == true {
                    att_tab.push('1');
                } else {
                    att_tab.push('0');
                }
                for q in 0..self.b_pos_table.len() {
                    if square == self.b_pos_table[q] {
                        pos_tab.push('1');
                        fgd = true;
                        break;
                    }
                }
                if fgd {
                   continue; 
                }
                pos_tab.push('0');
            }
        }
        println!("{pos_tab}");
        println!("{att_tab}");
        self.rotate();
        if change {
            self.rotate();
        }
    }

    pub fn rotate(&mut self) {
        self.ep = 119 - self.ep;
        self.kp = 119 - self.kp;
        (self.myc, self.oppc) = (self.oppc, self.myc);
        for i in 0..60 {
            (self.board[119-i], self.board[i]) = (self.board[i].swap_color(), self.board[119-i].swap_color());
            (self.my_attack_table[119-i], self.my_attack_table[i]) = (self.my_attack_table[i], self.my_attack_table[119-i]);
            (self.opp_attack_table[119-i], self.opp_attack_table[i]) = (self.opp_attack_table[i], self.opp_attack_table[119-i]);
        }
        self.color = match self.color {
            'w' => 'b',
            'b' => 'w',
            _ => 'w',
        };
        self.checkers.0 = 119 - self.checkers.0;
        self.checkers.1 = 119 - self.checkers.1;
        (self.my_king, self.opp_king) = (119 - self.opp_king, 119 - self.my_king);
        (self.my_attack_table, self.opp_attack_table) = (self.opp_attack_table, self.my_attack_table);
        for i in 0..self.pinned.len() {
            self.pinned[i].0 = 119 - self.pinned[i].0;
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
            self.my_king = mv.to;
            self.myc = (false, false);
            if mv.from as i32 - mv.to as i32 == 2{
                self.kp = (mv.to+mv.from)/2;
                (self.board[A1], self.board[self.kp]) = (self.board[self.kp], self.board[A1]);
                if self.color == 'w' {
                    for i in 0..self.w_pos_table.len() {
                        if self.w_pos_table[i] == A1 {
                            self.w_pos_table.swap_remove(i);
                            break;
                        }
                    }
                    self.w_pos_table.push(self.kp);
                } else if self.color == 'b' {
                    for i in 0..self.b_pos_table.len() {
                        if self.b_pos_table[i] == A1 {
                            self.b_pos_table.swap_remove(i);
                            break;
                        }
                    }
                    self.b_pos_table.push(self.kp);
                }
            }
            if mv.to as i32 - mv.from as i32 == 2{
                self.kp = (mv.to+mv.from)/2;
                (self.board[H1], self.board[self.kp]) = (self.board[self.kp], self.board[H1]);
                if self.color == 'w' {
                    for i in 0..self.w_pos_table.len() {
                        if self.w_pos_table[i] == H1 {
                            self.w_pos_table.swap_remove(i);
                            break;
                        }
                    }
                    self.w_pos_table.push(self.kp);
                } else if self.color == 'b' {
                    for i in 0..self.b_pos_table.len() {
                        if self.b_pos_table[i] == H1 {
                            self.b_pos_table.swap_remove(i);
                            break;
                        }
                    }
                    self.b_pos_table.push(self.kp);
                }
            }
        }
        if piece == Square::MyPawn {
            if mv.to as i32 + Directions::SOUTH == self.ep as i32 {
                self.board[(mv.to as i32 + Directions::SOUTH) as usize] = Square::Empty;
                self.ep = 0;
                if self.color == 'w' {
                    for i in 0..self.b_pos_table.len() {
                        if self.b_pos_table[i] == 119 - (mv.to as i32 + Directions::SOUTH) as usize {
                            self.b_pos_table.swap_remove(i);
                            break;
                        }
                    }
                } else if self.color == 'b' {
                    for i in 0..self.w_pos_table.len() {
                        if self.w_pos_table[i] == 119 - (mv.to as i32 + Directions::SOUTH) as usize {
                            self.w_pos_table.swap_remove(i);
                            break;
                        }
                    }
                }
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
        if self.color == 'w' {
            for i in 0..self.w_pos_table.len() {
                if self.w_pos_table[i] == mv.from {
                    self.w_pos_table.swap_remove(i);
                    break;
                }
            }
            for i in 0..self.b_pos_table.len() {
                if self.b_pos_table[i] == 119 - mv.to {
                    self.b_pos_table.swap_remove(i);
                    break;
                }
            }
            self.w_pos_table.push(mv.to);
        } else if self.color == 'b' {
            for i in 0..self.b_pos_table.len() {
                if self.b_pos_table[i] == mv.from {
                    self.b_pos_table.swap_remove(i);
                    break;
                }
            }
            for i in 0..self.w_pos_table.len() {
                if self.w_pos_table[i] == 119 - mv.to {
                    self.w_pos_table.swap_remove(i);
                    break;
                }
            }
            self.b_pos_table.push(mv.to);
        }
        (self.board[mv.from], self.board[mv.to]) = (Square::Empty, self.board[mv.from]);
        self.gen_captures();
    }

    pub fn unmake(&mut self, mv: &Move, dest: &Square, ori_myc: &(bool, bool), ori_oppc: &(bool, bool), ep: &usize, kp: &usize) {
        if self.board[mv.to] == Square::MyKing {
            self.my_king = mv.from;
            if mv.from as i32 - mv.to as i32 == 2{
                self.kp = (mv.to+mv.from)/2;
                (self.board[A1], self.board[self.kp]) = (self.board[self.kp], self.board[A1]);
                if self.color == 'w' {
                    for i in 0..self.w_pos_table.len() {
                        if self.w_pos_table[i] == self.kp {
                            self.w_pos_table.swap_remove(i);
                            break;
                        }
                    }
                    self.w_pos_table.push(A1);
                } else if self.color == 'b' {
                    for i in 0..self.b_pos_table.len() {
                        if self.b_pos_table[i] == self.kp {
                            self.b_pos_table.swap_remove(i);
                            break;
                        }
                    }
                    self.b_pos_table.push(A1);
                }
            }
            if mv.to as i32 - mv.from as i32 == 2{
                self.kp = (mv.to+mv.from)/2;
                (self.board[H1], self.board[self.kp]) = (self.board[self.kp], self.board[H1]);
                if self.color == 'w' {
                    for i in 0..self.w_pos_table.len() {
                        if self.w_pos_table[i] == self.kp {
                            self.w_pos_table.swap_remove(i);
                            break;
                        }
                    }
                    self.w_pos_table.push(H1);
                } else if self.color == 'b' {
                    for i in 0..self.b_pos_table.len() {
                        if self.b_pos_table[i] == self.kp {
                            self.b_pos_table.swap_remove(i);
                            break;
                        }
                    }
                    self.b_pos_table.push(H1);
                }
            }
        }
        if self.board[mv.to] == Square::MyPawn {
            if mv.to as i32 + Directions::SOUTH == *ep as i32 {
                self.board[(mv.to as i32 + Directions::SOUTH) as usize] = Square::OpponentPawn;
                if self.color == 'w' {
                    self.b_pos_table.push(119 - (mv.to as i32 + Directions::SOUTH) as usize);
                } else if self.color == 'b' {
                    self.w_pos_table.push(119 - (mv.to as i32 + Directions::SOUTH) as usize);
                }
            }
        }
        if A8 <= mv.to && mv.to <= H8 {
            match mv.prom {
                Some(_) => self.board[mv.to] = Square::MyPawn,
                None => (),
            }
        }
        if self.color == 'w' {
            for i in 0..self.w_pos_table.len() {
                if self.w_pos_table[i] == mv.to {
                    self.w_pos_table.swap_remove(i);
                    break;
                }
            }
            if dest.is_opponent_piece() {
                self.b_pos_table.push(119 - mv.to);
            }
            self.w_pos_table.push(mv.from);
        } else if self.color == 'b' {
            for i in 0..self.b_pos_table.len() {
                if self.b_pos_table[i] == mv.to {
                    self.b_pos_table.swap_remove(i);
                    break;
                }
            }
            if dest.is_opponent_piece() {
                self.w_pos_table.push(119 - mv.to);
            }
            self.b_pos_table.push(mv.from);
        }
        (self.board[mv.to], self.board[mv.from]) = (*dest, self.board[mv.to]);
        self.myc = *ori_myc;
        self.oppc = *ori_oppc;
        self.ep = *ep;
        self.kp = *kp;
    }

    //pub fn search_check(&mut self) -> bool {
    //    let mut king = 1;
    //    for i in 0..120 {
    //        if self.board[i] == Square::OpponentKing {
    //            king = i;
    //        }
    //    }
    //    if king == 1 {
    //        return true;
    //    }
    //    let castling;
    //    let king_ori = kingori(&self.kp);
    //    if king_ori == 0 {
    //        castling = false;
    //    } else {
    //        castling = true;
    //    }
    //    let move_list = self.gen_move();
    //    for i in 0..move_list.len() {
    //        if move_list[i].to == king || move_list[i].to == self.kp {
    //            return true;
    //        }
    //        if castling {
    //            if move_list[i].to == king_ori {
    //                return true;
    //            }
    //        }
    //    }
    //    false
    //}

    pub fn evaluate_pos(&mut self) -> i32 {
        let mut score = 0;
        for i in 0..self.w_pos_table.len() {
            let piece;
            if self.color == 'b' {
                piece = self.board[119 - self.w_pos_table[i]];
                //println!("{:?}", piece);
                //println!("{}", - piece.swap_color().pst()[self.w_pos_table[i]]);
                //println!("{}", self.w_pos_table[i]);
                score += piece.value();
                score -= piece.swap_color().pst()[self.w_pos_table[i]];
            } else {
                piece = self.board[self.w_pos_table[i]];
                //println!("{:?}", piece);
                //println!("{}", piece.pst()[self.w_pos_table[i]]);
                //println!("{}", self.w_pos_table[i]);
                score += piece.value() + piece.pst()[self.w_pos_table[i]];
            }
        }
        for i in  0..self.b_pos_table.len() {
            let piece;
            if self.color == 'w' {
                piece = self.board[119 - self.b_pos_table[i]];
                //println!("{:?}", piece);
                //println!("{}", - piece.swap_color().pst()[self.b_pos_table[i]]);
                //println!("{}", self.b_pos_table[i]);
                score += piece.value();
                score -= piece.swap_color().pst()[self.b_pos_table[i]];
            } else {
                piece = self.board[self.b_pos_table[i]];
                //println!("{:?}", piece);
                //println!("{}", piece.pst()[self.b_pos_table[i]]);
                //println!("{}", self.b_pos_table[i]);
                score += piece.value() + piece.pst()[self.b_pos_table[i]];
            }
        }
        //println!("score");
        //score = 0;
        //for i in 0..120 {
        //    let mut piece = self.board[i];
        //    if piece.is_my_piece() {
        //        score += piece.value() + piece.pst()[i];
        //    } else if piece.is_opponent_piece() {
        //        println!("{:?}", piece);
        //        println!("{i}");
        //        score += piece.value();
        //        self.rotate();
        //        piece = self.board[i];
        //        self.rotate();
        //        println!("{:?}", piece);
        //        println!("{score}");
        //        score = score - piece.pst()[119-i];
        //        println!("{score}");
        //    }
        //    if piece != Square::Empty && piece != Square::NotOnTheBoard {
        //        println!("{:?}", piece);
        //        println!("{score}");
        //    }
        //}
        score
    }

    pub fn gen_captures(&mut self) {
        let mut pin = 0;
        let mut check = 0;
        self.pinned = [(0, 0); 8];
        self.my_attack_table = [false; 120];
        self.checkers = (0, 0);
        let mut piece;
        let pieces;
        let mut i;
        if self.color == 'w' {
            pieces = 0..self.w_pos_table.len();
        } else {
            pieces = 0..self.b_pos_table.len();
        }
        for k in pieces {
            if self.color == 'w' {
                piece = self.board[self.w_pos_table[k]];
                i = self.w_pos_table[k];
            } else {
                piece = self.board[self.b_pos_table[k]];
                i = self.b_pos_table[k];
            }
            for d in piece.directions() {
                let mut j = (i as i32 + *d) as usize;
                loop {
                    let destination = self.board[j];
                    if destination == Square::NotOnTheBoard {
                        break;
                    }
                    if piece == Square::MyPawn {
                        if *d == Directions::NORTH || *d == Directions::NORTH * 2 {
                            break;
                        }
                    }
                    self.my_attack_table[j] = true;
                    if j == self.opp_king {
                        if check == 0 {
                            self.checkers.0 = i;
                            check += 1;
                        } else if check == 1 {
                            self.checkers.1 = i;
                            check += 1;
                        }
                    }
                    if destination.is_my_piece() {
                        break;
                    }
                    match piece {
                        Square::MyPawn => break,
                        Square::MyKnight => break,
                        Square::MyKing => break,
                        _ => {
                            if destination.is_opponent_piece() && destination != Square::OpponentKing {
                                let mut l = j;
                                loop {
                                    l = (l as i32 + *d) as usize;
                                    let destination = self.board[l];
                                    if destination == Square::Empty {
                                        continue;
                                    }
                                    if destination == Square::NotOnTheBoard {
                                        break;
                                    }
                                    if destination != Square::OpponentKing {
                                        break;
                                    }
                                    self.pinned[pin] = (j, *d);
                                    pin += 1;
                                    break;
                                }
                                break;
                            }
                        }
                    };
                    j = (j as i32 + *d) as usize;
                }
            }
        }
    }

    pub fn gen_move(&mut self) -> Vec<Move> {
        let mut move_list: Vec<Move> = Vec::new();
        let mut piece;
        let pieces;
        let mut i;

        if self.checkers.1 != 0 && self.checkers.1 != 119 {
            let king = self.board[self.my_king];
            for d in king.directions() {
                let j = (self.my_king as i32 + *d) as usize;
                let destination = self.board[j];
                if destination == Square::NotOnTheBoard || destination.is_my_piece() {
                    continue;
                }
                if self.opp_attack_table[j] {
                    continue;
                }
                move_list.push(Move { from: self.my_king, to: j, prom: None });
            }
            return move_list;
        }

        if self.color == 'w' {
            pieces = 0..self.w_pos_table.len();
        } else {
            pieces = 0..self.b_pos_table.len();
        }

        let check = self.checkers.0 != 0 && self.checkers.0 != 119;
        let raycheck = match self.board[self.checkers.0] {
            Square::OpponentRook | Square::OpponentQueen | Square::OpponentBishop => true,
            _ => false,
        };
        let mut intercept = [true; 120];
        if check {
            intercept = [false; 120];
            intercept[self.checkers.0] = true;
        }
        if raycheck {
            let directions = self.checkers.0 as i32 - self.my_king as i32;
            if directions % 10 == 0 {
                if self.checkers.0 > self.my_king {
                    let mut inter = self.checkers.0;
                    loop {
                        inter -= 10;
                        if inter == self.my_king {
                            break;
                        }
                        intercept[inter] = true;
                    }
                } else {
                    let mut inter = self.checkers.0;
                    loop {
                        inter += 10;
                        if inter == self.my_king {
                            break;
                        }
                        intercept[inter] = true;
                    }
                }
            } else if directions % 9 == 0 {
                if self.checkers.0 > self.my_king {
                    let mut inter = self.checkers.0;
                    loop {
                        inter -= 9;
                        if inter == self.my_king {
                            break;
                        }
                        intercept[inter] = true;
                    }
                } else {
                    let mut inter = self.checkers.0;
                    loop {
                        inter += 9;
                        if inter == self.my_king {
                            break;
                        }
                        intercept[inter] = true;
                    }
                }
            } else if directions % 11 == 0 {
                if self.checkers.0 > self.my_king {
                    let mut inter = self.checkers.0;
                    loop {
                        inter -= 11;
                        if inter == self.my_king {
                            break;
                        }
                        intercept[inter] = true;
                    }
                } else {
                    let mut inter = self.checkers.0;
                    loop {
                        inter += 11;
                        if inter == self.my_king {
                            break;
                        }
                        intercept[inter] = true;
                    }
                }
            } else if directions % 1 == 0 {
                if self.checkers.0 > self.my_king {
                    let mut inter = self.checkers.0;
                    loop {
                        inter -= 1;
                        if inter == self.my_king {
                            break;
                        }
                        intercept[inter] = true;
                    }
                } else {
                    let mut inter = self.checkers.0;
                    loop {
                        inter += 1;
                        if inter == self.my_king {
                            break;
                        }
                        intercept[inter] = true;
                    }
                }
            }
        }
        for k in pieces {
            if self.color == 'w' {
                piece = self.board[self.w_pos_table[k]];
                i = self.w_pos_table[k];
            } else {
                piece = self.board[self.b_pos_table[k]];
                i = self.b_pos_table[k];
            }

            let mut pin = false;
            for pinned in 0..self.pinned.len() {
                if i == self.pinned[pinned].0 {
                    for d in piece.directions() {
                        if *d != self.pinned[pinned].1 && *d != - self.pinned[pinned].1 {
                            if *d == -20 && (self.pinned[pinned].1 == -10 || self.pinned[pinned].1 == 10) {
                                ();
                            } else {
                                continue;
                            }
                        }
                        let mut j = (i as i32 + *d) as usize;
                        loop {
                            let destination = self.board[j];
                            if destination == Square::NotOnTheBoard || destination.is_my_piece() { //ep for captures only ?
                                break;
                            }
                            if check {
                                if j != self.checkers.0 {
                                    j = (j as i32 + *d) as usize;
                                    continue;
                                }
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
                                    if destination == Square::Empty && (j as i32 +Directions::SOUTH) as usize != self.ep {
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
                            move_list.push(Move { from: i, to: j, prom: None } );
                            if piece == Square::MyPawn {
                                break;
                            }
                            if destination.is_opponent_piece() {
                                break;
                            }
                            j = (j as i32 + *d) as usize;
                        }
                    }
                    pin = true;
                    break;
                }
            }

            if pin {
                continue;
            }
            for d in piece.directions() {
                let mut j = (i as i32 + *d) as usize;
                loop {
                    let destination = self.board[j];
                    if destination == Square::NotOnTheBoard || destination.is_my_piece() { //ep for captures only ?
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
                            if destination == Square::Empty && (j as i32 +Directions::SOUTH) as usize != self.ep {
                                break;
                            } else if (j as i32 +Directions::SOUTH) as usize == self.ep {
                                let mut illegal = false;
                                if self.my_king > 50 && self.my_king < 59 {
                                    if self.my_king > i {
                                        let mut last_piece = Square::Empty;
                                        for l in 51..i {
                                            if l == (j as i32 + Directions::SOUTH) as usize {
                                                break;
                                            }
                                            if self.board[l] != Square::Empty {
                                                last_piece = self.board[l];
                                            }
                                        }
                                        if last_piece == Square::OpponentRook || last_piece == Square::OpponentQueen {
                                            let mut empty = true;
                                            for l in i+1..self.my_king {
                                                if l == (j as i32 + Directions::SOUTH) as usize {
                                                    continue;
                                                }
                                                if self.board[l] != Square::Empty {
                                                    empty = false;
                                                    break;
                                                }
                                            }
                                            if empty {
                                                illegal = true;
                                            }
                                        }
                                    } else {
                                        let mut first_piece = Square::Empty;
                                        for l in i+1..59 {
                                            if l == (j as i32 + Directions::SOUTH) as usize {
                                                continue;
                                            }
                                            if self.board[l] != Square::Empty {
                                                first_piece = self.board[l];
                                                break;
                                            }
                                        }
                                        if first_piece == Square::OpponentRook || first_piece == Square::OpponentQueen {
                                            let mut empty = true;
                                            for l in self.my_king+1..i {
                                                if l == (j as i32 + Directions::SOUTH) as usize {
                                                    break;
                                                }
                                                if self.board[l] != Square::Empty {
                                                    empty = false;
                                                    break;
                                                }
                                            }
                                            if empty {
                                                illegal = true;
                                            }
                                        }
                                    }
                                }
                                if illegal {
                                    break;
                                }
                            }
                        }
                        if A8 <= j && j <= H8  && intercept[j] {
                            let prom = [Prom::Q, Prom::R, Prom::B, Prom::N];
                            for p in 0..prom.len() {
                                move_list.push(Move {from: i, to: j, prom: Some(prom[p])});
                            }
                            break;
                        }
                    }
                    if piece == Square::MyKing {
                        if !self.opp_attack_table[j] {
                            move_list.push(Move {from: i, to: j, prom: None});
                            break;
                        } else {
                            break;
                        }
                    }
                    if intercept[j] || (intercept[(j as i32 + Directions::SOUTH) as usize] && (j as i32 + Directions::SOUTH) as usize == self.ep && piece == Square::MyPawn) {
                        move_list.push(Move {from: i, to: j, prom: None});
                    }
                    match piece {
                        Square::MyPawn => break,
                        Square::MyKnight => break,
                        Square::MyKing => break,
                        _ => {
                            if destination.is_opponent_piece() {
                                break;
                            }
                            if i == A1 && self.board[(j as i32 + Directions::EAST) as usize] == Square::MyKing && self.myc.0 == true && !check && !self.opp_attack_table[self.my_king-2] && !self.opp_attack_table[self.my_king-1] {
                                move_list.push(Move {from: (j as i32 + Directions::EAST) as usize, to: (j as i32 + Directions::WEST) as usize, prom: None});
                            }
                            if i == H1 && self.board[(j as i32 + Directions::WEST) as usize] == Square::MyKing && self.myc.1 == true && !check && !self.opp_attack_table[self.my_king+1] && !self.opp_attack_table[self.my_king+2]{
                                move_list.push(Move {from: (j as i32 + Directions::WEST) as usize, to: (j as i32 + Directions::EAST) as usize, prom: None});
                            }
                        }
                    };
                    j = (j as i32 + *d) as usize;
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

//fn kingori(kp: &usize) -> usize {
//    let king_ori;
//    match *kp {
//        23 | 25 => {
//            king_ori = 24;
//        },
//        24 | 26 => {
//            king_ori = 25;
//        },
//        93 | 95 => {
//            king_ori = 94;
//        },
//        94 | 96 => {
//            king_ori = 95;
//        },
//        _ => {
//            king_ori = 0;
//        },
//    };
//    king_ori
//}

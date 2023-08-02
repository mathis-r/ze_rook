use crate::piece::Square;
use crate::BoardState;
use ze_rook::A1;

pub fn fen(fen_str: String) -> BoardState {
    let mut boardstate = BoardState::new();
    let parts: Vec<&str> = fen_str.split(' ').collect();
    let rows: Vec<&str> = parts[0].split('/').collect();
    if rows.len() != 8 {
        println!("FEN string should have 8 rows");
        return boardstate;
    }
    for r in 0..8 {
        let mut index = r*10 +21;
        for p in rows[r].chars() {
            match p {
                'K' => {
                    boardstate.board[index] = Square::MyKing;
                    index+=1;
                },
                'Q' => {
                    boardstate.board[index] = Square::MyQueen;
                    index+=1;
                },
                'R' => {
                    boardstate.board[index] = Square::MyRook;
                    index+=1;
                },
                'B' => {
                    boardstate.board[index] = Square::MyBishop;
                    index+=1;
                },
                'N' => {
                    boardstate.board[index] = Square::MyKnight;
                    index+=1;
                },
                'P' => {
                    boardstate.board[index] = Square::MyPawn;
                    index+=1;
                },
                'k' => {
                    boardstate.board[index] = Square::OpponentKing;
                    index+=1;
                },
                'q' => {
                    boardstate.board[index] = Square::OpponentQueen;
                    index+=1;
                },
                'r' => {
                    boardstate.board[index] = Square::OpponentRook;
                    index+=1;
                },
                'b' => {
                    boardstate.board[index] = Square::OpponentBishop;
                    index+=1;
                },
                'n' => {
                    boardstate.board[index] = Square::OpponentKnight;
                    index+=1;
                },
                'p' => {
                    boardstate.board[index] = Square::OpponentPawn;
                    index+=1;
                },
                '1' => {
                    for _ in 0..1 {
                        boardstate.board[index] = Square::Empty;
                        index+=1;
                    }
                }
                '2' => {
                    for _ in 0..2 {
                        boardstate.board[index] = Square::Empty;
                        index+=1;
                    }
                }
                '3' => {
                    for _ in 0..3 {
                        boardstate.board[index] = Square::Empty;
                        index+=1;
                    }
                }
                '4' => {
                    for _ in 0..4 {
                        boardstate.board[index] = Square::Empty;
                        index+=1;
                    }
                }
                '5' => {
                    for _ in 0..5 {
                        boardstate.board[index] = Square::Empty;
                        index+=1;
                    }
                }
                '6' => {
                    for _ in 0..6 {
                        boardstate.board[index] = Square::Empty;
                        index+=1;
                    }
                }
                '7' => {
                    for _ in 0..7 {
                        boardstate.board[index] = Square::Empty;
                        index+=1;
                    }
                }
                '8' => {
                    for _ in 0..8 {
                        boardstate.board[index] = Square::Empty;
                        index+=1;
                    }
                }
                _ => {
                    println!("Invalid FEN string");
                    return BoardState::new();
                },
            }
        }
    }
    let color = parts[1];
    let castling = parts[2];
    boardstate.myc = (false, false);
    boardstate.oppc = (false, false);
    for i in castling.chars() {
        match i {
            'K' => boardstate.myc.1 = true,
            'Q' => boardstate.myc.0 = true,
            'k' => boardstate.oppc.0 = true,
            'q' => boardstate.oppc.1 = true,
            _ => continue,
        };
    }
    boardstate.ep = if parts[3] != "-" {
        parse(parts[3].to_string()) as usize + 10
    } else {
        0
    };
    if color == "b" {
        boardstate.rotate();
    }
    boardstate
}

pub fn parse(s: String) -> i64 {
    let mut c = Vec::new();
    for i in s.chars() {
        c.push(i);
    }
    let (fil, rank) = (c[0] as u32 - 'a' as u32, c[1] as u32 -49);
    (A1 as u32 + fil) as i64 -10 * rank as i64
}

pub fn render(i: i32) -> String {
    let rank;
    let fil;
    let r = i - A1 as i32;
    if r < 0 && (i - A1 as i32) % 10 != 0 {
        rank = (i - A1 as i32) / 10 - 1;
        fil = (i - A1 as i32) % 10 +10;
    } else {
        rank = (i - A1 as i32) / 10;
        fil = (i - A1 as i32) % 10;
    }
    let e = char::from_u32(fil as u32+ 'a' as u32);
    let c = match e {
        Some(p) => p,
        _ => 'e',
    };
    let s = format!("{}{}", c, - (rank as i32) + 1);
    s
}

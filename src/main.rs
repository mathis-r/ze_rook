use std::time::{Duration, Instant};
use crate::search::alphabeta;
use crate::uci::{fen, render, parse};
use crate::debug::perft;
use std::collections::HashMap;
use std::io;

pub mod search;
pub mod uci;
pub mod debug;
pub mod board;
pub mod piece;

use crate::board::{Board, BoardState};
use crate::piece::Move;
use crate::piece::Prom;

use ze_rook::MATEUPPER;

fn main() {
    let mut tt: HashMap<Board,(Vec<Move>, i32, i32)> = HashMap::new();
    let (mut wtime, mut winc) = (60000, 0);
    let (mut btime, mut binc): (i64, i64);
    let mut boardstate = BoardState::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let args: Vec<&str> = input.trim().split(' ').collect();
        if args[0] == "uci".to_string() {
            println!("id name ZE_ROOK v0.5");
            println!("uciok");
        } else if args[0] == "isready" {
            println!("readyok");
        } else if args[0] == "d" || args[0] == "print" {
            boardstate.print();
        } else if args[0] == "quit" {
            break;
        } else if args[0] == "ucinewgame" {
            tt.clear();
            tt.shrink_to_fit();
            boardstate = BoardState::new();
        } else if args[0] == "eval" {
            println!("{}", boardstate.evaluate_pos());
        } else if args[0] == "go" {
            if args.len() >= 3 {
                if args[1] == "perft" {
                    let depthmax = match args[2] {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        _ => 0,
                    };
                    println!("Nodes searched: {}", perft(&depthmax, &depthmax, &mut boardstate));
                    boardstate.rotate();
                    boardstate.gen_att_map();
                    boardstate.rotate();
                    continue;
                }
            }
            if args.len() >= 9 {
                wtime = match args[2].trim().parse() {
                    Ok(num) => num,
                    Err(_) => panic!("White time error: not a number"),
                };
                btime = match args[4].trim().parse() {
                    Ok(num) => num,
                    Err(_) => panic!("Black time error: not a number"),
                };
                winc = match args[6].trim().parse() {
                    Ok(num) => num,
                    Err(_) => panic!("White time increment error: not a number"),
                };
                binc = match args[8].trim().parse() {
                    Ok(num) => num,
                    Err(_) => panic!("Black time increment error: not a number"),
                };
                if boardstate.color == 'b' {
                    (wtime, winc) = (btime, binc);
                }
            } else if args.len() >= 3 {
                wtime = match args[2].trim().parse() {
                    Ok(num) => num,
                    Err(_) => panic!("White time error: not a number"),
                };
            }
            let think = Duration::from_millis((std::cmp::min(wtime / 40 + winc, wtime / 2 -1)) as u64);
            let start = Instant::now();
            let mut move_bfr_string: Option<Move> = None;
            let mut move_prv_iter: Option<Move> = None;
            for depthmax in 1..129 {
                let scmv = alphabeta(&mut boardstate, -MATEUPPER, MATEUPPER, depthmax, &mut tt, &depthmax, &think, &start, &move_prv_iter);
                move_bfr_string = scmv.1.get(0).copied();
                if scmv.0 == 999999 {
                    move_bfr_string = move_prv_iter;
                    break;
                }
                if start.elapsed() > (think / 125) * 100 {
                    break;
                }
                let mut move_str;
                match move_bfr_string {
                    Some(mv) => {
                        move_prv_iter = move_bfr_string;
                        let (mut i, mut j) = (mv.from, mv.to);
                        let mut k = 0;
                        if boardstate.color == 'b' {
                            (i, j) = (119-i, 119-j);
                            k+=1;
                        }
                        let prom = match mv.prom {
                            Some(Prom::Q) => "q",
                            Some(Prom::R) => "r",
                            Some(Prom::B) => "b",
                            Some(Prom::N) => "n",
                            _ => "",
                        };
                        move_str = format!("{}{}{}", render(i as i32), render(j as i32), prom);
                        for pv in 1..scmv.1.len() {
                            k+=1;
                            (i, j) = (scmv.1[pv].from, scmv.1[pv].to);
                            if k % 2 == 1 {
                                (i, j) = (119-i, 119-j);
                            }
                            let prom = match scmv.1[pv].prom {
                                Some(Prom::Q) => "q",
                                Some(Prom::R) => "r",
                                Some(Prom::B) => "b",
                                Some(Prom::N) => "n",
                                _ => "",
                            };
                            move_str = format!("{move_str} {}{}{}", render(i as i32), render(j as i32), prom);
                        }
                        println!("info depth {} score cp {} pv {}", depthmax, scmv.0, move_str);
                        },
                    None => break,
                }
                boardstate.rotate();
                boardstate.gen_att_map();
                boardstate.rotate();
            }
            match move_bfr_string {
                Some(mv) => {
                    let (mut i, mut j) = (mv.from, mv.to);
                    if boardstate.color == 'b' {
                        (i, j) = (119-i, 119-j);
                    }
                    let prom = match mv.prom {
                        Some(Prom::Q) => "q",
                        Some(Prom::R) => "r",
                        Some(Prom::B) => "b",
                        Some(Prom::N) => "n",
                        _ => "",
                    };
                    let move_str = format!("{}{}{}", render(i as i32), render(j as i32), prom);
                    println!("bestmove {move_str}");
                },
                None => println!("bestmove (none)"),
            }
            boardstate.rotate();
            boardstate.gen_att_map();
            boardstate.rotate();
        } else if args.len() >=2 {
            if args[..2] == ["position", "startpos"] {
                boardstate = BoardState::new();
                if args.len() > 3 {
                    if args[2] == "moves" {
                        let mut final_ply = 0;
                        for (ply, move_str) in args[3..].iter().enumerate() {
                            let (mut i, mut j, prom) = (parse(move_str[..2].to_string()), parse(move_str[2..4].to_string()), &move_str[4..]);
                            let prom = match prom {
                                "q" => Some(Prom::Q),
                                "r" => Some(Prom::R),
                                "b" => Some(Prom::B),
                                "n" => Some(Prom::N),
                                _ => None,
                            };
                            if boardstate.color == 'b' {
                                (i, j) = (119 - i, 119 -j);
                            }
                            if ply % 2 == 1 {
                                (i, j) = (119 - i, 119 -j);
                                boardstate.rotate();
                                boardstate.apply_move(&Move {from: i as usize, to: j as usize, prom});
                                boardstate.rotate();
                            } else {
                                boardstate.apply_move(&Move {from: i as usize, to: j as usize, prom});
                            }
                            final_ply = ply
                        }
                        if final_ply % 2 == 0 && boardstate.color == 'w' {
                            boardstate.rotate();
                        } else if final_ply % 2 == 0 && boardstate.color == 'b' {
                            boardstate.rotate();
                        }
                    }
                    boardstate.kp = 0;
                }
            } else if args[..2] == ["position", "fen"] {
                if args.len() < 8 {
                    continue;
                }
                let mut fen_str = String::new();
                for i in 0..args[2..8].len() {
                    fen_str = fen_str.to_owned() + args[2..8][i] + " ";
                }
                boardstate = fen(fen_str);
                if args.len() >= 9 {
                    if args[8] == "moves" {
                        let mut final_ply = 0;
                        for (ply, move_str) in args[9..].iter().enumerate() {
                            let (mut i, mut j, prom) = (parse(move_str[..2].to_string()), parse(move_str[2..4].to_string()), &move_str[4..]);
                            let prom = match prom {
                                "q" => Some(Prom::Q),
                                "r" => Some(Prom::R),
                                "b" => Some(Prom::B),
                                "n" => Some(Prom::N),
                                _ => None,
                            };
                            if boardstate.color == 'b' {
                                (i, j) = (119 - i, 119 -j);
                            }
                            if ply % 2 == 1 {
                                (i, j) = (119 - i, 119 -j);
                                boardstate.rotate();
                                boardstate.apply_move(&Move {from: i as usize, to: j as usize, prom});
                                boardstate.rotate();
                            } else {
                                boardstate.apply_move(&Move {from: i as usize, to: j as usize, prom});
                            }
                            final_ply = ply;
                        }
                        if final_ply % 2 == 0 && boardstate.color == 'w' {
                            boardstate.rotate();
                        } else if final_ply % 2 == 0 && boardstate.color == 'b' {
                            boardstate.rotate();
                        }
                    }
                    boardstate.kp = 0;
                }
            }
        }
    }
}

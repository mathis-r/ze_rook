use std::time::{Duration, Instant};
use ze_rook::{MATEUPPER, MATELOWER};
use std::collections::HashMap;
use crate::BoardState;
use crate::{Prom, Move};
use crate::Board;

pub fn alphabeta(boardstate: &mut BoardState, mut alpha: i32, beta: i32, depth: usize, tt: &mut HashMap<Board,(Vec<Move>, i32, i32)>, depthmax: &usize, think: &Duration, start: &Instant, move_prv_iter: &Option<Move>) -> (i32, Vec<Move>){
    let mut bestmove: Vec<Move> = Vec::new();
    if depth == 0 {
        return (quiesce(boardstate, alpha, beta, think, start, move_prv_iter), vec![]);
    }
    if start.elapsed() > (*think / 125) * 100 {
        match move_prv_iter {
            Some(_mv) => return (999999, vec![]),
            _ => (),
        };
    }
    let binding = (vec![Move {from: 0, to: 0, prom: None}], -1, -MATEUPPER);
    let mut killer = (*tt).get(&boardstate.board);
    match killer {
        None => killer = Some(&binding),
        Some(&(ref vecmv, killer_depth, killerscore)) => {
            if killer_depth >= depth as i32 || killerscore.abs() >= MATELOWER - 15 {
                alpha = killerscore;
                bestmove = vecmv.to_vec();
                return (alpha, bestmove);
            }
        }
    }
    let mut move_list = boardstate.gen_move();
    if move_list.is_empty() {
        if boardstate.checkers.0 != 0 && boardstate.checkers.0 != 119 {
            alpha = - MATELOWER + *depthmax as i32 - depth as i32;
            bestmove = vec![];
        } else {
            alpha = 0;
            bestmove = vec![];
        }
        return (alpha, bestmove);
    }
    for i in 0..move_list.len() {
        let promi = match move_list[i].prom {
            Some(prom) => match prom {
                Prom::Q => "q",
                Prom::R => "r",
                Prom::B => "b",
                Prom::N => "n",
            },
            None => " ",
        };
        match killer {
            Some(&(ref vecmv, _, _)) => {
                let firstmv = vecmv[0];
                let prom = match firstmv.prom {
                    Some(prom) => match prom {
                        Prom::Q => "q",
                        Prom::R => "r",
                        Prom::B => "b",
                        Prom::N => "n",
                    },
                    None => " ",
                };
                if firstmv.from == move_list[i].from && firstmv.to == move_list[i].to && prom == promi {
                    let killer = move_list.remove(i);
                    move_list.insert(0, killer);
                }
            },
            _ => println!("error: killer do not exist anymore"),
        };
    }
    for i in 0..move_list.len() {
        let dest = boardstate.board[move_list[i].to];
        let ori_myc = boardstate.myc;
        let ori_oppc = boardstate.oppc;
        let ep = boardstate.ep;
        let kp = boardstate.kp;
        let current_board = boardstate.board;
        boardstate.apply_move(&move_list[i]);
        boardstate.rotate();
        let scmv = alphabeta(boardstate, -beta, -alpha, depth-1, tt, depthmax, think, start, move_prv_iter);
        boardstate.rotate();
        boardstate.unmake(&move_list[i], &dest, &ori_myc, &ori_oppc, &ep, &kp);
        let score = -scmv.0;
        let mut pv = scmv.1;
        match scmv.0 {
            999999 => {
                return (999999, vec![]);
            },
            _ => {
                if score >= beta {
                    (*tt).remove(&current_board);
                    return (beta, vec![]);
                }
                if score > alpha {
                    alpha = score;
                    bestmove = vec![move_list[i]];
                    bestmove.append(&mut pv);
                    (*tt).insert(current_board, (bestmove.clone(), depth as i32, alpha));
                }
            },
        }
    }
    (alpha, bestmove)
}

fn quiesce(boardstate: &mut BoardState, mut alpha: i32, beta: i32, think: &Duration, start: &Instant, move_prv_iter: &Option<Move>) -> i32 {
    if start.elapsed() > (*think / 125) * 100 {
        match move_prv_iter {
            Some(_mv) => return 999999,
            _ => (),
        };
    }
    let stand_pat = boardstate.evaluate_pos();
    if stand_pat >= beta {
        return beta;
    }
    if alpha < stand_pat {
        alpha = stand_pat;
    }
    let captures = boardstate.gen_captures();
    for i in 0..captures.len() {
        let dest = boardstate.board[captures[i].to];
        let ori_myc = boardstate.myc;
        let ori_oppc = boardstate.oppc;
        let ep = boardstate.ep;
        let kp = boardstate.kp;
        boardstate.apply_move(&captures[i]);
        boardstate.rotate();
        let sc = quiesce(boardstate, -beta, -alpha, think, start, move_prv_iter);
        boardstate.rotate();
        boardstate.unmake(&captures[i], &dest, &ori_myc, &ori_oppc, &ep, &kp);
        let score = -sc;
        if sc == 999999 {
            return 999999;
        }

        if score >= beta {
            return beta;
        }
        if score > alpha {
           alpha = score;
        }
    }
    return alpha;
}

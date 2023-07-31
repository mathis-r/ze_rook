use std::time::{Duration, Instant};
use ze_rook::{MATEUPPER, MATELOWER};
use std::collections::HashMap;
use crate::BoardState;
use crate::{Prom, Move};
use crate::Board;

pub fn alphabeta(boardstate: &mut BoardState, mut alpha: i32, beta: i32, depth: usize, tt: &mut HashMap<Board,(Move, i32, i32)>, depthmax: &usize, think: &Duration, start: &Instant, move_prv_iter: &Option<Move>) -> (i32,Option<Move>){
    let bestmove: Option<Move>;
    if depth == 0 {
        return (boardstate.evaluate_pos(), None);
    }
    if start.elapsed() > (*think / 125) * 100 {
        match move_prv_iter {
            Some(_mv) => return (999999, None),
            _ => (),
        };
    }
    let mut killer = tt.get(&boardstate.board);
    match killer {
        None => killer = Some(&(Move {from: 0, to: 0, prom: None}, -1, -MATEUPPER)),
        Some((Move { from, to, prom }, killer_depth, killerscore)) => {
            if killer_depth > & (depth as i32)  || killerscore >= &(MATELOWER - 15) {
                alpha = boardstate.evaluate_pos() + Move{ from: *from, to: *to, prom: *prom}.evaluate_mv(boardstate);
                bestmove = Some(Move{ from: *from, to: *to, prom: *prom});
                return (alpha, bestmove);
            }
        }
    }
    let mut moves: Vec<Move> = Vec::new();
    let mut score_m: Vec<i32> = Vec::new();
    let mut move_list = boardstate.gen_move();
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
            Some(&(Move { from, to, prom }, _, _)) => {
                let prom = match prom {
                    Some(prom) => match prom {
                        Prom::Q => "q",
                        Prom::R => "r",
                        Prom::B => "b",
                        Prom::N => "n",
                    },
                    None => " ",
                };
                if from == move_list[i].from && to == move_list[i].to && prom == promi {
                    let killer = move_list.remove(i);
                    move_list.insert(0, killer);
                }
            },
            _ => panic!("killer do not exist anymore"),
        };
    }
    for i in 0..move_list.len() {
        let dest = boardstate.board[move_list[i].to];
        let ori_myc = boardstate.myc;
        let ori_oppc = boardstate.oppc;
        let ep = boardstate.ep;
        let kp = boardstate.kp;
        boardstate.apply_move(&move_list[i]);
        boardstate.rotate();
        if !boardstate.search_check() {
            let scmv = alphabeta(boardstate, -beta, -alpha, depth-1, tt, depthmax, think, start, move_prv_iter);
            let score = -scmv.0;
            match scmv.0 {
                999999 => {
                    boardstate.rotate();
                    boardstate.unmake(&move_list[i], &dest, &ori_myc, &ori_oppc, &ep, &kp);
                    return (999999, None);
                },
                _ => {
                    moves.push(move_list[i]);
                    score_m.push(score);
                    if score >= beta {
                        boardstate.rotate();
                        boardstate.unmake(&move_list[i], &dest, &ori_myc, &ori_oppc, &ep, &kp);
                        return (beta, None);
                    }
                    if score > alpha {
                        alpha = score;
                    }
                },
            }
        }
        boardstate.rotate();
        boardstate.unmake(&move_list[i], &dest, &ori_myc, &ori_oppc, &ep, &kp);
    }
    if score_m.len() == 0 {
        boardstate.rotate();
        if boardstate.search_check() {
            alpha = - MATELOWER + *depthmax as i32 - depth as i32;
            bestmove = None;
        } else {
            alpha = 0;
            bestmove = None;
        }
        boardstate.rotate();
    } else {
            let maxscore = score_m.iter().max();
            let indexmax = score_m.iter().position(|&x| Some(x) == maxscore.copied());
            match indexmax {
                Some(index) => {
                    (*tt).insert(boardstate.board, (moves[index], depth as i32, score_m[index]));
                    bestmove = Some(moves[index]);
                },
                _ => panic!("indexmax n'est pas positif"),
            }
        }
    (alpha, bestmove)
}

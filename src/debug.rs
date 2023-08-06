use crate::piece::Prom;
use crate::BoardState;
use crate::render;

pub fn perft(depth: &usize, depthperft: &usize, boardstate: &mut BoardState) -> usize{
    if *depth == 0 {
        return 1;
    }
    let mut nodes = 0;
    let movelist = boardstate.gen_move();
    for i in 0..movelist.len() {
        let dest = boardstate.board[movelist[i].to];
        let ori_myc = boardstate.myc;
        let ori_oppc = boardstate.oppc;
        let ep = boardstate.ep;
        let kp = boardstate.kp;
        boardstate.apply_move(&movelist[i]);
        //boardstate.gen_captures();
        boardstate.rotate();
        let subnodes = perft(&(depth - 1), depthperft, boardstate);
        boardstate.rotate();
        boardstate.unmake(&movelist[i], &dest, &ori_myc, &ori_oppc, &ep, &kp);
        if depth == depthperft {
            let (mut k, mut j) = (movelist[i].from as i32,movelist[i].to as i32);
            if boardstate.color == 'b' {
                (k, j) = (119 - k, 119 - j);
            }
            let prom = match movelist[i].prom {
                Some(prom) => match prom {
                    Prom::Q => "q",
                    Prom::R => "r",
                    Prom::B => "b",
                    Prom::N => "n",
                }
                None => "",
            };
            let lmove = render(k) + &render(j) + prom;
            println!("{lmove}: {}", subnodes);
        }
        nodes += subnodes;
    }
    nodes
}

#[cfg(test)]
mod tests {
    use crate::BoardState;
    use crate::perft;
    #[test]
    fn perft_test_startpos() {
        let mut boardstate = BoardState::new();
        assert_eq!(perft(&5, &5, &mut boardstate), 4865609);
    }
    #[test]
    fn perft_test_pos2() {
        let mut boardstate = crate::fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string());
        assert_eq!(perft(&4, &4, &mut boardstate), 4085603);
    }
    #[test]
    fn perft_test_pos3() {
        let mut boardstate = crate::fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string());
        assert_eq!(perft(&6, &6, &mut boardstate), 11030083);
    }
    #[test]
    fn perft_test_pos4() {
        let mut boardstate = crate::fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string());
        assert_eq!(perft(&5, &5, &mut boardstate), 15833292);
    }
    #[test]
    fn perft_test_pos5() {
        let mut boardstate = crate::fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string());
        assert_eq!(perft(&4, &4, &mut boardstate), 2103487);
    }
    #[test]
    fn perft_test_pos6() {
        let mut boardstate = crate::fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10".to_string());
        assert_eq!(perft(&4, &4, &mut boardstate), 3894594);
    }
}

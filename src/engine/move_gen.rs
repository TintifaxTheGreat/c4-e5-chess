use crate::misc::types::*;
use cozy_chess::{Board, Move};
use std::ops::Not;

/// A trait to extend the move generator of crate Cozy Chess.
pub trait MoveGenPrime {
    fn get_legal_sorted(&self, old_move: Option<Move>) -> Vec<AnnotatedMove>;
}

impl MoveGenPrime for Board {
    /// Get all legal moves for given position, sort captures first.
    /// En passant captures are not considered.
    /// Also takes a proven good move ("old move") to be sorted first.
    fn get_legal_sorted(&self, old_move: Option<Move>) -> Vec<AnnotatedMove> {
        let mut result: Vec<AnnotatedMove> = Vec::new();
        let enemy_pieces = self.colors(!self.side_to_move());
        let other_squares = enemy_pieces.not();

        self.generate_moves(|moves| {
            let mut captures = moves;
            captures.to &= enemy_pieces;
            for mv in captures {
                result.push(AnnotatedMove {
                    mv,
                    sc: 0,
                    node_count: 0,
                    cp: true,
                });
            }

            let mut others = moves;
            others.to &= other_squares;
            for mv in others {
                result.push(AnnotatedMove {
                    mv,
                    sc: 0,
                    node_count: 0,
                    cp: false,
                });
            }

            false
        });

        if let Some(mv) = old_move {
            for (i, c) in (&mut result.iter()).enumerate() {
                if c.mv == mv {
                    result.swap(0, i);
                    break;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::helpers::parse_epd;
    use std::str::FromStr;

    static TEST_RESOURCE: &[&str] = &[
        "1k1r4/pp1b1R2/3q2pp/4p3/2B5/4Q3/PPP2B2/2K5 b - - bm Qd1+",
        "3r1k2/4npp1/1ppr3p/p6P/P2PPPP1/1NR5/5K2/2R5 w - - bm d5",
        "2q1rr1k/3bbnnp/p2p1pp1/2pPp3/PpP1P1P1/1P2BNNP/2BQ1PRK/7R b - - bm f5",
        "rnbqkb1r/p3pppp/1p6/2ppP3/3N4/2P5/PPP1QPPP/R1B1KB1R w KQkq - bm e6",
        "r1b2rk1/2q1b1pp/p2ppn2/1p6/3QP3/1BN1B3/PPP3PP/R4RK1 w - - bm Nd5 a4",
        "2r3k1/pppR1pp1/4p3/4P1P1/5P2/1P4K1/P1P5/8 w - - bm g6",
        "1nk1r1r1/pp2n1pp/4p3/q2pPp1N/b1pP1P2/B1P2R2/2P1B1PP/R2Q2K1 w - - bm Nf6",
        "4b3/p3kp2/6p1/3pP2p/2pP1P2/4K1P1/P3N2P/8 w - - bm f5",
        "2kr1bnr/pbpq4/2n1pp2/3p3p/3P1P1B/2N2N1Q/PPP3PP/2KR1B1R w - - bm f5",
        "3rr1k1/pp3pp1/1qn2np1/8/3p4/PP1R1P2/2P1NQPP/R1B3K1 b - - bm Ne5",
        "2r1nrk1/p2q1ppp/bp1p4/n1pPp3/P1P1P3/2PBB1N1/4QPPP/R4RK1 w - - bm f4",
        "r3r1k1/ppqb1ppp/8/4p1NQ/8/2P5/PP3PPP/R3R1K1 b - - bm Bf5",
        "r2q1rk1/4bppp/p2p4/2pP4/3pP3/3Q4/PP1B1PPP/R3R1K1 w - - bm b4",
        "rnb2r1k/pp2p2p/2pp2p1/q2P1p2/8/1Pb2NP1/PB2PPBP/R2Q1RK1 w - - bm Qd2",
        "2r3k1/1p2q1pp/2b1pr2/p1pp4/6Q1/1P1PP1R1/P1PN2PP/5RK1 w - - bm Qxg7+",
        "r1bqkb1r/4npp1/p1p4p/1p1pP1B1/8/1B6/PPPN1PPP/R2Q1RK1 w kq - bm Ne4",
        "r2q1rk1/1ppnbppp/p2p1nb1/3Pp3/2P1P1P1/2N2N1P/PPB1QP2/R1B2RK1 b - - bm h5",
        "r1bq1rk1/pp2ppbp/2np2p1/2n5/P3PP2/N1P2N2/1PB3PP/R1B1QRK1 b - - bm Nb3",
        "3rr3/2pq2pk/p2p1pnp/8/2QBPP2/1P6/P5PP/4RRK1 b - - bm Rxe4",
        "r4k2/pb2bp1r/1p1qp2p/3pNp2/3P1P2/2N3P1/PPP1Q2P/2KRR3 w - - bm g4",
        "3rn2k/ppb2rpp/2ppqp2/5N2/2P1P3/1P5Q/PB3PPP/3RR1K1 w - - bm Nh6",
        "2r2rk1/1bqnbpp1/1p1ppn1p/pP6/N1P1P3/P2B1N1P/1B2QPP1/R2R2K1 b - - bm Bxe4",
        "r1bqk2r/pp2bppp/2p5/3pP3/P2Q1P2/2N1B3/1PP3PP/R4RK1 b kq - bm f6",
        "r2qnrnk/p2b2b1/1p1p2pp/2pPpp2/1PP1P3/PRNBB3/3QNPPP/5RK1 w - - bm f4",
        "r1b1kbnr/pp3ppp/2n5/2pq4/Q3p3/2PP1N2/PP2BPPP/RNB1K2R b KQkq - bm exf3",
    ];

    #[test]
    fn test_get_legal_sorted() {
        for i in 0..TEST_RESOURCE.len() {
            let (fen, bestmoves, _) = parse_epd(TEST_RESOURCE[i].to_string());
            let first_move = bestmoves[0].parse::<Move>().unwrap();
            let board = Board::from_str(fen.as_str()).unwrap();
            let mut moves_length = 0;
            board.generate_moves(|moves| {
                moves_length += moves.len() as u64;
                false
            });
            let moves_sorted = board.get_legal_sorted(Some(first_move));
            assert_eq!(moves_length, moves_sorted.len() as u64);
            assert_eq!(moves_sorted[0].mv, first_move);
        }
    }
}

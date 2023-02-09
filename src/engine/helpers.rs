use chess::Color;
use chess::Piece;
use chess::{BitBoard, Board};

fn north_fill(p: u64) -> u64 {
    let mut result = p | (p << 8);
    result = result | (result << 16);
    return result | (result << 32);
}

fn south_fill(p: u64) -> u64 {
    let mut result = p | (p >> 8);
    result = result | (result >> 16);
    return result | (result >> 32);
}

fn file_fill(p: u64) -> u64 {
    return north_fill(p) | south_fill(p);
}

fn open_files(b: Board) -> u64 {
    let pawns = b.pieces(Piece::Pawn).0;
    return !file_fill(pawns);
}

fn half_open_files(b: Board) -> u64 {
    let white = file_fill(b.pieces(Piece::Pawn).0 & b.color_combined(Color::White).0);
    let black = file_fill(b.pieces(Piece::Pawn).0 & b.color_combined(Color::Black).0);
    return (white & !black) | (!white & black);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_north_fill() {
        assert_eq!(north_fill(17179869184), 289360691284934656);
    }

    #[test]
    fn test_south_fill() {
        assert_eq!(south_fill(17179869184), 17247241220);
    }
    #[test]
    fn test_file_fill() {
        assert_eq!(file_fill(17179869184), 289360691352306692);
    }

    #[test]
    fn test_open_files() {
        let board = Board::from_fen(String::from(
            "rnbqkbnr/p1ppp1p1/8/8/8/8/P1P1PPP1/RNBQKBNR w KQkq - 0 1",
        ));
        if board.is_some() {
            assert_eq!(open_files(board.unwrap()), 0x8282828282828282);
        }
    }

    #[test]
    fn test_half_open_files() {
        let board = Board::from_fen(String::from(
            "rnbqkbnr/p1ppp1p1/8/8/8/8/P1P1PPP1/RNBQKBNR w KQkq - 0 1",
        ));
        if board.is_some() {
            assert_eq!(half_open_files(board.unwrap()), 0x2828282828282828);
        }
    }
}

/*



func halfOpenFiles(b *dragontoothmg.Board) uint64 {
    fw := fileFill(b.White.Pawns)
    fb := fileFill(b.Black.Pawns)
    return (fw & ^fb) | (^fw & fb)
}

func countFiguresMoves(b *dragontoothmg.Board, fig uint64) int {
    count := 0
    square := uint8(bits.TrailingZeros64(fig))

    moves := b.GenerateLegalMoves()
    for _, move := range moves {
        if move.From() == square {
            count++
        }
    }

    return count
}

func distance(x uint64, y uint64) int {
    xLz := bits.LeadingZeros64(x)
    yLz := bits.LeadingZeros64(y)
    fx := xLz % 8
    fy := yLz % 8
    rx := xLz / 8
    ry := yLz / 8

    fD := fy - fx
    if fD < 0 {
        fD = -fD
    }

    rD := ry - rx
    if rD < 0 {
        rD = -rD
    }

    if rD < fD {
        return fD
    }
    return rD
}

func generateMovesPrime(b *dragontoothmg.Board) []dragontoothmg.Move {
    var captures []dragontoothmg.Move
    var nonCaptures []dragontoothmg.Move
    moves := b.GenerateLegalMoves()
    for _, m := range moves {
        if dragontoothmg.IsCapture(m, b) {
            captures = append(captures, m)
        } else {
            nonCaptures = append(nonCaptures, m)
        }
    }

    return append(captures, nonCaptures...)
}

func testCapture(m dragontoothmg.Move, b *dragontoothmg.Board) bool {
    bb := (uint64(1) << m.To())
    return (bb&b.White.All != 0) || (bb&b.Black.All != 0)
}

*/

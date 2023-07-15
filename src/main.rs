use tardigrade::{
    analyzer::MoveGenerator,
    board::{Board, Color, Move, MoveType},
};
fn main() {
    let mut board =
        Board::from_fen("r1bqk2r/ppppbpp1/2n2n1p/4p3/4P3/2NPBQ2/PPP2PPP/R3KBNR w KQkq - 0 6");
    board.print();

    let mut all_moves: Vec<Move> = Vec::new();
    for row in board.board {
        for sq in row {
            if sq.piece.is_none() {
                continue;
            }

            if sq.piece.unwrap().1 == Color::Black {
                continue;
            }
            let mut moves = MoveGenerator::gen_pseudo_moves(&sq, &board);
            all_moves.append(&mut moves);
        }
    }

    println!("All moves: {}", all_moves.len());
    for mov in all_moves.iter() {
        mov.print();
    }

    let mov = Move::new("e1", "c1", MoveType::CastleQueenSide);
    board.move_piece(&mov);
    board.print();

    let mov = Move::new("a8", "b8", MoveType::Normal);
    board.move_piece(&mov);
    board.print();

    let mov = Move::new("b2", "b3", MoveType::Normal);
    board.move_piece(&mov);

    let mov = Move::new("e8", "g8", MoveType::CastleKingSide);
    board.move_piece(&mov);
    board.print();
}

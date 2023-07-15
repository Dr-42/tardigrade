use tardigrade::{
    analyzer::MoveGenerator,
    board::{Board, Color, Move},
};
fn main() {
    let board =
        Board::from_fen("r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4");
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
}

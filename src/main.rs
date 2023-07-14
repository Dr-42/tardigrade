use tardigrade::{
    analyzer::MoveGenerator,
    board::{Board, Color, Move},
};
fn main() {
    let board = Board::from_fen("rnbqkbnr/ppp2ppp/3p4/4pP2/8/8/PPPPP1PP/RNBQKBNR w KQkq e6 0 3");
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

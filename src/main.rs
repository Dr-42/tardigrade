use tardigrade::board::{Board, Color, Piece};
fn main() {
    let mut board = Board::new();
    board.place_piece("a1", Some((Piece::Rook, Color::White)));
    board.place_piece("b1", Some((Piece::Knight, Color::White)));
    board.place_piece("c1", Some((Piece::Bishop, Color::White)));
    board.place_piece("d1", Some((Piece::Queen, Color::White)));
    board.place_piece("e1", Some((Piece::King, Color::White)));
    board.place_piece("f1", Some((Piece::Bishop, Color::White)));
    board.place_piece("g1", Some((Piece::Knight, Color::White)));
    board.place_piece("h1", Some((Piece::Rook, Color::White)));

    board.place_piece("a2", Some((Piece::Pawn, Color::White)));
    board.place_piece("b2", Some((Piece::Pawn, Color::White)));
    board.place_piece("c2", Some((Piece::Pawn, Color::White)));
    board.place_piece("d2", Some((Piece::Pawn, Color::White)));
    board.place_piece("e2", Some((Piece::Pawn, Color::White)));
    board.place_piece("f2", Some((Piece::Pawn, Color::White)));
    board.place_piece("g2", Some((Piece::Pawn, Color::White)));
    board.place_piece("h2", Some((Piece::Pawn, Color::White)));

    board.place_piece("a8", Some((Piece::Rook, Color::Black)));
    board.place_piece("b8", Some((Piece::Knight, Color::Black)));
    board.place_piece("c8", Some((Piece::Bishop, Color::Black)));
    board.place_piece("d8", Some((Piece::Queen, Color::Black)));
    board.place_piece("e8", Some((Piece::King, Color::Black)));
    board.place_piece("f8", Some((Piece::Bishop, Color::Black)));
    board.place_piece("g8", Some((Piece::Knight, Color::Black)));
    board.place_piece("h8", Some((Piece::Rook, Color::Black)));

    board.place_piece("a7", Some((Piece::Pawn, Color::Black)));
    board.place_piece("b7", Some((Piece::Pawn, Color::Black)));
    board.place_piece("c7", Some((Piece::Pawn, Color::Black)));
    board.place_piece("d7", Some((Piece::Pawn, Color::Black)));
    board.place_piece("e7", Some((Piece::Pawn, Color::Black)));
    board.place_piece("f7", Some((Piece::Pawn, Color::Black)));
    board.place_piece("g7", Some((Piece::Pawn, Color::Black)));
    board.place_piece("h7", Some((Piece::Pawn, Color::Black)));
    board.print();

    board.move_piece("e2", "e4");
    board.print();
}

use crate::board::{Board, Color, Move, MoveType, Piece, Square};

pub struct Analyzer {}
impl Analyzer {
    pub fn check_legal(mov: &Move, board: &Board) -> bool {
        let piece = board.get_square(&mov.from).piece;
        if piece.is_none() {
            return false;
        }

        let piece = piece.unwrap();
        let piece_type = piece.0;
        let piece_color = piece.1;

        let to_piece = board.get_square(&mov.to).piece;
        if to_piece.is_some() && to_piece.unwrap().1 == piece_color {
            return false;
        }

        let result = match piece_type {
            Piece::Pawn => Analyzer::check_pawn_legal(mov, &piece_color, board),
            Piece::Knight => Analyzer::check_knight_legal(mov, &piece_color, board),
            Piece::Bishop => Analyzer::check_bishop_legal(mov, &piece_color, board),
            Piece::Rook => Analyzer::check_rook_legal(mov, &piece_color, board),
            Piece::Queen => Analyzer::check_queen_legal(mov, &piece_color, board),
            Piece::King => Analyzer::check_king_legal(mov, &piece_color, board),
        };
        return result;
    }

    #[allow(unused_variables)]
    fn check_pawn_legal(mov: &Move, col: &Color, board: &Board) -> bool {
        todo!();
    }
    #[allow(unused_variables)]
    fn check_knight_legal(mov: &Move, col: &Color, board: &Board) -> bool {
        todo!();
    }
    #[allow(unused_variables)]
    fn check_bishop_legal(mov: &Move, col: &Color, board: &Board) -> bool {
        todo!();
    }
    #[allow(unused_variables)]
    fn check_rook_legal(mov: &Move, col: &Color, board: &Board) -> bool {
        todo!();
    }
    #[allow(unused_variables)]
    fn check_queen_legal(mov: &Move, col: &Color, board: &Board) -> bool {
        todo!();
    }
    #[allow(unused_variables)]
    fn check_king_legal(mov: &Move, col: &Color, board: &Board) -> bool {
        todo!();
    }
}

pub struct MoveGenerator {}

impl MoveGenerator {
    pub fn gen_pseudo_moves(square: &Square, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        if square.piece.is_some() {
            let piece = square.piece.unwrap().0;
            let pseudo_moves = match piece {
                Piece::Pawn => MoveGenerator::gen_pawn_pseudo_moves(&square, board),
                Piece::Knight => MoveGenerator::gen_knight_pseudo_moves(&square, board),
                Piece::Bishop => MoveGenerator::gen_bishop_pseudo_moves(&square, board),
                Piece::Rook => MoveGenerator::gen_rook_pseudo_moves(&square, board),
                Piece::Queen => MoveGenerator::gen_queen_pseudo_moves(&square, board),
                Piece::King => MoveGenerator::gen_king_pseudo_moves(&square, board),
            };
            moves.extend(pseudo_moves);
        }
        return moves;
    }

    fn gen_pawn_pseudo_moves(square: &Square, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut move_dir = 1;
        if square.piece.is_none() {
            return moves;
        }
        let col = &square.piece.unwrap().1;

        if *col == Color::Black {
            move_dir = -1;
        }

        let mut move_to = square.clone();
        let new_y = move_to.y as i32 + move_dir;
        if new_y < 8 && new_y >= 0 {
            move_to.y = new_y as usize;
            if board.get_square(&move_to.to_string()).piece.is_none() {
                if (new_y == 7 && *col == Color::White) || (new_y == 0 && *col == Color::Black) {
                    for ty in Piece::iter() {
                        if ty == Piece::Pawn {
                            continue;
                        }
                        let mut mv = Move::from_indices(square, &move_to, MoveType::Promotion);
                        mv.promotion = Some(ty);
                        moves.push(mv);
                    }
                } else {
                    moves.push(Move::from_indices(square, &move_to, MoveType::Normal));
                }
            }
        }

        if square.y == 1 && *col == Color::White || square.y == 6 && *col == Color::Black {
            move_to.y = (move_to.y as i32 + move_dir) as usize;
            if board.get_square(&move_to.to_string()).piece.is_none() {
                moves.push(Move::from_indices(square, &move_to, MoveType::PawnDouble));
            }
        }

        let mut move_to = square.clone();
        let new_y = move_to.y as i32 + move_dir;
        let new_x = move_to.x as i32 + 1;

        if new_x < 8 && new_y < 8 && new_x >= 0 && new_y >= 0 {
            move_to.y = new_y as usize;
            move_to.x = new_x as usize;
            if board.get_square(&move_to.to_string()).piece.is_some()
                && board.get_square(&move_to.to_string()).piece.unwrap().1 != *col
            {
                moves.push(Move::from_indices(square, &move_to, MoveType::Capture));
            }
        }

        let mut move_to = square.clone();
        let new_y = move_to.y as i32 + move_dir;
        let new_x = move_to.x as i32 - 1;
        if new_x < 8 && new_y < 8 && new_x >= 0 && new_y >= 0 {
            move_to.y = new_y as usize;
            move_to.x = new_x as usize;
            if board.get_square(&move_to.to_string()).piece.is_some()
                && board.get_square(&move_to.to_string()).piece.unwrap().1 != *col
            {
                moves.push(Move::from_indices(square, &move_to, MoveType::Capture));
            }
        }

        if board.en_passant_square.is_some() {
            let en_passant_square = board.en_passant_square.unwrap();
            if (square.x == en_passant_square.0 - 1) || (square.x == en_passant_square.0 + 1) {
                let allowed_y = match col {
                    Color::White => 4,
                    Color::Black => 3,
                };

                if square.y == allowed_y {
                    let mut move_to = square.clone();
                    move_to.x = en_passant_square.0;
                    move_to.y = en_passant_square.1;
                    moves.push(Move::from_indices(square, &move_to, MoveType::EnPassant));
                }
            }
        }

        moves
    }

    fn gen_knight_pseudo_moves(square: &Square, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        if square.piece.is_none() {
            return moves;
        }
        let col = &square.piece.unwrap().1;
        for x in -2..=2 as i32 {
            for y in -2..=2 as i32 {
                if x.abs() + y.abs() == 3 {
                    let mut move_to = square.clone();
                    let new_x = move_to.x as i32 + x;
                    let new_y = move_to.y as i32 + y;

                    if new_x > 7 || new_y > 7 || new_x < 0 || new_y < 0 {
                        continue;
                    }

                    move_to.x = new_x as usize;
                    move_to.y = new_y as usize;

                    let mut valid: bool;
                    let mut captured = false;
                    valid = board.get_square(&move_to.to_string()).piece.is_none();
                    if !valid {
                        if board.get_square(&move_to.to_string()).piece.unwrap().1 != *col {
                            valid = true;
                            captured = true;
                        }
                    }

                    if valid {
                        if captured {
                            moves.push(Move::from_indices(square, &move_to, MoveType::Capture));
                        } else {
                            moves.push(Move::from_indices(square, &move_to, MoveType::Normal));
                        }
                    }
                }
            }
        }

        moves
    }

    fn gen_bishop_pseudo_moves(square: &Square, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        if square.piece.is_none() {
            return moves;
        }
        let dirs = vec![(1, 1), (-1, -1), (-1, 1), (1, -1)];
        moves.extend(MoveGenerator::gen_straight_pseudos(dirs, square, board));
        moves
    }

    fn gen_rook_pseudo_moves(square: &Square, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        if square.piece.is_none() {
            return moves;
        }
        let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        moves.extend(MoveGenerator::gen_straight_pseudos(dirs, square, board));
        moves
    }

    fn gen_queen_pseudo_moves(square: &Square, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        if square.piece.is_none() {
            return moves;
        }
        let dirs = vec![
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
        ];
        moves.extend(MoveGenerator::gen_straight_pseudos(dirs, square, board));
        moves
    }

    fn gen_king_pseudo_moves(square: &Square, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        if square.piece.is_none() {
            return moves;
        }
        let col = &square.piece.unwrap().1;

        for x in -1..=1 as i32 {
            for y in -1..=1 as i32 {
                if x.abs() + y.abs() != 0 {
                    let mut move_to = square.clone();
                    let new_x = move_to.x as i32 + x;
                    let new_y = move_to.y as i32 + y;

                    if new_x > 7 || new_y > 7 || new_x < 0 || new_y < 0 {
                        continue;
                    }

                    move_to.x = new_x as usize;
                    move_to.y = new_y as usize;

                    let mut valid: bool;
                    let mut captured = false;
                    valid = board.get_square(&move_to.to_string()).piece.is_none();
                    if !valid {
                        if board.get_square(&move_to.to_string()).piece.unwrap().1 != *col {
                            valid = true;
                            captured = true;
                        }
                    }

                    if valid {
                        if captured {
                            moves.push(Move::from_indices(square, &move_to, MoveType::Capture));
                        } else {
                            moves.push(Move::from_indices(square, &move_to, MoveType::Normal));
                        }
                    }
                }
            }
        }

        moves
    }

    fn gen_straight_pseudos(dirs: Vec<(i32, i32)>, square: &Square, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        let col = &square.piece.unwrap().1;
        for dir in dirs {
            let mut finished_dir = false;
            for s in 1..=7 {
                let mut move_to = square.clone();
                let new_x = move_to.x as i32 + dir.0 * s;
                let new_y = move_to.y as i32 + dir.1 * s;

                if new_x > 7 || new_y > 7 || new_x < 0 || new_y < 0 {
                    continue;
                }

                move_to.x = new_x as usize;
                move_to.y = new_y as usize;

                let mut valid: bool;
                let mut captured = false;
                valid = board.get_square(&move_to.to_string()).piece.is_none();
                if !valid {
                    if board.get_square(&move_to.to_string()).piece.unwrap().1 != *col {
                        valid = true;
                        captured = true;
                    }
                }

                if valid {
                    if captured {
                        moves.push(Move::from_indices(square, &move_to, MoveType::Capture));
                    } else {
                        moves.push(Move::from_indices(square, &move_to, MoveType::Normal));
                    }
                }

                if board.get_square(&move_to.to_string()).piece.is_some() {
                    finished_dir = true;
                    break;
                }
            }
            if finished_dir {
                continue;
            }
        }
        moves
    }
}

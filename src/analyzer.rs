use crate::board::{Board, Color, Move, MoveType, Piece, Square};

pub struct Analyzer {}
impl Analyzer {
    pub fn check_legal(mov: &Move, board: &Board) -> bool {
        let piece = board.get_square(&mov.from).piece;
        if piece.is_none() {
            return false;
        }

        let mut cl_board = board.clone();
        cl_board.move_piece(mov, false);

        if Analyzer::is_king_in_check(&cl_board) {
            return false;
        }

        let mut all_moves = Vec::new();
        for row in cl_board.board {
            for sq in row {
                if sq.piece.is_none() {
                    continue;
                }

                if sq.piece.unwrap().1 == board.to_move {
                    continue;
                }
                let mut moves = MoveGenerator::gen_pseudo_moves(&sq, &cl_board);
                all_moves.append(&mut moves);
            }
        }

        for m in all_moves {
            if m.mov_type == MoveType::Capture {
                if cl_board.get_square(&m.to).piece.is_some() {
                    if cl_board.get_square(&m.to).piece.unwrap().0 == Piece::King {
                        return false;
                    }
                }
            }
        }

        if mov.mov_type == MoveType::CastleKingSide {
            let mut cl_board = board.clone();
            if board.to_move == Color::White {
                cl_board.place_piece("f1", Some((Piece::King, Color::White)));
                cl_board.place_piece("g1", Some((Piece::King, Color::White)));
                let mut all_moves = Vec::new();
                for row in cl_board.board {
                    for sq in row {
                        if sq.piece.is_none() {
                            continue;
                        }

                        if sq.piece.unwrap().1 == board.to_move {
                            continue;
                        }
                        let mut moves = MoveGenerator::gen_pseudo_moves(&sq, &cl_board);
                        all_moves.append(&mut moves);
                    }
                }

                for m in all_moves {
                    if m.mov_type == MoveType::Capture {
                        if cl_board.get_square(&m.to).piece.is_some() {
                            if cl_board.get_square(&m.to).piece.unwrap().0 == Piece::King {
                                return false;
                            }
                        }
                    }
                }
            } else {
                cl_board.place_piece("f8", Some((Piece::King, Color::Black)));
                cl_board.place_piece("g8", Some((Piece::King, Color::Black)));
                let mut all_moves = Vec::new();
                for row in cl_board.board {
                    for sq in row {
                        if sq.piece.is_none() {
                            continue;
                        }

                        if sq.piece.unwrap().1 == board.to_move {
                            continue;
                        }
                        let mut moves = MoveGenerator::gen_pseudo_moves(&sq, &cl_board);
                        all_moves.append(&mut moves);
                    }
                }

                for m in all_moves {
                    if m.mov_type == MoveType::Capture {
                        if cl_board.get_square(&m.to).piece.is_some() {
                            if cl_board.get_square(&m.to).piece.unwrap().0 == Piece::King {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        if mov.mov_type == MoveType::CastleQueenSide {
            let mut cl_board = board.clone();
            if board.to_move == Color::White {
                cl_board.place_piece("d1", Some((Piece::King, Color::White)));
                cl_board.place_piece("c1", Some((Piece::King, Color::White)));
                let mut all_moves = Vec::new();
                for row in cl_board.board {
                    for sq in row {
                        if sq.piece.is_none() {
                            continue;
                        }

                        if sq.piece.unwrap().1 == board.to_move {
                            continue;
                        }
                        let mut moves = MoveGenerator::gen_pseudo_moves(&sq, &cl_board);
                        all_moves.append(&mut moves);
                    }
                }

                for m in all_moves {
                    if m.mov_type == MoveType::Capture {
                        if cl_board.get_square(&m.to).piece.is_some() {
                            if cl_board.get_square(&m.to).piece.unwrap().0 == Piece::King {
                                return false;
                            }
                        }
                    }
                }
            } else {
                cl_board.place_piece("d8", Some((Piece::King, Color::Black)));
                cl_board.place_piece("c8", Some((Piece::King, Color::Black)));
                let mut all_moves = Vec::new();
                for row in cl_board.board {
                    for sq in row {
                        if sq.piece.is_none() {
                            continue;
                        }

                        if sq.piece.unwrap().1 == board.to_move {
                            continue;
                        }
                        let mut moves = MoveGenerator::gen_pseudo_moves(&sq, &cl_board);
                        all_moves.append(&mut moves);
                    }
                }

                for m in all_moves {
                    if m.mov_type == MoveType::Capture {
                        if cl_board.get_square(&m.to).piece.is_some() {
                            if cl_board.get_square(&m.to).piece.unwrap().0 == Piece::King {
                                return false;
                            }
                        }
                    }
                }
            }
        }

        return true;
    }

    pub fn is_king_in_check(board: &Board) -> bool {
        let mut all_moves = Vec::new();
        for row in board.board {
            for sq in row {
                if sq.piece.is_none() {
                    continue;
                }

                if sq.piece.unwrap().1 != board.to_move {
                    continue;
                }
                let mut moves = MoveGenerator::gen_pseudo_moves(&sq, board);
                all_moves.append(&mut moves);
            }
        }

        for m in all_moves {
            if m.mov_type == MoveType::Capture {
                if board.get_square(&m.to).piece.is_some() {
                    if board.get_square(&m.to).piece.unwrap().0 == Piece::King {
                        return true;
                    }
                }
            }
        }

        return false;
    }
}

pub struct MoveGenerator {}

impl MoveGenerator {
    pub fn gen_legal_moves(board: &Board) -> Vec<Move> {
        let mut all_moves = Vec::new();
        for row in board.board {
            for sq in row {
                if sq.piece.is_none() {
                    continue;
                }

                if sq.piece.unwrap().1 != board.to_move {
                    continue;
                }
                let mut moves = MoveGenerator::gen_pseudo_moves(&sq, board);
                all_moves.append(&mut moves);
            }
        }

        let mut legal_moves = Vec::new();
        for m in all_moves {
            if Analyzer::check_legal(&m, board) {
                legal_moves.push(m);
            }
        }

        legal_moves
    }

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
            let mut move_to1 = move_to.clone();
            move_to1.y = (move_to1.y as i32 - move_dir) as usize;
            if board.get_square(&move_to.to_string()).piece.is_none()
                && board.get_square(&move_to1.to_string()).piece.is_none()
            {
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
            let en_passant_square = (en_passant_square.1, en_passant_square.0);
            let en_pass_allowed: bool;
            if en_passant_square.0 == 0 {
                en_pass_allowed = square.x == en_passant_square.0 + 1;
            } else if en_passant_square.0 == 7 {
                en_pass_allowed = square.x == en_passant_square.0 - 1;
            } else {
                en_pass_allowed =
                    (square.x == en_passant_square.0 - 1) || (square.x == en_passant_square.0 + 1);
            }

            if en_pass_allowed {
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

        if square.y == 0 && square.x == 4 && *col == Color::White {
            if board.white_king_side_castle {
                let mut move_to = square.clone();
                move_to.x = 6;

                if board.get_square("f1").piece.is_none()
                    && board.get_square("g1").piece.is_none()
                    && board.get_square("h1").piece.is_some()
                    && board.get_square("h1").piece.unwrap().0 == Piece::Rook
                    && board.get_square("h1").piece.unwrap().1 == Color::White
                {
                    moves.push(Move::from_indices(
                        square,
                        &move_to,
                        MoveType::CastleKingSide,
                    ));
                }
            }
            if board.white_queen_side_castle {
                let mut move_to = square.clone();
                move_to.x = 2;

                if board.get_square("b1").piece.is_none()
                    && board.get_square("c1").piece.is_none()
                    && board.get_square("d1").piece.is_none()
                    && board.get_square("a1").piece.is_some()
                    && board.get_square("a1").piece.unwrap().0 == Piece::Rook
                    && board.get_square("a1").piece.unwrap().1 == Color::White
                {
                    moves.push(Move::from_indices(
                        square,
                        &move_to,
                        MoveType::CastleQueenSide,
                    ));
                }
            }
        }

        if square.y == 7 && square.x == 4 && *col == Color::Black {
            if board.black_king_side_castle {
                let mut move_to = square.clone();
                move_to.x = 6;

                if board.get_square("f8").piece.is_none()
                    && board.get_square("g8").piece.is_none()
                    && board.get_square("h8").piece.is_some()
                    && board.get_square("h8").piece.unwrap().0 == Piece::Rook
                    && board.get_square("h8").piece.unwrap().1 == Color::Black
                {
                    moves.push(Move::from_indices(
                        square,
                        &move_to,
                        MoveType::CastleKingSide,
                    ));
                }
            }
            if board.black_queen_side_castle {
                let mut move_to = square.clone();
                move_to.x = 2;

                if board.get_square("b8").piece.is_none()
                    && board.get_square("c8").piece.is_none()
                    && board.get_square("d8").piece.is_none()
                    && board.get_square("a8").piece.is_some()
                    && board.get_square("a8").piece.unwrap().0 == Piece::Rook
                    && board.get_square("a8").piece.unwrap().1 == Color::Black
                {
                    moves.push(Move::from_indices(
                        square,
                        &move_to,
                        MoveType::CastleQueenSide,
                    ));
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

pub struct Evaluator {}

#[derive(Debug)]
pub struct PerftResult {
    pub nodes: u64,
    pub captures: u64,
    pub en_passants: u64,
    pub castles: u64,
    pub promotions: u64,
}

impl Evaluator {
    pub fn perft(board: &Board, depth: u64, print: bool) -> PerftResult {
        let mut nodes = 0;
        let mut captures = 0;
        let mut en_passants = 0;
        let mut castles = 0;
        let mut promotions = 0;

        if depth == 0 {
            return PerftResult {
                nodes: 1,
                captures: 0,
                en_passants: 0,
                castles: 0,
                promotions: 0,
            };
        }

        let mut perft_prints = Vec::new();
        let moves = MoveGenerator::gen_legal_moves(board);
        for m in moves {
            let mut cl_board = board.clone();
            cl_board.move_piece(&m, true);
            let res = Evaluator::perft(&cl_board, depth - 1, false);

            if m.mov_type == MoveType::Capture {
                captures += res.nodes;
            } else if m.mov_type == MoveType::EnPassant {
                en_passants += res.nodes;
            } else if m.mov_type == MoveType::CastleKingSide
                || m.mov_type == MoveType::CastleQueenSide
            {
                castles += res.nodes;
            } else if m.mov_type == MoveType::Promotion {
                promotions += res.nodes;
            }

            nodes += res.nodes;
            captures += res.captures;
            en_passants += res.en_passants;
            castles += res.castles;
            promotions += res.promotions;

            if print {
                perft_prints.push(format!("{}: {}", m.to_string(), res.nodes));
            }
        }

        perft_prints.sort();
        for p in &perft_prints {
            println!("{}", p);
        }
        if print {
            println!("Nodes: {}", nodes);
        }
        PerftResult {
            nodes,
            captures,
            en_passants,
            castles,
            promotions,
        }
    }
}

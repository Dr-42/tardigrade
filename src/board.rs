use crate::analyzer::Analyzer;
use colored::Colorize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn iter() -> impl Iterator<Item = Piece> {
        [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ]
        .iter()
        .copied()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub piece: Option<(Piece, Color)>,
    pub x: usize,
    pub y: usize,
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Square {
    pub fn to_string(&self) -> String {
        format!(
            "{}{}",
            (self.x + 97) as u8 as char,
            (self.y + 49) as u8 as char
        )
    }
}

#[derive(Copy, Clone)]
pub struct Board {
    pub board: [[Square; 8]; 8],
    pub to_move: Color,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,

    pub white_king_side_castle: bool,
    pub white_queen_side_castle: bool,
    pub black_king_side_castle: bool,
    pub black_queen_side_castle: bool,

    pub en_passant_square: Option<(usize, usize)>,
}

impl Board {
    pub fn new() -> Board {
        let mut board = [[Square {
            piece: None,
            x: 0,
            y: 0,
        }; 8]; 8];

        for (r, row) in board.iter_mut().enumerate() {
            for (c, square) in row.iter_mut().enumerate() {
                square.x = c;
                square.y = r;
            }
        }

        let board = Board {
            board: board,
            to_move: Color::White,
            halfmove_clock: 0,
            fullmove_number: 1,

            white_king_side_castle: true,
            white_queen_side_castle: true,
            black_king_side_castle: true,
            black_queen_side_castle: true,

            en_passant_square: None,
        };

        board
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::new();
        let mut fen = fen.split_whitespace();
        let board_str = fen.next().unwrap();
        let to_move = fen.next().unwrap();
        let castle = fen.next().unwrap();
        let en_passant = fen.next().unwrap();
        let halfmove_clock = fen.next().unwrap();
        let fullmove_number = fen.next().unwrap();
        let board_str: Vec<&str> = board_str.split("/").collect();

        for row in 0..=7 {
            let mut col = 0;
            for c in board_str[7 - row].chars() {
                if c.is_digit(10) {
                    col += c.to_digit(10).unwrap() as usize;
                } else {
                    let piece = match c {
                        'p' => Some((Piece::Pawn, Color::Black)),
                        'n' => Some((Piece::Knight, Color::Black)),
                        'b' => Some((Piece::Bishop, Color::Black)),
                        'r' => Some((Piece::Rook, Color::Black)),
                        'q' => Some((Piece::Queen, Color::Black)),
                        'k' => Some((Piece::King, Color::Black)),
                        'P' => Some((Piece::Pawn, Color::White)),
                        'N' => Some((Piece::Knight, Color::White)),
                        'B' => Some((Piece::Bishop, Color::White)),
                        'R' => Some((Piece::Rook, Color::White)),
                        'Q' => Some((Piece::Queen, Color::White)),
                        'K' => Some((Piece::King, Color::White)),
                        _ => None,
                    };
                    board.set_piece((row, col), piece);
                    col += 1;
                }
            }
        }

        if to_move == "w" {
            board.to_move = Color::White;
        } else {
            board.to_move = Color::Black;
        }

        board.black_king_side_castle = false;
        board.black_queen_side_castle = false;
        board.white_king_side_castle = false;
        board.white_queen_side_castle = false;

        for c in castle.chars() {
            match c {
                'K' => board.white_king_side_castle = true,
                'Q' => board.white_queen_side_castle = true,
                'k' => board.black_king_side_castle = true,
                'q' => board.black_queen_side_castle = true,
                _ => (),
            }
        }

        if en_passant != "-" {
            board.en_passant_square = Some(en_passant.to_board_indices());
        }

        board.halfmove_clock = halfmove_clock.parse::<u8>().unwrap();
        board.fullmove_number = fullmove_number.parse::<u8>().unwrap();

        board
    }

    pub fn print(&self) {
        for (r, row) in self.board.iter().rev().enumerate() {
            for (c, square) in row.iter().enumerate() {
                let to_print: &str = match square.piece {
                    Some((Piece::Pawn, Color::White)) => "♙ ",
                    Some((Piece::Knight, Color::White)) => "♘ ",
                    Some((Piece::Bishop, Color::White)) => "♗ ",
                    Some((Piece::Rook, Color::White)) => "♖ ",
                    Some((Piece::Queen, Color::White)) => "♕ ",
                    Some((Piece::King, Color::White)) => "♔ ",
                    Some((Piece::Pawn, Color::Black)) => "♟︎ ",
                    Some((Piece::Knight, Color::Black)) => "♞ ",
                    Some((Piece::Bishop, Color::Black)) => "♝ ",
                    Some((Piece::Rook, Color::Black)) => "♜ ",
                    Some((Piece::Queen, Color::Black)) => "♛ ",
                    Some((Piece::King, Color::Black)) => "♚ ",
                    None => "  ",
                };
                if (r + c) % 2 == 0 {
                    if square.piece.is_some() && square.piece.unwrap().1 == Color::White {
                        print!("{}", to_print.white().on_yellow());
                    } else {
                        print!("{}", to_print.black().on_yellow());
                    }
                } else {
                    if square.piece.is_some() && square.piece.unwrap().1 == Color::White {
                        print!("{}", to_print.white().on_green());
                    } else {
                        print!("{}", to_print.black().on_green());
                    }
                }
            }
            println!();
        }
        println!();

        println!(
            "Turn: {:?}, Enpassant: {:?}, HMC: {} FMC: {}",
            self.to_move, self.en_passant_square, self.halfmove_clock, self.fullmove_number
        );
        println!(
            "WKC: {}, WQC: {}, BKC: {}, BQC:{}",
            self.white_king_side_castle,
            self.white_queen_side_castle,
            self.black_king_side_castle,
            self.black_queen_side_castle
        );
    }

    pub fn set_piece(&mut self, (x, y): (usize, usize), piece: Option<(Piece, Color)>) {
        self.board[x][y].piece = piece;
    }

    pub fn place_piece(&mut self, square: &str, piece: Option<(Piece, Color)>) {
        let indices = square.to_board_indices();
        let indices = (indices.1, indices.0);
        self.set_piece(indices, piece);
    }

    pub fn move_piece(&mut self, mov: &Move, _check_legal: bool) {
        if _check_legal {
            if !Analyzer::check_legal(mov, &self) {
                println!("Illegal move!");
                return;
            }
        }

        if mov.mov_type == MoveType::CastleKingSide || mov.mov_type == MoveType::CastleQueenSide {
            if self.to_move == Color::White {
                self.white_king_side_castle = false;
                self.white_queen_side_castle = false;
            } else {
                self.black_king_side_castle = false;
                self.black_queen_side_castle = false;
            }
        }

        if self.get_square(&mov.from).piece.unwrap().0 == Piece::King {
            if self.to_move == Color::White {
                self.white_king_side_castle = false;
                self.white_queen_side_castle = false;
            } else {
                self.black_king_side_castle = false;
                self.black_queen_side_castle = false;
            }
        }

        if self.get_square(&mov.from).piece.unwrap().0 == Piece::Rook {
            if self.to_move == Color::White {
                if &mov.from == "a1" {
                    self.white_queen_side_castle = false;
                } else {
                    self.white_king_side_castle = false;
                }
            } else {
                if &mov.from == "a8" {
                    self.black_queen_side_castle = false;
                } else {
                    self.black_king_side_castle = false;
                }
            }
        }

        let (from, to) = mov.get_indices();
        if mov.mov_type == MoveType::EnPassant {
            let (y, x) = self.en_passant_square.unwrap();
            if y == 5 {
                self.set_piece((4, x), None);
            } else {
                self.set_piece((3, x), None);
            }
        }
        if mov.mov_type == MoveType::PawnDouble {
            let dx: i32;
            if self.to_move == Color::White {
                dx = 1;
            } else {
                dx = -1;
            }
            self.en_passant_square = Some(((from.0 as i32 + dx) as usize, from.1 as usize));
        } else {
            self.en_passant_square = None;
        }
        if mov.mov_type == MoveType::CastleKingSide {
            if self.to_move == Color::White {
                if self.get_square("f1").piece.is_none()
                    && self.get_square("g1").piece.is_none()
                    && self.get_square("h1").piece.is_some()
                    && self.get_square("h1").piece.unwrap().0 == Piece::Rook
                    && self.get_square("h1").piece.unwrap().1 == Color::White
                {
                    self.place_piece("f1", Some((Piece::Rook, Color::White)));
                    self.place_piece("h1", None);
                }
            } else {
                if self.get_square("f8").piece.is_none()
                    && self.get_square("g8").piece.is_none()
                    && self.get_square("h8").piece.is_some()
                    && self.get_square("h8").piece.unwrap().0 == Piece::Rook
                    && self.get_square("h8").piece.unwrap().1 == Color::Black
                {
                    self.place_piece("f8", Some((Piece::Rook, Color::Black)));
                    self.place_piece("h8", None);
                }
            }
        }

        if mov.mov_type == MoveType::CastleQueenSide {
            if self.to_move == Color::White {
                if self.get_square("d1").piece.is_none()
                    && self.get_square("c1").piece.is_none()
                    && self.get_square("b1").piece.is_none()
                    && self.get_square("a1").piece.is_some()
                    && self.get_square("a1").piece.unwrap().0 == Piece::Rook
                    && self.get_square("a1").piece.unwrap().1 == Color::White
                {
                    self.place_piece("d1", Some((Piece::Rook, Color::White)));
                    self.place_piece("a1", None);
                }
            } else {
                if self.get_square("d8").piece.is_none()
                    && self.get_square("c8").piece.is_none()
                    && self.get_square("b8").piece.is_none()
                    && self.get_square("a8").piece.is_some()
                    && self.get_square("a8").piece.unwrap().0 == Piece::Rook
                    && self.get_square("a8").piece.unwrap().1 == Color::Black
                {
                    self.place_piece("d8", Some((Piece::Rook, Color::Black)));
                    self.place_piece("a8", None);
                }
            }
        }
        self.set_piece(to, self.get_square(&mov.from).piece);
        self.set_piece(from, None);

        if mov.mov_type == MoveType::Promotion || mov.mov_type == MoveType::PromotionCapture {
            self.set_piece(to, Some((mov.promotion.unwrap(), self.to_move)));
        }

        if mov.mov_type == MoveType::Capture || mov.mov_type == MoveType::EnPassant {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        if self.to_move == Color::Black {
            self.fullmove_number += 1;
        }

        if self.to_move == Color::White {
            self.to_move = Color::Black;
        } else {
            self.to_move = Color::White;
        }
    }

    pub fn get_square(&self, square: &str) -> Square {
        let (x, y) = square.to_board_indices();
        self.board[y][x]
    }
}

pub trait ToBoardIndices {
    fn to_board_indices(&self) -> (usize, usize);
}

impl ToBoardIndices for String {
    fn to_board_indices(&self) -> (usize, usize) {
        if self.len() != 2 {
            panic!("Invalid square!");
        }
        let y = self.chars().nth(0).unwrap() as usize - 97;
        let x = self.chars().nth(1).unwrap() as usize - 49;
        (x, y)
    }
}

impl ToBoardIndices for &str {
    fn to_board_indices(&self) -> (usize, usize) {
        if self.len() != 2 {
            panic!("Invalid square!");
        }
        let x = self.chars().nth(0).unwrap() as usize - 97;
        let y = self.chars().nth(1).unwrap() as usize - 49;
        (x, y)
    }
}

#[derive(Debug, PartialEq)]
pub enum MoveType {
    Normal,
    PawnDouble,
    Capture,
    EnPassant,
    CastleKingSide,
    CastleQueenSide,
    Promotion,
    PromotionCapture,
}

impl std::fmt::Display for MoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_print: &str = match self {
            MoveType::Normal => "Normal",
            MoveType::PawnDouble => "PawnDouble",
            MoveType::Capture => "Capture",
            MoveType::EnPassant => "EnPassant",
            MoveType::CastleKingSide => "O-O",
            MoveType::CastleQueenSide => "O-O-O",
            MoveType::Promotion => "Promotion",
            MoveType::PromotionCapture => "PromotionCapture",
        };
        write!(f, "{}", to_print)
    }
}

#[derive(Debug)]
pub struct Move {
    pub from: String,
    pub to: String,
    pub mov_type: MoveType,
    pub promotion: Option<Piece>,
}

impl Move {
    pub fn new(from: &str, to: &str, typ: MoveType) -> Move {
        Move {
            from: from.to_string(),
            to: to.to_string(),
            mov_type: typ,
            promotion: None,
        }
    }

    pub fn print(&self) {
        print!("{} -> {} ({})", self.from, self.to, self.mov_type);
        if self.promotion.is_some() {
            print!(" ({:?})", self.promotion.unwrap());
        }
        println!();
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", self.from, self.to)
    }

    pub fn get_indices(&self) -> ((usize, usize), (usize, usize)) {
        let (from_x, from_y) = self.from.to_board_indices();
        let (to_x, to_y) = self.to.to_board_indices();
        ((from_x, from_y), (to_x, to_y))
    }

    pub fn from_indices(from: &Square, to: &Square, typ: MoveType) -> Move {
        let from = format!(
            "{}{}",
            (from.x + 97) as u8 as char,
            (from.y + 49) as u8 as char
        );
        let to = format!("{}{}", (to.x + 97) as u8 as char, (to.y + 49) as u8 as char);
        Move::new(&from, &to, typ)
    }
}

pub fn from_indices(y: usize, x: usize) -> String {
    format!("{}{}", (x + 97) as u8 as char, (y + 49) as u8 as char)
}

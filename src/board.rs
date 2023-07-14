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

pub struct Board {
    pub board: [[Square; 8]; 8],
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

        Board { board: board }
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
    }

    pub fn set_piece(&mut self, (x, y): (usize, usize), piece: Option<(Piece, Color)>) {
        self.board[x][y].piece = piece;
    }

    pub fn place_piece(&mut self, square: &str, piece: Option<(Piece, Color)>) {
        let indices = square.to_board_indices();
        self.set_piece(indices, piece);
    }

    pub fn move_piece(&mut self, mov: &Move) {
        /*
        if !Analyzer::check_legal(mov, &self) {
            println!("Illegal move!");
            return;
        }
        */

        let (from, to) = mov.get_indices();
        self.set_piece(to, self.get_square(&mov.from).piece);
        self.set_piece(from, None);
    }

    pub fn get_square(&self, square: &str) -> Square {
        let (x, y) = square.to_board_indices();
        self.board[x][y]
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
        let y = self.chars().nth(0).unwrap() as usize - 97;
        let x = self.chars().nth(1).unwrap() as usize - 49;
        (x, y)
    }
}

#[derive(Debug)]
pub struct Move {
    pub from: String,
    pub to: String,
}

impl Move {
    pub fn new(from: &str, to: &str) -> Move {
        Move {
            from: from.to_string(),
            to: to.to_string(),
        }
    }

    pub fn print(&self) {
        println!("{} -> {}", self.from, self.to);
    }

    pub fn get_indices(&self) -> ((usize, usize), (usize, usize)) {
        let (from_x, from_y) = self.from.to_board_indices();
        let (to_x, to_y) = self.to.to_board_indices();
        ((from_x, from_y), (to_x, to_y))
    }

    pub fn from_indices(from: &Square, to: &Square) -> Move {
        let from = format!(
            "{}{}",
            (from.x + 97) as u8 as char,
            (from.y + 49) as u8 as char
        );
        let to = format!("{}{}", (to.x + 97) as u8 as char, (to.y + 49) as u8 as char);
        Move::new(&from, &to)
    }
}

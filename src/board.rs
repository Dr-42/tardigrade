use colored::Colorize;

#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq)]
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

pub struct Board {
    pub board: [[Option<(Piece, Color)>; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[None; 8]; 8],
        }
    }

    pub fn print(&self) {
        for (r, row) in self.board.iter().rev().enumerate() {
            for (c, piece) in row.iter().enumerate() {
                let to_print: &str = match piece {
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
                    if piece.is_some() && piece.unwrap().1 == Color::White {
                        print!("{}", to_print.white().on_yellow());
                    } else {
                        print!("{}", to_print.black().on_yellow());
                    }
                } else {
                    if piece.is_some() && piece.unwrap().1 == Color::White {
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

    pub fn get_piece(&self, x: usize, y: usize) -> Option<(Piece, Color)> {
        self.board[x][y]
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece: Option<(Piece, Color)>) {
        self.board[x][y] = piece;
    }

    pub fn place_piece(&mut self, square: &str, piece: Option<(Piece, Color)>) {
        let y = square.chars().nth(0).unwrap() as usize - 97;
        let x = square.chars().nth(1).unwrap() as usize - 49;
        self.set_piece(x, y, piece);
    }

    pub fn move_piece(&mut self, from: &str, to: &str) {
        let from_y = from.chars().nth(0).unwrap() as usize - 97;
        let from_x = from.chars().nth(1).unwrap() as usize - 49;
        let to_y = to.chars().nth(0).unwrap() as usize - 97;
        let to_x = to.chars().nth(1).unwrap() as usize - 49;
        self.set_piece(to_x, to_y, self.get_piece(from_x, from_y));
        self.set_piece(from_x, from_y, None);
    }
}

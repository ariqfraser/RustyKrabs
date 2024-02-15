use std::str::Chars;

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub has_moved: bool,
    pub piece_type: PieceType,
    pub is_white: bool,
}

impl Piece {
    pub fn new(char: char, has_moved: bool) -> Piece {
        Piece {
            has_moved,
            piece_type: PieceType::new_from_char(&char),
            is_white: char.is_ascii_uppercase(),
        }
    }

    pub fn new_empty() -> Piece {
        Piece {
            has_moved: false,
            piece_type: PieceType::Empty,
            is_white: false,
        }
    }

    pub fn get_value(&self) -> i32 {
        self.piece_type.value()
    }

    pub fn get_char(&self) -> char {
        if self.is_white {
            return self.piece_type.get_fancy_char_white();
        }

        self.piece_type.get_char()
    }

    pub fn is_king(&self) -> bool {
        self.piece_type.eq(&PieceType::King)
    }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    #[default]
    Empty,
}

impl PieceType {
    pub fn new_from_char(char: &char) -> PieceType {
        match char.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'b' => PieceType::Bishop,
            'n' => PieceType::Knight,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => PieceType::Empty,
        }
    }
    pub fn get_char(&self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'n',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
            PieceType::Empty => ' ',
        }
    }

    pub fn get_fancy_char_white(&self) -> char {
        match self {
            PieceType::Pawn => '♙',
            PieceType::Bishop => '♗',
            PieceType::Knight => '♘',
            PieceType::Rook => '♖',
            PieceType::Queen => '♕',
            PieceType::King => '♚',
            PieceType::Empty => ' ',
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Rook => 5,
            PieceType::King => 15,
            PieceType::Queen => 9,
            PieceType::Bishop => 3,
            PieceType::Knight => 3,
            PieceType::Empty => 0,
        }
    }
}

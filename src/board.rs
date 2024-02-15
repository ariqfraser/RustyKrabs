use crate::{
    action::Action,
    piece::{Piece, PieceType},
};

pub type BoardArray = [Piece; 64];
pub type PieceIndices = Vec<i32>;

#[derive(Debug, Clone)]
pub struct Board {
    pub array: BoardArray,
    pub white_pieces: PieceIndices,
    pub black_pieces: PieceIndices,
    pub white_in_check: bool,
    pub black_in_check: bool,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            array: [Piece::new('-', false); 64],
            white_pieces: Vec::new(),
            black_pieces: Vec::new(),
            white_in_check: false,
            black_in_check: false,
        }
    }
}

impl Board {
    pub fn new(fen: &str) -> Board {
        let fen_vec: Vec<&str> = fen_to_vector(fen);
        let mut board = Board::default();
        Self::initialise_board(&mut board, fen_vec);

        Self::filter_pieces(&mut board);
        println!("Black pieces {:?}", board.black_pieces);
        println!("White pieces {:?}", board.white_pieces);
        board
    }

    pub fn perform_action(&mut self, action: Action) {
        self.move_piece_in_array(action.to, action.from);
        self.filter_pieces();
    }

    fn filter_pieces(&mut self) {
        let (white_pieces, black_pieces): (PieceIndices, PieceIndices) = self
            .array
            .chunks_exact(64)
            .flat_map(|row| row.iter())
            .enumerate()
            .fold((vec![], vec![]), |mut acc, (i, &piece)| {
                match piece.piece_type {
                    PieceType::Empty => {}
                    _ => {
                        if piece.is_white {
                            acc.0.push(i as i32);
                        } else {
                            acc.1.push(i as i32);
                        }
                    }
                }
                acc
            });

        self.white_pieces = white_pieces;
        self.black_pieces = black_pieces;
    }

    fn move_piece_in_array(&mut self, to: i32, from: i32) {
        let piece_to_move = self.array[from as usize];
        self.array[from as usize] = Piece::new_empty();
        self.array[to as usize] = piece_to_move;
    }

    fn initialise_board(board: &mut Board, fen_vec: Vec<&str>) {
        let mut file: i32 = 0;
        let mut rank: i32 = 7;

        for char in fen_vec[0].chars() {
            if char == '/' {
                rank -= 1;
                file = 0;
            } else if char::is_digit(char, 10) {
                file += char.to_digit(10).unwrap() as i32;
            } else {
                let i = rank * 8 + file;

                let piece = Piece::new(char, false);
                board.array[i as usize] = piece;
                file += 1;
            }
        }
    }

    pub fn print(&self) {
        let mut board = self.array;
        board.reverse();
        for rank in 0..8 {
            let mut row = String::from("");
            for file in 0..8 {
                let i = rank * 8 + file;
                let piece = board[i];

                // row = format!("{} {} {:?} ", row, i, piece.get_char());
                row = format!("{} {:?} ", row, piece.get_char());
            }
            println!("{}", row);
        }
    }

    pub fn pos_has_king(&self, pos: i32) -> bool {
        self.array[pos as usize].is_king()
    }
}

fn fen_to_vector(fen: &str) -> Vec<&str> {
    // TODO: Validate fen
    fen.split_whitespace().collect()
}

use crate::Serializable;
use crate::Error;
use crate::Square;
use crate::Piece;
use crate::Move;
use crate::indexing;

#[derive(Debug)]
pub struct Board {
    pub squares: [Square; 64],
    pub white_to_move: bool,
}

impl Board {
    /// Turns a &str in FEN notation, turns it into a Board
    pub fn parse_fen(fen: &str) -> Result<Self, Error> {
        let mut squares: [Square; 64] = [const {Square::Empty}; 64];
        let white_to_move: bool;

        let mut rank: u8 = 7;
        let mut file: u8 = 0;

        let mut chars = fen.chars();

        for c in chars.by_ref() {

            if c == ' ' {
                break;
            }
            if c.is_ascii_digit() {
                file += c.to_digit(10).ok_or(Error::InvalidFen(fen.to_string()))? as u8;
                continue;
            }
            if c == '/' {
                rank -= 1;
                file = 0;
                continue;
            }
            let index = indexing::index_from_position(rank, file)?;
            if index as usize >= squares.len(){
                return Err(Error::IndexOutOfBounds(index as u8));
            }
            squares[index as usize] = Square::from_char(c as u8)?;
            file += 1;
        }

        match chars.next().ok_or(Error::InvalidFen(fen.to_string()))? {
            'w' => white_to_move = true,
            'b' => white_to_move = false,
            _   =>  return Err(Error::InvalidFen(fen.to_string()))
        }

        Ok(
            Board{
                squares,
                white_to_move,
            }
        )
    }

    /// Changes the position on the board by moving a piece
    pub fn apply_move(&mut self, mv: &Move) -> Result<(), Error> {
        // Account for:
        //
        // Castling
        // En passant
        // Promotions

        let from_square = self.squares[mv.from_square as usize].clone();
        let to_square = self.squares[mv.to_square as usize].clone();

        let (from_rank, from_file) = indexing::position_from_index(mv.from_square).unwrap();
        let (_to_rank, to_file) = indexing::position_from_index(mv.to_square).unwrap();

        let piece_type = from_square.piece_type().ok_or(Error::InvalidMove)?;

        match piece_type {
            Piece::Pawn => {
                // En passant
                if from_file != to_file && to_square.is_empty() {
                    let taken_pawn_rank: u8 = if self.white_to_move {5} else {2}; // zero-based rank
                    let taken_pawn_file = to_file;

                    let taken_pawn_index = indexing::index_from_position(taken_pawn_rank, taken_pawn_file).unwrap();

                    self.squares[mv.to_square as usize] =
                        if self.white_to_move {Square::White(Piece::Pawn)}
                        else {Square::Black(Piece::Pawn)};

                    self.squares[mv.from_square as usize] = Square::Empty;
                    self.squares[taken_pawn_index as usize] = Square::Empty;
                    self.white_to_move = !self.white_to_move;
                    return Ok(());
                }


                // Promotion
                else if (self.white_to_move && from_rank == 6) ||
                    (!self.white_to_move && from_rank == 1) {

                    match &mv.promotion {
                        None => return Err(Error::InvalidMove), // invalid promotion
                        Some(promotion) => {
                            self.squares[mv.from_square as usize] = Square::Empty;
                            self.squares[mv.to_square as usize] = promotion.to_square(self.white_to_move);
                            self.white_to_move = !self.white_to_move;
                            return Ok(());
                        }
                    }
                }
            },
            Piece::King => {
                // Castling
                if from_file == 4 && (to_file == 2 || to_file == 6) {

                    let rook_from_file: u8;
                    let rook_to_file: u8;
                    if to_file == 2 {
                        rook_from_file = 0; // rook from a1
                        rook_to_file = 3; // to d1
                    }
                    else if to_file == 6 {
                        rook_from_file = 7; // rook from a1
                        rook_to_file = 5; // to f1
                    }
                    else {panic!()}

                    let rook_from_rank = from_rank; // rook is on the same rank as king when castling
                    let rook_to_rank = rook_from_rank;

                    let rook_from_index = indexing::index_from_position(rook_from_rank, rook_from_file).unwrap();
                    let rook_to_index = indexing::index_from_position(rook_to_rank, rook_to_file).unwrap();

                    
                    // Remove king from original position
                    self.squares[mv.from_square as usize] = Square::Empty;

                    // Remove rook from original position
                    self.squares[rook_from_index as usize] = Square::Empty;

                    // Place rook at new position
                    self.squares[rook_to_index as usize] = 
                        if self.white_to_move {Square::White(Piece::Rook)}
                        else {Square::Black(Piece::Rook)};

                    // Place king at new position
                    self.squares[mv.to_square as usize] = from_square;

                    self.white_to_move = !self.white_to_move;
                    return Ok(());
                }
            },
            _ => {},
        }

        // Base case
        self.squares[mv.from_square as usize] = Square::Empty;
        self.squares[mv.to_square as usize] = from_square;
        self.white_to_move = !self.white_to_move;

        Ok(())

    }
}


impl Serializable for Board {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::size_in_memory());

        for square in &self.squares {
            result.extend(square.serialize());
        }

        result.push(u8::from(self.white_to_move));

        return result;
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != Self::size_in_memory() {
            return Err(Error::DeserializationError);
        }

        let mut squares: [Square; 64] =
            [const { Square::Empty }; 64];

        for (i, byte) in bytes[..64].iter().enumerate() {
            squares[i] = Square::from_char(*byte)?;
        }

        let white_to_move = bool::try_from(bytes[64]).map_err(|_| Error::DeserializationError)?; 

        Ok(Board {
            squares,
            white_to_move,
        })
    }
    fn size_in_memory() -> usize {65}
}

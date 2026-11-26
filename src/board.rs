use crate::Serializable;
use crate::Square;
use crate::{Piece, PieceType};
use crate::Move;

use crate::{Rank, File};

#[derive(Debug)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub white_to_move: bool,
}

impl Board {
    /// Turns a &str in FEN notation, turns it into a Board
    pub fn parse_fen(fen: &str) -> Result<Self, ()> {
        let mut squares: [Option<Piece>; 64] = [None; 64];
        let white_to_move: bool;

        let mut rank = Rank::Eight;
        let mut file: u8 = 0;

        let mut chars = fen.chars();

        for c in chars.by_ref() {

            if c == ' ' {
                break;
            }

            if c.is_ascii_digit() {

                let skip_files = c.to_digit(10).ok_or(())? as u8;
                file += skip_files;

                if file > 8 {
                    return Err(());
                }

                continue;
            }

            if c == '/' {
                rank -= 1;
                file = 0;
                continue;
            }
            let square = Square {file: File::try_from(file)?, rank};
            let index = square.index() as usize;

            if index >= squares.len(){
                return Err(());
            }
            let piece_char = c as u8;
            squares[index] = Some(Piece::from_ascii(piece_char)?);

            file += 1;
        }

        match chars.next().ok_or(())? {
            'w' => white_to_move = true,
            'b' => white_to_move = false,
            _   =>  return Err(())
        }

        Ok(
            Board{
                squares,
                white_to_move,
            }
        )
    }

    /// Changes the position on the board by moving a piece. Does not verify move validity
    pub fn apply_move(&mut self, mv: &Move) -> Result<(), ()> {
        // Account for:
        //
        // Castling
        // En passant
        // Promotions
        
        let Some(piece_moved) = self.squares[mv.from_square.index() as usize]
        else {return Err(())};

        match piece_moved.kind{
            PieceType::Pawn => {
                // En passant
                if mv.from_square.file != mv.to_square.file && // If pawn captured
                    self.squares[mv.to_square.index() as usize].is_none() // and the square at which
                                                                          // it captured is empty
                {

                    let taken_pawn_square = Square {
                        file: mv.to_square.file,
                        rank: if self.white_to_move {Rank::Five} else {Rank::Four}
                    };

                    self.squares[mv.to_square.index() as usize] = Some(Piece {
                        kind: PieceType::Pawn,
                        white: self.white_to_move,
                    });

                    self.squares[mv.from_square.index() as usize] = None;
                    self.squares[taken_pawn_square.index() as usize] = None;
                    self.white_to_move = !self.white_to_move;
                    return Ok(());
                }


                // Promotion
                else if (self.white_to_move && mv.from_square.rank == Rank::Seven) ||
                       (!self.white_to_move && mv.from_square.rank == Rank::Two) {

                    match &mv.promotion {
                        None => return Err(()), // invalid promotion
                        Some(promotion) => {

                            self.squares[mv.from_square.index() as usize] = None;

                            self.squares[mv.to_square.index() as usize] = Some(Piece {
                                kind: promotion.to_piece_type(),
                                white: self.white_to_move,
                            });

                            self.white_to_move = !self.white_to_move;
                            return Ok(());
                        }
                    }
                }
            },
            PieceType::King => {
                // Castling
                if mv.from_square.file == File::E && 
                (mv.to_square.file == File::C || mv.to_square.file == File::G) {

                    let rook_from_file: File;
                    let rook_to_file: File;

                    if mv.to_square.file == File::C {
                        rook_from_file = File::A;
                        rook_to_file = File::D;
                    }
                    else if mv.to_square.file == File::G {
                        rook_from_file = File::H;
                        rook_to_file = File::F;
                    }
                    else {panic!()}

                    let rook_from_rank = mv.from_square.rank; // rook is on the same rank as king when castling
                    let rook_to_rank = rook_from_rank;

                    let rook_from_square = Square {
                        file: rook_from_file,
                        rank: rook_from_rank,
                    };

                    let rook_to_square = Square {
                        file: rook_to_file,
                        rank: rook_to_rank,
                    };
                    
                    // Remove king from original position
                    self.squares[mv.from_square.index() as usize] = None;

                    // Remove rook from original position
                    self.squares[rook_from_square.index() as usize] = None;

                    // Place rook at new position
                    self.squares[rook_to_square.index() as usize] = Some(Piece {
                        kind: PieceType::Rook,
                        white: self.white_to_move,
                    });

                    // Place king at new position
                    self.squares[mv.to_square.index() as usize] = Some(Piece {
                        kind: PieceType::King,
                        white: self.white_to_move,
                    });

                    self.white_to_move = !self.white_to_move;
                    return Ok(());
                }
            },
            _ => {},
        }

        // Base case
        self.squares[mv.to_square.index() as usize] = self.squares[mv.from_square.index() as usize];
        self.squares[mv.from_square.index() as usize] = None;
        self.white_to_move = !self.white_to_move;

        Ok(())

    }
}


impl Serializable for Board {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::size_in_memory());

        for square in &self.squares {
            let serialized_square = match square {
                None => vec![0],
                Some(piece) => piece.serialize(),
            };
            result.extend(serialized_square);
        }

        result.push(u8::from(self.white_to_move));

        return result;
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, ()> {
        if bytes.len() != Self::size_in_memory() {
            return Err(());
        }

        let mut squares: [Option<Piece>; 64] = [None; 64];

        for (i, byte) in bytes[..64].iter().enumerate() {
            // squares[i] = Square::from_char(*byte)?;
            squares[i] = match byte {
                0 => None,
                other => Some(Piece::from_ascii(*other)?),
            }
        }

        let white_to_move = bool::try_from(bytes[64]).map_err(|_| ())?; 

        Ok(Board {
            squares,
            white_to_move,
        })
    }
    fn size_in_memory() -> usize {65}
}









#[cfg(test)]
mod tests {
    use super::*;

    fn square(file: File, rank: Rank) -> Square {
        Square { file, rank }
    }

    #[test]
    fn parses_starting_position_and_side_to_move() {
        let board = Board::parse_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1",
        )
        .unwrap();

        assert!(board.white_to_move);
        assert_eq!(
            board.squares[square(File::E, Rank::One).index() as usize]
                .unwrap()
                .kind,
            PieceType::King
        );
        assert_eq!(
            board.squares[square(File::E, Rank::Eight).index() as usize]
                .unwrap()
                .kind,
            PieceType::King
        );
    }

    #[test]
    fn parses_black_to_move() {
        let board = Board::parse_fen("8/8/8/8/8/8/8/8 b - - 0 1").unwrap();
        assert!(!board.white_to_move);
    }

    #[test]
    fn rejects_invalid_side_to_move() {
        assert!(Board::parse_fen("8/8/8/8/8/8/8/8 x - - 0 1").is_err());
    }

    #[test]
    fn rejects_rank_with_too_many_squares() {
        assert!(Board::parse_fen("9/8/8/8/8/8/8/8 w - - 0 1").is_err());
    }

    #[test]
    fn applies_basic_pawn_move_and_changes_turn() {
        let mut board = Board::parse_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1",
        )
        .unwrap();

        let mv = Move {
            from_square: square(File::E, Rank::Two),
            to_square: square(File::E, Rank::Four),
            promotion: None,
        };

        board.apply_move(&mv).unwrap();

        assert!(board.squares[square(File::E, Rank::Two).index() as usize].is_none());
        assert_eq!(
            board.squares[square(File::E, Rank::Four).index() as usize]
                .unwrap()
                .kind,
            PieceType::Pawn
        );
        assert!(!board.white_to_move);
    }

    #[test]
    fn en_passant_removes_the_captured_pawn() {
        let mut board =
            Board::parse_fen("8/8/8/3pP3/8/8/8/8 w - - 0 1").unwrap();

        let mv = Move {
            from_square: square(File::E, Rank::Five),
            to_square: square(File::D, Rank::Six),
            promotion: None,
        };

        board.apply_move(&mv).unwrap();

        assert!(board.squares[square(File::D, Rank::Five).index() as usize].is_none());
        assert!(board.squares[square(File::E, Rank::Five).index() as usize].is_none());
        assert_eq!(
            board.squares[square(File::D, Rank::Six).index() as usize]
                .unwrap()
                .kind,
            PieceType::Pawn
        );
    }
}


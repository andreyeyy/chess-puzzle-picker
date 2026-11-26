use std::fmt;

use crate::{Serializable, Square, PieceType};

#[derive(Debug, Clone)]
pub enum Promotion {
    Queen,
    Rook,
    Knight,
    Bishop,
}

impl Promotion {
    pub fn from_ascii(c: u8) -> Result<Self, ()> {
        Self::deserialize(&[c])
    }

    pub fn to_piece_type(&self) -> PieceType {
        match self {
            Self::Queen => PieceType::Queen,
            Self::Rook => PieceType::Rook,
            Self::Knight => PieceType::Knight,
            Self::Bishop => PieceType::Bishop,
        }
    }
}

impl fmt::Display for Promotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.serialize()[0] as char)
    }
}

impl Serializable for Promotion {

    fn serialize(&self) -> Vec<u8> {
        match self {
            Promotion::Queen => vec![b'q'],
            Promotion::Rook => vec![b'r'],
            Promotion::Knight => vec![b'n'],
            Promotion::Bishop => vec![b'b'],
        }
    }
    fn deserialize(bytes: &[u8]) -> Result<Self, ()> {
        match bytes[0] {
            b'q' => Ok(Promotion::Queen),
            b'r' => Ok(Promotion::Rook),
            b'n' => Ok(Promotion::Knight),
            b'b' => Ok(Promotion::Bishop),
            _    => Err(())
        }
    }
    fn size_in_memory() -> usize {1}
}

#[derive(Debug)]
pub struct Move {
    pub from_square: Square,
    pub to_square: Square,
    pub promotion: Option<Promotion>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let from_square_str = String::from_utf8(self.from_square.to_ascii().to_vec()).unwrap();
        let to_square_str = String::from_utf8(self.to_square.to_ascii().to_vec()).unwrap();

        let promotion_str = match &self.promotion {
            None => "".to_string(),
            Some(p) => p.to_string(),
        };

        write!(f, "{from_square_str}{to_square_str}{promotion_str}")
    }
}

impl Move {

    pub fn from_ascii(squares: &[u8]) -> Result<Self, ()> {
        let promotion: Option<Promotion>;

        if squares.len() == 4 {
            promotion = None
        }
        else if squares.len() == 5 {
            promotion = Some(Promotion::from_ascii(squares[4])?);
        }
        else {
            return Err(());
        }
        // dbg!("we're parsing move");

        Ok(Move {
            from_square: Square::from_ascii(&squares[0..2])?,
            to_square: Square::from_ascii(&squares[2..4])?,
            promotion,
        })
    }
}

impl Serializable for Move {

    fn serialize(&self) -> Vec<u8> {

        let promotion_byte = match &self.promotion {
            Some(p) => p.serialize()[0],
            None => 0,
        };

        vec![self.from_square.serialize()[0], self.to_square.serialize()[0], promotion_byte]
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, ()> {
        if bytes.len() != 3 {
            Err(())
        }
        else {
            let promotion = match bytes[2] {
                0 => None,
                c => Some(Promotion::deserialize(&[c])?),
            };
            Ok(Move {
                from_square: Square::try_from(bytes[0])?,
                to_square: Square::try_from(bytes[1])?,
                promotion,
            })
        }
    }
    fn size_in_memory() -> usize {3}
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promotion_parses_and_serializes_each_piece() {
        for (byte, expected) in [
            (b'q', b'q'),
            (b'r', b'r'),
            (b'n', b'n'),
            (b'b', b'b'),
        ] {
            let promotion = Promotion::from_ascii(byte).unwrap();
            assert_eq!(promotion.serialize(), vec![expected]);
            assert_eq!(promotion.to_string(), (expected as char).to_string());
        }
    }

    #[test]
    fn promotion_rejects_unknown_piece() {
        assert!(Promotion::from_ascii(b'x').is_err());
    }

    #[test]
    fn move_parses_and_displays_without_promotion() {
        let chess_move = Move::from_ascii(b"e2e4").unwrap();

        assert!(chess_move.promotion.is_none());
        assert_eq!(chess_move.to_string(), "e2e4");
    }

    #[test]
    fn move_parses_and_displays_with_promotion() {
        let chess_move = Move::from_ascii(b"e7e8q").unwrap();

        assert_eq!(chess_move.promotion.as_ref().unwrap().to_string(), "q");
        assert_eq!(chess_move.to_string(), "e7e8q");
    }

    #[test]
    fn move_rejects_invalid_lengths() {
        assert!(Move::from_ascii(b"e2e").is_err());
        assert!(Move::from_ascii(b"e2e4qz").is_err());
    }

    #[test]
    fn move_rejects_invalid_squares_or_promotion() {
        assert!(Move::from_ascii(b"z2e4").is_err());
        assert!(Move::from_ascii(b"e2e4x").is_err());
    }

    #[test]
    fn move_serializes_and_deserializes() {
        let original = Move::from_ascii(b"e7e8n").unwrap();
        let bytes = original.serialize();
        let decoded = Move::deserialize(&bytes).unwrap();

        assert_eq!(decoded.to_string(), "e7e8n");
        assert_eq!(bytes.len(), Move::size_in_memory());
    }

    #[test]
    fn move_without_promotion_round_trips() {
        let original = Move::from_ascii(b"e2e4").unwrap();
        let bytes = original.serialize();
        let decoded = Move::deserialize(&bytes).unwrap();

        assert_eq!(decoded.to_string(), "e2e4");
        assert_eq!(bytes.len(), Move::size_in_memory());
    }

    #[test]
    fn move_deserialization_rejects_wrong_length_or_bad_promotion() {
        assert!(Move::deserialize(&[]).is_err());
        assert!(Move::deserialize(&[0, 0]).is_err());
        assert!(Move::deserialize(&[0, 0, b'x']).is_err());
    }
}


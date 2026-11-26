use crate::Serializable;

pub trait Coordinate {
    fn index(&self) -> u8;
    fn to_ascii(&self) -> u8;
    fn from_ascii(c: u8) -> Result<Self, ()> where Self: Sized;
    fn inverted(&self) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum File {
    A, B, C, D, E, F, G, H,
}

impl std::ops::Add<u8> for File{
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        let value = self as u8 + rhs;
        Self::try_from(value).expect("Non-overflowing addition")
    }
}

impl std::ops::Sub<u8> for File{
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        let value = self as u8 - rhs;
        Self::try_from(value).unwrap()
    }
}

impl std::ops::AddAssign<u8> for File {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs
    }
}

impl std::ops::SubAssign<u8> for File {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs
    }
}

impl Coordinate for File {
    fn index(&self) -> u8 {
        *self as u8
    }
    fn to_ascii(&self) -> u8 {
        b'a' + *self as u8
    }

    fn from_ascii(c: u8) -> Result<Self, ()> {
        if c < b'a' || c > b'h' {
            Err(())
        }
        else {
            Ok(File::try_from(c - b'a').unwrap())
        }
    }

    fn inverted(&self) -> Self {
        Self::try_from(7 - *self as u8).unwrap()
    }
}

impl Serializable for File {
    fn serialize(&self) -> Vec<u8> {
        vec![self.to_ascii()]
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, ()> {
        if bytes.len() != 1 {
            return Err(());
        }
        Self::from_ascii(bytes[0])
    }

    fn size_in_memory() -> usize {1}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Rank {
    One, Two, Three, Four, Five, Six, Seven, Eight,
}

impl std::ops::Add<u8> for Rank {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        let value = self as u8 + rhs;
        Self::try_from(value).expect("Non-overflowing addition")
    }
}

impl std::ops::Sub<u8> for Rank {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        let value = self as u8 - rhs;
        Self::try_from(value).unwrap()
    }
}

impl std::ops::AddAssign<u8> for Rank{
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs
    }
}

impl std::ops::SubAssign<u8> for Rank{
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs
    }
}

impl Coordinate for Rank{

    fn index(&self) -> u8 {
        *self as u8
    }

    fn to_ascii(&self) -> u8 {
        b'1' + *self as u8
    }

    fn from_ascii(c: u8) -> Result<Self, ()> {
        if c < b'1' || c > b'8' {
            Err(())
        }
        else {
            Ok(Rank::try_from(c - b'1').unwrap())
        }
    }

    fn inverted(&self) -> Self {
        Self::try_from(7 - *self as u8).unwrap()
    }
}

impl Serializable for Rank {
    fn serialize(&self) -> Vec<u8> {
        vec![self.to_ascii()]
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, ()> {
        if bytes.len() != 1 {
            return Err(());
        }
        Self::from_ascii(bytes[0])
    }

    fn size_in_memory() -> usize {1}
}

impl TryFrom<u8> for File {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            7 => Ok(File::H),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Rank {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::One),
            1 => Ok(Rank::Two),
            2 => Ok(Rank::Three),
            3 => Ok(Rank::Four),
            4 => Ok(Rank::Five),
            5 => Ok(Rank::Six),
            6 => Ok(Rank::Seven),
            7 => Ok(Rank::Eight),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    pub fn index(&self) -> u8 {
        8 * self.rank.index() + self.file.index()
    }
    pub fn to_ascii(&self) -> [u8; 2] {
        [self.file.to_ascii(), self.rank.to_ascii()]
    }
    pub fn from_ascii(c: &[u8]) -> Result<Self, ()> {
        if c.len() != 2 {
            return Err(());
        }
        Ok(Self {
            file: File::from_ascii(c[0])?,
            rank: Rank::from_ascii(c[1])?,
        })
    }
}

impl Serializable for Square {
    fn serialize(&self) -> Vec<u8> {
        vec![self.index()]
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, ()> {
        if bytes.len() != 1 {
            return Err(());
        }
        Self::try_from(bytes[0])
    }

    fn size_in_memory() -> usize {1}
}

impl TryFrom<u8> for Square{
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 64 {
            return Err(());
        }
        Ok (
            Square {
                file: File::try_from(value % 8).unwrap(),
                rank: Rank::try_from(value / 8).unwrap(),
            }
        )
    }
}








#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_ascii_round_trip() {
        for i in 0..8 {
            let file = File::try_from(i).unwrap();
            assert_eq!(file.index(), i);
            assert_eq!(file.to_ascii(), b'a' + i);
            assert_eq!(File::from_ascii(b'a' + i), Ok(file));
            assert_eq!(File::deserialize(&[b'a' + i]), Ok(file));
        }
    }

    #[test]
    fn rank_ascii_round_trip() {
        for i in 0..8 {
            let rank = Rank::try_from(i).unwrap();
            assert_eq!(rank.index(), i);
            assert_eq!(rank.to_ascii(), b'1' + i);
            assert_eq!(Rank::from_ascii(b'1' + i), Ok(rank));
            assert_eq!(Rank::deserialize(&[b'1' + i]), Ok(rank));
        }
    }

    #[test]
    fn coordinate_ascii_rejects_out_of_range() {
        for c in [b'@', b'i', 0, 255] {
            assert_eq!(File::from_ascii(c), Err(()));
        }
        for c in [b'0', b'9', 0, 255] {
            assert_eq!(Rank::from_ascii(c), Err(()));
        }
    }

    #[test]
    fn serialization_rejects_wrong_lengths() {
        assert_eq!(File::deserialize(&[]), Err(()));
        assert_eq!(File::deserialize(b"ab"), Err(()));
        assert_eq!(Rank::deserialize(&[]), Err(()));
        assert_eq!(Rank::deserialize(b"12"), Err(()));
        assert_eq!(Square::deserialize(&[]), Err(()));
        assert_eq!(Square::deserialize(&[0, 1]), Err(()));
    }

    #[test]
    fn file_and_rank_invert() {
        assert_eq!(File::A.inverted(), File::H);
        assert_eq!(File::D.inverted(), File::E);
        assert_eq!(File::H.inverted().inverted(), File::H);
        assert_eq!(Rank::One.inverted(), Rank::Eight);
        assert_eq!(Rank::Four.inverted(), Rank::Five);
        assert_eq!(Rank::Eight.inverted().inverted(), Rank::Eight);
    }

    #[test]
    fn file_and_rank_arithmetic() {
        assert_eq!(File::B + 3, File::E);
        assert_eq!(File::H - 2, File::F);
        assert_eq!(Rank::Two + 5, Rank::Seven);
        assert_eq!(Rank::Seven - 4, Rank::Three);

        let mut file = File::C;
        file += 2;
        assert_eq!(file, File::E);
        file -= 1;
        assert_eq!(file, File::D);

        let mut rank = Rank::Three;
        rank += 2;
        assert_eq!(rank, Rank::Five);
        rank -= 1;
        assert_eq!(rank, Rank::Four);
    }

    #[test]
    fn square_index_ascii_and_round_trip() {
        let samples = [
            (Square { file: File::A, rank: Rank::One }, 0, *b"a1"),
            (Square { file: File::H, rank: Rank::One }, 7, *b"h1"),
            (Square { file: File::A, rank: Rank::Eight }, 56, *b"a8"),
            (Square { file: File::H, rank: Rank::Eight }, 63, *b"h8"),
        ];
        for (square, index, ascii) in samples {
            assert_eq!(square.index(), index);
            assert_eq!(square.to_ascii(), ascii);
            assert_eq!(Square::from_ascii(&ascii), Ok(square));
            assert_eq!(Square::serialize(&square), vec![index]);
            assert_eq!(Square::deserialize(&[index]), Ok(square));
        }
    }

    #[test]
    fn every_square_index_round_trips() {
        for index in 0..64 {
            let square = Square::try_from(index).unwrap();
            assert_eq!(square.index(), index);
            assert_eq!(Square::deserialize(&[index]), Ok(square));
        }
    }

    #[test]
    fn square_from_ascii_rejects_invalid_input() {
        assert_eq!(Square::from_ascii(b"a"), Err(()));
        assert_eq!(Square::from_ascii(b"a10"), Err(()));
        assert_eq!(Square::from_ascii(b"i1"), Err(()));
        assert_eq!(Square::from_ascii(b"a9"), Err(()));
    }

    #[test]
    fn square_index_rejects_values_above_board() {
        assert_eq!(Square::try_from(64), Err(()));
        assert_eq!(Square::try_from(255), Err(()));
    }
}

use crate::Error;

pub fn index_from_position(rank: u8, file: u8) -> Result<u8, Error> {
    if rank >= 8 || file >= 8 {
        return Err(Error::InvalidPosition((rank, file)));
    }
    Ok (
        8 * rank + file
    )
}

pub fn position_from_index(index: u8) -> Result<(u8, u8), Error> {
    if index >= 64 {
        return Err(Error::IndexOutOfBounds(index));
    }
    Ok (
        (index / 8, index % 8)
    )
}

pub fn invert_coordinate(coordinate: u8) -> Result<u8, Error> {
    if coordinate >= 8 {
        return Err(Error::IndexOutOfBounds(coordinate));
    }
    else {
        Ok(7 - coordinate)
    }

}

pub fn index_from_ascii(square: &[u8]) -> Result<u8, Error> {
    if square.len() != 2 {
        return Err(Error::InvalidStringLength(square.len()));
    }
    match square[0] {
        b'a' | b'b' | b'c' | b'd' | b'e' | b'f' | b'g' | b'h' => {}
        c => {
            return Err(Error::InvalidMoveChar(c as char));
        }
    }
    match square[1] {
        b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' => {}
        c => {
            return Err(Error::InvalidMoveChar(c as char));
        }
    }
    let rank = square[1] - b'1';
    let file = square[0] - b'a';

    return index_from_position(rank, file);
}

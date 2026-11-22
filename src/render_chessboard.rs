use std::io::Cursor;
use image::{RgbaImage, Rgba, ImageBuffer, ImageFormat};
use ab_glyph::{FontRef, PxScale};
use imageproc::drawing::draw_text_mut;

use crate::board::Board;
use crate::square::Square;
use crate::square::Piece;
use crate::indexing::*;

const WP: &[u8] = include_bytes!("../assets/chess-pieces/150x150/wp-150x150.png");
const WN: &[u8] = include_bytes!("../assets/chess-pieces/150x150/wn-150x150.png");
const WB: &[u8] = include_bytes!("../assets/chess-pieces/150x150/wb-150x150.png");
const WR: &[u8] = include_bytes!("../assets/chess-pieces/150x150/wr-150x150.png");
const WQ: &[u8] = include_bytes!("../assets/chess-pieces/150x150/wq-150x150.png");
const WK: &[u8] = include_bytes!("../assets/chess-pieces/150x150/wk-150x150.png");

const BP: &[u8] = include_bytes!("../assets/chess-pieces/150x150/bp-150x150.png");
const BN: &[u8] = include_bytes!("../assets/chess-pieces/150x150/bn-150x150.png");
const BB: &[u8] = include_bytes!("../assets/chess-pieces/150x150/bb-150x150.png");
const BR: &[u8] = include_bytes!("../assets/chess-pieces/150x150/br-150x150.png");
const BQ: &[u8] = include_bytes!("../assets/chess-pieces/150x150/bq-150x150.png");
const BK: &[u8] = include_bytes!("../assets/chess-pieces/150x150/bk-150x150.png");

fn square_to_image(square: &Square) -> Option<&[u8]> {
    match square {
        Square::Empty => None,
        Square::White(piece) => Some(match piece {
            Piece::Pawn => WP,
            Piece::Knight => WN,
            Piece::Bishop => WB,
            Piece::Rook => WR,
            Piece::Queen => WQ,
            Piece::King => WK,
        }),
        Square::Black(piece) => Some(match piece {
            Piece::Pawn => BP,
            Piece::Knight => BN,
            Piece::Bishop => BB,
            Piece::Rook => BR,
            Piece::Queen => BQ,
            Piece::King => BK,
        }),
    }
}

pub struct BoardStyle {
    pub size: u32,
    pub color_white: Rgba<u8>,
    pub color_black: Rgba<u8>,
    pub font_size: f32,
}

impl Default for BoardStyle {
    fn default() -> Self {
        BoardStyle {
            size: 1024,
            color_white: Rgba([235, 235, 208, 255]),
            color_black: Rgba([115, 150, 82, 255]),
            font_size: 0.04,
        }
    }
}

impl BoardStyle {
    fn square_size(&self)  -> u32 {self.size / 8}
    fn font_size_px(&self) -> f32 {self.size as f32 * self.font_size}

}

fn fill_square(
    image: &mut RgbaImage,
    white_to_play: bool,
    square_index: u8,
    square: &Square) -> Result<(), Box<dyn std::error::Error>>  {

    assert!(square_index < 64);
    let piece_bytes = square_to_image(square);

    if piece_bytes.is_none() {return Ok(());}

    let piece = image::load_from_memory(piece_bytes.unwrap())?.to_rgba8();

    let (size, _) = image.dimensions();

    let square_size = size / 8;

    let piece = image::imageops::resize(
        &piece,
        square_size, square_size,
        image::imageops::FilterType::Lanczos3,
    );

    let (rank, file) = position_from_index(square_index).unwrap();

    let mut x = file;
    let mut y = invert_coordinate(rank).unwrap();

    if !white_to_play {
        x = invert_coordinate(x).unwrap();
        y = invert_coordinate(y).unwrap();
    }

    let x = x as u32 * square_size;
    let y = y as u32 * square_size;

    image::imageops::overlay(image, &piece, x as i64, y as i64);

    Ok(())
}

pub fn render_chessboard(board: &Board, style: BoardStyle) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    let font = FontRef::try_from_slice(include_bytes!("../assets/Roboto-SemiBold.ttf"))?;

    let mut image = ImageBuffer::from_fn(style.size, style.size, |x, y| {
        if (8*x/style.size + 8*y/style.size) % 2 == 0{
            style.color_white
        }
        else {style.color_black}
    });

    let mut letter = 
    if board.white_to_move {b'a'} else {b'h'};
    for file in 0..8 {
        let x = ((style.size as f64 * 0.09) + file as f64 * style.square_size() as f64) as i32;
        let y = (style.size as f64 * 0.955) as i32;

        let color = 
        if file % 2 == 0 {
            style.color_white
        }
        else {style.color_black};

        draw_text_mut(
            &mut image,
            color,
            x, y,
            PxScale::from(style.font_size_px()),
            &font,
            &String::from(letter as char),
        );
        if board.white_to_move {letter += 1}
        else {letter -= 1}
    }

    let mut number = 
    if !board.white_to_move {b'1'} else {b'8'};
    for rank in 0..8 {

        let x = (style.size as f64 * 0.003) as i32;
        let y = (rank as f64 * style.square_size() as f64) as i32;

        let color = 
        if rank % 2 == 1 {
            style.color_white
        }
        else {style.color_black};

        draw_text_mut(
            &mut image,
            color,
            x, y,
            PxScale::from(style.font_size_px()),
            &font,
            &String::from(number as char),
        );
        if !board.white_to_move {number += 1}
        else {number -= 1}
    }


    for (i, square) in board.squares.iter().enumerate() {
        fill_square(&mut image, board.white_to_move, i as u8, square)?;
    }

    let mut bytes = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)?;

    Ok(bytes)
}


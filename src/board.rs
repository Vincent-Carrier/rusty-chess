#![allow(overflowing_literals)]

use std::collections::HashMap;
use crate::sq::*;
use crate::color::Color::*;
use crate::piece::{*, PieceKind::*};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
pub struct Board {
  pieces: HashMap<Sq, Piece>
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut output = String::new();
    for y in 0..8 {
      for x in 0..8 {
        let content = self.at(Sq { x, y });
        let ch = match content {
          Some(p) => p.char(),
          None => '.'
        };
        output.push(ch);
      }
      output.push('\n');
    }
    write!(f, "{}", output)
  }
}

impl Board {
  pub fn at(&self, sq: Sq) -> Option<&Piece> {
    self.pieces.get(&sq)
  }

  pub fn piece_at(&self, sq: Sq) -> Piece {
    self.pieces[&sq]
  }

  pub fn set_at(&mut self, sq: Sq, piece: Piece) {
    self.pieces.insert(sq, piece);
  }

  pub fn clear(&mut self, sq: Sq) {
    self.pieces.remove(&sq);
  }

  pub fn new() -> Board {
    Board { pieces: HashMap::new() }
  }

  pub fn from_file(file: &'static str) -> Board {
    let mut f = File::open(file).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file");
    let mut result = Board::new();
    for (y, line) in contents.lines().enumerate() {
      for (x, ch) in line.chars().enumerate() {
        if ch != '.' {
          let color = if ch.is_ascii_uppercase() { White } else { Black };
          let kind = match ch.to_ascii_uppercase() {
            'P' => Pawn,
            'N' => Knight,
            'B' => Bishop,
            'R' => Rook,
            'Q' => Queen,
            'K' => King,
             _  => panic!("Unexpected character")
          };
          result.set_at(
            Sq { x: x as i32, y: y as i32 },
            Piece { color, kind }
          )
        }
      }
    }
    result
  }
}

lazy_static! {
  pub static ref INITIAL_BOARD: Board = Board::from_file("boards/initial.txt");
}

use std::collections::HashMap;
use std::ops::{self, RangeFrom};
use nom::{Parser, InputLength, Slice, AsChar, InputIter};
use nom::character::complete::newline;
use nom::error::ParseError;
use nom::{
  multi::{many1, separated_list1},
  IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
  pub x: i32,
  pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos64 {
  pub x: i64,
  pub y: i64,
}

pub type Grid<A> = HashMap<Pos, A>;

impl ops::Add<Pos> for Pos {
  type Output = Pos;

  fn add(self, rhs: Pos) -> Pos {
    Pos {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl ops::Add<Pos64> for Pos64 {
  type Output = Pos64;

  fn add(self, rhs: Pos64) -> Pos64 {
    Pos64 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

pub fn rotate_cw(pos: Pos) -> Pos {
  Pos {
    x: -pos.y,
    y: pos.x,
  }
}

pub fn rotate_ccw(pos: Pos) -> Pos {
  Pos {
    x: pos.y,
    y: -pos.x,
  }
}

pub fn parse_grid<I, O, E, F>(mut one_char_parser: F) -> impl FnMut(I) -> IResult<I, HashMap<Pos, O>, E>
  where I: Slice<RangeFrom<usize>> + InputIter + Clone + InputLength,
        <I as InputIter>::Item: AsChar,
        F: Parser<I, O, E>,
        E: ParseError<I>,
        O: Copy,
{ move |input: I| {
    let (input, rows) = separated_list1(newline, many1(|x| one_char_parser.parse(x)))(input)?;
    let mut res = HashMap::new();
    for (y, row) in rows.iter().enumerate() {
      for (x, v) in row.iter().enumerate() {
        res.insert(Pos{x: x as i32, y: y as i32}, *v);
      }
    }
    Ok((input, res))
  }
}

pub fn grid_max_dims<A>(grid: &Grid<A>) -> (i32, i32) {
  grid.iter().fold((0, 0), |(maxx, maxy), (p, _)| {
    (i32::max(maxx, p.x), i32::max(maxy, p.y))
  })
}
use crate::utils::*;
use itertools::Itertools;
use nom::character::complete::{char, hex_digit1, i32, newline, space1};
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::{branch::alt, combinator::map as pmap, IResult};

use crate::days::Day;

pub struct Day18;

#[derive(Debug, Clone)]
pub struct Instruction {
  direction: Direction,
  distance: i32,
  distance2: i64,
  direction2: Direction,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
  L,
  R,
  U,
  D,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
  let (input, direction) = alt((
    pmap(char('L'), |_| Direction::L),
    pmap(char('R'), |_| Direction::R),
    pmap(char('U'), |_| Direction::U),
    pmap(char('D'), |_| Direction::D),
  ))(input)?;
  let (input, _) = space1(input)?;
  let (input, distance) = i32(input)?;
  let (input, _) = space1(input)?;
  let (input, _) = pair(char('('), char('#'))(input)?;
  let (input, colour) = hex_digit1(input)?;
  let (input, _) = char(')')(input)?;
  let mut distance2 = 0;
  for c in colour.chars().take(5) {
    distance2 *= 16;
    distance2 += c.to_digit(16).unwrap();
  }
  let last_digit = colour.chars().last().unwrap();
  let direction2 = match last_digit {
    '0' => Direction::R,
    '1' => Direction::D,
    '2' => Direction::L,
    '3' => Direction::U,
    _ => panic!("invalid last digit"),
  };
  Ok((
    input,
    Instruction {
      direction,
      distance,
      distance2: distance2 as i64,
      direction2,
    },
  ))
}

fn run(
  input: &[Instruction],
  get_distance: fn(&Instruction) -> i64,
  get_direction: fn(&Instruction) -> Direction,
) -> i64 {
  let mut lines: Vec<(Pos64, Pos64)> = vec![];
  let mut current_pos = Pos64 { x: 0, y: 0 };
  for ins in input {
    let dir = match get_direction(ins) {
      Direction::L => Pos64 {
        x: -get_distance(ins),
        y: 0,
      },
      Direction::R => Pos64 {
        x: get_distance(ins),
        y: 0,
      },
      Direction::U => Pos64 {
        x: 0,
        y: -get_distance(ins),
      },
      Direction::D => Pos64 {
        x: 0,
        y: get_distance(ins),
      },
    };
    let end = current_pos + dir;
    lines.push((current_pos, end));
    current_pos = end;
  }
  let mut res = 0;
  let (miny, maxy) = lines.iter().fold(
    (i64::MAX, i64::MIN),
    |(miny, maxy), (Pos64 { x: _, y: y1 }, Pos64 { x: _, y: y2 })| {
      (
        i64::min(*y2, i64::min(*y1, miny)),
        i64::max(*y2, i64::max(*y1, maxy)),
      )
    },
  );
  for y in miny..=maxy {
    let mut xes: Vec<(i64, i64, bool)> = lines
      .iter()
      .filter(|(p1, p2)| {
        p1.y != p2.y && i64::min(p1.y, p2.y) <= y && i64::max(p1.y, p2.y) >= y
      })
      .map(|(p1, p2)| (p1.x, p2.x, false))
      .sorted_by(|v1, v2| i64::cmp(&v1.0, &v2.0))
      .collect();
    let mut i = 0;
    while i < xes.len() - 1 {
      let (x1, _, _) = xes[i];
      let (x2, _, _) = xes[i + 1];
      if lines.iter().any(|(p1, p2)| {
        p1.y == y && p2.y == y && ((p1.x == x1 && p2.x == x2) || (p2.x == x1 && p1.x == x2))
      }) {
        let mut is_island = false;
        if let (Some(a), Some(b)) = (
          lines
            .iter()
            .find(|(p1, p2)| p1.x == p2.x && p1.x == x1 && (p1.y == y || p2.y == y)),
          lines
            .iter()
            .find(|(p1, p2)| p1.x == p2.x && p1.x == x2 && (p1.y == y || p2.y == y)),
        ) {
          let a_miny = i64::min(a.0.y, a.1.y);
          let a_maxy = i64::max(a.0.y, a.1.y);
          let b_miny = i64::min(b.0.y, b.1.y);
          let b_maxy = i64::max(b.0.y, b.1.y);
          is_island = (a_maxy > y && b_maxy > y) || (a_miny < y && b_miny < y);
        }
        xes[i] = (x1, x2, is_island);
        xes.remove(i + 1);
      }
      i += 1;
    }

    if xes.is_empty() {
      continue;
    } else {
      let mut last = &xes[0];
      let mut inside = !last.2;
      for current in &xes[1..] {
        let (x1, x2, is_island) = current;
        if inside {
          res += x1 - last.1 - 1;
        }
        if x1 == x2 || !is_island {
          inside = !inside;
        }
        last = current;
      }
    }
  }

  res + lines
    .iter()
    .map(|(p1, p2)| i64::abs(p2.x - p1.x) + i64::abs(p2.y - p1.y))
    .sum::<i64>()
}

impl Day for Day18 {
  type Input = Vec<Instruction>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list1(newline, parse_instruction)(input)
  }

  type Output1 = i64;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    run(input, |i| i.distance as i64, |i| i.direction)
  }

  type Output2 = i64;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    run(input, |i| i.distance2, |i| i.direction2)
  }
}

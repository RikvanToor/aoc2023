use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, u32};
use nom::combinator::map as pmap;
use nom::multi::separated_list0;
use nom::sequence::pair;
use nom::IResult;

use crate::days::Day;

pub struct Day02;

#[derive(Debug)]
enum Colour {
  Red,
  Green,
  Blue,
}

#[derive(Debug)]
struct CubeSet {
  red: u32,
  green: u32,
  blue: u32,
}

#[derive(Debug)]
pub struct Game {
  id: u32,
  cubes: Vec<CubeSet>,
}

fn parse_cube(input: &str) -> IResult<&str, (Colour, u32)> {
  alt((
    pmap(pair(u32, tag(" red")), |(n, _)| (Colour::Red, n)),
    pmap(pair(u32, tag(" green")), |(n, _)| (Colour::Green, n)),
    pmap(pair(u32, tag(" blue")), |(n, _)| (Colour::Blue, n)),
  ))(input)
}

fn parse_cube_set(input: &str) -> IResult<&str, CubeSet> {
  let (input, cubes) = separated_list0(tag(", "), parse_cube)(input)?;
  let mut res = CubeSet {
    red: 0,
    green: 0,
    blue: 0,
  };
  for (c, n) in cubes.iter() {
    match c {
      Colour::Red => res.red += n,
      Colour::Green => res.green += n,
      Colour::Blue => res.blue += n,
    }
  }
  Ok((input, res))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
  let (input, _) = tag("Game ")(input)?;
  let (input, id) = u32(input)?;
  let (input, _) = tag(": ")(input)?;
  let (input, cubes) = separated_list0(tag("; "), parse_cube_set)(input)?;
  let res = Game { id, cubes };
  Ok((input, res))
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

impl Day for Day02 {
  type Input = Vec<Game>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, parse_game)(input)
  }

  type Output1 = u32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    input.iter().fold(0, |acc, Game { id, cubes }| {
      if cubes
        .iter()
        .all(|c| c.red <= MAX_RED && c.green <= MAX_GREEN && c.blue <= MAX_BLUE)
      {
        acc + id
      } else {
        acc
      }
    })
  }

  type Output2 = u32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    input
      .iter()
      .map(|Game { id: _, cubes }| {
        *(cubes.iter().map(|cs| cs.red).max().get_or_insert(MAX_RED))
        * *(cubes.iter().map(|cs| cs.green).max().get_or_insert(MAX_GREEN))
        * *(cubes.iter().map(|cs| cs.blue).max().get_or_insert(MAX_BLUE))
      })
      .sum()
  }
}

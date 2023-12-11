use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::combinator::map as pmap;
use nom::multi::{many1, separated_list0};
use nom::IResult;
use std::collections::HashSet;

use crate::days::Day;

pub struct Day11;

#[derive(Debug, PartialEq)]
pub enum Pixel {
  Galaxy,
  Empty,
}

fn parse_pixel(input: &str) -> IResult<&str, Pixel> {
  alt((
    pmap(char('#'), |_| Pixel::Galaxy),
    pmap(char('.'), |_| Pixel::Empty),
  ))(input)
}

fn solve(steps: usize, input: &[Vec<Pixel>]) -> usize {
  let mut galaxies: HashSet<(usize, usize)> = input
    .iter()
    .enumerate()
    .flat_map(|(y, row)| {
      row.iter()
        .enumerate()
        .filter_map(|(x, p)| match *p {
          Pixel::Galaxy => Some((x, y)),
          _ => None,
        })
        .collect::<Vec<(usize, usize)>>()
    })
    .collect();
  let mut y = 0;
  let mut max_y = input.len() - 1;
  while y <= max_y {
    if galaxies.iter().filter(|(_, gy)| *gy == y).count() == 0 {
      galaxies = galaxies
        .iter()
        .map(|(gx, gy)| (*gx, if *gy > y { gy + steps - 1 } else { *gy }))
        .collect();
      max_y += steps - 1;
      y += steps;
    } else {
      y += 1;
    }
  }
  let mut x = 0;
  let mut max_x = input[0].len() - 1;
  while x <= max_x {
    if galaxies.iter().filter(|(gx, _)| *gx == x).count() == 0 {
      galaxies = galaxies
        .iter()
        .map(|(gx, gy)| (if *gx > x { gx + steps - 1 } else { *gx }, *gy))
        .collect();
      max_x += steps - 1;
      x += steps;
    } else {
      x += 1;
    }
  }

  galaxies
    .iter()
    .enumerate()
    .map(|(i, (gx, gy))| {
      galaxies
        .clone()
        .iter()
        .skip(i + 1)
        .map(|(gx2, gy2)| gx.abs_diff(*gx2) + gy.abs_diff(*gy2))
        .sum::<usize>()
    })
    .sum()
}

impl Day for Day11 {
  type Input = Vec<Vec<Pixel>>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, many1(parse_pixel))(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    solve(2, input)
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    solve(1000000, input)
  }
}

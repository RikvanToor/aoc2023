use nom::character::complete::{i32, newline, space1};
use nom::multi::separated_list0;
use nom::IResult;

use crate::days::Day;

pub struct Day09;

fn estimate_next(row: &[i32]) -> i32 {
  let differences: Vec<i32> = (1..row.len()).map(|i| row[i] - row[i - 1]).collect();
  let first = differences.first();
  // Check if they're all equal
  row.last().unwrap()
    + if differences
      .iter()
      .fold(first, |acc, x| if Some(x) == acc { acc } else { None })
      .is_some()
    {
      *first.unwrap()
    } else {
      estimate_next(&differences)
    }
}

impl Day for Day09 {
  type Input = Vec<Vec<i32>>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, separated_list0(space1, i32))(input)
  }

  type Output1 = i32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    input.iter().map(|x| estimate_next(x)).sum::<i32>()
  }

  type Output2 = i32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    input
      .iter()
      .map(|x| estimate_next(&x.iter().rev().copied().collect::<Vec<i32>>()))
      .sum::<i32>()
  }
}

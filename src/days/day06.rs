use nom::bytes::complete::tag;
use nom::character::complete::{newline, space1, u64};
use nom::multi::separated_list0;
use nom::IResult;

use crate::days::Day;

pub struct Day06;

fn is_winning_tactic(time: &u64, distance: &u64, charge_time: usize) -> bool {
  let run_time = *time as usize - charge_time;
  let res = run_time * charge_time;
  res > *distance as usize
}

fn run_race((time, distance): &(u64, u64)) -> usize {
  // Binary search would be great here, but I'm too lazy to do it
  let min = (1..*time as usize)
    .find(|charge_time| is_winning_tactic(time, distance, *charge_time))
    .unwrap();
  let max = (1..*time as usize)
    .rev()
    .find(|charge_time| is_winning_tactic(time, distance, *charge_time))
    .unwrap();
  max - min + 1
}

fn combine_numbers(input: &[u64]) -> u64 {
  let mut res: u64 = **input.first().get_or_insert(&0);
  for i in input.iter().skip(1) {
    for j in 1..=10 {
      if u64::pow(10, j) >= *i {
        res *= u64::pow(10, j);
        res += *i;
        break;
      }
    }
  }
  res
}

impl Day for Day06 {
  type Input = Vec<(u64, u64)>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list0(space1, u64)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = separated_list0(space1, u64)(input)?;
    Ok((input, times.into_iter().zip(distances).collect()))
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    input.iter().map(run_race).product::<usize>()
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let time = combine_numbers(&input.iter().map(|x| x.0).collect::<Vec<u64>>());
    let distance = combine_numbers(&input.iter().map(|x| x.1).collect::<Vec<u64>>());
    run_race(&(time, distance))
  }
}

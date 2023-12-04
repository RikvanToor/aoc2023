use nom::bytes::complete::tag;
use nom::{
  character::complete::{newline, space1, u32},
  multi::separated_list0,
  IResult,
};

use crate::days::Day;

pub struct Day04;

#[derive(Debug)]
pub struct Card {
  winning: Vec<u32>,
  having: Vec<u32>,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
  let (input, _) = tag("Card")(input)?;
  let (input, _) = space1(input)?;
  let (input, _) = u32(input)?;
  let (input, _) = tag(":")(input)?;
  let (input, _) = space1(input)?;
  let (input, winning) = separated_list0(space1, u32)(input)?;
  let (input, _) = space1(input)?;
  let (input, _) = tag("|")(input)?;
  let (input, _) = space1(input)?;
  let (input, having) = separated_list0(space1, u32)(input)?;
  Ok((input, Card { winning, having }))
}

impl Day for Day04 {
  type Input = Vec<Card>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, parse_card)(input)
  }

  type Output1 = u32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    input
      .iter()
      .map(|c| 1 << (c.having.iter().filter(|n| c.winning.contains(n)).count()) >> 1)
      .sum()
  }

  type Output2 = u32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let mut matching: Vec<(u32, usize)> = input
      .iter()
      .map(|c| (1, c.having.iter().filter(|n| c.winning.contains(n)).count()))
      .collect();
    for i in 0..matching.len() {
      let (amount, points) = matching[i];
      for other_card in matching.iter_mut().take(i + points + 1).skip(i + 1) {
        other_card.0 += amount;
      }
    }
    matching.iter().map(|(amount, _)| amount).sum()
  }
}

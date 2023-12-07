use crate::days::Day;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, newline, space1, u64};
use nom::combinator::map as pmap;
use nom::multi::{many0, separated_list0};
use nom::IResult;
use std::cmp::Ordering;

pub struct Day07;

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone, Copy)]
pub enum Card {
  C2,
  C3,
  C4,
  C5,
  C6,
  C7,
  C8,
  C9,
  T,
  J,
  Q,
  K,
  A,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
pub struct Hand {
  cards: Vec<Card>,
  bid: u64,
}

fn groups_to_score(groups: &[usize]) -> usize {
  match groups[..] {
    [1, 1, 1, 1, 1] => 1,
    [1, 1, 1, 2] => 2,
    [1, 2, 2] => 3,
    [1, 1, 3] => 4,
    [2, 3] => 5,
    [1, 4] => 6,
    [5] => 7,
    _ => 0,
  }
}

fn cards_to_score(cs: &[Card]) -> usize {
  groups_to_score(
    &cs.iter()
      .sorted()
      .group_by(|c| *c)
      .into_iter()
      .map(|(_, cs)| cs.count())
      .sorted()
      .collect::<Vec<usize>>(),
  )
}

fn cmp_hands(h1: &Hand, h2: &Hand) -> Ordering {
  let h1_score = cards_to_score(&h1.cards);
  let h2_score = cards_to_score(&h2.cards);
  if h1_score == h2_score {
    h1.cards.cmp(&h2.cards)
  } else {
    h1_score.cmp(&h2_score)
  }
}

// Replace all Js with the most occurring other type of card, if there is any.
fn replace_js(cs: &[Card]) -> Vec<Card> {
  let max = **cs
    .iter()
    .filter(|c| **c != Card::J)
    .sorted()
    .group_by(|c| *c)
    .into_iter()
    .map(|(k, cs)| (k, cs.count()))
    .max_by(|a, b| a.1.cmp(&b.1))
    .map(|(c, _)| c)
    .get_or_insert(&Card::J);
  cs.iter()
    .map(|c| match c {
      Card::J => max,
      _ => *c,
    })
    .collect()
}

// Compare hands by counting all Js with the most occurring other type of card,
// but making Js the weakest possible card
fn cmp_hands_2(h1: &Hand, h2: &Hand) -> Ordering {
  let new_h1_cards = replace_js(&h1.cards);
  let new_h2_cards = replace_js(&h2.cards);
  let h1_score = cards_to_score(&new_h1_cards);
  let h2_score = cards_to_score(&new_h2_cards);
  if h1_score == h2_score {
    cmp_joker_vec(&h1.cards, &h2.cards)
  } else {
    h1_score.cmp(&h2_score)
  }
}

fn cmp_jokers(c1: &Card, c2: &Card) -> Ordering {
  match (c1, c2) {
    (Card::J, Card::J) => Ordering::Equal,
    (Card::J, _) => Ordering::Less,
    (_, Card::J) => Ordering::Greater,
    _ => c1.cmp(c2),
  }
}

fn cmp_joker_vec(cs1: &[Card], cs2: &[Card]) -> Ordering {
  match (cs1.first(), cs2.first()) {
    (None, None) => Ordering::Equal,
    (Some(_), None) => Ordering::Greater,
    (None, Some(_)) => Ordering::Less,
    (Some(c1), Some(c2)) => match cmp_jokers(c1, c2) {
      Ordering::Equal => cmp_joker_vec(&cs1[1..], &cs2[1..]),
      o => o,
    },
  }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
  alt((
    pmap(char('2'), |_| Card::C2),
    pmap(char('3'), |_| Card::C3),
    pmap(char('4'), |_| Card::C4),
    pmap(char('5'), |_| Card::C5),
    pmap(char('6'), |_| Card::C6),
    pmap(char('7'), |_| Card::C7),
    pmap(char('8'), |_| Card::C8),
    pmap(char('9'), |_| Card::C9),
    pmap(char('T'), |_| Card::T),
    pmap(char('J'), |_| Card::J),
    pmap(char('Q'), |_| Card::Q),
    pmap(char('K'), |_| Card::K),
    pmap(char('A'), |_| Card::A),
  ))(input)
}

fn parse_line(input: &str) -> IResult<&str, Hand> {
  let (input, cards) = many0(parse_card)(input)?;
  let (input, _) = space1(input)?;
  let (input, bid) = u64(input)?;
  let res = Hand { cards, bid };
  Ok((input, res))
}

impl Day for Day07 {
  type Input = Vec<Hand>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, parse_line)(input)
  }

  type Output1 = u64;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    input
      .iter()
      .sorted_by(|h1, h2| cmp_hands(h1, h2))
      .zip(1..=input.len())
      .map(|(h, rank)| h.bid * rank as u64)
      .sum()
  }

  type Output2 = u64;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    input
      .iter()
      .sorted_by(|h1, h2| cmp_hands_2(h1, h2))
      .zip(1..=input.len())
      .map(|(h, rank)| h.bid * rank as u64)
      .sum()
  }
}

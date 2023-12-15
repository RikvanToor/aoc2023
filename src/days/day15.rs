use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, none_of, u32};
use nom::combinator::map as pmap;
use nom::{
  multi::{many1, separated_list0},
  sequence::pair,
  IResult,
};
use std::collections::HashMap;

use crate::days::Day;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
  Add(Vec<char>, u32),
  Remove(Vec<char>),
}

fn parse_node(input: &str) -> IResult<&str, Node> {
  alt((
    pmap(
      pair(many1(none_of("-=")), pair(char('='), u32)),
      |(cs, (_, n))| Node::Add(cs, n),
    ),
    pmap(pair(many1(none_of("-=")), char('-')), |(cs, _)| {
      Node::Remove(cs)
    }),
  ))(input)
}

fn get_hash(cs: &[char]) -> u32 {
  cs.iter().fold(0, |acc, c| ((acc + *c as u32) * 17) % 256)
}

pub struct Day15;

impl Day for Day15 {
  type Input = Vec<Vec<char>>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (input, res) = separated_list0(char(','), many1(none_of(",")))(input)?;
    Ok((input, res))
  }

  type Output1 = u32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    input.iter().map(|x| get_hash(x)).sum::<u32>()
  }

  type Output2 = u32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let nodes: Vec<Node> = input
      .iter()
      .map(|s| {
        let s1 = s.iter().collect::<String>();
        parse_node(&s1).unwrap().1
      })
      .collect();
    let mut hm: HashMap<u32, Vec<(Vec<char>, u32)>> = HashMap::new();
    for i in 0..256 {
      hm.insert(i, vec![]);
    }

    for n in nodes {
      match n {
        Node::Add(cs, nr) => {
          let h = get_hash(&cs);
          let vec = hm.get_mut(&h).unwrap();
          match vec.iter().find_position(|(cs2, _)| cs2 == &cs) {
            Some((i, _)) => {
              vec.remove(i);
              vec.insert(i, (cs, nr));
            }
            None => {
              vec.push((cs, nr));
            }
          }
        }
        Node::Remove(cs) => {
          let h = get_hash(&cs);
          let vec = hm.get(&h).unwrap();
          let new_vec = vec.iter().filter(|(cs2, _)| cs2 != &cs).cloned().collect();
          hm.insert(h, new_vec);
        }
      }
    }

    hm.iter()
      .map(|(i, nodes)| {
        nodes
          .iter()
          .enumerate()
          .map(|(j, (_, nr))| (*i + 1) * nr * (j as u32 + 1))
          .sum::<u32>()
      })
      .sum()
  }
}

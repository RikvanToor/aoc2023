use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char, newline};
use nom::combinator::map as pmap;
use nom::multi::{many1, separated_list0};
use nom::IResult;
use num::integer::lcm;
use std::collections::HashMap;

use crate::days::Day;

pub struct Day08;

#[derive(Debug, Clone, Copy)]
pub enum Dir {
  L,
  R,
}

type Node = (String, (String, String));

#[derive(Debug)]
pub struct Instructions {
  dirs: Vec<Dir>,
  nodes: HashMap<String, (String, String)>,
}

fn parse_node(input: &str) -> IResult<&str, Node> {
  let (input, own) = alphanumeric1(input)?;
  let (input, _) = tag(" = (")(input)?;
  let (input, left) = alphanumeric1(input)?;
  let (input, _) = tag(", ")(input)?;
  let (input, right) = alphanumeric1(input)?;
  let (input, _) = tag(")")(input)?;
  Ok((input, (own.to_owned(), (left.to_owned(), right.to_owned()))))
}

fn lcms(nrs: &[usize]) -> usize {
  let mut res: usize = **nrs.first().get_or_insert(&0);
  for n in nrs.iter().skip(1) {
    res = lcm(*n, res);
  }
  res
}

impl Day for Day08 {
  type Input = Instructions;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (input, dirs) = many1(alt((
      pmap(char('L'), |_| Dir::L),
      pmap(char('R'), |_| Dir::R),
    )))(input)?;
    let (input, _) = many1(newline)(input)?;
    let (input, nodes) = separated_list0(newline, parse_node)(input)?;
    let m = HashMap::from_iter(nodes);
    let res = Instructions { dirs, nodes: m };
    Ok((input, res))
  }

  type Output1 = u32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let mut pos = "AAA".to_string();
    for i in 0.. {
      if pos == "ZZZ" {
        return i as u32;
      }
      let current_node = input.nodes[&pos].clone();
      match input.dirs[i % input.dirs.len()] {
        Dir::L => pos = current_node.0,
        Dir::R => pos = current_node.1,
      }
    }
    0
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let mut poses: Vec<String> = input
      .nodes
      .keys()
      .filter(|s| s.chars().nth(2) == Some('A'))
      .cloned()
      .collect();
    let mut steps_needed: Vec<usize> = vec![];
    for i in 0.. {
      if poses.is_empty() {
        break;
      }
      poses = poses
        .iter()
        .filter_map(|pos| {
          if pos.chars().nth(2) == Some('Z') {
            steps_needed.push(i);
            None
          } else {
            let current_node = input.nodes[pos].clone();
            match input.dirs[i % input.dirs.len()] {
              Dir::L => Some(current_node.0),
              Dir::R => Some(current_node.1),
            }
          }
        })
        .collect();
    }
    lcms(&steps_needed)
  }
}

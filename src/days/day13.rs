use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::combinator::map as pmap;
use nom::multi::{many1, separated_list0};
use nom::sequence::pair;
use nom::IResult;

use crate::days::Day;

pub struct Day13;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Node {
  Rock,
  Ash,
}

type Map = Vec<Vec<Node>>;

fn parse_map(input: &str) -> IResult<&str, Map> {
  separated_list0(
    newline,
    many1(alt((
      pmap(char('.'), |_| Node::Ash),
      pmap(char('#'), |_| Node::Rock),
    ))),
  )(input)
}

fn solve(m: &Map) -> (Vec<usize>, Vec<usize>) {
  let mut hor = vec![];
  let mut ver = vec![];
  for y in 0..m.len() - 1 {
    let mut eq = true;
    for y2 in 0..=usize::min(y, m.len() - y - 2) {
      if *m[y - y2] != *m[y + 1 + y2] {
        eq = false;
        break;
      }
    }
    if eq {
      hor.push(y + 1);
    }
  }
  for x in 0..m[0].len() - 1 {
    let mut eq = true;
    for x2 in 0..=usize::min(x, m[0].len() - x - 2) {
      for row in m {
        if row[x - x2] != row[x + 1 + x2] {
          eq = false;
          break;
        }
      }
      if !eq {
        break;
      }
    }
    if eq {
      ver.push(x + 1);
    }
  }
  (hor, ver)
}

impl Day for Day13 {
  type Input = Vec<Map>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(pair(newline, newline), parse_map)(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    input
      .iter()
      .map(|m| {
        let (hor, ver) = solve(m);
        hor.iter().sum::<usize>() * 100 + ver.iter().sum::<usize>()
      })
      .sum()
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    input
      .iter()
      .map(|m| {
        let solution = solve(m);
        for y in 0..m.len() {
          for x in 0..m[0].len() {
            let mut m2 = m.clone();
            if m2[y][x] == Node::Rock {
              m2[y][x] = Node::Ash;
            } else {
              m2[y][x] = Node::Rock;
            }
            let solution2 = solve(&m2);
            if solution2
              .0
              .iter()
              .filter(|i| !solution.0.contains(i))
              .count()
              >= 1
              || solution2
                .1
                .iter()
                .filter(|i| !solution.1.contains(i))
                .count()
                >= 1
            {
              let new_hors = solution2
                .0
                .iter()
                .filter(|i| !solution.0.contains(i))
                .sum::<usize>();
              let new_vers = solution2
                .1
                .iter()
                .filter(|i| !solution.1.contains(i))
                .sum::<usize>();
              return new_hors * 100 + new_vers;
            }
          }
        }
        0
      })
      .sum()
  }
}

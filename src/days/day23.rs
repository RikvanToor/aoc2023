use crate::utils::*;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map as pmap;
use nom::IResult;
use std::collections::{HashMap, HashSet};

use crate::days::Day;

pub struct Day23;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
  Path,
  Forest,
  Slope(Pos),
}

fn parse_tile(input: &str) -> IResult<&str, Tile> {
  alt((
    pmap(char('.'), |_| Tile::Path),
    pmap(char('#'), |_| Tile::Forest),
    pmap(char('^'), |_| Tile::Slope(Pos { x: 0, y: -1 })),
    pmap(char('>'), |_| Tile::Slope(Pos { x: 1, y: 0 })),
    pmap(char('v'), |_| Tile::Slope(Pos { x: 0, y: 1 })),
    pmap(char('<'), |_| Tile::Slope(Pos { x: -1, y: 0 })),
  ))(input)
}

fn step(grid: &Grid<Tile>, pos: &Pos, goal: &Pos, seen: &HashSet<Pos>) -> usize {
  if pos == goal {
    0
  } else {
    let mut new_seen = seen.clone();
    new_seen.insert(*pos);
    let new_poses = match grid.get(pos) {
      None => vec![],
      Some(Tile::Forest) => vec![],
      Some(Tile::Path) => vec![
        Pos {
          x: pos.x + 1,
          y: pos.y,
        },
        Pos {
          x: pos.x - 1,
          y: pos.y,
        },
        Pos {
          x: pos.x,
          y: pos.y + 1,
        },
        Pos {
          x: pos.x,
          y: pos.y - 1,
        },
      ],
      Some(Tile::Slope(p)) => vec![*pos + *p],
    };
    *new_poses
      .iter()
      .filter(|p| {
        !new_seen.contains(p)
          && matches!(grid.get(p), Some(Tile::Path) | Some(Tile::Slope(_)))
      })
      .map(|p| step(grid, p, goal, &new_seen))
      .max()
      .get_or_insert(0)
      + 1
  }
}

fn longest_path(grid: &Grid<Tile>, start: &Pos, end: &Pos) -> usize {
  let mut seen: HashSet<Pos> = HashSet::new();
  seen.insert(*start);
  step(grid, start, end, &seen)
}

fn step2(
  pos: &Pos,
  goal: &Pos,
  seen: &HashSet<Pos>,
  successors: &HashMap<Pos, Vec<(Pos, usize)>>,
) -> usize {
  if pos == goal {
    0
  } else {
    let mut new_seen = seen.clone();
    new_seen.insert(*pos);
    *successors
      .get(pos)
      .unwrap()
      .iter()
      .filter(|(p, _)| !new_seen.contains(p))
      .map(|(p, d)| step2(p, goal, &new_seen, successors) + *d)
      .max()
      .get_or_insert(0)
  }
}

fn longest_path2(start: &Pos, end: &Pos, successors: &HashMap<Pos, Vec<(Pos, usize)>>) -> usize {
  let seen: HashSet<Pos> = HashSet::new();
  step2(start, end, &seen, successors)
}

fn create_successor_map<F>(grid: &Grid<Tile>, f: F) -> HashMap<Pos, Vec<(Pos, usize)>>
where
  F: Fn(&Pos) -> Vec<Pos>,
{
  let mut res = grid
    .iter()
    .filter_map(|(pos, t)| match t {
      Tile::Forest => None,
      _ => Some((*pos, f(pos).iter().map(|p| (*p, 1)).collect())),
    })
    .collect();
  while simplify_step(&mut res) {}
  res
}

fn simplify_step(successors: &mut HashMap<Pos, Vec<(Pos, usize)>>) -> bool {
  for (p, nexts) in successors.clone() {
    if nexts.len() == 2 {
      let (p1, n1) = nexts[0];
      let (p2, n2) = nexts[1];
      let s1 = successors.get_mut(&p1).unwrap();
      s1.retain(|(q, _)| *q != p);
      s1.push((p2, n2 + n1));
      let s2 = successors.get_mut(&p2).unwrap();
      s2.retain(|(q, _)| *q != p);
      s2.push((p1, n2 + n1));
      successors.remove(&p);
      return true;
    }
  }
  false
}

impl Day for Day23 {
  type Input = Grid<Tile>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    parse_grid(parse_tile)(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let (start, _) = input
      .iter()
      .find(|(p, t)| p.y == 0 && **t == Tile::Path)
      .unwrap();
    let (_, maxy) = grid_max_dims(input);
    let (end, _) = input
      .iter()
      .find(|(p, t)| p.y == maxy && **t == Tile::Path)
      .unwrap();
    longest_path(input, start, end)
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let (start, _) = input
      .iter()
      .find(|(p, t)| p.y == 0 && **t == Tile::Path)
      .unwrap();
    let (_, maxy) = grid_max_dims(input);
    let (end, _) = input
      .iter()
      .find(|(p, t)| p.y == maxy && **t == Tile::Path)
      .unwrap();

    let successors: HashMap<Pos, Vec<(Pos, usize)>> = create_successor_map(input, |pos| {
      [
        Pos {
          x: pos.x + 1,
          y: pos.y,
        },
        Pos {
          x: pos.x - 1,
          y: pos.y,
        },
        Pos {
          x: pos.x,
          y: pos.y + 1,
        },
        Pos {
          x: pos.x,
          y: pos.y - 1,
        },
      ]
      .into_iter()
      .filter(|p| matches!(input.get(p), Some(Tile::Path) | Some(Tile::Slope(_))))
      .collect::<Vec<Pos>>()
    });
    longest_path2(start, end, &successors)
  }
}

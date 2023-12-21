use std::collections::{HashSet, VecDeque};

use crate::utils::*;
use nom::character::complete::one_of;
use nom::IResult;

use crate::days::Day;

pub struct Day21;

fn run(input: &Grid<char>, extend: i32) -> usize {
  let rocks: HashSet<Pos> = input
    .iter()
    .filter_map(|(pos, c)| (*c == '#').then_some(*pos))
    .collect();
  let max_steps = 65 + extend * 131;
  let mut q: VecDeque<(Pos, i32)> = VecDeque::new();
  let mut seen: HashSet<Pos> = HashSet::new();
  let mut counter = 0;
  let start = input
    .iter()
    .find_map(|(pos, c)| (*c == 'S').then_some(*pos))
    .unwrap();
  q.push_back((start, max_steps));
  while let Some((new_pos, to_go)) = q.pop_front() {
    if !rocks.contains(&Pos {
      x: new_pos.x.rem_euclid(131),
      y: new_pos.y.rem_euclid(131),
    }) && seen.insert(new_pos)
    {
      if to_go % 2 == 1 {
        counter += 1;
      }
      if to_go > 0 {
        q.push_back((
          Pos {
            x: new_pos.x + 1,
            y: new_pos.y,
          },
          to_go - 1,
        ));
        q.push_back((
          Pos {
            x: new_pos.x - 1,
            y: new_pos.y,
          },
          to_go - 1,
        ));
        q.push_back((
          Pos {
            x: new_pos.x,
            y: new_pos.y + 1,
          },
          to_go - 1,
        ));
        q.push_back((
          Pos {
            x: new_pos.x,
            y: new_pos.y - 1,
          },
          to_go - 1,
        ));
      }
    }
  }
  counter
}

impl Day for Day21 {
  type Input = Grid<char>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    parse_grid(one_of(".#S"))(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let rocks: HashSet<Pos> = input
      .iter()
      .filter_map(|(pos, c)| (*c == '#').then_some(*pos))
      .collect();
    let mut counter = 0;
    let mut seen: HashSet<Pos> = HashSet::new();
    let mut q: VecDeque<(Pos, usize)> = VecDeque::new();
    let start = input
      .iter()
      .find_map(|(pos, c)| if *c == 'S' { Some(*pos) } else { None })
      .unwrap();
    let steps = 64;
    q.push_back((start, steps));
    let (minx, maxx, miny, maxy) = input.iter().fold(
      (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
      |(minx, maxx, miny, maxy), (p, _)| {
        (
          i32::min(p.x, minx),
          i32::max(p.x, maxx),
          i32::min(p.y, miny),
          i32::max(p.y, maxy),
        )
      },
    );
    while let Some((new_pos, to_go)) = q.pop_front() {
      if !rocks.contains(&new_pos) && seen.insert(new_pos) {
        if to_go % 2 == steps % 2 {
          counter += 1;
        }
        if to_go > 0 {
          if new_pos.x < maxx {
            q.push_back((
              Pos {
                x: new_pos.x + 1,
                y: new_pos.y,
              },
              to_go - 1,
            ));
          }
          if new_pos.x > minx {
            q.push_back((
              Pos {
                x: new_pos.x - 1,
                y: new_pos.y,
              },
              to_go - 1,
            ));
          }
          if new_pos.y < maxy {
            q.push_back((
              Pos {
                x: new_pos.x,
                y: new_pos.y + 1,
              },
              to_go - 1,
            ));
          }
          if new_pos.y > miny {
            q.push_back((
              Pos {
                x: new_pos.x,
                y: new_pos.y - 1,
              },
              to_go - 1,
            ));
          }
        }
      }
    }
    counter
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let p0 = run(input, 0);
    let p1 = run(input, 1);
    let p2 = run(input, 2);
    let step1 = p1 - p0;
    let step2 = p2 - p1;
    let delta = step2 - step1;

    p1 + 202300 * step1 + (202300 / 2) * 202301 * delta
  }
}

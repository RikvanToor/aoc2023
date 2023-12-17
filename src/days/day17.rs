use crate::utils::*;
use nom::character::complete::one_of;
use nom::combinator::map as pmap;
use nom::IResult;
use pathfinding::directed::astar::astar;
use std::collections::HashMap;

use crate::days::Day;

fn get_new_states(
  input: &HashMap<Pos, i32>,
  (pos, dir, count): &(Pos, Pos, u16),
  straight_max: u16,
  turn_min: u16,
) -> Vec<((Pos, Pos, u16), i32)> {
  let mut new_dirs = vec![];
  if *count < straight_max {
    new_dirs.push((*dir, count + 1));
  }
  if *count >= turn_min {
    new_dirs.push((rotate_cw(*dir), 1));
    new_dirs.push((rotate_ccw(*dir), 1));
  }
  let res = new_dirs
    .iter()
    .filter_map(|(d, new_count)| {
      let new_pos = *pos + *d;
      let costs = input.get(&new_pos)?;
      Some(((new_pos, *d, *new_count), *costs))
    })
    .collect::<Vec<((Pos, Pos, u16), i32)>>();
  res
}

pub struct Day17;

impl Day for Day17 {
  type Input = Grid<i32>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    parse_grid(pmap(one_of("0123456789"), |c| {
      c.to_digit(10).unwrap() as i32
    }))(input)
  }

  type Output1 = i32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let (maxx, maxy) = input.iter().fold((0, 0), |(maxx, maxy), (p, _)| {
      (i32::max(maxx, p.x), i32::max(maxy, p.y))
    });
    let start_state = (Pos { x: 0, y: 0 }, Pos { x: 1, y: 0 }, 0);
    let res = astar(
      &start_state,
      |state| get_new_states(input, state, 3, 0),
      |_| 1,
      |(pos, _, _)| pos.x == maxx && pos.y == maxy,
    )
    .unwrap();
    res.1
  }

  type Output2 = i32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let (maxx, maxy) = input.iter().fold((0, 0), |(maxx, maxy), (p, _)| {
      (i32::max(maxx, p.x), i32::max(maxy, p.y))
    });
    let start_state = (Pos { x: 0, y: 0 }, Pos { x: 1, y: 0 }, 0);
    let res = astar(
      &start_state,
      |state| get_new_states(input, state, 10, 4),
      |_| 1,
      |(pos, _, c)| pos.x == maxx && pos.y == maxy && *c >= 4,
    )
    .unwrap();
    res.1
  }
}

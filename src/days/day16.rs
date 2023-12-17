use crate::utils::*;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map as pmap;
use nom::IResult;
use std::collections::HashSet;

use crate::days::Day;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
  Empty,
  NEMirror,
  SEMirror,
  HorSplitter,
  VerSplitter,
}

fn parse_tile(input: &str) -> IResult<&str, Tile> {
  alt((
    pmap(char('.'), |_| Tile::Empty),
    pmap(char('/'), |_| Tile::NEMirror),
    pmap(char('\\'), |_| Tile::SEMirror),
    pmap(char('-'), |_| Tile::HorSplitter),
    pmap(char('|'), |_| Tile::VerSplitter),
  ))(input)
}

fn run(map: &Grid<Tile>, current_pos: Pos, current_dir: Pos, seen: &mut HashSet<(Pos, Pos)>) {
  if let Some(t) = map.get(&current_pos) {
    let key = (current_pos, current_dir);
    if !seen.insert(key) {
      return;
    }
    match t {
      Tile::Empty => run(map, current_pos + current_dir, current_dir, seen),
      Tile::NEMirror => {
        let new_dir = if current_dir.y == 0 {
          rotate_ccw(current_dir)
        } else {
          rotate_cw(current_dir)
        };
        run(map, current_pos + new_dir, new_dir, seen);
      }
      Tile::SEMirror => {
        let new_dir = if current_dir.y == 0 {
          rotate_cw(current_dir)
        } else {
          rotate_ccw(current_dir)
        };
        run(map, current_pos + new_dir, new_dir, seen);
      }
      Tile::HorSplitter => {
        if current_dir.x == 0 {
          let new_pos_left = current_pos + Pos { x: -1, y: 0 };
          let new_pos_right = current_pos + Pos { x: 1, y: 0 };
          let new_dir_left = Pos { x: -1, y: 0 };
          let new_dir_right = Pos { x: 1, y: 0 };
          run(map, new_pos_left, new_dir_left, seen);
          run(map, new_pos_right, new_dir_right, seen);
        } else {
          run(map, current_pos + current_dir, current_dir, seen);
        }
      }
      Tile::VerSplitter => {
        if current_dir.y == 0 {
          let new_pos_up = current_pos + Pos { x: 0, y: -1 };
          let new_pos_down = current_pos + Pos { x: 0, y: 1 };
          let new_dir_up = Pos { x: 0, y: -1 };
          let new_dir_down = Pos { x: 0, y: 1 };
          run(map, new_pos_up, new_dir_up, seen);
          run(map, new_pos_down, new_dir_down, seen);
        } else {
          run(map, current_pos + current_dir, current_dir, seen);
        }
      }
    }
  }
}

fn solve(input: &Grid<Tile>, start_pos: Pos, start_dir: Pos) -> usize {
  let mut seen: HashSet<(Pos, Pos)> = HashSet::new();
  run(input, start_pos, start_dir, &mut seen);
  let energised: HashSet<&Pos> = seen.iter().map(|(pos, _)| pos).collect();
  energised.len()
}

pub struct Day16;

impl Day for Day16 {
  type Input = Grid<Tile>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    parse_grid(parse_tile)(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let start_pos = Pos { x: 0, y: 0 };
    let start_dir = Pos { x: 1, y: 0 };
    solve(input, start_pos, start_dir)
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let (maxx, maxy) = input.iter().fold((0, 0), |(maxx, maxy), (p, _)| {
      (i32::max(maxx, p.x), i32::max(maxy, p.y))
    });
    let mut res = 0;
    res = usize::max(
      res,
      (0..=maxx)
        .map(|x| solve(input, Pos { x, y: 0 }, Pos { x: 0, y: 1 }))
        .max()
        .unwrap_or(0),
    );
    res = usize::max(
      res,
      (0..=maxx)
        .map(|x| solve(input, Pos { x, y: maxy }, Pos { x: 0, y: -1 }))
        .max()
        .unwrap_or(0),
    );
    res = usize::max(
      res,
      (0..=maxy)
        .map(|y| solve(input, Pos { x: 0, y }, Pos { x: 1, y: 0 }))
        .max()
        .unwrap_or(0),
    );
    res = usize::max(
      res,
      (0..=maxy)
        .map(|y| solve(input, Pos { x: maxx, y }, Pos { x: -1, y: 0 }))
        .max()
        .unwrap_or(0),
    );
    res
  }
}

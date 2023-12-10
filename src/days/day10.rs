use std::collections::HashMap;

use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::combinator::map as pmap;
use nom::multi::{many1, separated_list0};
use nom::IResult;

use crate::days::Day;

pub struct Day10;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pipe {
  Hor,
  Ver,
  NE,
  NW,
  SW,
  SE,
  Start,
  Ground,
}

fn parse_pipe(input: &str) -> IResult<&str, Pipe> {
  alt((
    pmap(char('-'), |_| Pipe::Hor),
    pmap(char('|'), |_| Pipe::Ver),
    pmap(char('L'), |_| Pipe::NE),
    pmap(char('J'), |_| Pipe::NW),
    pmap(char('7'), |_| Pipe::SW),
    pmap(char('F'), |_| Pipe::SE),
    pmap(char('S'), |_| Pipe::Start),
    pmap(char('.'), |_| Pipe::Ground),
  ))(input)
}

fn next_move(
  hm: &HashMap<(i16, i16), Pipe>,
  prev_pos: (i16, i16),
  cur_pos: (i16, i16),
) -> Option<(i16, i16)> {
  let p = hm.get(&cur_pos)?;
  let (x, y) = cur_pos;
  let poses = match *p {
    Pipe::Hor => Some([(x - 1, y), (x + 1, y)]),
    Pipe::Ver => Some([(x, y - 1), (x, y + 1)]),
    Pipe::NE => Some([(x, y - 1), (x + 1, y)]),
    Pipe::NW => Some([(x, y - 1), (x - 1, y)]),
    Pipe::SW => Some([(x, y + 1), (x - 1, y)]),
    Pipe::SE => Some([(x, y + 1), (x + 1, y)]),
    _ => None,
  }?;
  let filtered: Vec<&(i16, i16)> = poses.iter().filter(|pos| **pos != prev_pos).collect();
  if filtered.len() != 1 {
    None
  } else {
    filtered.first().copied().copied()
  }
}

fn find_loop(hm: &HashMap<(i16, i16), Pipe>) -> Vec<(i16, i16)> {
  let (&start, _) = hm.iter().find(|(_, p)| p == &&Pipe::Start).unwrap();
  let mut loops: Vec<Vec<(i16, i16)>> = vec![];
  let (x, y) = start;
  for p in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
    let mut prev = start;
    let mut cur = p;
    let mut loop_pipes = vec![cur];
    while let Some(next_pos) = next_move(hm, prev, cur) {
      if next_pos == start {
        loops.push(loop_pipes);
        break;
      } else {
        loop_pipes.push(next_pos);
        prev = cur;
        cur = next_pos;
      }
    }
  }

  loops
    .iter()
    .max_by(|l1, l2| l1.len().cmp(&l2.len()))
    .unwrap()
    .to_owned()
}

fn flood_fill(pos: (i16, i16), l: &[(i16, i16)]) -> (bool, Vec<(i16, i16)>) {
  let mut res = vec![];
  let mut enclosed = true;
  let mut stack = vec![pos];
  let minx = l.iter().map(|p| p.0).min().unwrap();
  let maxx = l.iter().map(|p| p.0).max().unwrap();
  let miny = l.iter().map(|p| p.1).min().unwrap();
  let maxy = l.iter().map(|p| p.1).max().unwrap();
  while let Some(p) = stack.pop() {
    if !l.contains(&p) && !res.contains(&p) {
      res.push(p);
      let (x, y) = p;
      if x <= minx || x >= maxx || y <= miny || y >= maxy {
        enclosed = false;
        break;
      }
      stack.push((x - 1, y));
      stack.push((x + 1, y));
      stack.push((x, y - 1));
      stack.push((x, y + 1));
    }
  }

  (enclosed, res)
}

impl Day for Day10 {
  type Input = HashMap<(i16, i16), Pipe>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (input, pipes) = separated_list0(newline, many1(parse_pipe))(input)?;
    let mut hm = HashMap::new();
    for (y, row) in pipes.iter().enumerate() {
      for (x, pipe) in row.iter().enumerate() {
        if pipe != &Pipe::Ground {
          hm.insert((x as i16, y as i16), *pipe);
        }
      }
    }
    Ok((input, hm))
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let l = find_loop(input);
    (l.len() + 1) / 2
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let (&start, _) = input.iter().find(|(_, p)| p == &&Pipe::Start).unwrap();
    let mut l = find_loop(input);
    l.push(start);
    l.insert(0, start);

    let mut l2 = vec![];
    l2.push((l[0].0 * 2, l[0].1 * 2));
    for i in 1..l.len() {
      let (prevx, prevy) = l[i - 1];
      let (curx, cury) = l[i];
      let (newx, newy) = (curx * 2, cury * 2);
      let (dx, dy) = ((newx - prevx * 2) / 2, (newy - prevy * 2) / 2);
      l2.push((prevx * 2 + dx, prevy * 2 + dy));
      l2.push((newx, newy));
    }

    let minx = l2.iter().map(|p| p.0).min().unwrap();
    let maxx = l2.iter().map(|p| p.0).max().unwrap();
    let miny = l2.iter().map(|p| p.1).min().unwrap();
    let maxy = l2.iter().map(|p| p.1).max().unwrap();
    let mut candidates = vec![];
    for y in miny + 1..maxy {
      for x in minx + 1..maxx {
        if !l2.contains(&(x, y)) && x % 2 == 0 && y % 2 == 0 {
          candidates.push((x, y));
        }
      }
    }

    let mut enclosed_counter = 0;

    while let Some(c) = candidates.pop() {
      let (enclosed, filled) = flood_fill(c, &l2);
      if enclosed {
        enclosed_counter += filled
          .iter()
          .filter(|p| p.0 % 2 == 0 && p.1 % 2 == 0)
          .count();
      }
      candidates.retain(|p| !filled.contains(p));
    }

    enclosed_counter
  }
}

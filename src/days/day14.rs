use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::{
  multi::{many1, separated_list0},
  IResult,
};
use std::collections::HashMap;

use crate::days::Day;

pub struct Day14;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rock {
  Round,
  Cube,
}

type Map = HashMap<(usize, usize), Rock>;

fn tilt_north(hm: &mut Map, width: usize, height: usize) -> bool {
  let mut made_change = false;
  for y in 1..height {
    for x in 0..width {
      if hm.get(&(x, y)) == Some(&Rock::Round) && hm.get(&(x, y - 1)).is_none() {
        hm.insert((x, y - 1), Rock::Round);
        hm.remove(&(x, y));
        made_change = true;
      }
    }
  }
  made_change
}

fn tilt_west(hm: &mut Map, width: usize, height: usize) -> bool {
  let mut made_change = false;
  for x in 1..width {
    for y in 0..height {
      if hm.get(&(x, y)) == Some(&Rock::Round) && hm.get(&(x - 1, y)).is_none() {
        hm.insert((x - 1, y), Rock::Round);
        hm.remove(&(x, y));
        made_change = true;
      }
    }
  }
  made_change
}

fn tilt_east(hm: &mut Map, width: usize, height: usize) -> bool {
  let mut made_change = false;
  for x in (0..width - 1).rev() {
    for y in 0..height {
      if hm.get(&(x, y)) == Some(&Rock::Round) && hm.get(&(x + 1, y)).is_none() {
        hm.insert((x + 1, y), Rock::Round);
        hm.remove(&(x, y));
        made_change = true;
      }
    }
  }
  made_change
}

fn tilt_south(hm: &mut Map, width: usize, height: usize) -> bool {
  let mut made_change = false;
  for y in (0..height - 1).rev() {
    for x in 0..width {
      if hm.get(&(x, y)) == Some(&Rock::Round) && hm.get(&(x, y + 1)).is_none() {
        hm.insert((x, y + 1), Rock::Round);
        hm.remove(&(x, y));
        made_change = true;
      }
    }
  }
  made_change
}

fn tilt_cycle(hm: &mut Map, width: usize, height: usize) {
  while tilt_north(hm, width, height) {}
  while tilt_west(hm, width, height) {}
  while tilt_south(hm, width, height) {}
  while tilt_east(hm, width, height) {}
}

impl Day for Day14 {
  type Input = (Map, usize, usize);

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (input, rows) =
      separated_list0(newline, many1(alt((char('.'), char('#'), char('O')))))(input)?;
    let mut res = HashMap::new();
    for (y, row) in rows.iter().enumerate() {
      for (x, c) in row.iter().enumerate() {
        match c {
          '#' => res.insert((x, y), Rock::Cube),
          'O' => res.insert((x, y), Rock::Round),
          _ => None,
        };
      }
    }
    let height = rows.len();
    let width = rows[0].len();
    Ok((input, (res, width, height)))
  }

  type Output1 = usize;

  fn part_1((map, width, height): &Self::Input) -> Self::Output1 {
    let mut map2 = map.clone();
    while tilt_north(&mut map2, *width, *height) {}
    map2.iter()
      .map(|(&(_, y), &r)| if r == Rock::Round { height - y } else { 0 })
      .sum()
  }

  type Output2 = usize;

  fn part_2((map, width, height): &Self::Input) -> Self::Output2 {
    let mut memo: HashMap<Vec<(usize, usize)>, (usize, Map)> = HashMap::new();
    let mut map2 = map.clone();
    let mut counter: usize = 0;
    loop {
      let mut key = map2
        .iter()
        .filter_map(|(&(x, y), &r)| if r == Rock::Round { Some((x, y)) } else { None })
        .collect::<Vec<(usize, usize)>>();
      key.sort();
      match memo.get(&key) {
        Some((start, _)) => {
          let delta = counter - start;
          let end_counter = start + ((1000000000 - start) % delta);
          return memo
            .values()
            .find(|(c, _)| *c == end_counter)
            .unwrap()
            .clone()
            .1
            .iter()
            .map(|(&(_, y), &r)| if r == Rock::Round { height - y } else { 0 })
            .sum();
        }
        None => {
          memo.insert(key, (counter, map2.clone()));
          tilt_cycle(&mut map2, *width, *height);
          counter += 1;
        }
      }
    }
  }
}

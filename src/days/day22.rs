use std::collections::{HashMap, HashSet};

use nom::character::complete::{char, i32, newline};
use nom::multi::separated_list0;
use nom::{sequence::tuple, IResult};

use crate::days::Day;

pub struct Day22;

fn parse_line(input: &str) -> IResult<&str, Brick> {
  let (input, ((x1, _, y1, _, z1), _, (x2, _, y2, _, z2))) = tuple((
    tuple((i32, char(','), i32, char(','), i32)),
    char('~'),
    tuple((i32, char(','), i32, char(','), i32)),
  ))(input)?;
  Ok((input, ((x1, y1, z1), (x2, y2, z2))))
}

type Pos3 = (i32, i32, i32);
type Brick = (Pos3, Pos3);

// I think this is probably unnecessary, and you don't actually need to simulate
// the falling. Knowing which bricks are above which should be enough. But oh well,
// can't be bothered to change that now.
fn fall_step(input: &[Brick]) -> Vec<Brick> {
  input
    .iter()
    .map(|((x1, y1, z1), (x2, y2, z2))| {
      let z_below: i32 = *input
        .iter()
        .filter_map(|((x3, y3, z3), (x4, y4, z4))| {
          assert!(x3 <= x4);
          assert!(y3 <= y4);
          assert!(z3 <= z4);
          assert!(x1 <= x2);
          assert!(y1 <= y2);
          assert!(z1 <= z2);
          (z4 < z1 && x3 <= x2 && x4 >= x1 && y3 <= y2 && y4 >= y1).then_some(*z4)
        })
        .max()
        .get_or_insert(0);
      ((*x1, *y1, z_below + 1), (*x2, *y2, z_below + 1 + (z2 - z1)))
    })
    .collect()
}

fn get_parents_count(leans_on: &HashMap<usize, Vec<usize>>, i: &usize) -> usize {
  let mut removed: HashSet<usize> = HashSet::new();
  removed.insert(*i);
  loop {
    let c = removed.len();
    for (j, bs) in leans_on.iter() {
      if !removed.contains(j) && !bs.is_empty() && !bs.iter().any(|k| !removed.contains(k)) {
        removed.insert(*j);
      }
    }
    if c == removed.len() {
      break;
    }
  }
  removed.len() - 1
}

fn create_leaning_on_tree(input: &[Brick]) -> HashMap<usize, Vec<usize>> {
  let mut final_state: Vec<Brick> = input.to_owned();
  loop {
    let new_state = fall_step(&final_state);
    if new_state == final_state {
      final_state = new_state;
      break;
    }
    final_state = new_state;
  }
  let mut leans_on: HashMap<usize, Vec<usize>> = HashMap::new();
  for (i, ((x1, y1, z1), (x2, y2, _))) in final_state.iter().enumerate() {
    leans_on.insert(
      i,
      final_state
        .iter()
        .enumerate()
        .filter(|(_, ((x3, y3, _), (x4, y4, z4)))| {
          *z4 == z1 - 1 && x3 <= x2 && x4 >= x1 && y3 <= y2 && y4 >= y1
        })
        .map(|(j, _)| j)
        .collect(),
    );
  }
  leans_on
}

impl Day for Day22 {
  type Input = Vec<Brick>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, parse_line)(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let leans_on = create_leaning_on_tree(input);
    leans_on
      .iter()
      .enumerate()
      .filter(|(i, _)| {
        !leans_on
          .iter()
          .any(|(_, bs)| bs.contains(i) && bs.len() == 1)
      })
      .count()
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let leans_on = create_leaning_on_tree(input);
    leans_on
      .iter()
      .enumerate()
      .map(|(i, _)| get_parents_count(&leans_on, &i))
      .sum()
  }
}

use nom::character::complete::{char, newline, one_of, space1, u16};
use nom::multi::{many1, separated_list0};
use nom::IResult;
use std::collections::HashMap;

use crate::days::Day;

pub struct Day12;

type Row = (String, Vec<u16>);

fn parse_row(input: &str) -> IResult<&str, Row> {
  let (input, conds) = many1(one_of(".#?"))(input)?;
  let (input, _) = space1(input)?;
  let (input, nums) = separated_list0(char(','), u16)(input)?;
  Ok((input, (conds.into_iter().collect(), nums)))
}

fn solve(
  s: &str,
  working_on: Option<u16>,
  nrs: &[u16],
  memo: &mut HashMap<(String, Option<u16>, Vec<u16>), usize>,
) -> usize {
  if let Some(res) = memo.get(&(s.to_owned(), working_on, nrs.to_vec())) {
    return *res;
  }
  let res = if s.is_empty() {
    if (working_on == Some(0) || working_on.is_none()) && nrs.is_empty() {
      1
    } else {
      0
    }
  } else {
    match (s.chars().next().unwrap(), working_on) {
      ('#', Some(0)) => 0,
      ('#', None) => {
        if nrs.is_empty() {
          0
        } else {
          solve(&s[1..], Some(nrs[0] - 1), &nrs[1..], memo)
        }
      }
      ('#', Some(nr)) => solve(&s[1..], Some(nr - 1), nrs, memo),
      ('.', None) => solve(&s[1..], None, nrs, memo),
      ('.', Some(0)) => solve(&s[1..], None, nrs, memo),
      ('.', Some(_)) => 0,
      ('?', None) if !nrs.is_empty() => {
        solve(&s[1..], None, nrs, memo) + solve(&s[1..], Some(nrs[0] - 1), &nrs[1..], memo)
      }
      ('?', None) => solve(&s[1..], None, nrs, memo),
      ('?', Some(0)) => solve(&s[1..], None, nrs, memo),
      ('?', Some(x)) => solve(&s[1..], Some(x - 1), nrs, memo),
      _ => panic!("Unexpected character"),
    }
  };
  memo.insert((s.to_owned(), working_on, nrs.to_vec()), res);
  res
}

impl Day for Day12 {
  type Input = Vec<Row>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, parse_row)(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    input
      .iter()
      .map(|(r, nrs)| solve(r, None, nrs, &mut HashMap::new()))
      .sum()
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    input
      .iter()
      .map(|(r, ns)| {
        let new_r = format!("{}?{}?{}?{}?{}", r, r, r, r, r);
        let new_ns: Vec<u16> = vec![ns, ns, ns, ns, ns]
          .into_iter()
          .flatten()
          .copied()
          .collect();
        // let s: Vec<String> = new_r.split('.').filter_map(|s| if s != "" {Some(s.to_owned())} else {None}).collect();
        solve(&new_r, None, &new_ns, &mut HashMap::new())
      })
      .sum()
  }
}

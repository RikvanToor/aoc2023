use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline, space1, u64};
use nom::multi::separated_list0;
use nom::sequence::{pair, separated_pair};
use nom::IResult;

use crate::days::Day;

pub struct Day05;

#[derive(Debug, Clone)]
pub struct Map {
  ranges: Vec<(u64, u64, u64)>,
}

fn parse_map(input: &str) -> IResult<&str, Map> {
  let (input, _) = separated_pair(alpha1, tag("-to-"), alpha1)(input)?;
  let (input, _) = tag(" map:")(input)?;
  let (input, _) = newline(input)?;
  let (input, ranges) = separated_list0(
    newline,
    separated_pair(separated_pair(u64, tag(" "), u64), tag(" "), u64),
  )(input)?;
  let res = Map {
    ranges: ranges.iter().map(|((a, b), c)| (*a, *b, *c)).collect(),
  };
  Ok((input, res))
}

#[derive(Debug)]
pub struct Almanac {
  seeds: Vec<u64>,
  maps: Vec<Map>,
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
  let (input, _) = tag("seeds: ")(input)?;
  let (input, seeds) = separated_list0(space1, u64)(input)?;
  let (input, _) = pair(newline, newline)(input)?;
  let (input, maps) = separated_list0(pair(newline, newline), parse_map)(input)?;
  let res = Almanac { seeds, maps };
  Ok((input, res))
}

impl Day for Day05 {
  type Input = Almanac;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    parse_almanac(input)
  }

  type Output1 = u64;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    *input
      .seeds
      .iter()
      .map(|x| {
        let mut res = *x;
        for m in input.maps.iter() {
          match m
            .ranges
            .iter()
            .find(|(_, src, lngth)| res >= *src && res < src + lngth)
          {
            None => {}
            Some((dest, src, _)) => {
              res = dest + (res - src);
            }
          }
        }
        res
      })
      .min()
      .get_or_insert(0)
  }

  type Output2 = u64;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    // Get the initial ranges
    let mut seed_ranges: Vec<(u64, u64)> = input
      .seeds
      .chunks(2)
      .map(|pair| (pair[0], pair[0] + pair[1] - 1))
      .collect();
    // For each set of maps
    for m in input.maps.iter() {
      // Apply the maps to the ranges, potentially splitting them up into multiple sub-ranges
      seed_ranges = seed_ranges
        .iter()
        .flat_map(|(start, end)| {
          // Get all ranges that overlap the input range
          let overlaps: Vec<&(u64, u64, u64)> = m
            .ranges
            .iter()
            .filter(|(_, mstart, mlength)| {
              let mend = mstart + mlength - 1;
              mstart <= end && mend >= *start
            })
            .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
            .collect();
          
          // Split the input range into subranges at the start of every map range
          // that starts in between the input range
          let mut new_ranges: Vec<(u64, u64)> = vec![];
          let (mut current_start, current_end) = (*start, *end);
          for (_, o_start, o_length) in overlaps.iter() {
            if *o_start > current_start {
              new_ranges.push((current_start, o_start - 1));
              current_start = *o_start;
              let o_end = o_start + o_length - 1;
              if o_end < current_end {
                new_ranges.push((current_start, o_end));
                current_start = o_end + 1;
              }
            }
          }
          new_ranges.push((current_start, current_end));

          // For all these subranges, apply the remapping.
          new_ranges
            .iter()
            .map(|(start, end)| {
              match overlaps.iter().find(|(_, o_start, o_length)| {
                o_start <= start && (o_start + o_length - 1) >= *end
              }) {
                Some((o_dest, o_start, _)) => {
                  (start - o_start + o_dest, end - o_start + o_dest)
                }
                None => (*start, *end),
              }
            })
            .collect::<Vec<(u64, u64)>>()
        })
        .collect();
    }
    *seed_ranges
      .iter()
      .map(|(start, _)| *start)
      .min()
      .get_or_insert(0)
  }
}

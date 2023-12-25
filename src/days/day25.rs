use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use nom::{IResult, character::complete::{alpha1, space1, newline}, bytes::complete::tag, multi::separated_list1};

use crate::days::Day;

pub struct Day25;

fn parse_line(input: &str) -> IResult<&str, (String, Vec<String>)> {
  let (input, key) = alpha1(input)?;
  let (input, _) = tag(": ")(input)?;
  let (input, connections) = separated_list1(space1, alpha1)(input)?;
  Ok((input, (key.to_owned(), connections.into_iter().map(|v| v.to_owned()).collect())))
}

fn count_nodes(hm: &HashMap<&str, HashSet<&str>>, ignored: &HashSet<(&str, &str)>, start: &str) -> usize {
  let mut seen: HashSet<&str> = HashSet::new();
  seen.insert(start);
  let mut explore: Vec<&str> = vec![start];
  while let Some(n) = explore.pop() {
    for n2 in hm.get(n).unwrap().iter().filter(|n2| !ignored.contains(&(n, n2)) && !ignored.contains(&(n2, n))) {
      if seen.insert(n2) {
        explore.push(n2);
      }
    }
  }
  seen.len()
}

impl Day for Day25 {
  type Input = Vec<(String, Vec<String>)>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list1(newline, parse_line)(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let mut hm: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut connections: HashSet<(&str, &str)> = HashSet::new();
    for (k1, vals) in input {
      for k2 in vals {
        let k1e = hm.entry(k1).or_insert(HashSet::new());
        k1e.insert(k2);
        let k2e = hm.entry(k2).or_insert(HashSet::new());
        k2e.insert(k1);
        connections.insert((k1, k2));
      }
    }
    let nodes: Vec<&str> = hm.keys().map(|x| *x).collect();
    let mut counts = HashMap::new();
    for n in nodes.iter() {
      counts.insert(n, 0);
    }
    for i in 0..nodes.len() {
      for j in 0..nodes.len() {
        let n1 = nodes[i];
        let n2 = nodes[j];
        let (path, _) = dijkstra(&n1,|n| hm.get(n).unwrap().iter().map(|n3| (*n3, 1)), |n| n == &n2).unwrap();
        for n in path {
          let c = counts.get_mut(&n).unwrap();
          *c += 1;
        }
      }
    }
    let top6: Vec<&str> = counts.into_iter().sorted_by(|(_,c1),(_,c2)| c2.cmp(c1)).take(6).map(|(s,_)| *s).collect::<Vec<&str>>();
    
    let start_node = &input.first().unwrap().0;
    let total_count = count_nodes(&hm, &HashSet::new(), &start_node);
    let mut options = vec![];
    for i in 0..top6.len() {
      for j in i+1..top6.len() {
        options.push((top6[i], top6[j]));
      }
    }

    for i in 0..options.len() {
      for j in i+1..options.len() {
        for k in j+1..options.len() {
          let ignored: HashSet<(&str, &str)> = HashSet::from_iter([options[i], options[j], options[k]]);
          let new_count = count_nodes(&hm, &ignored, &start_node);
          if new_count != total_count {
            println!("new count: {}", new_count);
            return new_count * (total_count - new_count);
          }
        }
      }
    }
    0
  }

  type Output2 = usize;

  fn part_2(_input: &Self::Input) -> Self::Output2 {
    0
  }
}

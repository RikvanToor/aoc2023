use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::{
  branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::map as pmap,
  sequence::pair, IResult,
};
use num::integer::lcm;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use crate::days::Day;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Module {
  Broadcaster,
  FlipFlop(String),
  Conjunction(String),
  Output(String),
}

fn parse_module(input: &str) -> IResult<&str, Module> {
  alt((
    pmap(tag("broadcaster"), |_| Module::Broadcaster),
    pmap(pair(tag("%"), alpha1), |(_, s): (&str, &str)| {
      Module::FlipFlop(s.to_owned())
    }),
    pmap(pair(tag("&"), alpha1), |(_, s): (&str, &str)| {
      Module::Conjunction(s.to_owned())
    }),
  ))(input)
}

fn parse_line(input: &str) -> IResult<&str, (Module, Vec<String>)> {
  let (input, m) = parse_module(input)?;
  let (input, _) = tag(" -> ")(input)?;
  let (input, targets) = separated_list1(tag(", "), alpha1)(input)?;
  Ok((
    input,
    (
      m,
      targets
        .iter()
        .map(|t| String::from_str(t).unwrap())
        .collect::<Vec<String>>(),
    ),
  ))
}

fn run(
  mods: &HashMap<Module, Vec<Module>>,
  state: &mut HashMap<Module, bool>,
  high_counter: &mut usize,
  low_counter: &mut usize,
  conjunction_state: &mut HashMap<Module, HashMap<Module, bool>>,
) -> usize {
  let mut q: VecDeque<(&Module, &Module, bool)> = VecDeque::new();
  let mut rx_counter = 0;
  q.push_back((&Module::Broadcaster, &Module::Broadcaster, false));
  while let Some((from, to, signal)) = q.pop_front() {
    if signal {
      *high_counter += 1;
    } else {
      *low_counter += 1;
    }
    match to {
      Module::FlipFlop(_) => {
        if !signal {
          let s = state.get(to).unwrap();
          let new_signal = !s;
          *state.get_mut(to).unwrap() = new_signal;
          for m2 in mods.get(to).unwrap() {
            q.push_back((to, m2, new_signal));
          }
        }
      }
      Module::Conjunction(_) => {
        let memory = conjunction_state.get_mut(to).unwrap();
        *memory.get_mut(from).unwrap() = signal;
        let new_signal = memory.values().any(|x| !x);
        for m2 in mods.get(to).unwrap() {
          q.push_back((to, m2, new_signal));
        }
      }
      Module::Broadcaster => {
        for m2 in mods.get(to).unwrap() {
          q.push_back((to, m2, signal));
        }
      }
      Module::Output(s) => {
        if s == "rx" && !signal {
          rx_counter += 1;
        }
      }
    }
  }
  rx_counter
}

fn extend_subnetwork(
  mods: &HashMap<Module, Vec<Module>>,
  target: &Module,
  parents: &mut HashSet<Module>,
) {
  for (m, _) in mods.iter().filter(|(_, ms)| ms.contains(target)) {
    if parents.insert(m.clone()) {
      extend_subnetwork(mods, m, parents)
    }
  }
}

fn get_subnetwork(
  mods: &HashMap<Module, Vec<Module>>,
  target: &Module,
) -> HashMap<Module, Vec<Module>> {
  let mut parents = HashSet::new();
  extend_subnetwork(mods, target, &mut parents);
  parents.insert(target.clone());
  mods.iter()
    .filter_map(|(m, ms)| {
      if parents.contains(m) {
        Some((
          m.clone(),
          ms.iter()
            .filter(|m2| parents.contains(m2))
            .cloned()
            .collect(),
        ))
      } else {
        None
      }
    })
    .collect()
}

fn find_cycle(mods: &HashMap<Module, Vec<Module>>) -> usize {
  let init_state: HashMap<Module, bool> = mods.iter().map(|(m, _)| (m.clone(), false)).collect();
  let init_conjunction_state: HashMap<Module, HashMap<Module, bool>> = mods
    .iter()
    .filter_map(|(m, _)| {
      if let Module::Conjunction(_) = m {
        Some((
          m.clone(),
          mods.iter()
            .filter(|(_, ms)| ms.contains(m))
            .map(|(m2, _)| (m2.clone(), false))
            .collect(),
        ))
      } else {
        None
      }
    })
    .collect();
  let mut state = init_state.clone();
  let mut conjunction_state = init_conjunction_state.clone();
  let mut high_counter = 0;
  let mut low_counter = 0;

  run(
    mods,
    &mut state,
    &mut high_counter,
    &mut low_counter,
    &mut conjunction_state,
  );

  let check_state = state.clone();
  let check_c_state = conjunction_state.clone();

  for i in 1.. {
    run(
      mods,
      &mut state,
      &mut high_counter,
      &mut low_counter,
      &mut conjunction_state,
    );
    if state == check_state && conjunction_state == check_c_state {
      return i;
    }
  }
  0
}

pub struct Day20;

impl Day for Day20 {
  type Input = HashMap<Module, Vec<String>>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (input, vec) = separated_list1(newline, parse_line)(input)?;
    Ok((
      input,
      vec.into_iter().collect::<HashMap<Module, Vec<String>>>(),
    ))
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let mods: HashMap<Module, Vec<Module>> = input
      .iter()
      .map(|(m, ss)| {
        (
          m.clone(),
          ss.iter()
            .map(|s| {
              input
                .keys()
                .find(|k| match k {
                  Module::Broadcaster => s == "broadcaster",
                  Module::FlipFlop(s2) => s == s2,
                  Module::Conjunction(s2) => s == s2,
                  Module::Output(s2) => s == s2,
                })
                .cloned()
                .unwrap_or(Module::Output(s.clone()))
            })
            .collect(),
        )
      })
      .collect();

    let mut state: HashMap<Module, bool> =
      input.iter().map(|(m, _)| (m.clone(), false)).collect();
    let mut conjunction_state: HashMap<Module, HashMap<Module, bool>> = mods
      .iter()
      .filter_map(|(m, _)| {
        if let Module::Conjunction(_) = m {
          Some((
            m.clone(),
            mods.iter()
              .filter(|(_, ms)| ms.contains(m))
              .map(|(m2, _)| (m2.clone(), false))
              .collect(),
          ))
        } else {
          None
        }
      })
      .collect();
    let mut high_counter = 0;
    let mut low_counter = 0;

    for _ in 0..1000 {
      run(
        &mods,
        &mut state,
        &mut high_counter,
        &mut low_counter,
        &mut conjunction_state,
      );
    }
    high_counter * low_counter
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let mods: HashMap<Module, Vec<Module>> = input
      .iter()
      .map(|(m, ss)| {
        (
          m.clone(),
          ss.iter()
            .map(|s| {
              input
                .keys()
                .find(|k| match k {
                  Module::Broadcaster => s == "broadcaster",
                  Module::FlipFlop(s2) => s == s2,
                  Module::Conjunction(s2) => s == s2,
                  Module::Output(s2) => s == s2,
                })
                .cloned()
                .unwrap_or(Module::Output(s.clone()))
            })
            .collect(),
        )
      })
      .collect();

    mods.iter()
      .filter(|(_, ms)| ms.contains(&Module::Output("rx".to_owned())))
      .flat_map(|(m, _)| {
        mods.iter()
          .filter_map(|(m2, ms)| if ms.contains(m) { Some(m2) } else { None })
      })
      .map(|m| find_cycle(&get_subnetwork(&mods, m)))
      .fold(1, lcm)
  }
}

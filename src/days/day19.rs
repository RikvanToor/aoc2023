use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, newline, u32};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{pair, tuple};
use nom::{branch::alt, combinator::map as pmap, IResult};
use std::collections::HashMap;

use crate::days::Day;

pub struct Day19;

#[derive(Debug, Clone, Copy)]
pub struct Part {
  x: u32,
  m: u32,
  a: u32,
  s: u32,
}

fn parse_part(input: &str) -> IResult<&str, Part> {
  let (input, (_, x)) = pair(tag("{x="), u32)(input)?;
  let (input, (_, m)) = pair(tag(",m="), u32)(input)?;
  let (input, (_, a)) = pair(tag(",a="), u32)(input)?;
  let (input, (_, s)) = pair(tag(",s="), u32)(input)?;
  let (input, _) = tag("}")(input)?;
  Ok((input, Part { x, m, a, s }))
}

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
  Accept,
  Reject,
  Other(String),
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
  GT,
  LT,
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
  X,
  M,
  A,
  S,
}

#[derive(Debug, Clone, Copy)]
pub struct Condition {
  category: Category,
  operator: Operator,
  value: u32,
}

#[derive(Debug, Clone)]
pub enum Action {
  WhenThen(Condition, Target),
  Send(Target),
}

#[derive(Debug, Clone)]
pub struct Workflow {
  name: String,
  steps: Vec<Action>,
}

#[derive(Debug, Clone)]
pub struct Limits {
  x: (u32, u32),
  m: (u32, u32),
  a: (u32, u32),
  s: (u32, u32),
}

fn parse_target(input: &str) -> IResult<&str, Target> {
  alt((
    pmap(tag("A"), |_| Target::Accept),
    pmap(tag("R"), |_| Target::Reject),
    pmap(alpha1, |s: &str| Target::Other(s.to_owned())),
  ))(input)
}

fn parse_category(input: &str) -> IResult<&str, Category> {
  alt((
    pmap(char('x'), |_| Category::X),
    pmap(char('m'), |_| Category::M),
    pmap(char('a'), |_| Category::A),
    pmap(char('s'), |_| Category::S),
  ))(input)
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
  alt((
    pmap(char('<'), |_| Operator::LT),
    pmap(char('>'), |_| Operator::GT),
  ))(input)
}

fn parse_action(input: &str) -> IResult<&str, Action> {
  alt((
    pmap(
      tuple((parse_category, parse_operator, u32, char(':'), parse_target)),
      |(category, operator, value, _, target)| {
        Action::WhenThen(
          Condition {
            category,
            operator,
            value,
          },
          target,
        )
      },
    ),
    pmap(parse_target, Action::Send),
  ))(input)
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
  let (input, name) = alpha1(input)?;
  let (input, _) = char('{')(input)?;
  let (input, actions) = separated_list0(char(','), parse_action)(input)?;
  let (input, _) = char('}')(input)?;
  Ok((
    input,
    Workflow {
      name: name.to_owned(),
      steps: actions,
    },
  ))
}

fn cond_holds(condition: &Condition, part: &Part) -> bool {
  let v = match condition.category {
    Category::X => part.x,
    Category::M => part.m,
    Category::A => part.a,
    Category::S => part.s,
  };
  match condition.operator {
    Operator::GT => v > condition.value,
    Operator::LT => v < condition.value,
  }
}

fn run_part<'a>(part: &Part, workflows: &'a HashMap<String, Vec<Action>>, key: &str) -> &'a Target {
  let action = workflows.get(key).unwrap();
  for a in action {
    match a {
      Action::WhenThen(cond, targ) => {
        if cond_holds(cond, part) {
          return targ;
        }
      }
      Action::Send(targ) => {
        return targ;
      }
    }
  }
  panic!("Could not run workflow {} for part {:?}", key, part);
}

fn accept_part(part: &Part, workflows: &HashMap<String, Vec<Action>>, key: &str) -> bool {
  match run_part(part, workflows, key) {
    Target::Accept => true,
    Target::Reject => false,
    Target::Other(s) => accept_part(part, workflows, s),
  }
}

fn action_is_relevant(action: &Action, target: &Target) -> bool {
  match action {
    Action::WhenThen(_, t) => t == target,
    Action::Send(t) => t == target,
  }
}

fn apply_cond(condition: &Condition, limits: &mut Limits, negative: bool) {
  let mut min = 1;
  let mut max = 4000;
  match condition.operator {
    Operator::GT => {
      if negative {
        max = condition.value;
      } else {
        min = condition.value + 1;
      }
    }
    Operator::LT => {
      if negative {
        min = condition.value;
      } else {
        max = condition.value - 1;
      }
    }
  }
  match condition.category {
    Category::X => {
      limits.x.0 = u32::max(limits.x.0, min);
      limits.x.1 = u32::min(limits.x.1, max);
    }
    Category::M => {
      limits.m.0 = u32::max(limits.m.0, min);
      limits.m.1 = u32::min(limits.m.1, max);
    }
    Category::A => {
      limits.a.0 = u32::max(limits.a.0, min);
      limits.a.1 = u32::min(limits.a.1, max);
    }
    Category::S => {
      limits.s.0 = u32::max(limits.s.0, min);
      limits.s.1 = u32::min(limits.s.1, max);
    }
  }
}

fn get_options(workflows: &[Workflow], target: &Target, limits: &Limits) -> usize {
  workflows
    .iter()
    .filter(|wf| wf.steps.iter().any(|s| action_is_relevant(s, target)))
    .map(|wf| {
      wf.steps
        .iter()
        .positions(|a| action_is_relevant(a, target))
        .map(|i| {
          let mut new_limits = limits.clone();
          for prev in wf.steps.iter().take(i) {
            match prev {
              Action::Send(_) => {
                panic!("Did not expect conditionless action before end of chain.")
              }
              Action::WhenThen(cond, _) => {
                apply_cond(cond, &mut new_limits, true);
              }
            }
          }
          match wf.steps[i] {
            Action::WhenThen(cond, _) => {
              apply_cond(&cond, &mut new_limits, false);
            }
            Action::Send(_) => {}
          }
          if wf.name == "in" {
            (new_limits.x.1 - new_limits.x.0 + 1) as usize
              * (new_limits.m.1 - new_limits.m.0 + 1) as usize
              * (new_limits.a.1 - new_limits.a.0 + 1) as usize
              * (new_limits.s.1 - new_limits.s.0 + 1) as usize
          } else {
            get_options(workflows, &Target::Other(wf.name.clone()), &new_limits)
          }
        })
        .sum::<usize>()
    })
    .sum::<usize>()
}

impl Day for Day19 {
  type Input = (Vec<Workflow>, Vec<Part>);

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (input, wfs) = separated_list1(newline, parse_workflow)(input)?;
    let (input, _) = pair(newline, newline)(input)?;
    let (input, parts) = separated_list1(newline, parse_part)(input)?;
    Ok((input, (wfs, parts)))
  }

  type Output1 = u32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let (wfs, parts) = input;
    let workflows: HashMap<String, Vec<Action>> = wfs
      .iter()
      .map(|wf| (wf.name.clone(), wf.steps.clone()))
      .collect();
    parts
      .iter()
      .filter(|p| accept_part(p, &workflows, "in"))
      .map(|p| p.x + p.m + p.a + p.s)
      .sum()
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let (wfs, _) = input;
    let limits = Limits {
      x: (1, 4000),
      m: (1, 4000),
      a: (1, 4000),
      s: (1, 4000),
    };
    get_options(wfs, &Target::Accept, &limits)
  }
}

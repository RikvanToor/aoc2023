use nom::{
  character::complete::{newline, none_of},
  multi::{many0, separated_list0},
  IResult,
};

use crate::days::Day;

pub struct Day03;

type Symbol = ((usize, usize), char);
type Number = ((usize, usize), usize, u32);

fn get_nrs_and_symbols(input: &[Vec<char>]) -> (Vec<Number>, Vec<Symbol>) {
  let mut numbers: Vec<Number> = vec![];
  let mut symbols: Vec<Symbol> = vec![];
  for (y, row) in input.iter().enumerate() {
    let mut x = 0;
    while x < row.len() {
      match row[x].to_digit(10) {
        Some(n) => {
          let mut nr = n;
          let mut x2 = x + 1;
          while x2 < row.len() {
            match row[x2].to_digit(10) {
              Some(n2) => {
                nr *= 10;
                nr += n2;
                if x2 == row.len() - 1 {
                  numbers.push(((x, y), (x2 - x + 1), nr));
                  x = x2 + 1;
                }
                x2 += 1;
              }
              None => {
                numbers.push(((x, y), (x2 - x), nr));
                x = x2;
                break;
              }
            }
          }
        }
        None => {
          if row[x] != '.' && row[x] != '\n' {
            symbols.push(((x, y), row[x]));
          }
          x += 1;
        }
      }
    }
  }
  (numbers, symbols)
}

fn symbol_exists(x: usize, y: usize, symbols: &[Symbol]) -> bool {
  symbols.iter().any(|((x2, y2), _)| *x2 == x && *y2 == y)
}

fn adjacent_to_pos(x2: &usize, y2: &usize, width: &usize, x: &usize, y: &usize) -> bool {
  ((if x2 == &0 { 0 } else { x2 - 1 })..=x2 + width).contains(x)
    && ((if y2 == &0 { 0 } else { y2 - 1 })..=y2 + 1).contains(y)
}

impl Day for Day03 {
  type Input = Vec<Vec<char>>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, many0(none_of("\n")))(input)
  }

  type Output1 = u32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let (numbers, symbols) = get_nrs_and_symbols(input);

    let part_numbers = numbers.iter().filter(|((x, y), width, _)| {
      ((if *x == 0 { 0 } else { x - 1 })..=x + width).any(|x2| {
        ((if *y == 0 { 0 } else { y - 1 })..=y + 1)
          .any(|y2| symbol_exists(x2, y2, &symbols))
      })
    });
    part_numbers.map(|(_, _, n)| n).sum()
  }

  type Output2 = u32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let (numbers, symbols) = get_nrs_and_symbols(input);
    symbols
      .iter()
      .filter(|(_, c)| *c == '*')
      .map(|((x, y), _)| {
        numbers
          .iter()
          .filter(|((x2, y2), width, _)| adjacent_to_pos(x2, y2, width, x, y))
      })
      .filter(|l| l.clone().count() == 2)
      .map(|l| l.map(|(_, _, n)| *n).product::<u32>())
      .sum()
  }
}

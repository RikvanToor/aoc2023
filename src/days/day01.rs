use nom::IResult;

use crate::days::Day;

pub struct Day01;

fn check_line(l: &str) -> u32 {
  let cs: Vec<u32> = l
    .chars()
    .filter_map(|c| c.to_digit(10))
    .collect();
  *(cs.first().get_or_insert(&0)) * 10 + *(cs.last().get_or_insert(&0))
}

fn replace_words(l: &str) -> String {
  l
    .replace("one", "o1ne")
    .replace("two", "t2wo")
    .replace("three", "t3hree")
    .replace("four", "f4our")
    .replace("five", "f5ive")
    .replace("six", "s6ix")
    .replace("seven", "s7even")
    .replace("eight", "e8ight")
    .replace("nine", "n9ine")
}

impl Day for Day01 {
  type Input = Vec<String>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    // Faking a nom parser, since we really just need to split by lines.
    let ls = input.lines().map(String::from);
    Ok(("", ls.collect()))
  }

  type Output1 = u32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    input
      .iter()
      .map(|l| check_line(l))
      .sum()
  }

  type Output2 = u32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    input
      .iter()
      .map(|l| check_line(&replace_words(l)))
      .sum()
  }
}

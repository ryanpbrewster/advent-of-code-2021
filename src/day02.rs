use std::str::FromStr;

use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::{all_consuming, value},
    sequence::delimited,
    IResult,
};
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}
impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let dir = match s {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => return Err(anyhow!("invalid direction: {}", s)),
        };
        Ok(dir)
    }
}
struct Move {
    dir: Direction,
    amount: i32,
}
#[derive(Default, Debug)]
struct Position {
    depth: i32,
    horizontal: i32,
    aim: i32,
}
impl Position {
    fn type1(&mut self, m: &Move) {
        match m.dir {
            Direction::Forward => self.horizontal += m.amount,
            Direction::Down => self.depth += m.amount,
            Direction::Up => self.depth -= m.amount,
        }
    }

    fn type2(&mut self, m: &Move) {
        match m.dir {
            Direction::Forward => {
                self.horizontal += m.amount;
                self.depth += m.amount * self.aim;
            }
            Direction::Down => self.aim += m.amount,
            Direction::Up => self.aim -= m.amount,
        }
    }
}

fn parse_input(raw: &str) -> anyhow::Result<Vec<Move>> {
    let parsed = raw
        .trim()
        .lines()
        .map(|line| parse_move(line))
        .collect::<anyhow::Result<_>>()?;
    Ok(parsed)
}

fn parse_move(input: &str) -> anyhow::Result<Move> {
    match all_consuming(delimited(multispace0, move_parser, multispace0))(input) {
        Ok((_, m)) => Ok(m),
        Err(_) => Err(anyhow!("could not parse move from {}", input)),
    }
}
fn move_parser(input: &str) -> IResult<&str, Move> {
    let (input, dir) = dir_parser(input)?;
    let (input, _) = multispace1(input)?;
    let (input, amount) = nom::character::complete::i32(input)?;
    Ok((input, Move { dir, amount }))
}
fn dir_parser(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Forward, tag("forward")),
        value(Direction::Down, tag("down")),
        value(Direction::Up, tag("up")),
    ))(input)
}

fn score1(input: &[Move]) -> i32 {
    let mut pos = Position::default();
    for m in input {
        pos.type1(m);
    }
    pos.depth * pos.horizontal
}

fn score2(input: &[Move]) -> i32 {
    let mut pos = Position::default();
    for m in input {
        pos.type2(m);
    }
    pos.depth * pos.horizontal
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL: &str = r"
      forward 5
      down 5
      forward 8
      up 3
      down 8
      forward 2
    ";

    #[test]
    fn small() -> anyhow::Result<()> {
        let input = parse_input(SMALL)?;
        assert_eq!(score1(&input), 150);
        assert_eq!(score2(&input), 900);
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day02.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(score1(&input), 2073315);
        assert_eq!(score2(&input), 1840311528);
        Ok(())
    }
}

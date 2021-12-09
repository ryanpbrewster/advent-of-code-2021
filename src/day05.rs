use std::collections::HashMap;

use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point(i32, i32);

#[derive(Debug, Clone)]
struct Line(Point, Point);
impl Line {
    fn points(&self) -> impl Iterator<Item = Point> {
        let Point(x1, y1) = self.0;
        let Point(x2, y2) = self.1;
        let dx = i32::signum(x2 - x1);
        let dy = i32::signum(y2 - y1);
        let len = i32::abs(if x1 != x2 { x2 - x1 } else { y2 - y1 });
        (0..=len).map(move |i| Point(x1 + i * dx, y1 + i * dy))
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Line>> {
    match all_consuming(delimited(multispace0, lines_parser, multispace0))(input) {
        Ok((_, lines)) => Ok(lines),
        Err(e) => Err(anyhow!("could not parse points [{}]: {}", e, input)),
    }
}
fn lines_parser(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(multispace1, line_parser)(input)
}
fn line_parser(input: &str) -> IResult<&str, Line> {
    let (input, (p1, p2)) = separated_pair(point_parser, tag(" -> "), point_parser)(input)?;
    Ok((input, Line(p1, p2)))
}
fn point_parser(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(
        nom::character::complete::i32,
        tag(","),
        nom::character::complete::i32,
    )(input)?;
    Ok((input, Point(x, y)))
}

fn solve1(input: &[Line]) -> usize {
    let mut points: HashMap<Point, usize> = HashMap::new();
    for line @ Line(Point(x1, y1), Point(x2, y2)) in input {
        if x1 == x2 || y1 == y2 {
            for p in line.points() {
                *points.entry(p).or_default() += 1;
            }
        }
    }
    points.values().filter(|&&v| v > 1).count()
}

fn solve2(input: &[Line]) -> usize {
    let mut points: HashMap<Point, usize> = HashMap::new();
    for line in input {
        for p in line.points() {
            *points.entry(p).or_default() += 1;
        }
    }
    points.values().filter(|&&v| v > 1).count()
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL: &str = r"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    ";

    #[test]
    fn small() -> anyhow::Result<()> {
        let input = parse_input(SMALL)?;
        assert_eq!(solve1(&input), 5);
        assert_eq!(solve2(&input), 12);
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day05.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(solve1(&input), 6572);
        assert_eq!(solve2(&input), 21466);
        Ok(())
    }
}

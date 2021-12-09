use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::all_consuming,
    multi::{many_m_n, separated_list1},
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug)]
struct Board([u32; 25]);
impl Board {
    fn check(&self, draws: &[u32]) -> bool {
        (0..5).any(|i| {
            (0..5).all(|j| draws.contains(&self.0[5 * i + j]))
                || (0..5).all(|j| draws.contains(&self.0[5 * j + i]))
        })
    }
    fn unclaimed_sum(&self, draws: &[u32]) -> u32 {
        self.0.iter().filter(|&x| !draws.contains(x)).sum()
    }
}
struct Setup {
    boards: Vec<Board>,
    draws: Vec<u32>,
}

fn parse_input(input: &str) -> anyhow::Result<Setup> {
    match all_consuming(delimited(multispace0, setup_parser, multispace0))(input) {
        Ok((_, setup)) => Ok(setup),
        Err(e) => Err(anyhow!("could not parse input [{}]: {}", e, input)),
    }
}
fn setup_parser(input: &str) -> IResult<&str, Setup> {
    let (input, draws) = separated_list1(tag(","), nom::character::complete::u32)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, boards) = separated_list1(multispace0, board_parser)(input)?;
    Ok((input, Setup { boards, draws }))
}
fn board_parser(input: &str) -> IResult<&str, Board> {
    let (input, numbers) =
        many_m_n(25, 25, preceded(multispace0, nom::character::complete::u32))(input)?;
    Ok((input, Board(numbers.try_into().unwrap())))
}

fn solve1(input: &Setup) -> Option<u32> {
    for i in 1..input.draws.len() {
        let draws = &input.draws[..i];
        if let Some(winner) = input.boards.iter().find(|b| b.check(draws)) {
            return Some(winner.unclaimed_sum(draws) * draws.last().unwrap());
        }
    }
    None
}

fn solve2(input: &Setup) -> Option<u32> {
    let mut boards: Vec<&Board> = input.boards.iter().collect();
    for i in 1..input.draws.len() {
        let draws = &input.draws[..i];
        let (winners, losers) = boards
            .into_iter()
            .partition::<Vec<_>, _>(|b| b.check(draws));
        if losers.is_empty() {
            return Some(winners.first()?.unclaimed_sum(draws) * draws.last().unwrap());
        }
        boards = losers;
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL: &str = r"
      7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
      
      22 13 17 11  0
      8  2 23  4 24
      21  9 14 16  7
      6 10  3 18  5
      1 12 20 15 19
      
      3 15  0  2 22
      9 18 13 17  5
      19  8  7 25 23
      20 11 10 24  4
      14 21 16 12  6
      
      14 21 17 24  4
      10 16 15  9 19
      18  8 23 26 20
      22 11 13  6  5
      2  0 12  3  7
    ";

    #[test]
    fn small() -> anyhow::Result<()> {
        let input = parse_input(SMALL)?;
        assert_eq!(solve1(&input).unwrap(), 4512);
        assert_eq!(solve2(&input).unwrap(), 1924);
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day04.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(solve1(&input).unwrap(), 65325);
        assert_eq!(solve2(&input).unwrap(), 4624);
        Ok(())
    }
}

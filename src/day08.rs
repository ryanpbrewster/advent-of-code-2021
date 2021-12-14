use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

type Signal = Vec<u8>;
struct Sample {
    inputs: Vec<Signal>,
    outputs: Vec<Signal>,
}
impl Sample {
    fn signals(&self) -> impl Iterator<Item = &Signal> {
        self.inputs.iter().chain(self.outputs.iter())
    }
}
fn parse_input(input: &str) -> anyhow::Result<Vec<Sample>> {
    input.trim().lines().map(parse_sample).collect()
}
fn parse_sample(line: &str) -> anyhow::Result<Sample> {
    match sample_parser(line.trim()) {
        Ok((_, sample)) => Ok(sample),
        Err(e) => Err(anyhow!("parse error ({}): {}", e, line)),
    }
}
fn sample_parser(input: &str) -> IResult<&str, Sample> {
    let (input, input_signals) = separated_list1(multispace1, signal_parser)(input)?;
    let (input, _) = delimited(multispace0, tag("|"), multispace0)(input)?;
    let (input, output_signals) = separated_list1(multispace1, signal_parser)(input)?;
    Ok((
        input,
        Sample {
            inputs: input_signals,
            outputs: output_signals,
        },
    ))
}
fn signal_parser(input: &str) -> IResult<&str, Signal> {
    let (input, signal) = alpha1(input)?;
    Ok((input, signal.as_bytes().to_vec()))
}

fn solve1(input: &[Sample]) -> usize {
    // [1, 4, 7, 8] uses [2, 4, 3, 7] segments respectively.
    input
        .iter()
        .flat_map(|s| s.outputs.iter())
        .filter(|output| [2, 4, 3, 7].contains(&output.len()))
        .count()
}

fn solve2(input: &[Sample]) -> usize {
    // Basically, we need to figure out what the mapping translation is between
    // the expected "abcdefg" and whatever is actually being pushed out.
    // We can quickly do some narrowing down:
    //   - any two letter signals directly indicate what "cf" maps to
    //   - any three letter signals tell us "acf"
    //   - any four letter signals tell us "bcdf"
    // One observation: if we get entirely the same sequence of 5-letter signals,
    // there's nothing we can do. So this is not solvable in all scenarios.
    // This approach tries a simple heuristic and panics if that doesn't work.

    input
        .iter()
        .map(|sample| translated_sum(sample).unwrap())
        .sum()
}

fn translated_sum(sample: &Sample) -> Option<usize> {
    let one = sample.signals().find(|s| s.len() == 2)?;
    let four = sample.signals().find(|s| s.len() == 4)?;
    // I think there is enough info here to get the right answer in most cases without
    // any wacky permutations.
    //   - if a signal has 2, 3, 4, or 7 letters, we know it immediately
    //   - if a signal has 5 letters, it could be [2, 3, 5]
    //   - if a signal has 6 letters, it could be [0, 6, 9]

    // Consider the [2, 3, 5] case. If we happen to know what letters 1 contains, we can
    // distinguish 3 from [2, 5] --- 3 contains all of the letters in 1.
    // Similarly, if we happen to know 4, we can distinguish 2 from 5: 2 overlaps 4 w/ two segments,
    // while 5 overlaps 4 w/ three segments.

    // Consider the [0, 6, 9] case. Only 9 fully overlaps w/ 4. 0 and 6 differ in how they overlap w/ 1.

    let output = sample.outputs.iter().map(|signal| match signal.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        // length 5 --> [2, 3, 5]
        5 => match (overlap(signal, one), overlap(signal, four)) {
            (2, _) => 3,
            (_, 3) => 5,
            (_, 2) => 2,
            _ => panic!("{:?} overlaps w/ 1 and 4 strangely", signal),
        },
        // length 6 --> [0, 6, 9]
        6 => match (overlap(signal, one), overlap(signal, four)) {
            (_, 4) => 9,
            (2, _) => 0,
            (1, _) => 6,
            _ => panic!("{:?} overlaps w/ 1 and 4 strangely", signal),
        },
        _ => panic!("{:?} has a weird length", signal),
    });
    Some(output.fold(0, |acc, d| 10 * acc + d))
}
fn overlap(s1: &[u8], s2: &[u8]) -> usize {
    s1.iter().filter(|c| s2.contains(c)).count()
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL: &str = r"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    ";

    #[test]
    fn small() -> anyhow::Result<()> {
        let input = parse_input(SMALL)?;
        assert_eq!(solve1(&input), 26);
        assert_eq!(solve2(&input), 61229);
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day08.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(solve1(&input), 390);
        assert_eq!(solve2(&input), 1011785);
        Ok(())
    }
}

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
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day08.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(solve1(&input), 390);
        Ok(())
    }
}

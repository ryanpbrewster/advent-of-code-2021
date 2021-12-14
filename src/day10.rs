fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<u8>>> {
    Ok(input
        .trim()
        .lines()
        .map(|line| line.trim().as_bytes().to_vec())
        .collect())
}

fn solve1(inputs: &[Vec<u8>]) -> u64 {
    inputs
        .iter()
        .map(|input| match classify(input) {
            Outcome::Valid { .. } => 0,
            Outcome::Invalid { err } => score_error(err),
        })
        .sum()
}

enum Outcome {
    Valid { stack: Vec<u8> },
    Invalid { err: u8 },
}
fn classify(input: &[u8]) -> Outcome {
    let mut stack = Vec::new();
    for &b in input {
        match b {
            b'(' => stack.push(b')'),
            b'[' => stack.push(b']'),
            b'{' => stack.push(b'}'),
            b'<' => stack.push(b'>'),
            _ => {
                if stack.pop() != Some(b) {
                    return Outcome::Invalid { err: b };
                }
            }
        }
    }
    Outcome::Valid { stack }
}
fn score_error(b: u8) -> u64 {
    match b {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => 0,
    }
}

fn solve2(inputs: &[Vec<u8>]) -> u64 {
    let mut scores: Vec<u64> = inputs
        .iter()
        .filter_map(|input| match classify(input) {
            Outcome::Valid { stack } => Some(score_incomplete(&stack)),
            Outcome::Invalid { .. } => None,
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}
fn score_incomplete(stack: &[u8]) -> u64 {
    stack
        .iter()
        .rev()
        .fold(0, |acc, &b| 5 * acc + score_incomplete_char(b))
}
fn score_incomplete_char(b: u8) -> u64 {
    match b {
        b')' => 1,
        b']' => 2,
        b'}' => 3,
        b'>' => 4,
        _ => unreachable!("stack should not contain {}", b),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL: &str = r"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    ";

    #[test]
    fn small() -> anyhow::Result<()> {
        let input = parse_input(SMALL)?;
        assert_eq!(solve1(&input), 26397);
        assert_eq!(solve2(&input), 288957);
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day10.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(solve1(&input), 345441);
        assert_eq!(solve2(&input), 3235371166);
        Ok(())
    }
}

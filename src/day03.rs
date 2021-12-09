type Word = Vec<u8>;
fn parse_input(raw: &str) -> anyhow::Result<Vec<Word>> {
    let parsed = raw
        .split_ascii_whitespace()
        .map(|w| w.as_bytes().to_vec())
        .collect();
    Ok(parsed)
}

fn solve1(input: &[Word]) -> u32 {
    let mut most = 0;
    let mut least = 0;
    for p in 0..input[0].len() {
        let count = input.iter().filter(|v| v[p] == b'1').count();
        most <<= 1;
        least <<= 1;
        if 2 * count >= input.len() {
            most += 1;
        } else {
            least += 1;
        }
    }
    most * least
}

fn solve2(input: &[Word]) -> u32 {
    let most = find_extreme(input, true).map(from_binary).unwrap_or(0);
    let least = find_extreme(input, false).map(from_binary).unwrap_or(0);
    println!("most = {}, least = {}", most, least);
    most * least
}
fn find_extreme(input: &[Word], most_common: bool) -> Option<Word> {
    let mut work = input.to_vec();
    for p in 0..input[0].len() {
        let (one, zero) = work.into_iter().partition::<Vec<_>, _>(|v| v[p] == b'1');
        work = if (one.len() >= zero.len()) == most_common {
            one
        } else {
            zero
        };
        if work.len() <= 1 {
            return work.into_iter().next();
        }
    }
    None
}
fn from_binary(s: Word) -> u32 {
    s.into_iter()
        .fold(0, |acc, b| 2 * acc + if b == b'1' { 1 } else { 0 })
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL: &str = r"
      00100
      11110
      10110
      10111
      10101
      01111
      00111
      11100
      10000
      11001
      00010
      01010
    ";

    #[test]
    fn small() -> anyhow::Result<()> {
        let input = parse_input(SMALL)?;
        assert_eq!(solve1(&input), 198);
        assert_eq!(solve2(&input), 230);
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day03.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(solve1(&input), 3277364);
        assert_eq!(solve2(&input), 5736383);
        Ok(())
    }
}

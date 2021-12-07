fn parse_input(raw: &str) -> anyhow::Result<Vec<i32>> {
    let parsed = raw
        .split_ascii_whitespace()
        .map(|w| w.parse::<i32>())
        .collect::<Result<_, _>>()?;
    Ok(parsed)
}

fn score1(input: &[i32]) -> usize {
    input.windows(2).filter(|xs| xs[1] > xs[0]).count()
}

fn score2(input: &[i32]) -> usize {
    let agg: Vec<i32> = input.windows(3).map(|w| w.into_iter().sum()).collect();
    agg.windows(2).filter(|xs| xs[1] > xs[0]).count()
}
#[cfg(test)]
mod test {
    use super::*;

    const SMALL: &str = r"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    ";

    #[test]
    fn small() -> anyhow::Result<()> {
        let input = parse_input(SMALL)?;
        assert_eq!(score1(&input), 7);
        assert_eq!(score2(&input), 5);
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day01.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(score1(&input), 1553);
        assert_eq!(score2(&input), 1597);
        Ok(())
    }
}

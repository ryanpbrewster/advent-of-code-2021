fn parse_input(input: &str) -> anyhow::Result<Vec<i32>> {
    input
        .trim()
        .split(',')
        .map(|w| Ok(w.parse::<i32>()?))
        .collect()
}

fn solve1(input: &[i32]) -> i32 {
    let eval = |avg: i32| -> i32 { input.iter().map(|x| i32::abs(x - avg)).sum() };
    let lo = *input.iter().min().unwrap();
    let hi = *input.iter().max().unwrap();
    (lo..=hi).map(eval).min().unwrap()
}

fn solve2(input: &[i32]) -> i32 {
    let eval = |avg: i32| -> i32 {
        input
            .iter()
            .map(|x| {
                let d = i32::abs(x - avg);
                d * (d + 1) / 2
            })
            .sum()
    };
    let lo = *input.iter().min().unwrap();
    let hi = *input.iter().max().unwrap();
    (lo..=hi).map(eval).min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL: &str = r"
        16,1,2,0,4,2,7,1,2,14
    ";

    #[test]
    fn small() -> anyhow::Result<()> {
        let input = parse_input(SMALL)?;
        assert_eq!(solve1(&input), 37);
        assert_eq!(solve2(&input), 168);
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day07.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(solve1(&input), 344297);
        assert_eq!(solve2(&input), 344297);
        Ok(())
    }
}

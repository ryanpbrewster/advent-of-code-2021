type Counts = [usize; 9];

fn parse_input(input: &str) -> anyhow::Result<Counts> {
    let fish: Vec<u8> = input
        .trim()
        .split(',')
        .map(|w| Ok(w.parse::<u8>()?))
        .collect::<anyhow::Result<_>>()?;
    let mut counts = Counts::default();
    for f in fish {
        counts[f as usize] += 1;
    }
    Ok(counts)
}

fn solve1(mut cur: Counts, days: u32) -> usize {
    for _ in 0..days {
        let mut tmp = Counts::default();
        tmp[8] = cur[0];
        tmp[7] = cur[8];
        tmp[6] = cur[7] + cur[0];
        tmp[5] = cur[6];
        tmp[4] = cur[5];
        tmp[3] = cur[4];
        tmp[2] = cur[3];
        tmp[1] = cur[2];
        tmp[0] = cur[1];
        std::mem::swap(&mut cur, &mut tmp);
    }
    cur.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL: &str = r"
        3,4,3,1,2
    ";

    #[test]
    fn small() -> anyhow::Result<()> {
        let input = parse_input(SMALL)?;
        assert_eq!(solve1(input.clone(), 18), 26);
        assert_eq!(solve1(input.clone(), 80), 5934);
        assert_eq!(solve1(input.clone(), 256), 26984457539);
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day06.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(solve1(input, 80), 396210);
        assert_eq!(solve1(input.clone(), 256), 1770823541496);
        Ok(())
    }
}

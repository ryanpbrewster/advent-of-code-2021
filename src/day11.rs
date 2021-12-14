use std::collections::HashSet;

#[derive(Clone)]
struct Grid([u32; 100]);
fn parse_input(input: &str) -> anyhow::Result<Grid> {
    let values: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    Ok(Grid(values.try_into().unwrap()))
}
impl Grid {
    fn step(&mut self) {
        let mut flashing = Vec::new();
        let mut flashed = HashSet::new();
        let mut tmp = self.0;
        let mut bump = |i: i32, j: i32| -> bool {
            if !(0..10).contains(&i) || !(0..10).contains(&j) {
                return false;
            }
            let idx = (10 * i + j) as usize;
            tmp[idx] += 1;
            tmp[idx] > 9 && flashed.insert((i, j))
        };
        for i in 0..10 {
            for j in 0..10 {
                if bump(i, j) {
                    flashing.push((i, j));
                }
            }
        }
        while let Some((i, j)) = flashing.pop() {
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    if bump(i + di, j + dj) {
                        flashing.push((i + di, j + dj));
                    }
                }
            }
        }
        for &(i, j) in &flashed {
            tmp[(10 * i + j) as usize] = 0;
        }
        std::mem::swap(&mut self.0, &mut tmp);
    }
}

fn solve1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let mut total = 0;
    for _ in 0..100 {
        grid.step();
        total += grid.0.iter().filter(|&&v| v == 0).count();
    }
    total
}

fn solve2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    for step in 1.. {
        grid.step();
        if grid.0.iter().all(|&v| v == 0) {
            return step;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL: &str = r"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    ";

    #[test]
    fn small() -> anyhow::Result<()> {
        let input = parse_input(SMALL)?;
        assert_eq!(solve1(&input), 1656);
        assert_eq!(solve2(&input), 195);
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day11.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(solve1(&input), 1741);
        assert_eq!(solve2(&input), 440);
        Ok(())
    }
}

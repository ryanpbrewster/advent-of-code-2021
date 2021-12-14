use std::collections::HashSet;

use anyhow::anyhow;

struct Grid {
    width: i32,
    height: i32,
    values: Vec<u32>,
}
impl Grid {
    fn get(&self, i: i32, j: i32) -> u32 {
        if i < 0 || j < 0 || i >= self.height || j >= self.width {
            9
        } else {
            self.values[(i as usize * self.width as usize + j as usize)]
        }
    }
}
fn parse_input(input: &str) -> anyhow::Result<Grid> {
    let mut values = Vec::new();
    let mut width = 1;
    for line in input.trim().lines().map(|l| l.trim()) {
        width = line.len();
        for c in line.chars() {
            values.push(
                c.to_digit(10)
                    .ok_or_else(|| anyhow!("invalid digit: {}", c))?,
            );
        }
    }
    Ok(Grid {
        width: width as i32,
        height: values.len() as i32 / width as i32,
        values,
    })
}

fn solve1(grid: &Grid) -> u32 {
    let mut risk = 0;
    for i in 0..grid.height {
        for j in 0..grid.width {
            let v = grid.get(i, j);
            if grid.get(i - 1, j) > v
                && grid.get(i + 1, j) > v
                && grid.get(i, j - 1) > v
                && grid.get(i, j + 1) > v
            {
                risk += v + 1;
            }
        }
    }
    risk
}

fn solve2(grid: &Grid) -> usize {
    let mut explorer = Explorer {
        grid,
        vis: Default::default(),
    };
    let mut basins = Vec::new();
    for i in 0..grid.height {
        for j in 0..grid.width {
            let size = explorer.explore(i, j);
            if size > 0 {
                basins.push(size);
            }
        }
    }
    basins.sort();
    basins.into_iter().rev().take(3).product()
}
struct Explorer<'a> {
    grid: &'a Grid,
    vis: HashSet<(i32, i32)>,
}
impl<'a> Explorer<'a> {
    fn explore(&mut self, i: i32, j: i32) -> usize {
        if self.grid.get(i, j) >= 9 || self.vis.contains(&(i, j)) {
            return 0;
        }
        self.vis.insert((i, j));
        1 + self.explore(i - 1, j)
            + self.explore(i + 1, j)
            + self.explore(i, j - 1)
            + self.explore(i, j + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL: &str = r"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    ";

    #[test]
    fn small() -> anyhow::Result<()> {
        let input = parse_input(SMALL)?;
        assert_eq!(solve1(&input), 15);
        assert_eq!(solve2(&input), 1134);
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day09.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(solve1(&input), 526);
        assert_eq!(solve2(&input), 1123524);
        Ok(())
    }
}

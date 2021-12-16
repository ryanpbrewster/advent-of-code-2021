use std::collections::{hash_map::Entry, HashMap};

use anyhow::anyhow;

struct Edge(String, String);
fn parse_input(input: &str) -> anyhow::Result<CaveMap> {
    let edges: Vec<Edge> = input
        .trim()
        .lines()
        .map(|line| {
            let (src, dst) = line
                .trim()
                .split_once('-')
                .ok_or_else(|| anyhow!("invalid edge: {}", line))?;
            Ok(Edge(src.to_owned(), dst.to_owned()))
        })
        .collect::<anyhow::Result<_>>()?;
    Ok(adjacency_matrix(&edges))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cave {
    Start,
    End,
    Small,
    Big,
}
impl Cave {
    fn classify(name: &str) -> Cave {
        if name.chars().next().unwrap().is_ascii_uppercase() {
            Cave::Big
        } else {
            Cave::Small
        }
    }
}
struct CaveMap {
    caves: Vec<Cave>,
    conns: Vec<Vec<usize>>,
}
const START: usize = 0;
const END: usize = 1;
fn adjacency_matrix(edges: &[Edge]) -> CaveMap {
    let mut registry = HashMap::new();
    let mut caves = Vec::new();
    let mut conns = Vec::new();

    registry.insert("start".to_owned(), START);
    caves.push(Cave::Start);
    conns.push(Vec::new());

    registry.insert("end".to_owned(), END);
    caves.push(Cave::End);
    conns.push(Vec::new());

    for Edge(a, b) in edges {
        let a = match registry.entry(a.to_owned()) {
            Entry::Occupied(occ) => *occ.get(),
            Entry::Vacant(vac) => {
                let n = caves.len();
                vac.insert(n);
                caves.push(Cave::classify(a));
                conns.push(Vec::new());
                n
            }
        };
        let b = match registry.entry(b.to_owned()) {
            Entry::Occupied(occ) => *occ.get(),
            Entry::Vacant(vac) => {
                let n = caves.len();
                vac.insert(n);
                caves.push(Cave::classify(b));
                conns.push(Vec::new());
                n
            }
        };
        conns[a].push(b);
        conns[b].push(a);
    }
    CaveMap { caves, conns }
}

fn solve1(adj: &CaveMap) -> u64 {
    let mut vis = vec![0; adj.caves.len()];
    count_paths(START, &adj, &mut vis, 0)
}
fn solve2(adj: &CaveMap) -> u64 {
    let mut vis = vec![0; adj.caves.len()];
    count_paths(START, &adj, &mut vis, 1)
}

fn count_paths(cur: usize, adj: &CaveMap, vis: &mut [usize], allowed_repeats: usize) -> u64 {
    let mut count = 0;
    for &neighbor in &adj.conns[cur] {
        count += match adj.caves[neighbor] {
            Cave::Start => 0,
            Cave::End => 1,
            Cave::Big => count_paths(neighbor, adj, vis, allowed_repeats),
            Cave::Small => {
                if allowed_repeats == 0 && vis[neighbor] > 0 {
                    0
                } else {
                    let allowed_repeats = if vis[neighbor] == 0 {
                        allowed_repeats
                    } else {
                        allowed_repeats - 1
                    };
                    vis[neighbor] += 1;
                    let c = count_paths(neighbor, adj, vis, allowed_repeats);
                    vis[neighbor] -= 1;
                    c
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL: &str = r"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    ";

    #[test]
    fn small() -> anyhow::Result<()> {
        let input = parse_input(SMALL)?;
        assert_eq!(solve1(&input), 10);
        assert_eq!(solve2(&input), 36);
        Ok(())
    }

    #[test]
    fn normal() -> anyhow::Result<()> {
        let raw = std::fs::read_to_string("data/day12.input")?;
        let input = parse_input(&raw)?;
        assert_eq!(solve1(&input), 3497);
        assert_eq!(solve2(&input), 93686);
        Ok(())
    }
}

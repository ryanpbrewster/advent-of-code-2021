use std::collections::HashMap;

use anyhow::anyhow;

struct Edge(String, String);
fn parse_input(input: &str) -> anyhow::Result<Vec<Edge>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (src, dst) = line
                .trim()
                .split_once('-')
                .ok_or_else(|| anyhow!("invalid edge: {}", line))?;
            Ok(Edge(src.to_owned(), dst.to_owned()))
        })
        .collect()
}

fn solve1(inputs: &[Edge]) -> u64 {
    let adj = adjacency_matrix(inputs);
    let mut path = vec!["start".to_owned()];
    count_paths(&adj, &mut path)
}
fn adjacency_matrix(edges: &[Edge]) -> HashMap<String, Vec<String>> {
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for Edge(a, b) in edges {
        adj.entry(a.clone()).or_default().push(b.clone());
        adj.entry(b.clone()).or_default().push(a.clone());
    }
    adj
}
fn count_paths(adj: &HashMap<String, Vec<String>>, path: &mut Vec<String>) -> u64 {
    let cur = path.last().unwrap();
    if cur == "end" {
        return 1;
    }

    let mut count = 0;
    for neighbor in &adj[cur] {
        if neighbor.chars().next().unwrap().is_ascii_uppercase() || !path.contains(neighbor) {
            path.push(neighbor.clone());
            count += count_paths(adj, path);
            path.pop();
        }
    }
    count
}

fn solve2(inputs: &[Edge]) -> u64 {
    let adj = adjacency_matrix(inputs);
    let mut path = vec!["start".to_owned()];
    count_paths_2(&adj, &mut path, false)
}
fn count_paths_2(adj: &HashMap<String, Vec<String>>, path: &mut Vec<String>, doubled: bool) -> u64 {
    let cur = path.last().unwrap();
    if cur == "end" {
        return 1;
    }

    let mut count = 0;
    for neighbor in &adj[cur] {
        if neighbor == "start" {
            continue;
        }
        if neighbor.chars().next().unwrap().is_ascii_uppercase() {
            path.push(neighbor.clone());
            count += count_paths_2(adj, path, doubled);
            path.pop();
        } else if !path.contains(neighbor) {
            path.push(neighbor.clone());
            count += count_paths_2(adj, path, doubled);
            path.pop();
        } else if !doubled {
            path.push(neighbor.clone());
            count += count_paths_2(adj, path, true);
            path.pop();
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

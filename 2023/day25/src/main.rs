use itertools::Itertools;
use petgraph::dot::{Config, Dot};
use petgraph::prelude::EdgeIndex;
use petgraph::prelude::*;
use petgraph::visit::{Dfs, Visitable, Walker};
use petgraph::{
    data::Build,
    graph::{NodeIndex, UnGraph},
};
use std::collections::HashSet;

use std::{collections::HashMap, io};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "jqt: rhn xhk nvd".to_string(),
            "rsh: frs pzl lsr".to_string(),
            "xhk: hfx".to_string(),
            "cmg: qnr nvd lhk bvb".to_string(),
            "rhn: xhk bvb hfx".to_string(),
            "bvb: xhk hfx".to_string(),
            "pzl: lsr hfx nvd".to_string(),
            "qnr: nvd".to_string(),
            "ntq: jqt hfx bvb xhk".to_string(),
            "nvd: lhk".to_string(),
            "lsr: lhk".to_string(),
            "rzs: qnr cmg lsr rsh".to_string(),
            "frs: qnr lhk lsr".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "54")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "unexpected")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
}

fn parse(lines: &Vec<String>) -> UnGraph<&str, ()> {
    // let nodes: HashSet<String> = HashSet::new();
    let mut graph = UnGraph::<&str, ()>::new_undirected();
    let mut nodes = HashMap::new();
    let edges = lines
        .iter()
        .flat_map(|l| {
            let (n, remaining) = l.split(": ").collect_tuple().expect("must tuple");
            let n_idx = *nodes.entry(n).or_insert_with(|| graph.add_node(n));
            remaining
                .split_whitespace()
                .map(|e| (n_idx, *nodes.entry(e).or_insert_with(|| graph.add_node(e))))
                .collect_vec()
        })
        .collect_vec();
    // let graph = UnGraph::<&str, ()>::from_edges(edges.iter());
    graph.extend_with_edges(edges);
    graph
}

fn part1(lines: &Vec<String>) -> String {
    let mut graph = parse(lines);

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeIndexLabel]));
    // look at the above graph visually for the edges
    //
    // I used inkscape after converting the dot to svg so I could drag the lines
    // down and read the labels, it was very clear what the 3 lines were

    graph.remove_edge(EdgeIndex::from(438));
    graph.remove_edge(EdgeIndex::from(3219));
    graph.remove_edge(EdgeIndex::from(1101));

    let grp_sizes = count_group_sizes(&graph);
    assert_eq!(2, grp_sizes.len());

    // println!("");
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeIndexLabel]));

    // LOL can't believe that worked
    format!("{}", grp_sizes.iter().product::<usize>())
}

fn count_group_sizes(graph: &UnGraph<&str, ()>) -> Vec<usize> {
    let mut group_sizes = Vec::new();
    let mut visited = HashSet::new();

    for node_index in graph.node_indices() {
        if !visited.contains(&node_index) {
            let mut size = 0;
            let mut dfs = Dfs::new(graph, node_index);

            while let Some(nx) = dfs.next(graph) {
                if visited.insert(nx) {
                    size += 1;
                }
            }

            group_sizes.push(size);
        }
    }

    group_sizes
}

fn part2(lines: &Vec<String>) -> String {
    let board = parse(lines);

    format!("{}", todo!())
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

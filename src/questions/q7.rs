use crate::{ProblemInput, Solution};

use petgraph::graph::NodeIndex;
use petgraph::visit::{Dfs, EdgeRef, Topo};
use petgraph::Graph;
use regex::Regex;
use std::collections::HashMap;

pub struct Q7;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Bag<'a> {
    modifier: &'a str,
    colour: &'a str,
}

impl<'a> Bag<'a> {
    fn from_str(s: &'a str) -> Self {
        let split = s.find(' ').expect("failed to parse");

        Self {
            modifier: &s[..split],
            colour: &s[split + 1..],
        }
    }
}

#[derive(Debug)]
struct BagRelation<'a> {
    source_bag: Bag<'a>,
    target_bags: Vec<(usize, Bag<'a>)>,
}

impl<'a> BagRelation<'a> {
    fn from_line(line: &'a str) -> Self {
        // Parse everything before bags.
        let bag = Bag::from_str(&line[..line.find("bags").unwrap() - 1]);

        let mut relations = Vec::new();
        let re = Regex::new(r"(?P<n>\d) (?P<m>\w+) (?P<c>\w+) bag[s]?").unwrap();

        for cap in re.captures_iter(line) {
            let target = Bag {
                modifier: cap.name("m").unwrap().as_str(),
                colour: cap.name("c").unwrap().as_str(),
            };
            let count = cap.name("n").unwrap().as_str().parse().unwrap();
            relations.push((count, target));
        }

        Self {
            source_bag: bag,
            target_bags: relations,
        }
    }
}

fn build_bag_graph(lines: &ProblemInput) -> (Graph<Bag, usize>, NodeIndex<u32>) {
    let relations: Vec<BagRelation> = lines
        .lines
        .iter()
        .map(|line| BagRelation::from_line(line.as_str()))
        .collect();

    let mut g = Graph::<Bag, usize>::new();
    let mut idx = HashMap::new();
    for r in relations.iter() {
        idx.insert(r.source_bag, g.add_node(r.source_bag));
    }

    for s in relations.iter() {
        for (count, t) in s.target_bags.iter() {
            g.add_edge(idx[&s.source_bag], idx[t], *count);
        }
    }

    (g, idx[&Bag::from_str("shiny gold")])
}

impl Solution for Q7 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let (mut g, idx) = build_bag_graph(lines);
        g.reverse();

        let mut dfs = Dfs::new(&g, idx);
        while dfs.next(&g).is_some() {}

        (dfs.discovered.count_ones(..) - 1).to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let (g, idx) = build_bag_graph(lines);
        let mut count = HashMap::new();
        count.insert(idx, 1);

        let mut topo = Topo::new(&g);
        while let Some(n) = topo.next(&g) {
            if let Some(c) = count.get(&n).copied() {
                for out_edge in g.edges(n) {
                    *count.entry(out_edge.target()).or_default() += c * out_edge.weight();
                }
            }
        }

        (count.values().sum::<usize>() - 1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q7 = Q7;
        assert_eq!(q7.part1(&load_problem_input(7)), 151.to_string());
    }

    #[test]
    fn test_part2_solution() {
        let q7 = Q7;
        assert_eq!(q7.part2(&load_problem_input(7)), 41559.to_string());
    }
}

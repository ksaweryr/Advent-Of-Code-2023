use std::{collections::{HashMap, HashSet}, io::{self, BufRead}, iter, fs};

pub fn solve(input: String) {
    let dot = graphviz(&input);
    fs::write("graph.txt", dot).expect("Couldn't generate Graphviz graph");
    println!("Saved graphiz representation of the input to graph.txt. Run `$ dot -Tpng -Kneato graph.txt > graph.png to generate a PNG file.");
    let g = parse_input(&input);
    println!("Edges to be ignored (from/to from/to from/to):");
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let ignored_edges = parse_ignored_edges(&line);

    println!("{}", part1(&g, ignored_edges));
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::<String, Vec<String>>::new();

    for line in input.lines() {
        let (from, to) = line.split_once(": ").expect("Invalid line format");
        let from = from.to_owned();

        to.split(" ").for_each(|v| {
            result.entry(from.clone()).or_default().push(v.to_owned());
            result.entry(v.to_owned()).or_default().push(from.clone());
        });
    }

    result
}

fn parse_ignored_edges<'a>(s: &'a str) -> Vec<(&'a str, &'a str)> {
    s.split(" ").flat_map(|p| {
        let (a, b) = p.split_once("/").expect("Invalid edge format");
        iter::once((a, b)).chain(iter::once((b, a)))
    }).collect()
}

fn part1(g: &HashMap<String, Vec<String>>, ignored_edges: Vec<(&str, &str)>) -> usize {
    let total = g.len();
    let mut cnt = 0;
    let mut visited = HashSet::new();
    let mut s = Vec::new();

    s.push(g.keys().next().unwrap());
    visited.insert(s[0]);

    while !s.is_empty() {
        let node = s.pop().unwrap();
        cnt += 1;

        for v in g[node].iter().filter(|t| !ignored_edges.contains(&(node, t))) {
            if visited.contains(v) {
                continue;
            }

            visited.insert(v);
            s.push(v);
        }
    }

    cnt * (total - cnt)
}

fn graphviz(input: &str) -> String {
    "graph {".to_owned()
    + input.lines().map(|l| "\t".to_owned() + l.replace(": ", " -- {").as_ref() + "};\n").collect::<String>().as_ref()
    + "}"
}
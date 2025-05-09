use crate::AoCResult;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day7a() -> AoCResult<usize> {
    let file = File::open("data/input_day7a.txt")?;
    let reader = BufReader::new(file);
    let mut inverse_graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        parse(&line, &mut inverse_graph);
    }

    Ok(count(&inverse_graph))
}

fn parse(line: &str, inverse_graph: &mut HashMap<String, Vec<String>>) {
    let (parent, children) = line
        .strip_suffix('.')
        .unwrap()
        .split_once(" contain ")
        .unwrap();
    let parent = parent.strip_suffix(" bags").unwrap().to_string();

    if children != "no other bags" {
        children.split(", ").for_each(|c| {
            let mut words = c.split_whitespace();
            let _count = words.next().unwrap().parse::<usize>().unwrap();
            let color = words.take(2).collect::<Vec<_>>().join(" ");
            inverse_graph.entry(color).or_default().push(parent.clone());
        })
    };
}

fn count(inverse_graph: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = HashSet::new();
    let mut stack = vec!["shiny gold".to_string()];

    while let Some(bag) = stack.pop() {
        if let Some(parents) = inverse_graph.get(&bag) {
            for parent in parents {
                if visited.insert(parent.clone()) {
                    stack.push(parent.clone());
                }
            }
        }
    }

    visited.len()
}

fn count_fp(inverse_graph: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = HashSet::new();
    let mut stack = vec!["shiny gold".to_string()];

    while let Some(bag) = stack.pop() {
        inverse_graph
            .get(&bag)
            .into_iter()
            .flatten()
            .filter(|parent| visited.insert((*parent).clone()))
            .for_each(|parent| stack.push(parent.clone()));
    }

    visited.len()
}

pub fn solve_day7b() -> AoCResult<usize> {
    let file = File::open("data/input_day7a.txt")?;
    let reader = BufReader::new(file);
    let mut graph: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        parse_graph(&line, &mut graph);
    }

    Ok(count_total("shiny gold", &graph) - 1)
}

fn count_total(name: &str, graph: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut total = 1;
    if let Some(children) = graph.get(name) {
        for (child_size, child_name) in children {
            total += child_size * count_total(child_name, graph);
        }
    }
    total
}

fn count_total_fp(bag: &str, graph: &HashMap<String, Vec<(usize, String)>>) -> usize {
    1 + graph
        .get(bag)
        .map(|children| {
            children
                .iter()
                .map(|(count, child)| count * count_total_fp(child, graph))
                .sum::<usize>()
        })
        .unwrap_or(0)
}

fn parse_graph(line: &str, graph: &mut HashMap<String, Vec<(usize, String)>>) {
    let (parent, children) = line
        .strip_suffix('.')
        .unwrap()
        .split_once(" contain ")
        .unwrap();
    let parent = parent.strip_suffix(" bags").unwrap().to_string();

    if children != "no other bags" {
        children.split(", ").for_each(|c| {
            let mut words = c.split_whitespace();
            let count = words.next().unwrap().parse::<usize>().unwrap();
            let color = words.take(2).collect::<Vec<_>>().join(" ");
            graph
                .entry(parent.clone())
                .or_default()
                .push((count, color));
        })
    };
}

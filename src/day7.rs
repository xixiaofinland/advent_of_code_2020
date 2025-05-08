use crate::AoCResult;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day7a() -> AoCResult<usize> {
    let file = File::open("data/input_day7a_simple.txt")?;
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
    // do DFS or BFS here
    0
}

pub fn solve_day7b() -> AoCResult<usize> {
    Ok(0)
}

use crate::AoCResult;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

pub fn solve_day21() -> AoCResult<String> {
    let input = fs::read_to_string("data/input_day21.txt").unwrap();
    let foods = parse_input(&input);
    let candidates = build_candidates(&foods);

    let part1 = count_safe_ingredients(&foods, &candidates);
    println!("Part 1: {}", part1);

    let resolved = resolve_allergens(candidates);
    let mut dangerous: Vec<_> = resolved.into_iter().collect();
    dangerous.sort_by_key(|(allergen, _)| allergen.clone());
    let part2 = dangerous
        .into_iter()
        .map(|(_, ingredient)| ingredient)
        .collect::<Vec<_>>()
        .join(",");
    println!("Part 2: {}", part2);
    Ok(part2)
}

fn parse_input(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.trim_end_matches(')').split(" (contains ").collect();
            let ingredients = parts[0].split_whitespace().map(|s| s.to_string()).collect();
            let allergens = parts[1].split(", ").map(|s| s.to_string()).collect();
            Food {
                ingredients,
                allergens,
            }
        })
        .collect()
}

fn build_candidates(foods: &[Food]) -> HashMap<String, HashSet<String>> {
    let mut candidates = HashMap::new();

    for food in foods {
        for allergen in &food.allergens {
            let entry = candidates
                .entry(allergen.clone())
                .or_insert_with(|| food.ingredients.clone());
            entry.retain(|ingredient| food.ingredients.contains(ingredient));
        }
    }

    candidates
}

fn count_safe_ingredients(foods: &[Food], candidates: &HashMap<String, HashSet<String>>) -> usize {
    let possible_allergen_ingredients: HashSet<String> =
        candidates.values().flatten().cloned().collect();

    foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .filter(|ingredient| !possible_allergen_ingredients.contains(*ingredient))
        .count()
}

fn resolve_allergens(mut candidates: HashMap<String, HashSet<String>>) -> HashMap<String, String> {
    let mut resolved = HashMap::new();

    while !candidates.is_empty() {
        let determined: Vec<(String, String)> = candidates
            .iter()
            .filter_map(|(allergen, ingredients)| {
                if ingredients.len() == 1 {
                    Some((allergen.clone(), ingredients.iter().next().unwrap().clone()))
                } else {
                    None
                }
            })
            .collect();

        for (allergen, ingredient) in determined {
            candidates.remove(&allergen);
            for ingredients in candidates.values_mut() {
                ingredients.remove(&ingredient);
            }
            resolved.insert(allergen, ingredient);
        }
    }

    resolved
}

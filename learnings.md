# D1
## ok() to turn Reuslt to Option in filter_map()
- use `?` to return quick

```rust
let numbers: Vec<usize> = reader
    .lines()
    .filter_map(|line| line.ok()?.trim().parse().ok())
    .collect();
```

## HashSet is much faster than double-loop
- `.contains(&x)` is O(1). So this solution is O(n) while double-loop is O(n_power_2)!

## HashSet v.s. BTTreeSet

HashSet is like a labeled box which we can jump to the box to check, thus O(1)
BTTreeSet is sorted thus O(logN), but can return sorted result, like all items
in range(a_to_b).

# D2

## I used `iter.next()` multiple times to pattern-match [1-3 a: abcdef].
But this is smarter:

```rust
let (range, char_part, password) = match line.split_whitespace().collect::<Vec<_>>()[..] {
    [range, char_part, password] => (range, char_part, password),
    _ => return Err("Invalid line format".into()),
};

let (min, max) = {
    let (min, max) = range.split_once('-').ok_or("Invalid range format")?;
    (min.parse::<usize>()?, max.parse::<usize>()?)
};

```

Interestingly, the FP way achieving the same:

```rust
let parts: [&str; 3] = vec.split_whitespace().collect::<Vec<_>>()
    .try_into()
    .map_err(|_| "Expected exactly 3 parts")?;

let (first, second) = range
    .split_once('-')
    .ok_or("Invalid range format")
    .and_then(|(a, b)| Ok((a.parse::<usize>()? - 1, b.parse::<usize>()? - 1)))?;

```

# D3

## `map()` v.s. `and_then()` in `Result<T>`
- map() won't produce new error, it only maps OK value from a to b
- and_then() returns a Result, the inner logic to process incoming OK value can
  propgrate new errors or use `?`.

## collect() over iterator with item = Result<T, E>
There can be two built-in ways as below. So we need turbofish or specific type.
- Vec<Result<T, E>>
- Result<Vec<T>, E> (i.e. all-or-none)

## FP can save intermediate variables for `for` loop

```rust
let mut count = 0;
let mut column_index = 0;

for row in maze.iter().skip(1) {
    column_index += 3;
    if row[column_index % width] {
        count += 1;
    }
}
```

```rust
let count = maze.iter().skip(1).enumerate().filter(|(i, row)| {
    let col = (i * 3) % width;
    row[col]
}).count();

```

# D5

c.clone().into_iter() v.s. c.iter().copied() v.s. c.iter().cloned()
When the element is `Copy` then use `copied()`

# D6

`hash_map.extend(line.chars())` is better than `for c in line chars()
{hash_map.insert(c);}`

functional answer is cool!

```rust
let input = std::fs::read_to_string("data/input_day6a.txt")?
let total: usize = input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| c.is_ascii_lowercase())  // ignores `\n`
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();

```

- intersection `BitAnd` & for `set` (not Vec) and defensive code
- `fold()` usage

```rust
fn intersection_size(group: &[String]) -> usize {
    let mut iter = group.iter().map(|s| s.chars().collect::<HashSet<_>>());
    if let Some(first) = iter.next() {
        iter.fold(first, |acc, set| &acc & &set).len()
    } else {
        0
    }
}

```

Functional is so compact!
- no need to handle the last element edge case:
- `reduce()` replaces `fold()`

```rust
pub fn solve_day6b_ff() -> AoCResult<usize> {
    let input = std::fs::read_to_string("data/input_day6a.txt")?;

    let total = input
        .split("\n\n") // group by blank lines
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|a, b| &a & &b) // set intersection
                .map_or(0, |set| set.len()) // default to 0 if group is empty
        })
        .sum();

    Ok(total)
}

```

# d7

- Forward Graph:
- Inverse/Reverse Graph:
- BFS v.s. DFS (Vec v.s. VecDeque)

HashMap (O(1)) is better than Vec to store a graph info (O(n)).

count() v.s. count_fp()

---

d7b recursion construct: best answer is to model the problem in one method
`count_total_bags()`. Mine works but it has `count_self() + count_child()` which
works but awkward.

```rust
count_total_bags("shiny gold", &graph) - 1;

fn count_total_bags(name: &str, graph: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut total = 1;
    if let Some(children) = graph.get(name) {
        for (child_size, child_name) in children {
            total += child_size * count_total_bags(child_name, graph);
        }
    }
    total
}
```

fp style: learn `unwrap_or()` for Option<T> and Result<T>
```rust
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

```
# d8

Read the code below to see how the chaining flows.

```rust
let content = reader
    .lines()
    .filter_map(|line_result| {
        line_result.ok().and_then(|line| {
            line.split_once(" ")
                .map(|(first, second)| (first.to_string(), second.to_string()))
        })
    })
    .collect::<Vec<(String, String)>>();

```

`filter_map()` takes care of both filtering out Nones and unwrapping Some. It
expects the inner clojure returns an Option<T> and will produce an iterator with
only extracted values from Some(T).

`and_then()` is essentially a way to chain operations that each might return
None, without having to nest a lot of if let Some(_) statements. It's
particularly useful for sequential operations that each might fail.

In our code example, and_then() links `line_result.ok()` with `split_once()`,
propagating None if either operation fails.

`filter_map()` will silently drop all None. If we wanna proper error handling,
we can switch to `map()` and let its clojure return Result as:

```rust
pub fn solve_day8a_with_error_handling() -> AoCResult<usize> {
    let file = File::open("data/input_day8a_simple.txt")?;
    let reader = BufReader::new(file);

    let parsed_lines: Vec<(String, String)> = reader
        .lines()
        .map(|line_result| -> AoCResult<(String, String)> {
            let line = line_result?; // Propagate IO errors

            // The simplest error option with Rust 2021+
            // as String has implemented std::error::Error
            let (first, second) = line
                .split_once(" ")
                .ok_or(format!("Line missing space: '{}'", line))?;

            Ok((first.to_string(), second.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?; // Collect and propagate any errors

    // Continue processing with parsed_lines
    println!("Successfully parsed {} lines", parsed_lines.len());

    Ok(0) // Replace with actual result calculation
}
```

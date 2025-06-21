# Learnings
It records what I learned in building the solutions

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

# D7

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
# D8

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

A learning tip: ask GPT "quick reference for common AoC input parsing idioms in Rust"

The enum construct is better!
```rust
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}
```

Better algorithm: track original path and flip only ones on the path. Check `day8b.rs`

# D9

When iter element is a reference `&`, it's more idiomatic to use `copied()` to
get the value instead of deref.

Both `any()` and `find()` return early!
Use `iter().any()` in `validate()`: it is short-circuiting; in other words, it will stop processing
as soon as it finds a `true`, given that no matter what else happens,
the result will also be `true`.
Use `.find()` when you need to access or use the matching value, not just know
it exists.


## d9a performance:

"remove oldest element sums and add the new" is the least computation way, but
hard to model with correct data struct (VecDeque). sliding window with hashset strikes the
balance in the middle.

sliding window also uses the "2-sum" problem solution with "one-pass" way (Day1)

## d9b

I couldn't figure out a good solution but relied on GPT.
The sliding window is smart! It requires 1) Contiguous Elements 2) All Positive
Numbers

# D10

## d10a

Learn Vec's `windows(n)`

learn HashMap APIs: `diffs.entry(diff).or_default() -> &mut usize`

Vec `sort()` v.s `sort_unstable()`

## d10b

The algorithm.

`checked_sub()` for overflow prevention
```rust
        let c_sum = (1..=3)
            .filter_map(|diff| c.checked_sub(diff))
            .filter_map(|prev| ways.get(&prev))
            .sum();
```

For chained Option, `map()` v.s. `and_then()`

```rust
    adapters
        .last()
        .and_then(|jolt| ways.get(jolt).copied())
        .ok_or_else(|| "No adapters found".into())

    // adapters
    //     .last()
    //     .map(|&jolt| ways[&jolt])
    //     .ok_or_else(|| "No adapters found".into())
```

# D11

## d11a

- `has_occupied_adjacent()`: idiomatic way to handle row and col boundaries
- `four_or_more_adjacent_occupied()`: filter().count() is better than fold() in
  this use-case
- `matches!` v.s. match-pattern in `calculate()`
- `std::mem:swap()` instead of `.clone()` after each loop

# D12

## d12a

Easy assignment, but how to model and gpt version is still much better.

# D13

## d13a

Instead of using .find(...) and losing the s, you can use .find_map(...) to
return both the matching timestamp and its corresponding s

```rust
let (wait_time, s) = (minutes..)
    .find_map(|timestamp| {
        schedules
            .iter()
            .find(|&&s| timestamp % s == 0)
            .map(|&s| (timestamp, s))
    })
    .unwrap();

// let result = (minutes..)
//     .into_iter()
//     .find(|timestamp| {
//         for s in &schedules {
//             if timestamp % s == 0 {
//                 return true;
//             }
//         }
//         false
//     })
//     .unwrap();

```

`lines.flatten()` turns Iterator<Result<String, _>> into Iterator<String>,
skipping errors.
It’s a concise way to avoid `.unwrap()` spam, but be aware it suppresses IO
errors, so use it only if you’re okay ignoring those.

## d13b

Good to know CRT algorithm exists and what the problem is. But I don't focus on
algorithm here.

how to build index into the `Schedule`.
```rust
let schedules: Vec<Schedule> = line2
    .split(',')
    .enumerate()
    .filter_map(|(i, id)| {
        if id == "x" {
            None
        } else {
            Some(Schedule {
                num: id.parse().unwrap(),
                time_offset: i,
            })
        }
    })
    .collect();
```

# D14

## d14a

bit calculation re-learn:
- `& 0` force 0
- `| 1` force 1
- `& 1 -> | 0` keep the original

Therefore, we construct the and_mask and or_mask bit by bit then apply
`(value & and_mask) | or_mask`

`'1' => or_mask |= 1 << i` and `'0' => and_mask &= !(1 << i)`

patterns to learn:
- To set bit i : `value |= 1 << i;` (force bit to 1)
- To clear bit i: `value &= !(1 << i);` (force bit to 0)
- To flip bit i: `value ^= 1 << i;` (toggle the bit)
- Read bit i: `(value >> i) & 1` (get bit as 0 or 1)

# D15

## d15a
compare two solutions
- pay attention to `by_ref()` in iterator

## d15b

using Vec<usize> to replace HashMap. It trades space for speed.

HashMap is amortized O(1) but involves hashing, probing, and more pointer
indirection than raw array access.

"You don’t really need to pre-allocate all 30_000_000 slots unless you expect
numbers spoken to reach that high. Often a safe size like 32_000_000 covers all
cases. You could also grow dynamically like:

```rust
if current >= last_seen.len() {
    last_seen.resize(current + 1, 0);
}
```

# D16

## d16a
compare two solutions
- use `.lines()` over `.split("\n")`
- use `.flat_map()` when one line produces intoIterator items and then collect.

## d16b
compare two solutions, especially after `while` section, how to do it idiomatically.
- `nth(1)` is clearer than `skip(1).next()`
- Used `then_some()` for cleaner optional filtering.
- Pattern-matched first item in loop: `while let Some((col_idx, candidates)) = sorted_fields.first() {` v.s. `while !sorted_fields.is_empty() {`
- `.retain()` is very similar to `.filter()`, but it modifies the vector in-place
  instead of returning a new collection.

# D17

## d17a
Getting difficult as the requirements need to be flipped and then implmented
with multiple dimensions.

- learn how to model the expanding discrete dimension nodes using `HashSet` and
  `HashMap`
- check `iproduct!` from itertools crate

## d17b
Nothing new

# D18

## d18a
- `.try_fold()` (which is short-circuiting) v.s. `Vec<Vec<_>>` + `flatten()`
- `strip_prefix()` and `strip_suffix()`
- regex crate is much cleaner to parse the token! Compare it with `fn split_token`
- understand `expr -> operand -> op` nesting in the parsing logic. Elegent for
  this problem!

# D19

## d19a

- Study `match_rule_imperative()` first as a stepping stone to understand the
functional `match_rule()`
- there can be partial match situation: Did rule 0 match the entire message,
  leaving nothing unconsumed?

## d19b
- I don't have the mood to finish this. Just don't feel like it :D. Skip!

# D20

## d20a
I keep forgetting how to parse data from input file!
- `ok_or()` and `ok_or_else()` expect a type E that is compatible to Result<T, E>

```rust
let result: Result<i32, &str> = Some(42).ok_or("no value");
let result: Result<i32, String> = Some(42).ok_or("no value".to_string());

// it works because Box<dyn std::error::Error> has implemented From<&str>
.map(|chunk| -> AoCResult<_> {
    let tile_line = chunk.lines().next().ok_or("missing title line")?;
})
```

- HashMap's Key must be `Eq` and `Hash`(make sense, right!?).
  - `Eq` is a marker trait, and must also implement `PartialEq`.
  - `Eq` guarantees strict, reflexive, total equality. (f64 is not `Eq` as `f64::NAN != f64::NAN`)
  - `PartialEq` lets you do `==` and `!=`.

## d20b
  skip.

# D21

## d21a
  - `Hashmap.entry().or_insert_with()`
  - `candidates.retain(|ingredient| food.ingredients.contains(ingredient));`
    v.s. `*candidates = candidates.intersection(&food.ingredients).cloned().collect();`

# D22

## d22a
- use `VecDeque`: `pop_front()` and `push_back()`

## d22b
Solution done by GPT, learned:
- Enum: `Player::One`, `Player::Two`
- `play()` returns `(winner, winner_deck)`
- Iterator: `.take().copied()`

# D23

## d23a
again good practice for `VecDeque`!
- `Vec` is optimized for appending at the end (push / pop from the back): O(1).
- `VecDeque` is optimized for both ends: push_front, pop_front, push_back,
  pop_back — all O(1).

  # D24

  ## d24a
- Hexagonal Grids to model the movement
- using `struct Hex(i32, i32)`
- Memory effeciency: parse the input with `Enum`, then use a `impl fn` to calculate
the movement
```rust
#[derive(Debug, Clone, Copy)]
enum Direction {
    E, W, NE, NW, SE, SW
}

impl Direction {
    fn delta(self) -> (i32, i32) {
        match self {
            Direction::E  => (1, 0),
            Direction::W  => (-1, 0),
            Direction::NE => (1, -1),
            Direction::NW => (0, -1),
            Direction::SE => (0, 1),
            Direction::SW => (-1, 1),
        }
    }
}
```

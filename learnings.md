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

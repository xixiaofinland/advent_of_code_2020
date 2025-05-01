# d1
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

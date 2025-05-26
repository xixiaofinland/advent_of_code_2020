use crate::AoCResult;

pub fn solve_day15b() -> AoCResult<usize> {
    let starting_numbers = [0, 20, 7, 16, 1, 18, 15];
    let max_turns = 30_000_000;

    // Use Vec to store the last turn each number was spoken
    // Most numbers will be small, so this is more efficient than HashMap
    let mut last_seen: Vec<usize> = vec![0; max_turns];

    // Initialize with starting numbers, except the last
    for (turn, &num) in starting_numbers.iter().enumerate().take(starting_numbers.len() - 1) {
        last_seen[num] = turn + 1;
    }

    // Start with the last initial number
    let mut current = *starting_numbers.last().unwrap();

    for turn in starting_numbers.len()..max_turns {
        let next = if last_seen[current] == 0 {
            0  // First time seeing this number
        } else {
            turn - last_seen[current]  // Difference between current turn and last seen
        };

        last_seen[current] = turn;  // Update when current number was last seen
        current = next;
    }

    Ok(current)
}

// Alternative implementation matching your HashMap style more closely
pub fn solve_day15b_alt() -> AoCResult<usize> {
    let starting_numbers = [0, 20, 7, 16, 1, 18, 15];
    let max_turns = 30_000_000;

    let mut last_seen: Vec<usize> = vec![0; max_turns];

    // Initialize all but last starting number
    for (i, &num) in starting_numbers.iter().enumerate().take(starting_numbers.len() - 1) {
        last_seen[num] = i + 1;
    }

    let mut current = *starting_numbers.last().unwrap();

    for turn in starting_numbers.len()..max_turns {
        let prev_turn = last_seen[current];
        last_seen[current] = turn;

        current = if prev_turn == 0 { 0 } else { turn - prev_turn };
    }

    Ok(current)
}

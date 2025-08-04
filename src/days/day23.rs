use crate::AoCResult;
use std::collections::VecDeque;

pub fn solve_day23a() -> AoCResult<usize> {
    let all_elements = [8, 7, 2, 4, 9, 5, 1, 3, 6];
    let mut circle: VecDeque<usize> = VecDeque::from(all_elements);
    let min = *all_elements.iter().min().unwrap();
    let max = *all_elements.iter().max().unwrap();

    for _move in 0..100 {
        // The current cup is always at front
        let current = circle[0];

        // Pick up 3 cups
        let mut pickup = Vec::with_capacity(3);
        for _ in 0..3 {
            // remove after rotating to simulate circular behavior
            circle.rotate_left(1);
            pickup.push(circle.pop_front().unwrap());
        }

        // Select destination cup
        let mut destination_label = current;
        loop {
            destination_label = if destination_label == min {
                max
            } else {
                destination_label - 1
            };

            if !pickup.contains(&destination_label) {
                break;
            }
        }

        // Find destination index
        let dest_index = circle.iter().position(|&x| x == destination_label).unwrap();

        // Insert the picked up cups after destination
        for (i, &cup) in pickup.iter().enumerate() {
            circle.insert(dest_index + 1 + i, cup);
        }

        // Move to next current cup
        circle.rotate_left(1);
    }

    // After 100 moves: rotate to cup 1
    while circle[0] != 1 {
        circle.rotate_left(1);
    }

    // Collect result after cup 1
    let result: usize = circle.iter().skip(1).fold(0, |acc, &x| acc * 10 + x);
    println!("Final result: {}", result);
    Ok(result)
}


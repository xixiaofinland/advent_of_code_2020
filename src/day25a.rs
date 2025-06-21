use crate::AoCResult;

const SUBJECT_7: usize = 7;

pub fn solve_day25a() -> AoCResult<usize> {
    let card_key = 5764801;
    let card_loop_size = get_loop_size(card_key, SUBJECT_7);

    let door_key = 17807724;
    let door_loop_size = get_loop_size(door_key, SUBJECT_7);

    let result = get_encrypt_key(door_key, card_loop_size);
    let result2 = get_encrypt_key(card_key, door_loop_size);

    println!("{}", card_loop_size);
    println!("{}", door_loop_size);

    println!("{}", result);
    println!("{}", result2);
    Ok(result)
}

fn get_loop_size(key: usize, subject: usize) -> usize {
    let mut result = 1;

    for i in 1..1000 {
        result = result * subject % 20201227;
        if result == key {
            return i;
        }
    }
    panic!("can't locate any result.");
}

fn get_encrypt_key(subject: usize, loop_size: usize) -> usize {
    let mut result = 1;

    for _ in 0..loop_size {
        result = result * subject % 20201227;
    }
    result
}

use std::collections::HashMap;

use crate::util::get_input_contents;

pub struct DayOne {}

impl DayOne {
    pub fn run() -> () {
        println!("part one -> {}", part_one());
        println!("part two -> {}", part_two());
    }
}

fn part_one() -> i32 {
    let input_buffer = get_input_contents("./assets/day1_input.txt");
    let (left, right) = prepare_input(input_buffer);

    left.iter()
        .enumerate()
        .map(|(i, e)| (e - right[i]).abs())
        .sum()
}

fn part_two() -> i32 {
    let input_buffer = get_input_contents("./assets/day1_input.txt");
    let (left, right) = prepare_input(input_buffer);
    let mut n_map: HashMap<i32, i32> = HashMap::new();

    right.iter().for_each(|e| {
        if left.contains(e) {
            n_map.insert(*e, n_map.get(e).unwrap_or(&0) + 1);
        }
    });

    n_map.iter().map(|(k, v)| k * v).sum()
}

fn prepare_input(buffer: Vec<u8>) -> (Vec<i32>, Vec<i32>) {
    let buffer_string = String::from_utf8(buffer).unwrap();
    let split_buffer = buffer_string.trim().split("\n");

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    split_buffer.for_each(|e| {
        for (i, n) in e
            .trim()
            .split(" ")
            .filter(|v| v.replace(" ", "").len() > 0)
            .enumerate()
        {
            let parsed_n = n.parse::<i32>().unwrap();

            if (i + 1) % 2 == 0 {
                right.push(parsed_n);
            } else {
                left.push(parsed_n);
            }
        }
    });

    let closure = |a: &i32, b: &i32| a.cmp(b);
    left.sort_by(closure);
    right.sort_by(closure);

    (left, right)
}

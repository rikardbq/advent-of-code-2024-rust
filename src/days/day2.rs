use crate::util::get_input_contents;

pub struct DayTwo {}

impl DayTwo {
    pub fn run() -> () {
        println!("part one -> {}", part_one());
        println!("part two -> {}", part_two());
    }
}

fn check_row_entries<'a>(items: &'a Vec<i32>, desc: bool) -> bool {
    let mut predicate = true;
    items.iter().enumerate().for_each(|(i, e)| {
        if i < items.len() - 1 {
            let next = items[i + 1];
            let dir_check = if desc { *e > next } else { *e < next };
            let diff = (*e - next).abs();
            predicate = predicate && diff > 0 && diff < 4 && dir_check;
        }
    });

    predicate
}

fn part_one() -> i32 {
    let input_buffer = get_input_contents("./assets/day2_input.txt");
    let inputs = prepare_input(input_buffer);

    inputs
        .iter()
        .filter(|items| {
            let deref = *items;
            let cloned = deref.clone();
            check_row_entries(&cloned, false) || check_row_entries(&cloned, true)
        })
        .count() as i32
}

fn part_two() -> i32 {
    let input_buffer = get_input_contents("./assets/day2_input.txt");
    let inputs = prepare_input(input_buffer);

    inputs
        .iter()
        .filter(|items| {
            let mut predicate = true;
            for i in 0..items.len() {
                let deref = *items;
                let mut cloned = deref.clone();
                let _ = cloned.remove(i);

                predicate = check_row_entries(&cloned, false) || check_row_entries(&cloned, true);

                if predicate {
                    break;
                }
            }

            predicate
        })
        .count() as i32
}

fn prepare_input(buffer: Vec<u8>) -> Vec<Vec<i32>> {
    let buffer_string = String::from_utf8(buffer).unwrap();

    buffer_string
        .trim()
        .split("\n")
        .map(|item| {
            item.trim()
                .split(" ")
                .map(|e| e.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

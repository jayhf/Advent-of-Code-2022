pub fn solve_part_1(input: &str) -> i32 {
    input
        .split("\n")
        .flat_map(|guideline| match guideline.split_once(" ") {
            Some(("A", "X")) => Some(1 + 3),
            Some(("A", "Y")) => Some(2 + 6),
            Some(("A", "Z")) => Some(3 + 0),
            Some(("B", "X")) => Some(1 + 0),
            Some(("B", "Y")) => Some(2 + 3),
            Some(("B", "Z")) => Some(3 + 6),
            Some(("C", "X")) => Some(1 + 6),
            Some(("C", "Y")) => Some(2 + 0),
            Some(("C", "Z")) => Some(3 + 3),
            _ => None,
        })
        .sum::<i32>()
}

pub fn solve_part_2(input: &str) -> i32 {
    input
        .split("\n")
        .flat_map(|guideline| match guideline.split_once(" ") {
            Some(("A", "X")) => Some(3 + 0),
            Some(("A", "Y")) => Some(1 + 3),
            Some(("A", "Z")) => Some(2 + 6),
            Some(("B", "X")) => Some(1 + 0),
            Some(("B", "Y")) => Some(2 + 3),
            Some(("B", "Z")) => Some(3 + 6),
            Some(("C", "X")) => Some(2 + 0),
            Some(("C", "Y")) => Some(3 + 3),
            Some(("C", "Z")) => Some(1 + 6),
            _ => None,
        })
        .sum::<i32>()
}

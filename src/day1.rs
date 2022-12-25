pub fn solve_part_1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elflines| {
            elflines
                .split("\n")
                .map(|line| line.parse::<i32>())
                .flatten()
                .sum()
        })
        .max()
        .unwrap()
}

pub fn solve_part_2(input: &str) -> i32 {
    let mut elves: Vec<i32> = input
        .split("\n\n")
        .map(|elflines| {
            elflines
                .split("\n")
                .map(|line| line.parse::<i32>())
                .flatten()
                .sum()
        })
        .collect();
    elves.sort();
    elves[elves.len() - 3..].iter().sum()
}

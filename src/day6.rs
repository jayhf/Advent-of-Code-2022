use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> Option<usize> {
    solve(input, 4)
}

pub fn solve_part_2(input: &str) -> Option<usize> {
    solve(input, 14)
}

fn solve(input: &str, n: usize) -> Option<usize> {
    for i in 0..input.len() - n {
        let unique: HashSet<char> = input[i..i + n].chars().collect();
        if unique.len() == n {
            return Some(i + n);
        }
    }
    return None;
}

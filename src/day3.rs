use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(c1, c2)| {
            let c1chars: HashSet<char> = c1.chars().collect();
            let overlap: Vec<char> = c2.chars().filter(|c| c1chars.contains(c)).collect();
            // assert!(overlap.len() == 1);
            let c = overlap[0];
            match c {
                'a'..='z' => c as i32 - 'a' as i32 + 1,
                'A'..='Z' => c as i32 - 'A' as i32 + 27,
                _ => panic!("Unexpected character"),
            }
        })
        .sum::<i32>()
}

pub fn solve_part_2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .chunks(3)
        .map(|chunk| {
            if let &[a, b, c] = chunk {
                let aset: HashSet<char> = a.chars().collect();
                let bset: HashSet<char> = b.chars().collect();
                let overlap: Vec<char> = c
                    .chars()
                    .filter(|c| aset.contains(c) && bset.contains(c))
                    .collect();
                let c = overlap[0];
                match c {
                    'a'..='z' => c as i32 - 'a' as i32 + 1,
                    'A'..='Z' => c as i32 - 'A' as i32 + 27,
                    _ => panic!("Unexpected character"),
                }
            } else {
                panic!("Not a multiple of 3")
            }
        })
        .sum::<i32>()
}

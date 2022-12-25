pub fn solve_part_1(input: &str) -> String {
    let sum = input.lines().map(|line| snafu_to_dec(line)).sum::<i64>();
    dec_to_snafu(sum)
}

fn snafu_to_dec(input: &str) -> i64 {
    let mut result = 0;
    let mut fives = 1;
    for c in input.chars().rev() {
        result += fives
            * match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("Unexpected char"),
            };
        fives *= 5;
    }
    result
}

fn dec_to_snafu(x: i64) -> String {
    let mut result = String::new();
    let mut remainder = x;
    if remainder == 0 {
        result.push('0');
    }
    while remainder != 0 {
        let (c, removed) = match remainder % 5 {
            2 => ('2', 2),
            1 => ('1', 1),
            0 => ('0', 0),
            4 => ('-', -1),
            3 => ('=', -2),
            _ => panic!("Logic error"),
        };
        remainder -= removed;
        assert_eq!(remainder % 5, 0);
        remainder /= 5;
        result.push(c);
    }
    result.chars().rev().collect()
}

pub fn solve_part_2(_: &str) -> usize {
    panic!("No part two. Advent of code is over for 2022!")
}

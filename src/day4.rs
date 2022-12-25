pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (e1, e2) = line.split_once(",").unwrap();
            let (a1, a2) = e1
                .split_once("-")
                .map(|(x1, x2)| (x1.parse::<i32>().unwrap(), x2.parse::<i32>().unwrap()))
                .unwrap();
            let (b1, b2) = e2
                .split_once("-")
                .map(|(x1, x2)| (x1.parse::<i32>().unwrap(), x2.parse::<i32>().unwrap()))
                .unwrap();
            (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2)
        })
        .count()
}

pub fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (e1, e2) = line.split_once(",").unwrap();
            let (a1, a2) = e1
                .split_once("-")
                .map(|(x1, x2)| (x1.parse::<i32>().unwrap(), x2.parse::<i32>().unwrap()))
                .unwrap();
            let (b1, b2) = e2
                .split_once("-")
                .map(|(x1, x2)| (x1.parse::<i32>().unwrap(), x2.parse::<i32>().unwrap()))
                .unwrap();
            (a1 >= b1 && a1 <= b2)
                || (a2 >= b1 && a2 <= b2)
                || (b1 >= a1 && b1 <= a2)
                || (b2 >= a1 && b2 <= a2)
        })
        .count()
}

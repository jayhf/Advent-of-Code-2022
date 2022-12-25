use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Eq, PartialOrd, PartialEq)]
enum Direction {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

pub fn solve_part_1(input: &str) -> i32 {
    let mut elves = parse_elves(input);

    let mut directions: VecDeque<Direction> = VecDeque::from_iter([
        Direction::NORTH,
        Direction::SOUTH,
        Direction::WEST,
        Direction::EAST,
    ]);
    for _ in 0..10 {
        elves = move_elves(&elves, &directions);
        let front = directions.pop_front().unwrap();
        directions.push_back(front);
    }

    let min_r = *elves.iter().map(|(r, _)| r).min().unwrap();
    let max_r = *elves.iter().map(|(r, _)| r).max().unwrap();
    let min_c = *elves.iter().map(|(_, c)| c).min().unwrap();
    let max_c = *elves.iter().map(|(_, c)| c).max().unwrap();
    (max_r - min_r + 1) * (max_c - min_c + 1) - elves.len() as i32
}

#[allow(dead_code)]
fn print_elves(elves: &HashSet<(i32, i32)>) {
    let min_r = *elves.iter().map(|(r, _)| r).min().unwrap();
    let max_r = *elves.iter().map(|(r, _)| r).max().unwrap();
    let min_c = *elves.iter().map(|(_, c)| c).min().unwrap();
    let max_c = *elves.iter().map(|(_, c)| c).max().unwrap();
    println!("{:?}", elves);
    for r in min_r..=max_r {
        for c in min_c..=max_c {
            print!("{}", if elves.contains(&(r, c)) { '#' } else { '.' });
        }
        println!();
    }
}

fn parse_elves(input: &str) -> HashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(c, _)| (r as i32, c as i32))
        })
        .collect()
}

fn move_elves(
    elves: &HashSet<(i32, i32)>,
    directions: &VecDeque<Direction>,
) -> HashSet<(i32, i32)> {
    let mut banned_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut new_positions: HashSet<(i32, i32)> = HashSet::new();
    for elf in elves.iter().map(|elf| move_elf(elves, *elf, directions)) {
        if !new_positions.insert(elf) {
            banned_positions.insert(elf);
        }
    }
    elves
        .iter()
        .map(|elf| {
            let new_position = move_elf(elves, *elf, directions);
            if banned_positions.contains(&new_position) {
                *elf
            } else {
                new_position
            }
        })
        .collect()
}

fn move_elf(
    elves: &HashSet<(i32, i32)>,
    (r, c): (i32, i32),
    directions: &VecDeque<Direction>,
) -> (i32, i32) {
    if (r - 1..=r + 1)
        .all(|re| (c - 1..=c + 1).all(|ce| (re == r && ce == c) || !elves.contains(&(re, ce))))
    {
        return (r, c);
    }

    for dir in directions {
        let to_check = match dir {
            Direction::NORTH => [(r - 1, c - 1), (r - 1, c), (r - 1, c + 1)],
            Direction::SOUTH => [(r + 1, c - 1), (r + 1, c), (r + 1, c + 1)],
            Direction::WEST => [(r + 1, c - 1), (r, c - 1), (r - 1, c - 1)],
            Direction::EAST => [(r + 1, c + 1), (r, c + 1), (r - 1, c + 1)],
        };
        if to_check.iter().all(|p| !elves.contains(p)) {
            return match dir {
                Direction::NORTH => (r - 1, c),
                Direction::SOUTH => (r + 1, c),
                Direction::WEST => (r, c - 1),
                Direction::EAST => (r, c + 1),
            };
        }
    }
    (r, c)
}

pub fn solve_part_2(input: &str) -> usize {
    let mut elves = parse_elves(input);

    let mut directions: VecDeque<Direction> = VecDeque::from_iter([
        Direction::NORTH,
        Direction::SOUTH,
        Direction::WEST,
        Direction::EAST,
    ]);
    for i in 1.. {
        let new_elves = move_elves(&elves, &directions);
        let front = directions.pop_front().unwrap();
        directions.push_back(front);
        if elves == new_elves {
            return i;
        }
        elves = new_elves;
    }
    panic!("Unreachable")
}

use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> usize {
    let mut tail = (0, 0);
    let mut head = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for line in input.lines() {
        let (direction, distance) = line.split_once(" ").unwrap();
        let dist = distance.parse::<i32>().unwrap();
        let offset = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("Unknown"),
        };
        for _ in 0..dist {
            head.0 += offset.0;
            head.1 += offset.1;
            move_knot(&head, &mut tail);
            visited.insert(tail);
        }
    }

    visited.len()
}

fn move_knot(head: &(i32, i32), tail: &mut (i32, i32)) {
    let tail_offset = match (head.0 - tail.0 as i32, head.1 - tail.1 as i32) {
        (2, 0) => (1, 0),
        (-2, 0) => (-1, 0),
        (0, 2) => (0, 1),
        (0, -2) => (0, -1),
        (2, 1) => (1, 1),
        (-2, 1) => (-1, 1),
        (1, 2) => (1, 1),
        (1, -2) => (1, -1),
        (2, -1) => (1, -1),
        (-2, -1) => (-1, -1),
        (-1, 2) => (-1, 1),
        (-1, -2) => (-1, -1),
        (-2, -2) => (-1, -1),
        (-2, 2) => (-1, 1),
        (2, 2) => (1, 1),
        (2, -2) => (1, -1),
        (h, t) => {
            if h.abs() <= 1 && t.abs() <= 1 {
                (0, 0)
            } else {
                panic!("AAA")
            }
        }
    };

    tail.0 += tail_offset.0;
    tail.1 += tail_offset.1;
}

pub fn solve_part_2(input: &str) -> usize {
    let mut knot = Vec::new();
    for _ in 0..10 {
        knot.push((0, 0));
    }
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for line in input.lines() {
        let (direction, distance) = line.split_once(" ").unwrap();
        let dist = distance.parse::<i32>().unwrap();
        let offset = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("Unknown"),
        };
        for _ in 0..dist {
            knot[0].0 += offset.0;
            knot[0].1 += offset.1;
            for i in 0..knot.len() - 1 {
                let (before, after) = knot.split_at_mut(i + 1);
                move_knot(&before[i], &mut after[0]);
            }
            visited.insert(knot[knot.len() - 1]);
        }
    }

    visited.len()
}

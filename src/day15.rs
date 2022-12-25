use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> usize {
    const THE_ROW: i32 = 2000000;
    let bs = parse_locations(input);
    let mut possible: HashSet<(i32, i32)> = HashSet::new();
    for ((sx, sy), (bx, by)) in bs.iter() {
        let manhattan = (bx - sx).abs() + (by - sy).abs();
        let dx = manhattan - (sy - THE_ROW).abs();
        if dx >= 0 {
            for x in (sx - dx)..=(sx + dx) {
                possible.insert((x, THE_ROW));
            }
        }
    }
    for (_, b) in bs {
        possible.remove(&b);
    }
    possible.len()
}

pub fn solve_part_2(input: &str) -> i64 {
    const MAX_COORD: i32 = 4000000;
    let bs = parse_locations(input);
    let mut possible: HashSet<(i32, i32)> = HashSet::new();
    for ((sx, sy), (bx, by)) in bs.iter() {
        let target_dist = (bx - sx).abs() + (by - sy).abs() + 1;

        for cx in std::cmp::max(0, sx - target_dist)..=std::cmp::min(sx + target_dist, MAX_COORD) {
            for cy in [
                sy + (target_dist - (cx - sx).abs()),
                sy - (target_dist - (cx - sx).abs()),
            ] {
                if cy >= 0 && cy <= MAX_COORD {
                    if bs.iter().all(|((sx, sy), (bx, by))| {
                        let manhattan = (bx - sx).abs() + (by - sy).abs();
                        let cmanhat = (cx - sx).abs() + (cy - sy).abs();
                        cmanhat > manhattan
                    }) {
                        possible.insert((cx, cy));
                    };
                }
            }
        }
    }
    assert_eq!(possible.len(), 1);
    possible
        .iter()
        .map(|(x, y)| *x as i64 * 4000000 + *y as i64)
        .next()
        .unwrap()
}

fn parse_locations(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    input
        .lines()
        .map(|line| {
            let (sense, beacon) = line
                .split_once("Sensor at x=")
                .unwrap()
                .1
                .split_once(": closest beacon is at x=")
                .unwrap();
            let (sx, sy) = sense.split_once(", y=").unwrap();
            let (bx, by) = beacon.split_once(", y=").unwrap();
            (
                (sx.parse::<i32>().unwrap(), sy.parse::<i32>().unwrap()),
                (bx.parse::<i32>().unwrap(), by.parse::<i32>().unwrap()),
            )
        })
        .collect()
}

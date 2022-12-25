use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> usize {
    let map = parse_map(input);
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    let width = map[0].len();
    let height = map.len();

    for r in 0..map.len() {
        let row = &map[r];
        let mut highest = -1;
        for c in 0..row.len() {
            if row[c] > highest {
                visible.insert((r, c));
                highest = row[c];
            }
        }
        highest = -1;
        for c in (0..row.len()).rev() {
            if row[c] > highest {
                visible.insert((r, c));
                highest = row[c];
            }
        }
    }

    for c in 0..width {
        let mut highest = -1;
        for r in 0..height {
            if map[r][c] > highest {
                visible.insert((r, c));
                highest = map[r][c];
            }
        }
        highest = -1;
        for r in (0..height).rev() {
            if map[r][c] > highest {
                visible.insert((r, c));
                highest = map[r][c];
            }
        }
    }

    visible.len()
}

pub fn solve_part_2(input: &str) -> usize {
    let map = parse_map(input);

    let width = map[0].len();
    let height = map.len();

    (0..height)
        .map(|r| (0..width).map(|c| scenic_score(&map, r, c)).max().unwrap())
        .max()
        .unwrap()
}

fn parse_map(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn scenic_score(map: &Vec<Vec<i32>>, tr: usize, tc: usize) -> usize {
    let tree = map[tr][tc];
    let width = map[0].len();
    let height = map.len();

    let east = ((tr + 1)..height)
        .find(|r| map[*r][tc] >= tree)
        .unwrap_or(height - 1)
        - tr;
    let west = tr
        - (0..tr.saturating_sub(1))
            .rev()
            .find(|r| map[*r][tc] >= tree)
            .unwrap_or(0);
    let north = ((tc + 1)..width)
        .find(|c| map[tr][*c] >= tree)
        .unwrap_or(width - 1)
        - tc;
    let south = tc
        - (0..tc.saturating_sub(1))
            .rev()
            .find(|c| map[tr][*c] >= tree)
            .unwrap_or(0);
    east * west * north * south
}

use std::collections::{BTreeSet, HashSet};

pub fn solve_part_1(input: &str) -> usize {
    let points = parse_points(input);

    surface_area(points)
}

fn surface_area(points: HashSet<[i32; 3]>) -> usize {
    let mut surface_area = points.len() * 6;
    for p in points.iter() {
        for a in adjacent_to(p) {
            if points.contains(&a) {
                surface_area -= 1;
            }
        }
    }
    surface_area
}

fn adjacent_to(p: &[i32; 3]) -> [[i32; 3]; 6] {
    [
        [p[0] - 1, p[1], p[2]],
        [p[0] + 1, p[1], p[2]],
        [p[0], p[1] - 1, p[2]],
        [p[0], p[1] + 1, p[2]],
        [p[0], p[1], p[2] - 1],
        [p[0], p[1], p[2] + 1],
    ]
}

fn parse_points(input: &str) -> HashSet<[i32; 3]> {
    let points: HashSet<[i32; 3]> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| [v[0], v[1], v[2]])
        .collect();
    points
}

pub fn solve_part_2(input: &str) -> usize {
    let points = parse_points(input);

    let max_x = points.iter().map(|p| p[0]).max().unwrap();
    let max_y = points.iter().map(|p| p[1]).max().unwrap();
    let max_z = points.iter().map(|p| p[2]).max().unwrap();

    let mut checked: HashSet<[i32; 3]> = HashSet::new();
    let mut queue: BTreeSet<[i32; 3]> = BTreeSet::new();
    queue.insert([0, 0, 0]);

    loop {
        let next = queue.pop_first();
        match next {
            Some(p) => {
                checked.insert(p);
                for a in adjacent_to(&p) {
                    if !checked.contains(&a)
                        && !points.contains(&a)
                        && a[0] >= 0
                        && a[1] >= 0
                        && a[2] >= 0
                        && a[0] <= max_x
                        && a[1] <= max_y
                        && a[2] <= max_z
                    {
                        queue.insert(a.clone());
                    }
                }
            }
            None => break,
        }
    }

    let interior: HashSet<[i32; 3]> = (0..max_x)
        .flat_map(move |x| (0..max_y).flat_map(move |y| (0..max_z).map(move |z| [x, y, z])))
        .filter(|p| !checked.contains(p) && !points.contains(p))
        .collect();
    surface_area(points) - surface_area(interior)
}

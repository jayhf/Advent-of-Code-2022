use std::collections::{BTreeSet, HashMap};

// WORKED FIRST TRY!!!
pub fn solve_part_1(input: &str) -> usize {
    let sand_origin = (500, 0);
    let mut world = parse_world(input);

    for i in 0.. {
        if let Some((x, y)) = add_sand(&world, sand_origin) {
            world.get_mut(&x).unwrap().insert(y);
        } else {
            return i;
        }
    }

    panic!("Unreachable")
}

fn parse_world(input: &str) -> HashMap<i32, BTreeSet<i32>> {
    let mut world: HashMap<i32, BTreeSet<i32>> = HashMap::new();
    for parts in input.lines().map(|line| {
        line.split(" -> ")
            .map(|pair| {
                let (p1, p2) = pair.split_once(",").unwrap();
                (p1.parse::<i32>().unwrap(), p2.parse::<i32>().unwrap())
            })
            .collect::<Vec<(i32, i32)>>()
    }) {
        for ((x1, y1), (x2, y2)) in parts.windows(2).map(|x| (x[0], x[1])) {
            if x1 == x2 {
                for y in std::cmp::min(y1, y2)..=std::cmp::max(y1, y2) {
                    world.entry(x1).or_insert(BTreeSet::new()).insert(y);
                }
            } else if y1 == y2 {
                for x in std::cmp::min(x1, x2)..=std::cmp::max(x1, x2) {
                    world.entry(x).or_insert(BTreeSet::new()).insert(y1);
                }
            }
        }
    }
    world
}

fn add_sand(world: &HashMap<i32, BTreeSet<i32>>, (x, y): (i32, i32)) -> Option<(i32, i32)> {
    if let Some(bottom_y) = world
        .get(&x)
        .and_then(|t| t.range(y + 1..).min().map(|x| x - 1))
    {
        match (
            world
                .get(&(x - 1))
                .and_then(|t| Some(t.contains(&(bottom_y + 1)))),
            world
                .get(&(x + 1))
                .and_then(|t| Some(t.contains(&(bottom_y + 1)))),
        ) {
            (Some(true), Some(true)) => Some((x, bottom_y)),
            (Some(true), _) => add_sand(world, (x + 1, bottom_y)),
            _ => add_sand(world, (x - 1, bottom_y)),
        }
    } else {
        None
    }
}

pub fn solve_part_2(input: &str) -> usize {
    let sand_origin = (500, 0);
    let mut world = parse_world(input);
    let max_y = world.values().flat_map(|t| t).max().unwrap() + 2;
    for x in sand_origin.0 - max_y - 1..sand_origin.0 + max_y + 1 {
        world.entry(x).or_insert(BTreeSet::new()).insert(max_y);
    }

    for i in 1.. {
        if let Some((x, y)) = add_sand(&world, sand_origin) {
            world.get_mut(&x).unwrap().insert(y);
            if (x, y) == sand_origin {
                return i;
            }
        } else {
            panic!("Broken floor")
        }
    }

    panic!("Unreachable")
}

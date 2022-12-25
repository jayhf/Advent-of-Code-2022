#[derive(Clone, Copy, Debug, Eq, PartialOrd, PartialEq)]
enum Direction {
    DOWN = 1,
    RIGHT = 0,
    LEFT = 2,
    UP = 3,
}

pub fn solve_part_1(input: &str) -> usize {
    let (map, directions, remaining) = parse_input(input);
    // println!("{:?}", directions);

    let mut row: usize = 0;
    let mut col: usize = map[0].iter().position(|o| o.is_some()).unwrap();
    let mut direction: Direction = Direction::RIGHT;

    for (dist, left) in directions {
        walk(&map, &mut row, &mut col, direction, dist);
        direction = rotate(direction, left);
        // println!("{row}, {col}, {:?}", direction);
    }

    walk(&map, &mut row, &mut col, direction, remaining);
    // println!("{row}, {col}, {:?}", direction);

    1000 * (row + 1) + 4 * (col + 1) + (direction as usize)
}

fn rotate(direction: Direction, left: bool) -> Direction {
    if left {
        match direction {
            Direction::DOWN => Direction::RIGHT,
            Direction::RIGHT => Direction::UP,
            Direction::LEFT => Direction::DOWN,
            Direction::UP => Direction::LEFT,
        }
    } else {
        match direction {
            Direction::DOWN => Direction::LEFT,
            Direction::RIGHT => Direction::DOWN,
            Direction::LEFT => Direction::UP,
            Direction::UP => Direction::RIGHT,
        }
    }
}

fn reverse(direction: Direction) -> Direction {
    match direction {
        Direction::DOWN => Direction::UP,
        Direction::RIGHT => Direction::LEFT,
        Direction::LEFT => Direction::RIGHT,
        Direction::UP => Direction::DOWN,
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Option<bool>>>, Vec<(usize, bool)>, usize) {
    let (map_str, direction_str) = input.split_once("\n\n").unwrap();
    let map: Vec<Vec<Option<bool>>> = map_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Some(false),
                    '#' => Some(true),
                    ' ' => None,
                    _ => panic!("Unknown char {c}"),
                })
                .collect()
        })
        .collect();

    let mut directions: Vec<(usize, bool)> = Vec::new();
    let mut parsed: usize = 0;
    for c in direction_str.chars() {
        match c.to_digit(10) {
            Some(d) => parsed = parsed * 10 + d as usize,
            None => match c {
                'L' => {
                    directions.push((parsed, true));
                    parsed = 0;
                }
                'R' => {
                    directions.push((parsed, false));
                    parsed = 0;
                }
                _ => (),
            },
        }
    }
    (map, directions, parsed)
}

fn walk(
    map: &Vec<Vec<Option<bool>>>,
    row: &mut usize,
    col: &mut usize,
    direction: Direction,
    dist: usize,
) {
    for _ in 0..dist {
        let mut next = (*row, *col);
        loop {
            match direction {
                Direction::DOWN => {
                    if next.0 == map.len() - 1 {
                        next.0 = 0;
                    } else {
                        next.0 += 1;
                    }
                }
                Direction::RIGHT => {
                    if next.1 == map[next.0 as usize].len() - 1 {
                        next.1 = 0;
                    } else {
                        next.1 += 1;
                    }
                }
                Direction::LEFT => {
                    if next.1 == 0 {
                        next.1 = map[next.0 as usize].len() - 1;
                    } else {
                        next.1 -= 1;
                    }
                }
                Direction::UP => {
                    if next.0 == 0 {
                        next.0 = map.len() - 1;
                    } else {
                        next.0 -= 1;
                    }
                }
            };
            match map[next.0].get(next.1) {
                Some(Some(wall)) => {
                    if *wall {
                        return;
                    } else {
                        break;
                    }
                }
                _ => (),
            }
        }
        (*row, *col) = next;
    }
}

fn walk_cube(
    map: &Vec<Vec<Option<bool>>>,
    mut row: usize,
    mut col: usize,
    mut direction: Direction,
    dist: usize,
) -> (usize, usize, Direction) {
    for _ in 0..dist {
        let (next_row, next_col, next_dir) = step_cube(row, col, direction);
        let wall = map[next_row][next_col].unwrap();
        if wall {
            break;
        }
        (row, col, direction) = (next_row, next_col, next_dir);
    }
    (row, col, direction)
}

fn step_cube(row: usize, col: usize, direction: Direction) -> (usize, usize, Direction) {
    // Mostly hardcoding based on my cube because I don't have time for a general solution
    const SIDE: usize = 50;
    match direction {
        Direction::DOWN => {
            if row == 4 * SIDE - 1 {
                (0, 2 * SIDE + col, Direction::DOWN)
            } else if row == 3 * SIDE - 1 && col >= SIDE {
                (col + 2 * SIDE, SIDE - 1, Direction::LEFT)
            } else if row == SIDE - 1 && col >= 2 * SIDE {
                (col - SIDE, SIDE * 2 - 1, Direction::LEFT)
            } else {
                (row + 1, col, Direction::DOWN)
            }
        }
        Direction::RIGHT => {
            if col == 3 * SIDE - 1 {
                (3 * SIDE - 1 - row, 2 * SIDE - 1, Direction::LEFT)
            } else if col == 2 * SIDE - 1 && row >= SIDE {
                if row < 2 * SIDE {
                    (SIDE - 1, row + SIDE, Direction::UP)
                } else {
                    (3 * SIDE - 1 - row, 3 * SIDE - 1, Direction::LEFT)
                }
            } else if col == SIDE - 1 && row >= 3 * SIDE {
                (3 * SIDE - 1, row - 2 * SIDE, Direction::UP)
            } else {
                (row, col + 1, Direction::RIGHT)
            }
        }
        Direction::LEFT => {
            if col == 0 {
                if row < 3 * SIDE {
                    (3 * SIDE - 1 - row, SIDE, Direction::RIGHT)
                } else {
                    (0, row - 2 * SIDE, Direction::DOWN)
                }
            } else if col == SIDE && row < 2 * SIDE {
                if row < SIDE {
                    (3 * SIDE - 1 - row, 0, Direction::RIGHT)
                } else {
                    (2 * SIDE, row - SIDE, Direction::DOWN)
                }
            } else {
                (row, col - 1, Direction::LEFT)
            }
        }
        Direction::UP => {
            if row == 0 {
                if col < 2 * SIDE {
                    (col + 2 * SIDE, 0, Direction::RIGHT)
                } else {
                    (4 * SIDE - 1, col - 2 * SIDE, Direction::UP)
                }
            } else if row == 2 * SIDE && col < SIDE {
                (SIDE + col, SIDE, Direction::RIGHT)
            } else {
                (row - 1, col, Direction::UP)
            }
        }
    }
}

pub fn solve_part_2(input: &str) -> usize {
    let (map, directions, remaining) = parse_input(input);

    for (r, row) in map.iter().enumerate() {
        for (c, o) in row.iter().enumerate() {
            match o {
                None => {}
                Some(_) => {
                    for d in [
                        Direction::RIGHT,
                        Direction::LEFT,
                        Direction::UP,
                        Direction::DOWN,
                    ] {
                        let (mut new_r, mut new_c, mut new_dir) = step_cube(r, c, d);
                        let (sr, sc, sd) = step_cube(new_r, new_c, reverse(new_dir));
                        assert_eq!(
                            (r, c, d),
                            (sr, sc, reverse(sd)),
                            "Intermediate was {:?}",
                            (new_r, new_c, new_dir)
                        );
                        (new_r, new_c, new_dir) = (r, c, d);
                        for _ in 0..200 {
                            // println!("{:?}", (new_r, new_c, new_dir));
                            (new_r, new_c, new_dir) = step_cube(new_r, new_c, new_dir);
                        }
                        assert_eq!((r, c, d), (new_r, new_c, new_dir));
                    }
                }
            }
        }
    }

    // println!("{:?}", directions);

    let mut row: usize = 0;
    let mut col: usize = map[0].iter().position(|o| o.is_some()).unwrap();
    let mut direction: Direction = Direction::RIGHT;

    for (dist, left) in directions {
        let new_pose = walk_cube(&map, row, col, direction, dist);
        (row, col, direction) = new_pose;
        direction = rotate(direction, left);
        // println!("{row}, {col}, {:?}", direction);
    }

    (row, col, direction) = walk_cube(&map, row, col, direction, remaining);
    // println!("{row}, {col}, {:?}", direction);

    1000 * (row + 1) + 4 * (col + 1) + (direction as usize)
}
//too high 45481

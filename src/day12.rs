use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> i32 {
    let map = parse_map(input);
    let mut reached: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: Vec<(i32, i32)> = Vec::new();
    let mut next_queue: Vec<(i32, i32)> = Vec::new();
    queue.push(find(&map, 'S'));
    reached.insert(find(&map, 'S'));
    let end = find(&map, 'E');

    let width = map[0].len();
    let height = map.len();

    for i in 1.. {
        for p in queue.iter() {
            let map_height = map[p.0 as usize][p.1 as usize];
            let current_height = if map_height == 'S' { 'a' } else { map_height };
            for (r, c) in [(0, 1), (1, 0), (-1, 0), (0, -1)]
                .into_iter()
                .filter(|(r, c)| {
                    0 <= p.0 + *r
                        && p.0 + *r < height as i32
                        && 0 <= p.1 + *c
                        && p.1 + *c < width as i32
                })
            {
                let next_map_height = map[(p.0 + r) as usize][(p.1 + c) as usize];
                let next_height = if next_map_height == 'E' {
                    'z'
                } else {
                    next_map_height
                };
                if 1 + current_height as i32 >= next_height as i32 {
                    let new_point = (p.0 + r, p.1 + c);
                    if reached.insert(new_point) {
                        next_queue.push(new_point);
                        if new_point == end {
                            return i;
                        }
                    }
                }
            }
        }
        queue.clear();
        std::mem::swap(&mut next_queue, &mut queue);
        // for c in queue.iter() {
        //     print!("{}", map[c.0 as usize][c.1 as usize]);
        // }
        // println!();
        if queue.len() == 0 {
            panic!("No more elements");
        }
    }
    panic!("Impossible");
}

pub fn solve_part_2(input: &str) -> i32 {
    let map = parse_map(input);
    let mut reached: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: Vec<(i32, i32)> = Vec::new();
    let mut next_queue: Vec<(i32, i32)> = Vec::new();
    queue.push(find(&map, 'E'));
    reached.insert(find(&map, 'E'));

    let width = map[0].len();
    let height = map.len();

    for i in 1.. {
        for p in queue.iter() {
            let map_height = map[p.0 as usize][p.1 as usize];
            let current_height = if map_height == 'S' {
                'a'
            } else if map_height == 'E' {
                'z'
            } else {
                map_height
            };
            for (r, c) in [(0, 1), (1, 0), (-1, 0), (0, -1)]
                .into_iter()
                .filter(|(r, c)| {
                    0 <= p.0 + *r
                        && p.0 + *r < height as i32
                        && 0 <= p.1 + *c
                        && p.1 + *c < width as i32
                })
            {
                let next_map_height = map[(p.0 + r) as usize][(p.1 + c) as usize];
                let next_height = if next_map_height == 'E' {
                    'z'
                } else {
                    next_map_height
                };
                if 1 + next_height as i32 >= current_height as i32 {
                    let new_point = (p.0 + r, p.1 + c);
                    if reached.insert(new_point) {
                        next_queue.push(new_point);
                        if map[new_point.0 as usize][new_point.1 as usize] == 'a' {
                            return i;
                        }
                    }
                }
            }
        }
        queue.clear();
        std::mem::swap(&mut next_queue, &mut queue);
        if queue.len() == 0 {
            panic!("No more elements");
        }
    }
    panic!("Impossible");
}

fn find(map: &Vec<Vec<char>>, ch: char) -> (i32, i32) {
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == ch {
                return (r as i32, c as i32);
            }
        }
    }

    panic!("Not found");
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

use std::collections::{HashSet, VecDeque};

pub fn solve_part_1(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (start, end) = find_start_end(&map);
    let mut blizzard_forecast: Vec<HashSet<(i32, i32)>> = Vec::new();
    shortest_path_end_time(&map, start, end, 0, &mut blizzard_forecast)
}

pub fn solve_part_2(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (start, end) = find_start_end(&map);
    let mut blizzard_forecast: Vec<HashSet<(i32, i32)>> = Vec::new();
    let initial_goal = shortest_path_end_time(&map, start, end, 0, &mut blizzard_forecast);
    let back_to_start =
        shortest_path_end_time(&map, end, start, initial_goal, &mut blizzard_forecast);
    shortest_path_end_time(&map, start, end, back_to_start, &mut blizzard_forecast)
}

fn shortest_path_end_time(
    map: &Vec<Vec<char>>,
    start: (i32, i32),
    end: (i32, i32),
    start_time: usize,
    blizzard_forecast: &mut Vec<HashSet<(i32, i32)>>,
) -> usize {
    let (r_range, c_range) = (0..map.len() as i32, 0..map[0].len() as i32);
    let mut visited: HashSet<((i32, i32), usize)> = HashSet::new();
    let mut queue: VecDeque<((i32, i32), usize)> = VecDeque::new();
    queue.push_back((start, start_time));
    loop {
        let point = queue.pop_front().expect("Path not found");
        let ((r, c), t) = point;
        while blizzard_forecast.len() <= t + 1 {
            blizzard_forecast.push(blizzard_locations(map, blizzard_forecast.len()));
        }
        // println!("Time {}: {:?}", t, (r, c));
        let forecast = &blizzard_forecast[t + 1];
        for (ro, co) in [(0i32, 0i32), (0, 1), (1, 0), (-1, 0), (0, -1)] {
            let p = (r + ro, c + co);
            if !r_range.contains(&p.0)
                || !c_range.contains(&p.1)
                || map[p.0 as usize][p.1 as usize] == '#'
            {
                continue;
            }
            if forecast.contains(&p) {
                continue;
            }
            if p == end {
                return t + 1;
            }
            if !visited.insert((p, t + 1)) {
                continue;
            }
            queue.push_back((p, t + 1));
        }
    }
}

fn find_start_end(map: &Vec<Vec<char>>) -> ((i32, i32), (i32, i32)) {
    (
        (0, map[0].iter().position(|c| *c == '.').unwrap() as i32),
        (
            map.len() as i32 - 1,
            map[map.len() - 1].iter().position(|c| *c == '.').unwrap() as i32,
        ),
    )
}

fn blizzard_locations(map: &Vec<Vec<char>>, time: usize) -> HashSet<(i32, i32)> {
    let mut result: HashSet<(i32, i32)> = HashSet::new();
    for (r, row) in map.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            match ch {
                // rem_euclid is %, except the result is in [0, n)
                '>' => result.insert((
                    r as i32,
                    1 + ((c - 1 + time).rem_euclid(row.len() - 2)) as i32,
                )),
                '<' => result.insert((
                    r as i32,
                    1 + ((c as i64 - 1 - time as i64).rem_euclid(row.len() as i64 - 2)) as i32,
                )),
                '^' => result.insert((
                    1 + ((r as i64 - 1 - time as i64).rem_euclid(map.len() as i64 - 2)) as i32,
                    c as i32,
                )),
                'v' => result.insert((
                    1 + ((r - 1 + time).rem_euclid(map.len() - 2)) as i32,
                    c as i32,
                )),
                _ => false,
            };
        }
    }
    result
}

#[allow(dead_code)]
fn print_forecast(map: &Vec<Vec<char>>, time: usize, forecast: &HashSet<(i32, i32)>) {
    println!("Forecast for t={}", time);
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == '#' {
                print!("#");
            } else if forecast.contains(&(r as i32, c as i32)) {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

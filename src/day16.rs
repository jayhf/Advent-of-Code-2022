use std::collections::HashMap;

struct Tunnel {
    rate: usize, //pressure per minute
    dests: Vec<u32>,
}

fn max_pressure_released(
    current: u32,
    tunnels: &HashMap<u32, Tunnel>,
    opened: usize,
    time_remaining: usize,
    cache: &mut HashMap<(u32, usize, usize), usize>,
) -> usize {
    let maybe_hit = cache.get(&(current, opened, time_remaining));
    if maybe_hit.is_some() {
        return *maybe_hit.unwrap();
    }

    if time_remaining == 0 {
        return 0;
    }
    let tunnel = &tunnels[&current];
    let result = if tunnel.rate > 0 && (opened & (1 << current)) == 0 {
        (time_remaining - 1) * tunnel.rate
            + max_pressure_released(
                current,
                tunnels,
                opened | (1 << current),
                time_remaining - 1,
                cache,
            )
    } else {
        tunnel
            .dests
            .iter()
            .map(|tunnel| {
                max_pressure_released(*tunnel, tunnels, opened, time_remaining - 1, cache)
            })
            .max()
            .unwrap_or(0)
    };
    cache.insert((current, opened, time_remaining), result);
    result
}

pub fn solve_part_1(input: &str) -> usize {
    const TIME: usize = 30;
    let start: u32 = 0;
    let mut tunnel_ids: HashMap<&str, u32> = HashMap::new();
    tunnel_ids.insert("AA", 0);
    let tunnels = parse(input, &mut tunnel_ids);
    let mut cache: HashMap<(u32, usize, usize), usize> = HashMap::new();
    let result = max_pressure_released(start, &tunnels, 0, TIME, &mut cache);
    // println!("{}", cache.len());
    result
}

fn max_pressure_released_elephant(
    me: u32,
    elephant: u32,
    is_elephant: bool,
    tunnels: &HashMap<u32, Tunnel>,
    opened: usize,
    time_remaining: u32,
    cache: &mut HashMap<(u32, u32, usize, u32, bool), usize>,
) -> usize {
    let maybe_hit = cache.get(&(me, elephant, opened, time_remaining, is_elephant));
    if maybe_hit.is_some() {
        return *maybe_hit.unwrap();
    }

    if time_remaining == 0 {
        return 0;
    }
    let current = if is_elephant { elephant } else { me };
    let next_time_remaining = if is_elephant {
        time_remaining - 1
    } else {
        time_remaining
    };
    let tunnel = &tunnels[&current];

    let result: usize = if is_elephant {
        if tunnel.rate > 0 && (opened & (1 << current)) == 0 {
            (time_remaining as usize - 1) * tunnel.rate
                + max_pressure_released_elephant(
                    me,
                    elephant,
                    !is_elephant,
                    tunnels,
                    opened | (1 << current),
                    next_time_remaining,
                    cache,
                )
        } else {
            tunnel
                .dests
                .iter()
                .map(|tunnel| {
                    max_pressure_released_elephant(
                        me,
                        *tunnel,
                        !is_elephant,
                        tunnels,
                        opened,
                        next_time_remaining,
                        cache,
                    )
                })
                .max()
                .unwrap_or(0)
        }
    } else {
        if tunnel.rate > 0 && (opened & (1 << current)) == 0 {
            (time_remaining as usize - 1) * tunnel.rate
                + max_pressure_released_elephant(
                    me,
                    elephant,
                    !is_elephant,
                    tunnels,
                    opened | (1 << current),
                    next_time_remaining,
                    cache,
                )
        } else {
            tunnel
                .dests
                .iter()
                .map(|tunnel| {
                    max_pressure_released_elephant(
                        *tunnel,
                        elephant,
                        !is_elephant,
                        tunnels,
                        opened,
                        next_time_remaining,
                        cache,
                    )
                })
                .max()
                .unwrap_or(0)
        }
    };
    cache.insert((me, elephant, opened, time_remaining, is_elephant), result);
    result
}

pub fn solve_part_2(input: &str) -> usize {
    const TIME: u32 = 26;
    let start: u32 = 0;
    let mut tunnel_ids: HashMap<&str, u32> = HashMap::new();
    tunnel_ids.insert("AA", 0);
    let tunnels = parse(input, &mut tunnel_ids);
    let mut cache: HashMap<(u32, u32, usize, u32, bool), usize> = HashMap::new();
    let result = max_pressure_released_elephant(start, start, false, &tunnels, 0, TIME, &mut cache);
    // println!("{}", cache.len());
    result
}

fn parse<'a>(input: &'a str, tunnel_ids: &mut HashMap<&'a str, u32>) -> HashMap<u32, Tunnel> {
    let mut current_tunnel_id = 1;
    let mut tunnel_id = |name| {
        if tunnel_ids.contains_key(name) {
            tunnel_ids[name]
        } else {
            let id = current_tunnel_id;
            tunnel_ids.insert(name, id);
            current_tunnel_id = current_tunnel_id + 1;
            id
        }
    };
    let tunnels: HashMap<u32, Tunnel> = input
        .lines()
        .map(|line| {
            let (info, lead) = line.split_once("; tunnels lead to valves ").unwrap();
            let (name, rate) = info
                .split_once("Valve ")
                .unwrap()
                .1
                .split_once(" has flow rate=")
                .unwrap();
            (
                tunnel_id(name),
                Tunnel {
                    rate: rate.parse::<usize>().unwrap(),
                    dests: lead.split(", ").map(|t| tunnel_id(t)).collect(),
                },
            )
        })
        .collect();
    tunnels
}

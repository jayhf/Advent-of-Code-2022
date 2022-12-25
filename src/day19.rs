struct Blueprint {
    number: i32,
    ore_cost: i32,
    clay_cost_ore: i32,
    obsidian_cost_ore: i32,
    obsidian_cost_clay: i32,
    geode_cost_ore: i32,
    geode_cost_obsidian: i32,
    ore_threshold: i32,
}

#[derive(Copy, Clone)]
struct MaterialState {
    rate: i32,
    current: i32,
}

pub fn solve_part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| parse_blueprint(line))
        .map(|bp| best_geode_number(&bp, 24) * bp.number)
        .sum()
}

fn parse_blueprint(line: &str) -> Blueprint {
    let blueprint_parts: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    // Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 10 clay. Each geode robot costs 2 ore and 7 obsidian.
    let number = blueprint_parts[0];
    let ore_cost = blueprint_parts[1];
    let clay_cost_ore = blueprint_parts[2];
    let obsidian_cost_ore = blueprint_parts[3];
    let obsidian_cost_clay = blueprint_parts[4];
    let geode_cost_ore = blueprint_parts[5];
    let geode_cost_obsidian = blueprint_parts[6];
    Blueprint {
        number,
        ore_cost,
        clay_cost_ore,
        obsidian_cost_ore,
        obsidian_cost_clay,
        geode_cost_ore,
        geode_cost_obsidian,
        ore_threshold: std::cmp::max(
            ore_cost,
            std::cmp::max(
                clay_cost_ore,
                std::cmp::max(obsidian_cost_ore, geode_cost_ore),
            ),
        ),
    }
}

fn div_ceil(x: i32, y: i32) -> i32 {
    std::cmp::max(0, (x + y - 1) / y)
}

fn best_geode_number(bp: &Blueprint, time_remaining: i32) -> i32 {
    best_geode_number_helper(
        bp,
        time_remaining,
        MaterialState {
            rate: 1,
            current: 0,
        },
        MaterialState {
            rate: 0,
            current: 0,
        },
        MaterialState {
            rate: 0,
            current: 0,
        },
        MaterialState {
            rate: 0,
            current: 0,
        },
        0,
    )
}

fn best_geode_number_helper(
    bp: &Blueprint,
    time_remaining: i32,
    ore: MaterialState,
    clay: MaterialState,
    obsidian: MaterialState,
    geode: MaterialState,
    previous_geode_rate: i32,
) -> i32 {
    if time_remaining <= 0 {
        return geode.current + previous_geode_rate * time_remaining;
    }
    let mut bests: [Option<i32>; 4] = [None; 4];
    if ore.rate < bp.ore_threshold {
        let time_to_ore = div_ceil(bp.ore_cost - ore.current, ore.rate) + 1;
        bests[0] = Some(best_geode_number_helper(
            bp,
            time_remaining - time_to_ore,
            MaterialState {
                current: ore.current + time_to_ore * ore.rate - bp.ore_cost,
                rate: ore.rate + 1,
            },
            MaterialState {
                current: clay.current + time_to_ore * clay.rate,
                rate: clay.rate,
            },
            MaterialState {
                current: obsidian.current + time_to_ore * obsidian.rate,
                rate: obsidian.rate,
            },
            MaterialState {
                current: geode.current + time_to_ore * geode.rate,
                rate: geode.rate,
            },
            geode.rate,
        ));
    }

    if clay.rate < bp.obsidian_cost_clay {
        let time_to_clay = div_ceil(bp.clay_cost_ore - ore.current, ore.rate) + 1;
        bests[1] = Some(best_geode_number_helper(
            bp,
            time_remaining - time_to_clay,
            MaterialState {
                current: ore.current + time_to_clay * ore.rate - bp.clay_cost_ore,
                rate: ore.rate,
            },
            MaterialState {
                current: clay.current + time_to_clay * clay.rate,
                rate: clay.rate + 1,
            },
            MaterialState {
                current: obsidian.current + time_to_clay * obsidian.rate,
                rate: obsidian.rate,
            },
            MaterialState {
                current: geode.current + time_to_clay * geode.rate,
                rate: geode.rate,
            },
            geode.rate,
        ));
    }
    if clay.rate > 0 && obsidian.rate < bp.geode_cost_obsidian {
        let time_to_obsidian = std::cmp::max(
            div_ceil(bp.obsidian_cost_ore - ore.current, ore.rate),
            div_ceil(bp.obsidian_cost_clay - clay.current, clay.rate),
        ) + 1;
        bests[2] = Some(best_geode_number_helper(
            bp,
            time_remaining - time_to_obsidian,
            MaterialState {
                current: ore.current + time_to_obsidian * ore.rate - bp.obsidian_cost_ore,
                rate: ore.rate,
            },
            MaterialState {
                current: clay.current + time_to_obsidian * clay.rate - bp.obsidian_cost_clay,
                rate: clay.rate,
            },
            MaterialState {
                current: obsidian.current + time_to_obsidian * obsidian.rate,
                rate: obsidian.rate + 1,
            },
            MaterialState {
                current: geode.current + time_to_obsidian * geode.rate,
                rate: geode.rate,
            },
            geode.rate,
        ));
    }
    if obsidian.rate > 0 {
        let time_to_geode = std::cmp::max(
            div_ceil(bp.geode_cost_ore - ore.current, ore.rate),
            div_ceil(bp.geode_cost_obsidian - obsidian.current, obsidian.rate),
        ) + 1;
        bests[3] = Some(best_geode_number_helper(
            bp,
            time_remaining - time_to_geode,
            MaterialState {
                current: ore.current + time_to_geode * ore.rate - bp.geode_cost_ore,
                rate: ore.rate,
            },
            MaterialState {
                current: clay.current + time_to_geode * clay.rate,
                rate: clay.rate,
            },
            MaterialState {
                current: obsidian.current + time_to_geode * obsidian.rate - bp.geode_cost_obsidian,
                rate: obsidian.rate,
            },
            MaterialState {
                current: geode.current + time_to_geode * geode.rate,
                rate: geode.rate + 1,
            },
            geode.rate,
        ));
    }
    *bests.iter().flatten().max().unwrap_or(&0)
}

pub fn solve_part_2(input: &str) -> i32 {
    input
        .lines()
        .take(3)
        .map(|line| parse_blueprint(line))
        .map(|bp| best_geode_number(&bp, 32))
        .product()
}

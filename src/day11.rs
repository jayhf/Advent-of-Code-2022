struct Monkey {
    operator: char,
    operator_val: usize,
    divisible_by: usize,
    true_monkey: usize,
    false_monkey: usize,
    inspected: usize,
}

pub fn solve_part_1(input: &str) -> usize {
    let mut items_by_monkey: Vec<Vec<usize>> = Vec::new();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in input.split("\n\n") {
        let lines: Vec<&str> = monkey.lines().collect();
        items_by_monkey.push(
            lines[1]
                .split_at(18)
                .1
                .split(", ")
                .flat_map(|x| x.parse::<usize>())
                .collect(),
        );
        let (operator, value) = lines[2].split_at(23).1.split_once(" ").unwrap();
        monkeys.push(Monkey {
            operator: operator.chars().nth(0).unwrap(),
            operator_val: value.parse::<usize>().unwrap_or(0),
            divisible_by: lines[3].split_at(21).1.parse::<usize>().unwrap(),
            true_monkey: lines[4].split_at(29).1.parse::<usize>().unwrap(),
            false_monkey: lines[5].split_at(30).1.parse::<usize>().unwrap(),
            inspected: 0,
        });
    }

    for _ in 0..20 {
        for monkey_id in 0..items_by_monkey.len() {
            let monkey: &mut Monkey = &mut monkeys[monkey_id];
            let (before, rest) = items_by_monkey.split_at_mut(monkey_id);
            let (mid, after) = rest.split_at_mut(1);
            for value in mid[0].iter() {
                let v = if monkey.operator_val == 0 {
                    *value
                } else {
                    monkey.operator_val
                };
                let operated = match monkey.operator {
                    '+' => value + v,
                    '*' => value * v,
                    _ => panic!("Bad operator"),
                };
                let new_value = operated / 3;
                let next_monkey = if (new_value % monkey.divisible_by) == 0 {
                    monkey.true_monkey
                } else {
                    monkey.false_monkey
                };
                monkey.inspected += 1;
                if next_monkey < monkey_id {
                    before[next_monkey].push(new_value);
                } else {
                    after[next_monkey - monkey_id - 1].push(new_value);
                }
            }
            mid[0].clear();
        }
    }

    let mut times: Vec<usize> = monkeys.iter().map(|m| m.inspected).collect();
    times.sort();
    times[times.len() - 2..].iter().product::<usize>()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut items_by_monkey: Vec<Vec<usize>> = Vec::new();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in input.split("\n\n") {
        let lines: Vec<&str> = monkey.lines().collect();
        items_by_monkey.push(
            lines[1]
                .split_at(18)
                .1
                .split(", ")
                .flat_map(|x| x.parse::<usize>())
                .collect(),
        );
        let (operator, value) = lines[2].split_at(23).1.split_once(" ").unwrap();
        monkeys.push(Monkey {
            operator: operator.chars().nth(0).unwrap(),
            operator_val: value.parse::<usize>().unwrap_or(0),
            divisible_by: lines[3].split_at(21).1.parse::<usize>().unwrap(),
            true_monkey: lines[4].split_at(29).1.parse::<usize>().unwrap(),
            false_monkey: lines[5].split_at(30).1.parse::<usize>().unwrap(),
            inspected: 0,
        });
    }

    let relevant_max: usize = monkeys.iter().map(|m| m.divisible_by).product();

    for _ in 0..10000 {
        for monkey_id in 0..items_by_monkey.len() {
            let monkey: &mut Monkey = &mut monkeys[monkey_id];
            let (before, rest) = items_by_monkey.split_at_mut(monkey_id);
            let (mid, after) = rest.split_at_mut(1);
            for value in mid[0].iter() {
                let v = if monkey.operator_val == 0 {
                    *value
                } else {
                    monkey.operator_val
                };
                let operated = match monkey.operator {
                    '+' => value + v,
                    '*' => value * v,
                    _ => panic!("Bad operator"),
                };
                let new_value = operated % relevant_max;
                let next_monkey = if (new_value % monkey.divisible_by) == 0 {
                    monkey.true_monkey
                } else {
                    monkey.false_monkey
                };
                monkey.inspected += 1;
                if next_monkey < monkey_id {
                    before[next_monkey].push(new_value);
                } else {
                    after[next_monkey - monkey_id - 1].push(new_value);
                }
            }
            mid[0].clear();
        }
    }

    let mut times: Vec<usize> = monkeys.iter().map(|m| m.inspected).collect();
    times.sort();
    times[times.len() - 2..].iter().product::<usize>()
}

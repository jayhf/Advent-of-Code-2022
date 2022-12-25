use std::collections::HashMap;

struct Operation<'a> {
    operator: char,
    source_a: &'a str,
    source_b: &'a str,
}

struct Monkey<'a> {
    value: Option<i64>,
    operation: Option<Operation<'a>>,
}

pub fn solve_part_1(input: &str) -> i64 {
    let mut monkeys = parse_monkeys(input);
    loop {
        assert!(compute_values(&mut monkeys));
        match monkeys.get("root").unwrap().value {
            Some(v) => return v,
            None => (),
        }
    }
}

fn compute_values(monkeys: &mut HashMap<&str, Monkey>) -> bool {
    let mut updates: Vec<(&str, i64)> = Vec::new();
    for (id, monkey) in monkeys.iter() {
        match monkey.value {
            Some(_) => (),
            None => {
                let op = monkey.operation.as_ref().unwrap();
                match (
                    monkeys.get(op.source_a).map(|m| m.value).flatten(),
                    monkeys.get(op.source_b).map(|m| m.value).flatten(),
                ) {
                    (Some(v1), Some(v2)) => {
                        let value = match op.operator {
                            '+' => v1 + v2,
                            '-' => v1 - v2,
                            '*' => v1 * v2,
                            '/' => v1 / v2,
                            _ => panic!("Unknown operator"),
                        };
                        updates.push((*id, value));
                    }
                    _ => {}
                }
            }
        }
    }
    for (id, v) in updates.iter() {
        monkeys.get_mut(id).unwrap().value = Some(*v);
    }
    updates.len() > 0
}

fn parse_monkeys(input: &str) -> HashMap<&str, Monkey> {
    return input
        .lines()
        .map(|line| {
            let (id, rest) = line.split_once(": ").unwrap();
            (
                id,
                match rest.parse::<i64>() {
                    Ok(value) => Monkey {
                        value: Some(value),
                        operation: None,
                    },
                    _ => {
                        let parts: Vec<&str> = rest.split(" ").collect();
                        Monkey {
                            value: None,
                            operation: Some(Operation {
                                operator: parts[1].chars().nth(0).unwrap(),
                                source_a: parts[0],
                                source_b: parts[2],
                            }),
                        }
                    }
                },
            )
        })
        .collect();
}

pub fn solve_part_2(input: &str) -> i64 {
    const HUMAN: &'static str = "humn";
    let mut monkeys = parse_monkeys(input);
    monkeys.remove(HUMAN);
    while compute_values(&mut monkeys) {}
    monkeys.insert(
        HUMAN,
        Monkey {
            value: None,
            operation: None,
        },
    );
    let mut current_root: &str = "root";
    let mut current_target: Option<i64> = None;
    loop {
        let op = monkeys
            .get(current_root)
            .unwrap()
            .operation
            .as_ref()
            .unwrap();
        let (value, next_root, left) = match (
            monkeys.get(op.source_a).unwrap().value,
            monkeys.get(op.source_b).unwrap().value,
        ) {
            (Some(v), None) => (v, op.source_b, true),
            (None, Some(v)) => (v, op.source_a, false),
            _ => panic!("Logic error"),
        };

        current_target = Some(match current_target {
            None => value,
            Some(target) => {
                if next_root == HUMAN {
                    return target;
                }
                match op.operator {
                    '+' => target - value,
                    '-' => {
                        if left {
                            value - target
                        } else {
                            target + value
                        }
                    }
                    '*' => target / value,
                    '/' => {
                        if left {
                            value / target
                        } else {
                            target * value
                        }
                    }
                    _ => panic!("Unknown operator"),
                }
            }
        });
        current_root = next_root;
    }
}

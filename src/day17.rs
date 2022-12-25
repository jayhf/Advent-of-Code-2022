use std::collections::HashMap;

struct Rock {
    surface: Vec<u8>,
    width: u8,
}

pub fn solve_part_1(input: &str) -> usize {
    let left_jets: Vec<bool> = input.chars().map(|c| c == '<').collect();
    let rocks = get_rocks();
    let mut pile: Vec<u8> = Vec::new();
    let mut jet_index: usize = 0;

    for i in 0..2022 {
        simulate_fall(
            &mut pile,
            &left_jets,
            &mut jet_index,
            &rocks[i % rocks.len()],
        );
    }

    pile.len()
}

fn simulate_fall(pile: &mut Vec<u8>, left_jets: &Vec<bool>, jet_index: &mut usize, rock: &Rock) {
    let mut x: u8 = 2;
    let mut y = pile.len() + 3;

    loop {
        let new_x = if left_jets[*jet_index % left_jets.len()] {
            x.saturating_sub(1)
        } else {
            std::cmp::min(x + 1, 7 - rock.width)
        };
        *jet_index += 1;
        if !collides(pile, rock, new_x, y) {
            x = new_x;
        }
        // println!("{x} {y}");
        if y == 0 || collides(pile, rock, x, y - 1) {
            while pile.len() < y + rock.surface.len() {
                pile.push(0);
            }
            for (p, r) in pile[y..].iter_mut().zip(rock.surface.iter()) {
                *p |= r << x
            }

            break;
        }
        y = y - 1;
    }
}

fn collides(pile: &mut Vec<u8>, rock: &Rock, x: u8, y: usize) -> bool {
    pile[std::cmp::min(y, pile.len())..]
        .iter()
        .zip(rock.surface.iter())
        .any(|(p, r)| ((r << x) & p) != 0)
}

pub fn solve_part_2(input: &str) -> usize {
    let left_jets: Vec<bool> = input.chars().map(|c| c == '<').collect();
    let rocks = get_rocks();
    let mut pile: Vec<u8> = Vec::new();
    let mut jet_index: usize = 0;

    // from jet_index, rock_index, to pile height, rocks
    let mut cycle_states: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    for i in 0..2022 {
        simulate_fall(
            &mut pile,
            &left_jets,
            &mut jet_index,
            &rocks[i % rocks.len()],
        );
    }

    let mut height_change = 0;
    let mut rocks_change = 0;
    let mut last_i = 0;
    for i in 2022usize.. {
        let key = (jet_index % left_jets.len(), i % rocks.len());
        if let Some((old_height, old_rocks)) = cycle_states.get(&key) {
            height_change = pile.len() - old_height;
            rocks_change = i - old_rocks;
            last_i = i;
            break;
        }
        cycle_states.insert(key, (pile.len(), i));
        simulate_fall(
            &mut pile,
            &left_jets,
            &mut jet_index,
            &rocks[i % rocks.len()],
        );
    }

    const TARGET_ROCKS: usize = 1000000000000;
    let remaining = TARGET_ROCKS - last_i;
    let cycle_copies = remaining / rocks_change;
    for i in last_i + cycle_copies * rocks_change..TARGET_ROCKS {
        simulate_fall(
            &mut pile,
            &left_jets,
            &mut jet_index,
            &rocks[i % rocks.len()],
        );
    }

    pile.len() + cycle_copies * height_change
}

fn get_rocks() -> Vec<Rock> {
    vec![
        Rock {
            width: 4,
            surface: vec![0b1111],
        },
        Rock {
            width: 3,
            surface: vec![0b010, 0b111, 0b010],
        },
        Rock {
            width: 3,
            surface: vec![0b111, 0b100, 0b100],
        },
        Rock {
            width: 1,
            surface: vec![0b1, 0b1, 0b1, 0b1],
        },
        Rock {
            width: 2,
            surface: vec![0b11, 0b11],
        },
    ]
}

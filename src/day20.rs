pub fn solve_part_1(input: &str) -> i64 {
    let mut values = parse_values(input);
    mix(&mut values);
    grove_coordinates(&mut values).iter().sum()
}

fn grove_coordinates(values: &mut Vec<(usize, i64)>) -> [i64; 3] {
    let zero_idx = values.iter().position(|(_, x)| *x == 0).unwrap();
    [
        values[(zero_idx + 1000) % values.len()].1,
        values[(zero_idx + 2000) % values.len()].1,
        values[(zero_idx + 3000) % values.len()].1,
    ]
}

fn parse_values(input: &str) -> Vec<(usize, i64)> {
    input
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .enumerate()
        .collect()
}

fn mix(values: &mut Vec<(usize, i64)>) {
    for v_i in 0..values.len() {
        // println!("{:?}", values);
        let idx = values.iter().position(|(i, _)| *i == v_i).unwrap();

        let mut i = idx;
        let v = values[idx].1;
        if v >= 0 {
            for _ in 0..(v as usize % (values.len() - 1)) {
                let new_i = (i + 1) % values.len();
                values[i] = values[new_i];
                i = new_i;
            }
        } else {
            for _ in 0..values.len() as i64 - 1 + (-v % (values.len() as i64 - 1)) {
                let new_i = (values.len() + i - 1) % values.len();
                values[i] = values[new_i];
                i = new_i;
            }
        }
        values[i] = (v_i, v);
    }
}

//11073
pub fn solve_part_2(input: &str) -> i64 {
    const KEY: i64 = 811589153;
    let mut values = parse_values(input);
    for (_, v) in values.iter_mut() {
        *v *= KEY;
    }
    for _ in 0..10 {
        mix(&mut values);
    }
    grove_coordinates(&mut values).iter().sum()
}

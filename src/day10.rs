pub fn solve_part_1(input: &str) -> i32 {
    let mut x = 1;
    let mut cycle = 1;
    let mut total = 0;
    for line in input.lines() {
        let noop = line == "noop";
        if !noop {
            if (cycle + 1 - 20) % 40 == 0 {
                total += (cycle + 1) * x;
            }
            let (_, val) = line.split_once(" ").unwrap();
            x += val.parse::<i32>().unwrap();
            cycle += 2;
        } else {
            cycle += 1;
        }
        if (cycle - 20) % 40 == 0 {
            total += cycle * x;
        }
    }
    total
}

pub fn solve_part_2(input: &str) -> String {
    let mut x = 1;
    let mut cycle: i32 = 1;
    let mut output = "\n".to_owned();
    for line in input.lines() {
        let noop = line == "noop";
        output.push(if ((cycle % 40) - x - 1).abs() <= 1 {
            '#'
        } else {
            '.'
        });
        if !noop {
            if (cycle + 1) % 40 == 1 {
                output.push('\n');
            }
            output.push(if (((cycle + 1) % 40) - x - 1).abs() <= 1 {
                '#'
            } else {
                '.'
            });
            let (_, val) = line.split_once(" ").unwrap();
            x += val.parse::<i32>().unwrap();
            cycle += 2;
        } else {
            cycle += 1;
        }
        if cycle % 40 == 1 {
            output.push('\n');
        }
    }
    output
}

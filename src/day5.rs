pub fn solve_part_1(input: &str) -> String {
    let mut config_done = false;
    let mut lines: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if config_done {
            let parts: Vec<usize> = line.split(" ").flat_map(|x| x.parse::<usize>()).collect();
            match (parts.get(0), parts.get(1), parts.get(2)) {
                (Some(&count), Some(&from), Some(&to)) => {
                    for _ in 0..count {
                        let element = lines[from - 1].pop().unwrap();
                        lines[to - 1].push(element);
                    }
                }
                _ => {}
            }
        } else if line.chars().nth(1) == Some('1') {
            for line in &mut lines {
                line.reverse();
            }
            config_done = true;
        } else {
            let line_count = (line.len() + 1) / 4;
            while lines.len() < line_count {
                lines.push(Vec::new());
            }
            for i in 0..line_count {
                line.chars()
                    .nth(4 * i + 1)
                    .filter(|c| *c != ' ')
                    .map(|c| lines[i].push(c));
            }
        }
    }
    lines.iter().flat_map(|line| line.last()).collect()
}

pub fn solve_part_2(input: &str) -> String {
    let mut config_done = false;
    let mut lines: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if config_done {
            let parts: Vec<usize> = line.split(" ").flat_map(|x| x.parse::<usize>()).collect();
            match (parts.get(0), parts.get(1), parts.get(2)) {
                (Some(&count), Some(&from), Some(&to)) => {
                    // if from != to{
                    //     let (from, to) = if from < to {
                    //         let (from_part, to_part) = lines.split_at_mut(from+1);
                    //         (from_part.last_mut(), to_part.get_mut(to - from - 1))
                    //     }
                    //     else{
                    //         let (to_part, from_part) = lines.split_at_mut(to+1);
                    //         (from_part.get_mut(from - to - 1), to_part.last_mut())
                    //     }
                    // }
                    let mut moved: Vec<char> = Vec::new();
                    {
                        let from_vec = lines.get_mut(from - 1).unwrap();
                        moved.extend_from_slice(&from_vec[from_vec.len() - count..]);
                        from_vec.truncate(from_vec.len() - count);
                    }
                    lines.get_mut(to - 1).unwrap().append(&mut moved);
                }
                _ => {}
            }
        } else if line.chars().nth(1) == Some('1') {
            for line in &mut lines {
                line.reverse();
            }
            config_done = true;
        } else {
            let line_count = (line.len() + 1) / 4;
            while lines.len() < line_count {
                lines.push(Vec::new());
            }
            for i in 0..line_count {
                line.chars()
                    .nth(4 * i + 1)
                    .filter(|c| *c != ' ')
                    .map(|c| lines[i].push(c));
            }
        }
    }
    lines.iter().flat_map(|line| line.last()).collect()
}

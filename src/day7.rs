use std::collections::HashMap;

pub fn solve_part_1(input: &str) -> usize {
    let filesystem = parse_filesystem(input);
    filesystem
        .iter()
        .map(|(dir, _)| total_size(&filesystem, dir))
        .filter(|size| size <= &100000)
        .sum()
}

fn parse_filesystem(input: &str) -> HashMap<String, Vec<(&str, &str)>> {
    let mut current_dir: Vec<&str> = [""].to_vec();
    let mut filesystem: HashMap<String, Vec<(&str, &str)>> = HashMap::new();
    for line in input.lines() {
        if line.starts_with("$") {
            if line.starts_with("$ cd") {
                match line.split(" ").nth(2) {
                    Some("/") => current_dir = [""].to_vec(),
                    Some("..") => current_dir.pop().map(|_| ()).unwrap(),
                    Some(path) => current_dir.push(path),
                    None => panic!("!!!"),
                }
            }
        } else {
            filesystem
                .entry(current_dir.join("/"))
                .or_insert(Vec::new())
                .push(line.split_once(" ").unwrap());
        }
    }
    filesystem
}

fn total_size(filesystem: &HashMap<String, Vec<(&str, &str)>>, dir: &String) -> usize {
    filesystem
        .get(dir)
        .map(|contents| {
            contents
                .iter()
                .map(|(size, name)| {
                    size.parse::<usize>().unwrap_or_else(|_| {
                        total_size(filesystem, &(String::new() + dir + "/" + name))
                    })
                })
                .sum()
        })
        .iter()
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let filesystem = parse_filesystem(input);
    let free_space = 70000000 - total_size(&filesystem, &"".to_owned());
    let space_needed = 30000000 - free_space;
    filesystem
        .iter()
        .map(|(dir, _)| total_size(&filesystem, dir))
        .filter(|size| size >= &space_needed)
        .min()
        .unwrap()
}

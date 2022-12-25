use std::collections::HashMap;
use std::fs;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let inputs: HashMap<i32, String> = (1..=25)
        .map(|i| (i, fs::read_to_string(format!("data/day{}.txt", i))))
        .filter(|(_, file)| file.is_ok())
        .map(|(i, file)| (i, file.unwrap()))
        .collect();

    println!("Most Calories: {:?} kcal", day1::solve_part_1(&&inputs[&1]));
    println!(
        "Top Three Calories: {:?} kcal",
        day1::solve_part_2(&inputs[&1])
    );
    println!(
        "Score with strategy guide: {:?}",
        day2::solve_part_1(&inputs[&2])
    );
    println!(
        "Score with strategy guide: {:?}",
        day2::solve_part_2(&inputs[&2])
    );
    println!(
        "Rucksack priority sum: {:?}",
        day3::solve_part_1(&inputs[&3])
    );
    println!(
        "Rucksack badge priority sum: {:?}",
        day3::solve_part_2(&inputs[&3])
    );
    println!("Range within range: {:?}", day4::solve_part_1(&inputs[&4]));
    println!(
        "Range contains range: {:?}",
        day4::solve_part_2(&inputs[&4])
    );
    println!("Final crates on top: {:?}", day5::solve_part_1(&inputs[&5]));
    println!("Final crates on top: {:?}", day5::solve_part_2(&inputs[&5]));
    println!(
        "Number of characters before the first start of packet: {:?}",
        day6::solve_part_1(&inputs[&6])
    );
    println!(
        "Number of characters before the first start of message: {:?}",
        day6::solve_part_2(&inputs[&6])
    );
    println!(
        "Total size of small directories: {:?}",
        day7::solve_part_1(&inputs[&7])
    );
    println!(
        "Size of the smallest directory to delete: {:?}",
        day7::solve_part_2(&inputs[&7])
    );
    println!("Visible Trees: {:?}", day8::solve_part_1(&inputs[&8]));
    println!(
        "Highest Scenic Score: {:?}",
        day8::solve_part_2(&inputs[&8])
    );
    println!("Rope Tail Positions: {:?}", day9::solve_part_1(&inputs[&9]));
    println!("Rope Tail Positions: {:?}", day9::solve_part_2(&inputs[&9]));
    println!("Signal Strength Sum: {}", day10::solve_part_1(&inputs[&10]));
    println!("CRT: {}", day10::solve_part_2(&inputs[&10]));
    println!(
        "Monkey Business Level: {}",
        day11::solve_part_1(&inputs[&11])
    );
    println!(
        "Monkey Business Level: {}",
        day11::solve_part_2(&inputs[&11])
    );
    println!(
        "Fewest Steps to Best Signal: {}",
        day12::solve_part_1(&inputs[&12])
    );
    println!(
        "Fewest Steps to Best Signal From Anywhere: {}",
        day12::solve_part_2(&inputs[&12])
    );
    println!(
        "Sum of Indices of Pairs: {}",
        day13::solve_part_1(&inputs[&13])
    );
    println!(
        "Decoder Key for the Distress Signal: {}",
        day13::solve_part_2(&inputs[&13])
    );
    println!("Grains of Sand: {}", day14::solve_part_1(&inputs[&14]));
    println!(
        "Grains of Sand with Floor: {}",
        day14::solve_part_2(&inputs[&14])
    );
    println!("Impossible Beacons: {}", day15::solve_part_1(&inputs[&15]));
    println!("Tuning Frequency: {}", day15::solve_part_2(&inputs[&15]));
    println!(
        "Most Pressure Released: {}",
        day16::solve_part_1(&inputs[&16])
    );
    println!(
        "Most Pressure Released with Elephant: {}",
        day16::solve_part_2(&inputs[&16])
    );
    println!(
        "Tower Height after 2022 Rocks: {}",
        day17::solve_part_1(&inputs[&17])
    );
    println!(
        "Tower Height after 1000000000000 Rocks: {}",
        day17::solve_part_2(&inputs[&17])
    );
    println!(
        "Surface Area of Lava: {}",
        day18::solve_part_1(&inputs[&18])
    );
    println!(
        "Exterior Surface Area of Lava: {}",
        day18::solve_part_2(&inputs[&18])
    );
    println!(
        "Sum of the Blueprint Quality Level: {}",
        day19::solve_part_1(&inputs[&19])
    );
    println!(
        "Product of the First 3 Blueprint Optimal Geode Production: {}",
        day19::solve_part_2(&inputs[&19])
    );
    println!(
        "Sum of the Grove Coordinates: {}",
        day20::solve_part_1(&inputs[&20])
    );
    println!(
        "Sum of the Grove Coordinates after Decryption: {}",
        day20::solve_part_2(&inputs[&20])
    );
    println!(
        "Number Yelled by Root: {}",
        day21::solve_part_1(&inputs[&21])
    );
    println!(
        "Number to Pass Root's Equality Test: {}",
        day21::solve_part_2(&inputs[&21])
    );
    println!("Final Password: {}", day22::solve_part_1(&inputs[&22]));
    println!(
        "Final Password Cubed: {}",
        day22::solve_part_2(&inputs[&22])
    );
    println!(
        "Empty Ground Tiles in Rectangle: {}",
        day23::solve_part_1(&inputs[&23])
    );
    println!(
        "First Round where No Elf Moves: {}",
        day23::solve_part_2(&inputs[&23])
    );
    println!(
        "Fewest Minutes to reach Goal avoiding the Blizzards: {}",
        day24::solve_part_1(&inputs[&24])
    );
    println!(
        "Fewest Minutes to reach Goal, head back and to Goal again: {}",
        day24::solve_part_2(&inputs[&24])
    );
    println!(
        "SNAFU number for Bob: {}",
        day25::solve_part_1(&inputs[&25])
    );
    println!(": {}", day25::solve_part_2(&inputs[&25]));
}

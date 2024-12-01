use std::fs;

pub fn run() {
    println!("module running");
    let input: String = fs::read_to_string("src/day1/input.txt").expect("Unable to read file");
    let pairs: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            (nums[0], nums[1])
        })
        .collect();
    println!("{:?}", pairs);
    let mut lefts: Vec<i32> = pairs.iter().map(|(left, _)| *left).collect();
    let mut rights: Vec<i32> = pairs.iter().map(|(_, right)| *right).collect();
    lefts.sort();
    rights.sort();
    let mut differences: Vec<i32> = lefts.iter()
        .zip(rights.iter())
        .map(|(left, right)| (left - right).abs())
        .collect();
    println!("differences: {:?}", differences);
    let total: i32 = differences.iter().sum::<i32>().abs();
    println!("total {:?}", total);
}
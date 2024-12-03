use std::fs;
use regex::Regex;

pub fn run() {
    let input: String = fs::read_to_string("src/day3/input.txt").expect("Unable to read file");

    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let valid_muls: Vec<&str> = mul_regex
        .find_iter(&*input)
        //why is match a keyword in rust lol
        .map(|matching| matching.as_str())
        .collect();

    println!("Filtered muls: {:?}", valid_muls);

    let int_pairs: Vec<(i32, i32)> = valid_muls
        .iter()
        .map(|s| {
            // Extract the substring between "mul(" and ")"
            let inner = &s[4..s.len() - 1];
            let mut parts = inner.split(',');
            let x = parts.next().unwrap().parse::<i32>().unwrap();
            let y = parts.next().unwrap().parse::<i32>().unwrap();
            (x, y)
        })
        .collect();

    let sum: i32 = int_pairs.into_iter().map(|(x, y)| x * y).sum();

    println!("sum: {}", sum);
}

pub fn run_part_2(){
    let input: String = fs::read_to_string("src/day3/input.txt").expect("Unable to read file");

    let valid_strings_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();

    let valid_items: Vec<&str> = valid_strings_regex
        .find_iter(&*input)
        .map(|matching| matching.as_str())
        .collect();

    println!("Items: {:?}", valid_items);

    //skip everything that comes after a don't
    let mut skipping = false;
    let mut result = Vec::new();
    for entry in valid_items {
        if entry == "don't()" {
            skipping = true; // Start skipping
        } else if entry == "do()" {
            skipping = false; // Stop skipping
        } else if !skipping {
            result.push(entry); // Add only if not skipping
        }
    }

    println!("Result: {:?}", result);

    let int_pairs: Vec<(i32, i32)> = result
        .iter()
        .map(|s| {
            // Extract the substring between "mul(" and ")"
            let inner = &s[4..s.len() - 1];
            let mut parts = inner.split(',');
            let x = parts.next().unwrap().parse::<i32>().unwrap();
            let y = parts.next().unwrap().parse::<i32>().unwrap();
            (x, y)
        })
        .collect();

    let sum: i32 = int_pairs.into_iter().map(|(x, y)| x * y).sum();

    println!("sum: {}", sum);

}

use std::fs;

pub fn run(){
    let input: String = fs::read_to_string("src/day2/input.txt").expect("Unable to read file");

    let input_vects: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let mut counter = 0;
    for (i, vec) in input_vects.iter().enumerate() {
        // println!("{:?}", vec);
        // println!("{}", check_safe(vec));
        if check_safe(vec) {
            counter += 1;
        }
    }
    println!("{}", counter);
}

pub fn runpart2() {
    let input: String = fs::read_to_string("src/day2/input.txt").expect("Unable to read file");

    let input_vects: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut counter = 0;
    for (i, vec) in input_vects.iter().enumerate() {
        // println!("{:?}", vec);
        // println!("{}", check_safe(vec));
        let mut vecsafe: bool = false;
        for i in 0..vec.len() {
            let mut vec_with_removes = vec.clone();
            vec_with_removes.remove(i);
            if check_safe(&vec_with_removes) {
                vecsafe = true;
                break;
            }
        }
        if vecsafe {
            counter += 1;
        }
    }
    println!("{}", counter);
}

fn check_safe(report: &Vec<i32>) -> bool {
    let mut previousDiff = None;
    for number in 0..report.len() - 1 {
        let diff = report[number] - report[number + 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            // println!("diff too small or big");
            return false;
        }

        if !previousDiff.is_none() {
            if (diff > 0 && previousDiff.unwrap() < 0) || (previousDiff.unwrap() > 0 && diff < 0) {
                // println!("switched from increase to decrease");
                return false;
            }
        }

        previousDiff = Some(diff);
    }
    true
}
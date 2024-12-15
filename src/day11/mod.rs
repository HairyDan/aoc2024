use std::collections::HashMap;

pub fn run() {
    let mut stones = vec![5, 89749, 6061, 43, 867, 1965860, 0, 206250];

    for i in 0..25 {
        println!("blink {}, stones: {:?}", i, stones);
        let mut new_stones = vec![];
        for &stone in &stones {
            if stone == 0 {
                new_stones.push(1);
            } else {
                let digits = stone.to_string();
                if digits.len() % 2 == 0 {
                    //if even, split into 2
                    let left = digits[..digits.len()/2].parse::<i64>().unwrap();
                    let right = digits[digits.len()/2..].parse::<i64>().unwrap();
                    new_stones.push(left);
                    new_stones.push(right);

                } else {
                    new_stones.push(stone*2024);
                }
            }
        }

        stones = new_stones;
    }

    println!("num of stones: {}", stones.len());
}



pub fn run_part_2() {
    let initial_stones = vec![5, 89749, 6061, 43, 867, 1965860, 0, 206250];
    let blinks = 75;

    //(stone, blinks) → resulting number of stones
    let mut memo: HashMap<(i64, i32), i64> = HashMap::new();

    for &stone in &initial_stones {
        //each stone is 1 stone after 0 blinks
        memo.insert((stone, 0), 1);
    }

    for b in 1..=blinks {
        let mut new_memo = HashMap::new();

        for (&(stone, _), &count) in memo.iter() {
            let resulting_stones = transform_stone(stone);
            for resulting_stone in resulting_stones {
                *new_memo.entry((resulting_stone, b)).or_insert(0) += count;
            }
        }

        // hate this pattern
        memo = new_memo;
    }

    let mut total = 0;
    for (stone, blinks) in memo.keys() {
        if (*blinks == 75) {
            total += memo.get(&(*stone, *blinks)).unwrap();
            println!("{:?}", memo.get(&(*stone, *blinks)));
        }
    }

    println!("total: {}", total);
}

fn transform_stone(stone: i64) -> Vec<i64> {
    if stone == 0 {
        vec![1]
    } else {
        let digits = stone.to_string();
        if digits.len() % 2 == 0 {
            // split even digits
            let left = digits[..digits.len() / 2].parse::<i64>().unwrap();
            let right = digits[digits.len() / 2..].parse::<i64>().unwrap();
            vec![left, right]
        } else {
            vec![stone * 2024]
        }
    }
}

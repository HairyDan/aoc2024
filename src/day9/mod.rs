use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("src/day9/input.txt").expect("Unable to read file");

    //gttttt
    let mut gaps: bool = false;
    let mut builder: Vec<String> = vec![];
    let mut id = 0;

    for ch in input.chars() {
        println!("{}", ch);
        let num: i32 = ch.to_digit(10).unwrap() as i32;
        println!("{}", num);
        if gaps {
            gaps = false;
            for _ in 0..num {
                builder.push(String::from("."));
            }
        } else {
            gaps = true;
            for _ in 0..num {
                builder.push(format!("{}", id));
            }
            id += 1;
        }
    }
    println!("{}", id);
    println!("{:?}", builder.join(""));

    while (builder.contains(&String::from("."))) {
        //take right most val and put into left most space
        if builder.last().unwrap() == &String::from(".") {
            builder.pop();
        } else {
            let lastval = builder.pop().unwrap();
            let first_empty_idx = builder.iter().position(|s| s == ".").unwrap();
            builder[first_empty_idx] = String::from(lastval);

        }
        // println!("{}", builder.len());

    }

    let mut total: i64 = 0;

    for i in 0..builder.len() {
        total = total + &builder[i].parse().unwrap() * i as i64;
    }

    println!("{}", total);
}

pub fn run_part_2() {
    let input: String = fs::read_to_string("src/day9/input.txt").expect("Unable to read file");

    let mut gaps: bool = false;
    let mut builder: Vec<String> = vec![];
    let mut id = 0;

    for ch in input.chars() {
        let num: i32 = ch.to_digit(10).unwrap() as i32;
        if gaps {
            gaps = false;
            for _ in 0..num {
                builder.push(String::from("."));
            }
        } else {
            gaps = true;
            for _ in 0..num {
                builder.push(format!("{}", id));
            }
            id += 1;
        }
    }
    println!("{}", id);
    println!("{:?}", builder);

    let mut gap_array: Vec<(i32, i32)> = vec![];
    gaps = false;
    let mut string_index = 0;

    for ch in input.chars() {
        let num: i32 = ch.to_digit(10).unwrap() as i32;
        if gaps {
            gaps = false;
            if num != 0 {
                gap_array.push((string_index, num));
            }
            string_index += num;
        } else {
            gaps = true;
            string_index += num;
        }
    }

    println!("{:?}", gap_array);

    let mut modify_this = builder.clone();

    let mut thing_to_move: Vec<(&String, usize)> = vec![];
    let mut id_been_moved = vec![];
    let mut remove_gap = false;
    let mut remove_gap_idx = 0;
    for (idx, entry) in builder.iter().enumerate().rev() {
        if entry != "." && (thing_to_move.len() == 0 || thing_to_move[0].0 == entry){
            thing_to_move.push((entry, idx));
        } else if thing_to_move.len() != 0 {
            // println!("thing to move is {:?}", thing_to_move);
            for (gap_idx, gap) in gap_array.iter_mut().enumerate() {
                //if ther's a big enough gap
                if gap.1 >= thing_to_move.len() as i32 && !id_been_moved.contains(thing_to_move[0].0)  && gap.0 <= idx as i32 {
                    //iterate upwards from the gap starting index replacing . with id
                    println!("doing a move at idx {}", idx);
                    for i in gap.0..gap.0+(thing_to_move.len() as i32) {
                        modify_this[i as usize] = thing_to_move[0].0.clone()
                    }
                    let mut count = 0;
                    for moved in &thing_to_move {
                        modify_this[moved.1] = ".".parse().unwrap();
                    }

                    gap.0 = gap.0 + thing_to_move.len() as i32;
                    gap.1 = (gap.1 - thing_to_move.len() as i32);
                    if (gap.1 == 0) {
                        remove_gap = true;
                        remove_gap_idx = gap_idx;
                    }
                    id_been_moved.push(thing_to_move[0].0.clone());
                }


            }
            if (remove_gap) {
                remove_gap = false;
                gap_array.remove(remove_gap_idx);
            }
            thing_to_move.clear();


        }
        if entry != "." && thing_to_move.len() == 0 {
            // println!("pushin p {:?}", entry);
            thing_to_move.push((entry, idx));
        }

    }

    println!("{:?}", modify_this);
    println!("{:?}", gap_array);


    let mut total: i64 = 0;

    for i in 0..modify_this.len() {
        if modify_this[i] != "." {
            total = total + &modify_this[i].parse().unwrap() * i as i64;
        }
    }

    println!("{}", total);
}
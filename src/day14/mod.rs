use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("src/day14/input.txt").expect("Unable to read file");

    let mut bot_moves: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let p_part = parts[0];
        let v_part = parts[1];

        let p_values = &p_part[2..];
        let v_values = &v_part[2..];

        let p_split: Vec<&str> = p_values.split(',').collect();
        println!("{:?}", p_split[0]);
        let px: i32 = p_split[0].parse().unwrap();
        let py: i32 = p_split[1].parse().unwrap();

        let v_split: Vec<&str> = v_values.split(',').collect();
        let vx: i32 = v_split[0].parse().unwrap();
        let vy: i32 = v_split[1].parse().unwrap();

        let position = (px, py);
        let velocity = (vx, vy);
        let entry = (position, velocity);

        bot_moves.push(entry);
    }

    println!("{:?}", bot_moves);

    let mut cols = Vec::new();

    let width = 101;
    let height: i32 = 103;
    for _ in 0..width {
        let mut row = vec!['.'; height as usize];
        cols.push(row);
    }

    // print_grid(&cols);

    let ticks = 100;

    let mut finished_bot_moves = Vec::new();

    for mv in bot_moves {
        //REMEMBER bot moves are x, y where 0,0 is top left
        let mut temp_x = mv.0.0;
        let mut temp_y = mv.0.1;
        for _ in 0..ticks {
            (temp_x, temp_y) = move_bot(width, height, temp_x, temp_y, mv.1.0, mv.1.1);
        }
        finished_bot_moves.push((temp_x, temp_y));
    }

    println!("{:?}", finished_bot_moves);

    let mut top_left = Vec::new();
    let mut top_right = Vec::new();
    let mut bottom_left = Vec::new();
    let mut bottom_right = Vec::new();

    top_left = finished_bot_moves.iter().filter (|(x, y)| x < &(width / 2) && y < &(height / 2)).collect();
    top_right = finished_bot_moves.iter().filter (|(x, y)| x > &(width / 2) && y < &(height / 2)).collect();
    bottom_left = finished_bot_moves.iter().filter (|(x, y)| x < &(width / 2) && y > &(height / 2)).collect();
    bottom_right = finished_bot_moves.iter().filter (|(x, y)| x > &(width / 2) && y > &(height / 2)).collect();


    println!("{:?}", top_left);
    println!("{:?}", top_right);
    println!("{:?}", bottom_left);
    println!("{:?}", bottom_right);

    println!("total safety factor = {}", top_left.len()*top_right.len()*bottom_left.len()*bottom_right.len());
}

pub fn run_part_2() {
    let input: String = fs::read_to_string("src/day14/input.txt").expect("Unable to read file");

    let mut bot_moves: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let p_part = parts[0];
        let v_part = parts[1];

        let p_values = &p_part[2..];
        let v_values = &v_part[2..];

        let p_split: Vec<&str> = p_values.split(',').collect();
        println!("{:?}", p_split[0]);
        let px: i32 = p_split[0].parse().unwrap();
        let py: i32 = p_split[1].parse().unwrap();

        let v_split: Vec<&str> = v_values.split(',').collect();
        let vx: i32 = v_split[0].parse().unwrap();
        let vy: i32 = v_split[1].parse().unwrap();

        let position = (px, py);
        let velocity = (vx, vy);
        let entry = (position, velocity);

        bot_moves.push(entry);
    }

    println!("{:?}", bot_moves);

    let mut cols = Vec::new();

    let width = 101;
    let height: i32 = 103;
    for _ in 0..width {
        let mut row = vec!['.'; height as usize];
        cols.push(row);
    }

    // print_grid(&cols);

    let ticks = 100;

    let mut new_bot_moves = bot_moves.clone();
    let mut counter = 0;
    while true {
        counter +=1;
        println!("{}", counter);
        for (idx, mv) in bot_moves.iter().enumerate() {
            //REMEMBER bot moves are x, y where 0,0 is top left
            new_bot_moves[idx] = (move_bot(width, height, mv.0.0, mv.0.1, mv.1.0, mv.1.1), (mv.1.0, mv.1.1));
        }
        bot_moves = new_bot_moves.clone();
        let copy_bot_moves_sick_of_rust = bot_moves.clone();

        let mut finished_bot_moves: Vec<(i32,i32)> = Vec::new();
        for mv in copy_bot_moves_sick_of_rust {
            finished_bot_moves.push(mv.0);
        }
        // ALL THIS IS SCAMMED BECAUSE THE TREE IS NOT SYMMETRICAL
        // let mut top_left = Vec::new();
        // let mut top_right = Vec::new();
        // let mut bottom_left = Vec::new();
        // let mut bottom_right = Vec::new();
        //
        // top_left = finished_bot_moves.iter().filter (|(x, y)| x < &(width / 2) && y < &(height / 2)).collect();
        // top_right = finished_bot_moves.iter().filter (|(x, y)| x > &(width / 2) && y < &(height / 2)).collect();
        // bottom_left = finished_bot_moves.iter().filter (|(x, y)| x < &(width / 2) && y > &(height / 2)).collect();
        // bottom_right = finished_bot_moves.iter().filter (|(x, y)| x > &(width / 2) && y > &(height / 2)).collect();
        //
        // if top_left.len() == top_right.len() && bottom_left.len() == bottom_right.len() {
        //     println!("symmetrical number {}", counter);
        //     let first_top_left = &top_left[0];
        //     let inverse_check_top_right = (width - first_top_left.0, first_top_left.1);
        //     if top_right.contains(&&inverse_check_top_right) {
        //         print_grid(&cols, &finished_bot_moves);
        //     }
        //     // break
        // }

        if print_grid(&cols, &finished_bot_moves){
            break
        }

    }
}

fn move_bot(width: i32, height: i32, x_pos: i32, y_pos: i32, x_mv: i32, y_mv: i32) -> (i32, i32) {
    let mut new_x = -1;
    let mut new_y = -1;
    if x_pos + x_mv >= width {
        new_x = x_pos + x_mv - width;
    } else if x_pos + x_mv < 0 {
        new_x = width + (x_pos + x_mv);
    } else {
        new_x = x_pos + x_mv;
    }

    if y_pos + y_mv >= height {
        new_y = y_pos + y_mv - height;
    } else if y_pos + y_mv < 0 {
        new_y = height + (y_pos + y_mv);
    } else {
        new_y = y_pos + y_mv;
    }

    (new_x, new_y)
}

fn print_grid(grid: &Vec<Vec<char>>, bot_posses: &Vec<((i32, i32))>) -> bool {
    let mut griddy = grid.clone();
    for pos in bot_posses {
        // println!("{:?}", pos);
        if griddy[pos.0 as usize][pos.1 as usize] == '.' {
            griddy[pos.0 as usize][pos.1 as usize] = '1';
        } else {
            // println!("{:?}", griddy[pos.0 as usize][pos.1 as usize]);
            griddy[pos.0 as usize][pos.1 as usize] = (griddy[pos.0 as usize][pos.1 as usize] as u8 + 1) as char;
        }
    }
    for row in griddy {
        // let row_str: String = row.iter().collect();
        let row_str: String = row.iter().map(|ch| format!("{}", ch)).collect();
        if row_str.contains("11111111") {
            println!("{}", row_str);
            return true;
        }


    }
    false
}
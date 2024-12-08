use std::collections::HashMap;
use std::fs;

const N: (i32, i32) = (-1, 0);
const E: (i32, i32) = (0, 1);
const S: (i32, i32) = (1, 0);
const W: (i32, i32) = (0, -1);
/*
      (-1, -1)  (-1, 0)  (-1, 1)
        NW         N         NE

        (0, -1)    .     (0, 1)
         W         .         E

       (1, -1)   (1, 0)   (1, 1)
        SW         S         SE
*/
pub fn run() {
    let input: String = fs::read_to_string("src/day6/input.txt").expect("Unable to read file");

    let mut input_matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    println!("input matrix: {:?}", input_matrix);

    let mut guard_pos: (usize, usize) = (0, 0);
    for (rownum, row) in input_matrix.iter().enumerate() {
        for (colnum, char) in row.iter().enumerate() {
            if *char == '^' {
                guard_pos = (rownum, colnum);
            }
        }
    }
    let x_lim = input_matrix[0].len();
    let y_lim = input_matrix.len();
    let mut guard_facing = N;

    println!("Guard start: {:?}", guard_pos);

    let mut new_pos = (0, 0);
    while new_pos != (-1, -1) {
        new_pos = add_pair(guard_pos, guard_facing, x_lim, y_lim);

        println!("New pos: {:?}", new_pos);

        if new_pos == (-1, -1) {
            break;
        }
        while input_matrix[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            guard_facing = rotate(guard_facing);
            new_pos = add_pair(guard_pos, guard_facing, x_lim, y_lim);
        }

        input_matrix[guard_pos.0][guard_pos.1] = 'X';
        guard_pos = (new_pos.0 as usize, new_pos.1 as usize);
        input_matrix[guard_pos.0][guard_pos.1] = get_guard_facing_char(guard_facing);

        // println!("input matrix: {:?}", input_matrix);
    }

    let sum = input_matrix
        .iter()
        .flatten()
        .filter(|&&c| (c == 'X') | (c == 'v') | (c == '>') | (c == '<') | (c == '^'))
        .count();
    println!("sum: {}", sum);
}

pub fn run_part_2() {
    let input: String = fs::read_to_string("src/day6/input.txt").expect("Unable to read file");

    let mut input_matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    println!("input matrix: {:?}", input_matrix);

    let mut guard_start: (usize, usize) = (0, 0);
    for (rownum, row) in input_matrix.iter().enumerate() {
        for (colnum, char) in row.iter().enumerate() {
            if *char == '^' {
                guard_start = (rownum, colnum);
            }
        }
    }
    let x_lim = input_matrix[0].len();
    let y_lim = input_matrix.len();
    let mut guard_facing = N;

    println!("Guard start: {:?}", guard_start);
    let mut obstacle_causes_loop_count = 0;

    let mut backup_matrix = input_matrix.clone();
    let mut changed_matrix = backup_matrix.clone();

    for rownum in 0..input_matrix[0].len() {
        for colnum in 0..input_matrix.len() {
            changed_matrix = backup_matrix.clone();
            println!("rownum: {}, colnum: {}", rownum, colnum);
            if (changed_matrix[rownum][colnum] == '.') {
                changed_matrix[rownum][colnum] = '#';
            }

            let mut guard_pos = guard_start;
            guard_facing = N;

            let mut new_pos = (0, 0);
            //map of position to direction, if we _ever_ have same pos, same dir, we're in a loop
            let mut previously_visited: HashMap<(usize, usize), (i32, i32)> = HashMap::new();
            while new_pos != (-1, -1) {
                new_pos = add_pair(guard_pos, guard_facing, x_lim, y_lim);
                if new_pos == (-1, -1) {
                    break;
                }
                while changed_matrix[new_pos.0 as usize][new_pos.1 as usize] == '#' {
                    guard_facing = rotate(guard_facing);
                    new_pos = add_pair(guard_pos, guard_facing, x_lim, y_lim);
                }
                if (previously_visited.contains_key(&guard_pos)
                    && previously_visited.get(&guard_pos).unwrap() == &guard_facing)
                {
                    //we have been here before
                    obstacle_causes_loop_count += 1;
                    //force break by overriding new_pos
                    new_pos = (-1,-1);
                } else {
                    //we haven't been here before
                    previously_visited.insert(guard_pos, guard_facing);
                }
                if (new_pos != (-1, -1)) {
                    guard_pos = (new_pos.0 as usize, new_pos.1 as usize);
                    changed_matrix[guard_pos.0][guard_pos.1] = get_guard_facing_char(guard_facing);
                }
            }
        }
    }
    println!("total obs that cause loop: {}", obstacle_causes_loop_count);
}

fn add_pair(a: (usize, usize), b: (i32, i32), lim_x: usize, lim_y: usize) -> (i32, i32) {
    let res = (a.0 as i32 + b.0, a.1 as i32 + b.1);
    // println!("res: {:?}", res);
    if (res.0 < 0 || res.0 >= lim_y as i32 || res.1 < 0 || res.1 >= lim_x as i32) {
        //if we are out of bounds, return an impossible position to trigger end state
        return (-1, -1);
    }
    res
}

fn rotate(facing: (i32, i32)) -> (i32, i32) {
    if facing == N {
        return E;
    } else if facing == E {
        return S;
    } else if facing == S {
        return W;
    } else if facing == W {
        return N;
    }
    return (999, 999);
}

fn get_guard_facing_char(guard_facing: (i32, i32)) -> char {
    if guard_facing == N {
        return '^';
    } else if guard_facing == E {
        return '>';
    } else if guard_facing == S {
        return 'v';
    } else if guard_facing == W {
        return '<';
    }
    return '?';
}

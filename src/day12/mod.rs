use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("src/day12/input.txt").expect("Unable to read file");

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut groups = Vec::new();

    // Directions for 4-neighbor connectivity
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // find groups
    for row in 0..rows {
        for col in 0..cols {
            if !visited[row][col] {
                let char_type = grid[row][col];
                let (area, perimeter) = dfs(&grid, &mut visited, &directions, row, col, char_type);
                groups.push((area, perimeter));
            }
        }
    }

    let mut total = 0;

    for (i, (area, perimeter)) in groups.iter().enumerate() {
        println!(
            "Group {}: Area = {}, Perimeter = {}",
            i + 1,
            area,
            perimeter
        );

        total += perimeter * area;
    }

    println!("Total: {}", total);
}

pub fn run_part_2() {
    let input: String = fs::read_to_string("src/day12/input.txt").expect("Unable to read file");

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut groups = Vec::new();

    // Directions for 4-neighbor connectivity
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // find groups
    for row in 0..rows {
        for col in 0..cols {
            if !visited[row][col] {
                let char_type = grid[row][col];
                let (area, sides) =
                    dfs_find_sides(&grid, &mut visited, &directions, row, col, char_type);
                groups.push((area, sides));
            }
        }
    }

    // let mut total_connected_count = 0;
    let mut total = 0.0;
    //need to convert sides coord vecs
    for (index, (area, sides_vec)) in &mut groups.iter().enumerate() {
        let mut sidecount = 0.0;
        println!("{:?}", sides_vec);
        let mut othersides_vec = sides_vec.clone();

        for othersides in othersides_vec.iter() {
            let mut connected_count = 0;
            for side in sides_vec.iter() {
                // println!("is connected {:?}, {:?}. {}", side, othersides, is_connected(side, othersides));
                if is_connected(side, othersides) {
                    connected_count += 1;
                }
            }
            println!("connected count: {}", connected_count);
            if (connected_count == 2) {
                //do nothing, it's a middle piece
            } else if (connected_count == 1) {
                //it's an end piece, need 2 of them to make a side
                sidecount += 0.5;
            } else if (connected_count == 0) {
                //has no connections, so it's a 1 length side
                sidecount += 1.0;
            } else {
                println!("ERROR ERROR ERROR");
            }
        }
        total += sidecount * *area as f64;
    }
    println!("Total: {}", total);

}

fn is_connected(line1: &((i32, i32), (i32, i32)), line2: &((i32, i32), (i32, i32))) -> bool {

    if line1 == line2 {
        return false;
    }
    let (start1, end1) = line1;
    let (start2, end2) = line2;

    //sameorientation
    if same_orientation(line1, line2) {
        // println!("{:?},{:?} same orientation", line1, line2);
        //both vertical
        if line1.0.0 == line1.1.0 {
            //y needs to be +-1
            if (line1.0.0 - line2.0.0).abs() == 1 && (line1.1.0 - line2.1.0).abs() == 1{
                if (line1.0.1 == line2.0.1 && line1.1.1 == line2.1.1) {
                    return true;
                }
            }
        } else {
            //both horizontal
            //don't think this logic works if they're not ordered properly, check if wrong
            if (line1.0.1 - line2.0.1).abs() == 1 && (line1.1.1 - line2.1.1).abs() == 1{
                if (line1.0.0 == line2.0.0 && line1.1.0 == line2.1.0) {
                    return true;
                }
            }
        }
    }

    false
}

fn same_orientation(line1: &((i32, i32), (i32, i32)), line2: &((i32, i32), (i32, i32))) -> bool {
    let is_vertical1 = line1.0 .0 == line1.1 .0;
    let is_vertical2 = line2.0 .0 == line2.1 .0;
    is_vertical1 == is_vertical2
}

fn dfs(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    directions: &[(i32, i32)],
    start_row: usize,
    start_col: usize,
    char_type: char,
) -> (usize, usize) {
    let mut stack = vec![(start_row, start_col)];
    let mut area = 0;
    let mut perimeter = 0;
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    while !stack.is_empty() {
        let (row, col) = stack.pop().unwrap();

        if visited[row][col] {
            continue;
        }
        visited[row][col] = true;
        area += 1;

        // check neighbors
        for &(y, x) in directions {
            let new_row = row as i32 + y;
            let new_col = col as i32 + x;

            if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
                // boundary, add to perimeter
                perimeter += 1;
            } else if grid[new_row as usize][new_col as usize] != char_type {
                // different character, add to perimeter
                perimeter += 1;
            } else if !visited[new_row as usize][new_col as usize] {
                // same character and not visited, add to stack
                stack.push((new_row as usize, new_col as usize));
            }
        }
    }
    (area, perimeter)
}

fn dfs_find_sides(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    directions: &[(i32, i32)],
    start_row: usize,
    start_col: usize,
    char_type: char,
) -> (usize, Vec<((i32, i32), (i32, i32))>) {
    let mut stack = vec![(start_row, start_col)];
    let mut area = 0;
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut edges: Vec<((i32, i32), (i32, i32))> = Vec::new();

    while !stack.is_empty() {
        let (row, col) = stack.pop().unwrap();

        if visited[row][col] {
            continue;
        }
        visited[row][col] = true;
        area += 1;

        // check neighbors
        for &(y, x) in directions {
            let new_row = row as i32 + y;
            let new_col = col as i32 + x;

            if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
                // boundary, add to edges
                let edge = ((row as i32, col as i32), (new_row, new_col));
                edges.push(edge);
            } else if grid[new_row as usize][new_col as usize] != char_type {
                // different character, add to esges
                let edge = ((row as i32, col as i32), (new_row, new_col));
                edges.push(edge);
            } else if !visited[new_row as usize][new_col as usize] {
                // same character and not visited, add to stack
                stack.push((new_row as usize, new_col as usize));
            }
        }
    }
    (area, edges)
}

use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("src/day10/input.txt").expect("Unable to read file");

    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut total_score = 0;

    let mut zero_counter = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                let res = bfs_find_nines(&grid, (i, j));
                zero_counter += 1;
                println!("zero number {} at grid {}.{} at  has score {}", zero_counter, i,j, res);
                total_score += res
            }
        }
    }

    println!("tot: {}", total_score);
}

pub fn run_part_2() {
    let input: String = fs::read_to_string("src/day10/input.txt").expect("Unable to read file");

    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut total_score = 0;

    let mut zero_counter = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                let res = bfs_find_nines_pt2(&grid, (i, j));
                zero_counter += 1;
                println!(
                    "zero number {} at grid {}.{} at  has score {}",
                    zero_counter, i, j, res
                );
                total_score += res
            }
        }
    }

    println!("tot: {}", total_score);
}

fn bfs_find_nines(grid: &Vec<Vec<i32>>, start: (usize, usize)) -> usize {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = vec![start];
    let mut count = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    visited[start.0][start.1] = true;

    let mut index = 0;
    while index < queue.len() {
        let (x, y) = queue[index];
        index += 1;

        for (dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let nx = nx as usize;
                let ny = ny as usize;

                if nx < grid.len() && ny < grid[0].len() {
                    let height = grid[nx][ny];

                    if height == grid[x][y] + 1 && !visited[nx][ny] {
                        visited[nx][ny] = true;
                        if height == 9 {
                            count += 1;
                        }
                        queue.push((nx, ny));
                    }
                }
            }
        }
    }

    count
}

fn bfs_find_nines_pt2(grid: &Vec<Vec<i32>>, start: (usize, usize)) -> usize {
    let mut queue = vec![start];
    let mut count = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut index = 0;
    while index < queue.len() {
        let (x, y) = queue[index];
        index += 1;

        for (dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let nx = nx as usize;
                let ny = ny as usize;

                if nx < grid.len() && ny < grid[0].len() {
                    let height = grid[nx][ny];

                    if height == grid[x][y] + 1 {
                        queue.push((nx, ny));
                        if height == 9 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

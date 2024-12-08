use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("src/day8/input.txt").expect("Unable to read file");

    let mut input_matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    // println!("{:?}", input_matrix);
    print_grid(&input_matrix);

    let mut clone_grid = input_matrix.clone();

    let x_lim = input_matrix[0].len();
    let y_lim = input_matrix.len();

    let non_dot_chars = input_matrix.iter().flatten().filter(|&&c| c != '.').collect::<Vec<&char>>();
    let mut antenna_types = Vec::new();
    for c in non_dot_chars {
        if (!antenna_types.contains(c)) {
            antenna_types.push(*c);
        }
    }

    for antenna in antenna_types {
        let all_instances = find_chars(&input_matrix, antenna);
        for instance in all_instances {
            let distances = find_char_distance(&input_matrix, antenna, instance);
            println!("antenna: {} distances:{:?}", antenna, distances);
            //convert relative distances into absolute positions of antinodes
            //take instance, double distance, add to instance
            let mut node_pos = Vec::new();
            for distance in distances {
                //for part 1, just do *2 instead of this loop
                for i in 2..999 {
                    node_pos.push((instance.0 + (distance.0 *i), instance.1 + (distance.1 *i)));
                }
            }

            for node in node_pos {
                if (node.0 < y_lim as i32 && node.0 >= 0 && node.1 < x_lim as i32 && node.1 >= 0) {
                    // if (input_matrix[node.0 as usize][node.1 as usize] == '.' || input_matrix[node.0 as usize][node.1 as usize] == '#') {
                        clone_grid[node.0 as usize][node.1 as usize] = '#';
                    // } else {
                    //     println!("didn't write a node because we are on a {:?}",input_matrix[node.0 as usize][node.1 as usize] );
                    // }
                }
            }
        }
    }

    print_grid(&clone_grid);

    //for part1, replace . with #
    println!("{:?}", clone_grid.iter().flatten().filter(|&&c| c != '.').count());

}

fn find_chars(grid: &Vec<Vec<char>>, c: char) -> Vec<(i32, i32)> {
    let mut positions = Vec::new();
    for (y_index, row) in grid.iter().enumerate() {
        for (x_index, col) in row.iter().enumerate() {
            if *col == c {
                positions.push((y_index as i32, x_index as i32));
            }
        }
    }
    positions
}

fn find_char_distance(grid: &Vec<Vec<char>>, c: char, input_position: (i32, i32)) -> Vec<(i32, i32)> {
    let mut relative_pos_index = Vec::new();
    for (y_index, row) in grid.iter().enumerate() {
        for (x_index, col) in row.iter().enumerate() {
            if *col == c {
                if (y_index as i32 - input_position.0, x_index as i32 - input_position.1) != (0,0) {
                    relative_pos_index.push((y_index as i32 - input_position.0, x_index as i32 - input_position.1));
                }
            }
        }
    }
    relative_pos_index
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        // let row_str: String = row.iter().collect();
        let row_str: String = row.iter().map(|ch| format!("{}", ch)).collect();
        println!("{}", row_str);

    }
}
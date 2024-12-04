use std::fs;

const N: (i32, i32) = (-1, 0);
const NE: (i32, i32) = (-1, 1);
const E: (i32, i32) = (0, 1);
const SE: (i32, i32) = (1, 1);
const S: (i32, i32) = (1, 0);
const SW: (i32, i32) = (1, -1);
const W: (i32, i32) = (0, -1);
const NW: (i32, i32) = (-1, -1);

pub fn run() {
    let input: String = fs::read_to_string("src/day4/input.txt").expect("Unable to read file");

    let input_matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    println!("{:?}", input_matrix);

    //find every X, then look all around for an M, then check same direction for A and S

    let mut xmas_counter = 0;
    for (y, row) in input_matrix.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == 'X' {
                let m_res = look_around_for(&input_matrix, (y as i32, x as i32), 'M');
                for m_direction in m_res {
                    //have ended up defining everything as (y, x) which is driving me mad
                    let m_position = (y as i32 + m_direction.0, x as i32 + m_direction.1);

                    let check_a_position = add_pair(
                        m_position,
                        m_direction,
                        input_matrix[0].len(),
                        input_matrix.len(),
                    );
                    if (input_matrix[check_a_position.0 as usize][check_a_position.1 as usize]
                        == 'A')
                    {
                        let check_s_position = add_pair(
                            check_a_position,
                            m_direction,
                            input_matrix[0].len(),
                            input_matrix.len(),
                        );
                        if (input_matrix[check_s_position.0 as usize][check_s_position.1 as usize]
                            == 'S')
                        {
                            xmas_counter += 1;
                        }
                    }
                }
            }
        }
    }
    println!("xmas_counter: {}", xmas_counter);
}

pub fn run_part_2() {
    let input: String = fs::read_to_string("src/day4/input.txt").expect("Unable to read file");

    let input_matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    println!("{:?}", input_matrix);

    let mut xmas_counter = 0;
    for (y, row) in input_matrix.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == 'A' {
                let m_res = look_around_for(&input_matrix, (y as i32, x as i32), 'M');
                //we only care about the diagonals
                let m_res_filtered: Vec<(i32, i32)> = m_res.iter()
                    .filter(|&pair| *pair == NE || *pair == SE || *pair == SW || *pair == NW).cloned()
                    .collect();
                //need exactly 2 of the diagonals to be M
                if (m_res_filtered.len() == 2) {
                    let mut s_opposite = 0;
                    for m in m_res_filtered {
                        //find the diagonal opposite by multiplying both coords by -1
                        let s_check_dir = (m.0 * -1, m.1 * -1);
                        let s_check_pos = add_pair((y as i32,x as i32), s_check_dir, input_matrix[0].len(), input_matrix.len());
                        if input_matrix[s_check_pos.0 as usize][s_check_pos.1 as usize] == 'S' {
                            s_opposite += 1;
                        }
                        if s_opposite == 2 {
                            xmas_counter += 1;
                        }
                    }
                }
            }
        }
    }
    println!("xmas_counter: {}", xmas_counter);
}

/*
      (-1, -1)  (-1, 0)  (-1, 1)
        NW         N         NE

        (0, -1)    .     (0, 1)
         W         .         E

       (1, -1)   (1, 0)   (1, 1)
        SW         S         SE
*/

fn look_around_for(matrix: &Vec<Vec<char>>, position: (i32, i32), ch: char) -> Vec<(i32, i32)> {
    let n_pos = add_pair(position, N, matrix[0].len(), matrix.len());
    let ne_pos = add_pair(position, NE, matrix[0].len(), matrix.len());
    let e_pos = add_pair(position, E, matrix[0].len(), matrix.len());
    let se_pos = add_pair(position, SE, matrix[0].len(), matrix.len());
    let s_pos = add_pair(position, S, matrix[0].len(), matrix.len());
    let sw_pos = add_pair(position, SW, matrix[0].len(), matrix.len());
    let w_pos = add_pair(position, W, matrix[0].len(), matrix.len());
    let nw_pos = add_pair(position, NW, matrix[0].len(), matrix.len());

    let mut found_vec = Vec::new();
    if (matrix[n_pos.0 as usize][n_pos.1 as usize]) == ch {
        found_vec.push(N);
    }
    if (matrix[ne_pos.0 as usize][ne_pos.1 as usize]) == ch {
        found_vec.push(NE);
    }
    if (matrix[e_pos.0 as usize][e_pos.1 as usize]) == ch {
        found_vec.push(E);
    }
    if (matrix[se_pos.0 as usize][se_pos.1 as usize]) == ch {
        found_vec.push(SE);
    }
    if (matrix[s_pos.0 as usize][s_pos.1 as usize]) == ch {
        found_vec.push(S);
    }
    if (matrix[sw_pos.0 as usize][sw_pos.1 as usize]) == ch {
        found_vec.push(SW);
    }
    if (matrix[w_pos.0 as usize][w_pos.1 as usize]) == ch {
        found_vec.push(W);
    }
    if (matrix[nw_pos.0 as usize][nw_pos.1 as usize]) == ch {
        found_vec.push(NW);
    }

    return found_vec;
}

fn add_pair(a: (i32, i32), b: (i32, i32), lim_x: usize, lim_y: usize) -> (i32, i32) {
    let res = (a.0 + b.0, a.1 + b.1);
    // println!("res: {:?}", res);
    if (res.0 < 0 || res.0 >= lim_y as i32 || res.1 < 0 || res.1 >= lim_x as i32) {
        // println!("out of bounds");
        //if we are out of bounds, return the original position, it will never be the char we're looking for so it's effectively ignored
        return a;
    }
    res
}

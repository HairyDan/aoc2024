use std::cmp::min;
use std::collections::HashSet;
use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("src/day13/input.txt").expect("Unable to read file");
    let mut groups: Vec<&str> = input.split("\n\n").collect();

    let mut input_tuples = Vec::new();

    for group in groups {
        // let mut input_tuple = ((0,0),(0,0),(0,0));
        let mut clean_group: String = group
            .chars()
            .filter(|&c| c.is_digit(10) || c == ',' || c == '\n')
            .collect();
        // println!("{}", clean_group);
        clean_group = clean_group.trim().to_string();
        let mut lines = clean_group.lines();
        let button_a = lines.next().unwrap();
        let button_b = lines.next().unwrap();
        let answer = lines.next().unwrap();

        println!("button a {}", button_a);
        println!("button b {}", button_b);
        println!("answer {}", answer);

        let mut ba_split = button_a.split(",");
        let mut bb_split = button_b.split(",");
        let mut a_split = answer.split(",");
        input_tuples.push((
            (
                ba_split.next().unwrap().parse().unwrap(),
                ba_split.next().unwrap().parse().unwrap(),
            ),
            (
                bb_split.next().unwrap().parse().unwrap(),
                bb_split.next().unwrap().parse().unwrap(),
            ),
            (
                a_split.next().unwrap().parse().unwrap(),
                a_split.next().unwrap().parse().unwrap(),
            ),
        ))
    }

    println!("{:?}", input_tuples);

    let mut total = 0;

    for (i, (a, b, target)) in input_tuples.iter().enumerate() {
        let moves = reach_target(*a, *b, *target);
        if moves >= 0 {
            total += moves;
        }
    }

    println!("Total: {}", total);
}

pub fn run_part_2() {
    let input: String = fs::read_to_string("src/day13/input.txt").expect("Unable to read file");
    let mut groups: Vec<&str> = input.split("\n\n").collect();

    let mut input_tuples = Vec::new();

    for group in groups {
        // let mut input_tuple = ((0,0),(0,0),(0,0));
        let mut clean_group: String = group
            .chars()
            .filter(|&c| c.is_digit(10) || c == ',' || c == '\n')
            .collect();
        // println!("{}", clean_group);
        clean_group = clean_group.trim().to_string();
        let mut lines = clean_group.lines();
        let button_a = lines.next().unwrap();
        let button_b = lines.next().unwrap();
        let answer = lines.next().unwrap();

        println!("button a {}", button_a);
        println!("button b {}", button_b);
        println!("answer {}", answer);

        let mut ba_split = button_a.split(",");
        let mut bb_split = button_b.split(",");
        let mut a_split = answer.split(",");
        input_tuples.push((
            (
                ba_split.next().unwrap().parse::<i64>().unwrap(),
                ba_split.next().unwrap().parse::<i64>().unwrap(),
            ),
            (
                bb_split.next().unwrap().parse::<i64>().unwrap(),
                bb_split.next().unwrap().parse::<i64>().unwrap(),
            ),
            (
                a_split.next().unwrap().parse::<i64>().unwrap() + 10000000000000,
                a_split.next().unwrap().parse::<i64>().unwrap() + 10000000000000,
            ),
        ))
    }

    println!("{:?}", input_tuples);

    println!("there's {} tests", input_tuples.len());

    let mut total = 0;

    for (i, (a, b, target)) in input_tuples.iter().enumerate() {
        println!("test {} out of {}", i, input_tuples.len());
        let moves = solve_linear_system_integers(a.0, a.1, b.0, b.1, target.0, target.1);

        println!("moves {}", moves);
        if moves >= 0 {
            total += moves;
        }
    }

    println!("Total: {}", total);
}
//cramer's rule implementation done by chatgpt
fn solve_linear_system_integers(
    a_x: i64, a_y: i64,
    b_x: i64, b_y: i64,
    goal_x: i64, goal_y: i64,
) -> i64 {
    // Calculate the determinant of the matrix
    let det = a_x * b_y - a_y * b_x;

    // Check if the determinant is zero (no unique solution)
    if det == 0 {
        panic!("The determinant is zero, the system has no unique integer solution.");
    }

    // Calculate numerator terms for n_a and n_b using Cramer's rule
    let n_a_numer = goal_x * b_y - goal_y * b_x;
    let n_b_numer = a_x * goal_y - a_y * goal_x;

    // Check if the results are integers by ensuring the determinant divides the numerators
    if n_a_numer % det != 0 || n_b_numer % det != 0 {
        println!("No integer solutions exist for the given system.");
        return -1;
    }

    // Compute the integer values of n_a and n_b
    let n_a = n_a_numer / det;
    let n_b = n_b_numer / det;


    return 3*n_a + n_b
}

// fn reach_target_linear(
//     a: (i64, i64),
//     b: (i64, i64),
//     target: (i64, i64),
// ) -> i64 {
//     let (ax, ay) = a;
//     let (bx, by) = b;
//     let (tx, ty) = target;
//
//     // Extended Euclidean Algorithm to solve ax + by = gcd(a, b)
//     fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
//         if b == 0 {
//             (a, 1, 0)
//         } else {
//             let (gcd, x, y) = extended_gcd(b, a % b);
//             (gcd, y, x - (a / b) * y)
//         }
//     }
//
//     // Solve for X coordinates
//     let (gcd_x, x_coeff, b_coeff) = extended_gcd(ax, bx);
//     if tx % gcd_x != 0 {
//         return -1; // No solution for X
//     }
//
//     // Scale solution to target X
//     let scale_x = tx / gcd_x;
//     let mut m_x = x_coeff * scale_x;
//     let mut n_x = b_coeff * scale_x;
//
//     // Solve for Y coordinates
//     let (gcd_y, y_coeff, b_coeff_y) = extended_gcd(ay, by);
//     if ty % gcd_y != 0 {
//         return -1; // No solution for Y
//     }
//
//     // Scale solution to target Y
//     let scale_y = ty / gcd_y;
//     let mut m_y = y_coeff * scale_y;
//     let mut n_y = b_coeff_y * scale_y;
//
//     // Align solutions to ensure non-negativity
//     let step_x = bx / gcd_x;
//     let step_y = by / gcd_y;
//
//     while m_x < 0 || n_x < 0 {
//         println!("first while loop {} {}", m_x, n_x);
//         m_x += step_x;
//         n_x -= ax / gcd_x;
//     }
//
//     while m_y < 0 || n_y < 0 {
//         println!("second while loop");
//
//         m_y += step_y;
//         n_y -= ay / gcd_y;
//     }
//
//     // Calculate cost
//     let cost_x = 3 * m_x + n_x;
//     let cost_y = 3 * m_y + n_y;
//
//     // Return the minimum cost that satisfies both
//     if cost_x >= 0 && cost_y >= 0 {
//         cost_x.min(cost_y)
//     } else {
//         -1
//     }
// }


fn reach_target(
    a: (i64, i64),      // increment pair A
    b: (i64, i64),      // increment pair B
    target: (i64, i64), // target coordinates
) -> i64 {
    let mut visited = HashSet::new();
    let mut queue = vec![(0, 0, 0)]; // current_x, current_y, moves
    visited.insert((0, 0));

    let mut index = 0;
    while index < queue.len() {
        let (x, y, moves) = queue[index];

        if (x, y) == target {
            return moves;
        }

        if x > target.0 || y > target.1 {
            index += 1;
            continue;
        }

        // Try increment A
        let next_a = (x + a.0, y + a.1);
        if next_a.0 <= target.0 && next_a.1 <= target.1 && !visited.contains(&next_a) {
            queue.push((next_a.0, next_a.1, moves + 3));
            visited.insert(next_a);
        }

        // Try increment B
        let next_b = (x + b.0, y + b.1);
        if next_b.0 <= target.0 && next_b.1 <= target.1 && !visited.contains(&next_b) {
            queue.push((next_b.0, next_b.1, moves + 1));
            visited.insert(next_b);
        }

        index += 1;
    }

    -1
}

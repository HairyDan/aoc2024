use std::fs;

pub fn run(){
    let input: String = fs::read_to_string("src/day7/input.txt").expect("Unable to read file");
    let lines: Vec<&str> = input.split("\n").collect();

    let mut results: Vec<i64> = Vec::new();
    let mut inputs: Vec<Vec<i64>> = Vec::new();

    for line in lines {
        // Split line into two parts
        let parts: Vec<&str> = line.split(':').collect();
        results.push(parts[0].trim().parse::<i64>().unwrap());
        inputs.push(parts[1].trim().split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect());
    }

    println!("{:?}", results);
    println!("{:?}", inputs);

    let mut total_of_trues = 0;

    for (index, result) in results.iter().enumerate() {
        let opses = create_operator_permutations((inputs[index].len() - 1) as i64);
        for ops in opses.iter() {
            if do_maths(&inputs[index], ops) == *result {
                total_of_trues += result;
                break;
            }
        }

    }

    println!("{:?}", total_of_trues);


}


pub fn run_part_2(){
    let input: String = fs::read_to_string("src/day7/input.txt").expect("Unable to read file");
    let lines: Vec<&str> = input.split("\n").collect();

    let mut results: Vec<i64> = Vec::new();
    let mut inputs: Vec<Vec<i64>> = Vec::new();

    for line in lines {
        // Split line into two parts
        let parts: Vec<&str> = line.split(':').collect();
        results.push(parts[0].trim().parse::<i64>().unwrap());
        inputs.push(parts[1].trim().split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect());
    }

    // println!("{:?}", results);
    // println!("{:?}", inputs);

    let mut total_of_trues = 0;

    for (index, result) in results.iter().enumerate() {
        println!("row {}", index);
        let opses = create_operator_permutations_part_2((inputs[index].len() - 1) as i64);
        // println!("{:?}", opses);
        for ops in opses.iter() {
            if do_maths(&inputs[index], ops) == *result {
                total_of_trues += result;
                break;
            }
        }

    }

    println!("{:?}", total_of_trues);


}

fn create_operator_permutations(num_gaps: i64) -> Vec<Vec<char>> {
    let mut permutations: Vec<Vec<char>> = Vec::new();
    let plus: Vec<char> = vec!['+'];
    let mult: Vec<char> = vec!['*'];
    permutations.push(plus);
    permutations.push(mult);

    //for each gap, make 2 vecs, append both operators, increasing length by 1 and doubling each time
    while permutations[0].len() < num_gaps as usize {
        let mut temp_permutations: Vec<Vec<char>> = Vec::new();
        for perm in permutations.iter() {
            let mut current_perm_plus = perm.clone();
            let mut current_perm_mult = perm.clone();
            current_perm_plus.push('+');
            current_perm_mult.push('*');
            temp_permutations.push(current_perm_plus);
            temp_permutations.push(current_perm_mult);
        }
        permutations = temp_permutations;
    }

    permutations
}

fn create_operator_permutations_part_2(num_gaps: i64) -> Vec<Vec<char>> {
    let mut permutations: Vec<Vec<char>> = Vec::new();
    let plus: Vec<char> = vec!['+'];
    let mult: Vec<char> = vec!['*'];
    let conc: Vec<char> = vec!['|'];
    permutations.push(plus);
    permutations.push(mult);
    permutations.push(conc);

    //for each gap, make 3 vecs, append both operators, increasing length by 1 and tripling each time
    while permutations[0].len() < num_gaps as usize {
        let mut temp_permutations: Vec<Vec<char>> = Vec::new();
        for perm in permutations.iter() {
            let mut current_perm_plus = perm.clone();
            let mut current_perm_mult = perm.clone();
            let mut current_perm_conc = perm.clone();
            current_perm_plus.push('+');
            current_perm_mult.push('*');
            current_perm_conc.push('|');
            temp_permutations.push(current_perm_plus);
            temp_permutations.push(current_perm_mult);
            temp_permutations.push(current_perm_conc);
        }
        permutations = temp_permutations;
    }

    permutations
}

fn do_maths(inputs: &Vec<i64>, ops: &Vec<char>) -> i64 {
    let mut inputs_clon = inputs.clone();
    for in_pos in 0..(inputs_clon.len() - 1) {
        if ops[in_pos] == '+' {
            inputs_clon[in_pos+1] = inputs_clon[in_pos] + inputs_clon[in_pos + 1];
        } else if ops[in_pos] == '*' {
            inputs_clon[in_pos+1] = inputs_clon[in_pos] * inputs_clon[in_pos + 1];
        } else if ops[in_pos] == '|' {
            inputs_clon[in_pos+1] = (inputs_clon[in_pos].to_string() + &*inputs_clon[in_pos + 1].to_string()).parse::<i64>().unwrap();
        }
    }
    // println!("last in input array should now be answer:: {:?}", inputs_clon);
    inputs_clon[inputs_clon.len() - 1]
}
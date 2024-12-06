use std::cmp::Ordering;
use std::fs;

pub fn run() {
    let input: String = fs::read_to_string("src/day5/input.txt").expect("Unable to read file");

    if let Some((rules_str, pages_str)) = input.split_once("\n\n") {
        println!("{}", input);
        println!("RULES RULES RULES RULES {}", rules_str);
        println!("PAGES PAGES PAGES {}", pages_str);

        //convert rules to before//after tuple
        let rules: Vec<(String, String)> = rules_str
            .lines()
            .map(|line| {
                let mut parts = line.split('|');
                (parts.next().unwrap().to_string(), parts.next().unwrap().to_string())
            })
            .collect();


        let pages_vec: Vec<Vec<String>> = pages_str
            .lines()
            .map(|s| s.split(',').map(|item| item.to_string()).collect())
            .collect();

        let mut good_pages: Vec<Vec<String>> = Vec::new();

        for pages in pages_vec.iter() {
            let mut bad_counter = 0;
            for rule in rules.iter() {
                if pages.contains(&rule.0) && pages.contains(&rule.1) {
                    let index_0 = pages.iter().position(|x| x == &rule.0);
                    let index_1 = pages.iter().position(|x| x == &rule.1);
                    if (index_0 < index_1) {

                    } else {
                        //bad
                        bad_counter += 1;
                    }
                }
            }
            if bad_counter == 0 {
                good_pages.push(pages.clone());
            }
        }

        println!("{:?}", good_pages);

        let sum: i32 = good_pages.iter()
            .map(|inner_vec| {
                let middle_string = &inner_vec[inner_vec.len() / 2]; // Get the middle string
                middle_string.parse::<i32>().unwrap() // Convert to int and unwrap
            })
            .sum(); // Sum all the values


        println!("{}", sum);

    } else {
        println!("input fail");
    }
}

pub fn run_part_2() {
    let input: String = fs::read_to_string("src/day5/input.txt").expect("Unable to read file");

    if let Some((rules_str, pages_str)) = input.split_once("\n\n") {
        println!("{}", input);
        println!("RULES RULES RULES RULES {}", rules_str);
        println!("PAGES PAGES PAGES {}", pages_str);

        //convert rules to before//after tuple
        let rules: Vec<(String, String)> = rules_str
            .lines()
            .map(|line| {
                let mut parts = line.split('|');
                (parts.next().unwrap().to_string(), parts.next().unwrap().to_string())
            })
            .collect();


        let pages_vec: Vec<Vec<String>> = pages_str
            .lines()
            .map(|s| s.split(',').map(|item| item.to_string()).collect())
            .collect();

        let mut bad_pages: Vec<Vec<String>> = Vec::new();

        for pages in pages_vec.iter() {
            let mut bad_counter = 0;
            for rule in rules.iter() {
                if pages.contains(&rule.0) && pages.contains(&rule.1) {
                    let index_0 = pages.iter().position(|x| x == &rule.0);
                    let index_1 = pages.iter().position(|x| x == &rule.1);
                    if (index_0 < index_1) {

                    } else {
                        //bad
                        bad_counter += 1;

                    }
                }
            }
            if bad_counter != 0 {
                bad_pages.push(pages.clone());
            }
        }

        println!("bad pages {:?}", bad_pages);

        let mut fix_bad_pages : Vec<Vec<String>> = Vec::new();
        for bad_page in bad_pages.iter() {
            let mut fix_bad_page = bad_page.clone();
            fix_bad_page.sort_by(|a, b| compare_strings(&&**a, &&**b, &rules));
            fix_bad_pages.push(fix_bad_page);
        }

        println!("bad pages fixed 1 {:?}", fix_bad_pages);

        let sum: i32 = fix_bad_pages.iter()
            .map(|inner_vec| {
                let middle_string = &inner_vec[inner_vec.len() / 2];
                middle_string.parse::<i32>().unwrap()
            })
            .sum();

        println!("bad pages fixed 2 {:?}", fix_bad_pages);

        println!("{}", sum);

    } else {
        println!("input fail");
    }
}

fn compare_strings(a: &&str, b: &&str, rules: &[(String, String)]) -> Ordering {
    if rules.iter().any(|(x, y)| x == *a && y == *b) {
        return Ordering::Less; // a should come before b
    }

    if rules.iter().any(|(x, y)| x == *b && y == *a) {
        return Ordering::Greater; // b should come before a
    }

    Ordering::Equal
}

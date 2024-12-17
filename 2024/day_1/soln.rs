use std::fs;

fn main() {
    let file = "input.txt";

    let input: String = fs::read_to_string(file)
        .expect("Unable to read input file");

    let lines: Vec<&str> = input.split('\n').collect(); 

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue
        }

        let mut iter = line.split_ascii_whitespace();

        left_list.push(iter.next().expect("No left element").parse::<i32>().unwrap());
        right_list.push(iter.next().expect("No right element").parse::<i32>().unwrap());
    }

    part_one_solution(&mut left_list.clone(), &mut right_list.clone());

    part_two_solution(left_list.clone(), right_list.clone());
}

fn part_one_solution(left_list: &mut Vec<i32>, right_list: &mut Vec<i32>) {
    left_list.sort();
    right_list.sort();
    
    let num_elements = left_list.len();
    let mut i = 0;
    let mut result = 0;

    while i < num_elements {
        result += i32::abs(left_list[i] - right_list[i]);
        i +=  1;
    }

    println!("Answer for part 1 is {result}")
}

fn part_two_solution(left_list: Vec<i32>, right_list: Vec<i32>) {
    let mut similarity_score = 0;
    
    for m in left_list {
        let right_list_clone = right_list.clone();
        for n in right_list_clone {
            if m == n {
                similarity_score += m;
            }
        }
    }

    println!("Answer for part 2 is {similarity_score}");
}

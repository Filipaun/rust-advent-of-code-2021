use std::fs;

pub fn day1_fn(_debug: bool) {
    let numbers: Vec<i32> = read_file();

    day1_part1(&numbers,_debug);
    day1_part2(&numbers,_debug);
}

fn read_file() -> Vec<i32> {
    let contents = fs::read_to_string("src/days/inputs/day1_input.txt")
        .expect("Unable to read file");
    //println!("{}",contents);
    let numbers: Vec<i32> = contents.split("\n")
                                    .filter_map(|s| s.parse::<i32>().ok())
                                    .collect();
    return numbers;
}

fn day1_part1(numbers: &Vec<i32>, _debug: bool) -> () {
    let mut bigger = 0;
    for n in 1..numbers.len() {
        if numbers[n] > numbers[n-1] {
            bigger += 1;
        }
    }
    println!("Entries bigger than last {}", bigger);
}

fn day1_part2(numbers: &Vec<i32>, _debug: bool) -> () {
    let mut bigger = 0;
    let mut current_tri : i32;
    let mut last_tri = i32::MAX;

    for n in 1..(numbers.len()-1) {
        current_tri = numbers[n-1]+numbers[n]+numbers[n+1];
        if current_tri > last_tri {
            bigger += 1;
        }
        last_tri = current_tri;
    }
    println!("Three-measurement bigger than last {}", bigger);
}
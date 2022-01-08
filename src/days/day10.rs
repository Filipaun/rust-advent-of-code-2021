use std::fs;

pub fn day10_fn(_debug : bool) {
    let data = read_file(_debug);
    day10_part1(&data);
    day10_part2(&data);

}

fn read_file(_debug : bool) -> Vec<String> {
    let contents =
        if _debug == true {
            fs::read_to_string("src/days/inputs/day10_test_input.txt")
                                    .expect("Unable to read file")
        } else {
            fs::read_to_string("src/days/inputs/day10_input.txt")
                                    .expect("Unable to read file")
        };

    //println!("{}",contents);
    let values: Vec<String> =  contents.split("\n")
                                        .map(|s| s.trim().to_owned())
                                        .collect();
    return values;
}

fn matching_character(current_char : char) -> (bool,char) {
    match current_char {
        '<' => return (true,'>'),
        '(' => return (true,')'),
        '{' => return (true,'}'),
        '[' => return (true,']'),

        '>' => return (false,'<'),
        ')' => return (false,'('),
        '}' => return (false,'{'),
        ']' => return (false,'['),
        _ => return (false,'.'),
    };
}

fn error_score_table(current_char : char) -> u64 {
    match current_char {
        ')' => return 3,
        ']' => return 57,
        '}' => return 1197,
        '>' => return 25137,
        _   => return 0,
    };
}

fn completion_score_table(current_char : char) -> u64 {
    match current_char {
        ')' => return 1,
        ']' => return 2,
        '}' => return 3,
        '>' => return 4,
        _   => return 0,
    }
}
 
fn generate_completion_string(active_characters : Vec<char>) -> String {
    let completion_string = active_characters.iter().rev().map(|s| matching_character(*s).1).collect::<String>();
    return completion_string;
}

fn day10_part1(data : &Vec<String>) {
    println!("PART 1:");

    let mut error_score = 0;
    let mut active_characters : Vec<char>;

    // Check validity of each line
    'outer : for line in data {
        // Reset active characters vector for each line
        active_characters = Vec::new();
        for current_char in line.chars() {
            // Find matching character and also determine type
            let matching_char = matching_character(current_char);

            // Adds 'beggining' bracket to active vec
            if matching_char.0 {
                active_characters.push(current_char);
            // Compares 'ending' bracket to latest 'beginning' char
            } else {
                let expected_opt = active_characters.pop();
                if expected_opt == None {
                    error_score += error_score_table(current_char);
                    println!("Expected opener, found: {}",current_char);
                    continue 'outer;
                }
                else if expected_opt.unwrap() == matching_char.1 {
                    continue
                } else {
                    error_score += error_score_table(current_char);
                    println!("Expected {}, found {}",matching_character(expected_opt.unwrap()).1,current_char);
                    continue 'outer;
                }
            }
        }
    }
    println!("Total error score: {}",error_score);
}

fn day10_part2(data : &Vec<String>) {

    println!("PART 2:");

    let mut completion_score_vector  : Vec<u64> = Vec::new();
    let mut active_characters : Vec<char>;

    // Check validity of each line
    'outer : for line in data {
        // Reset active characters vector for each line
        active_characters = Vec::new();
        for current_char in line.chars() {
            // Find matching character and also determine type
            let matching_char = matching_character(current_char);

            // Adds 'beggining' bracket to active vec
            if matching_char.0 {
                active_characters.push(current_char);
            // Compares 'ending' bracket to latest 'beginning' char
            } else {
                let expected_opt = active_characters.pop();
                if expected_opt == None {
                    continue 'outer;
                }
                else if expected_opt.unwrap() == matching_char.1 {
                    continue
                } else {
                    continue 'outer;
                }
            }
        }
        let completion_string = generate_completion_string(active_characters);
        let mut completion_score : u64 = 0;
        for current_char in completion_string.chars() {
            completion_score *= 5;
            completion_score += completion_score_table(current_char); 
        }
        println!("Completion string: {}, score: {}",completion_string, completion_score);
        completion_score_vector.push(completion_score);
    }
    completion_score_vector.sort();

    println!("Middle completion score: {}", completion_score_vector[completion_score_vector.len()/2]);
}
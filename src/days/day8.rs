use std::fs;
use std::collections::HashMap;

pub fn day8_fn(_debug: bool) {
    let signals:Vec<Vec<Vec<Vec<u32>>>> = read_file(false);

    day8_part1(&signals,_debug);
    day8_part2(&signals,_debug);

}


fn sum_i32(vector : &Vec<u32>) -> u32 {
    let mut sum_ : u32 = 0;

    for value in vector.iter() {
        sum_ += value;
    };
    return sum_;
}

fn read_file(_debug: bool)
 -> Vec<Vec<Vec<Vec<u32>>>> {
    
    let contents =
        if _debug == true {
            fs::read_to_string("src/days/inputs/day8_test_input.txt")
                                    .expect("Unable to read file")
        } else {
            fs::read_to_string("src/days/inputs/day8_input.txt")
                                    .expect("Unable to read file")
        };

    //println!("{}",contents);
    let signals: Vec<Vec<Vec<Vec<u32>>>> =  contents.split("\n")
                                        .map(|s| s.split(" | ")
                                                .map(|ss| ss.split(" ")
                                                            .map(|sss| sss.trim().chars()
                                                                                // map each letter to unqiue power of 2
                                                                                .map(|c| u32::pow(2,c as u32-97))
                                                                                .collect())
                                                            .collect())
                                                .collect())
                                        .collect();
    return signals;
}

fn day8_part1(signals: &Vec<Vec<Vec<Vec<u32>>>>, _debug: bool) {

    let mut n_unqiue_signal_patterns = 0;

    for signal in signals.iter() {
        for signal_pattern in signal[1].iter() {
            let signal_pattern_length = signal_pattern.len();
            if signal_pattern_length == 2 || signal_pattern_length == 3 || signal_pattern_length == 4 || signal_pattern_length == 7 {
                n_unqiue_signal_patterns += 1;
                //println!("Signal pattern : {:?}",signal_pattern)
            };
        }
    }
    
    println!("Number of 1,4,7 and 8: {}", n_unqiue_signal_patterns)
}

fn day8_part2(signals: &Vec<Vec<Vec<Vec<u32>>>>, _debug: bool){
    let full_sum = 1 + 2 + 4 + 8 + 16 + 32 + 64;
    let mut number_map = HashMap::new();
    let mut segment_map = HashMap::new();

    for n in 0..10 {
        number_map.insert(n,(0,0));
    }

    segment_map.insert("c",0);
    segment_map.insert("f",0);
    segment_map.insert("e",0);
    segment_map.insert("d",0);

    let mut all_outputs = vec![0;signals.len()];
    let mut output;

    for (j,signal) in signals.iter().enumerate() {
        output = 0;
        let mut signal_pattern_length;

        // find signal pattern of the unqiue segument amount numbers 
        for (i,signal_pattern) in signal[0].iter().enumerate() {
            signal_pattern_length = signal_pattern.len();
            match signal_pattern_length {
                2 => number_map.insert(1,(i,sum_i32(signal_pattern))),

                3 => number_map.insert(7,(i,sum_i32(signal_pattern))),

                4 => number_map.insert(4,(i,sum_i32(signal_pattern))),

                7 =>number_map.insert(8,(i,sum_i32(signal_pattern))),

                _ => None,
            };
        }
        // find segment "a" 
        segment_map.insert("a",number_map.get(&7).unwrap().1-number_map.get(&1).unwrap().1);

        // find 6 and "c"
        for (i,signal_pattern) in signal[0].iter().enumerate() {
            signal_pattern_length = signal_pattern.len();
            let value_1 = signal_pattern.iter().position(|x| x == &signal[0][number_map.get(&1).unwrap().0][0]);
            let value_2 = signal_pattern.iter().position(|x| x == &signal[0][number_map.get(&1).unwrap().0][1]);

            match signal_pattern_length {
                6 => {
                    if value_1 != None && value_2 == None {
                        number_map.insert(6,(i,sum_i32(signal_pattern)));
                        segment_map.insert("c",signal[0][number_map.get(&1).unwrap().0][1]);
                    }
                    else if value_1 == None && value_2 != None {
                        number_map.insert(6,(i,sum_i32(signal_pattern)));
                        segment_map.insert("c",signal[0][number_map.get(&1).unwrap().0][0]);
                    }
                },
                _ => (),
            }
        }
        // find "f"
        segment_map.insert("f",number_map.get(&1).unwrap().1-segment_map.get("c").unwrap());

        for (i,signal_pattern) in signal[0].iter().enumerate() {
            signal_pattern_length = signal_pattern.len();
            match signal_pattern_length {
                5 => {
                    // find 5
                    if signal_pattern.iter().position(|x| x == segment_map.get("c").unwrap()) == None {
                        let temp_sum = sum_i32(signal_pattern);
                        number_map.insert(5,(i,temp_sum));

                        segment_map.insert("e",full_sum - temp_sum-segment_map.get("c").unwrap());
                    // find 2
                    } else if signal_pattern.iter().position(|x| x == segment_map.get("f").unwrap()) == None {
                        number_map.insert(2,(i,sum_i32(signal_pattern)));
                    };
                },
                _ => (),
            }
        }
        
        for (i,signal_pattern) in signal[0].iter().enumerate() {
            signal_pattern_length = signal_pattern.len();
            match signal_pattern_length {
                // find 3
                5 => {
                    let temp_sum = sum_i32(signal_pattern);
                    if temp_sum != number_map.get(&2).unwrap().1 && temp_sum != number_map.get(&5).unwrap().1 {
                        number_map.insert(3,(i,temp_sum));
                    }
                },
                // find 9
                6 => {
                    if signal_pattern.iter().position(|x| x == segment_map.get("e").unwrap()) == None {
                        number_map.insert(9,(i,sum_i32(signal_pattern)));
                    }
                },
                _ => (),
            };
        }
        // find 0
        for (i,signal_pattern) in signal[0].iter().enumerate() {
            signal_pattern_length = signal_pattern.len();
            match signal_pattern_length {
                6 => {
                    let temp_sum = sum_i32(signal_pattern);
                    if temp_sum != number_map.get(&6).unwrap().1 && temp_sum != number_map.get(&9).unwrap().1 {
                        number_map.insert(0,(i,temp_sum));
                    }
                },
                _ => (),
            };
        }

        // match sums to find corrresponding number 
        for (i,output_digit) in signal[1].iter().rev().enumerate() {
            let number_sum = sum_i32(output_digit);
            for (key, val) in number_map.iter() {
                if number_sum == val.1 {
                    output += key*u32::pow(10,(i) as u32);
                }
            }
        }
        all_outputs[j] = output;
    }
    //println!("{:?}",all_outputs);
    println!("Sum of all output numbers : {}",sum_i32(&all_outputs))
}

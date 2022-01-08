use std::fs;
use std::collections::HashMap;

pub fn day5_fn(_debug : bool) {
    let data = read_file(true);
    println!("{:?}",data);

    day5_part1(&data);

}

fn read_file(_debug : bool) -> Vec<Vec<[u32;2]>> {
    let contents =
        if _debug == true {
            fs::read_to_string("src/days/inputs/day5_test_input.txt")
                                    .expect("Unable to read file")
        } else {
            fs::read_to_string("src/days/inputs/day5_input.txt")
                                    .expect("Unable to read file")
        };

    let mut values: Vec<Vec<[u32;2]>> =  contents.split("\n")
                                        .map(|s| s.trim().split(" -> ")
                                                .map(|t| 
                                                    [t.chars().nth(0).unwrap().to_digit(10).unwrap(),t.chars().nth(2).unwrap().to_digit(10).unwrap()])
                                                .collect())
                                        .collect();
    values.iter_mut().for_each(|v| v.sort());
    return values;
}

struct Line {
    a : (u32,u32),
    b : (u32,u32),
}

fn inv(int : usize) -> usize {
    match int {
        0 => return 1,
        1 => return 0,
        _ => return 0,
    }
}

fn day5_part1(data : &Vec<Vec<[u32;2]>>) {
    //let mut grid = gen_grid(&data);
    let mut overlaps : HashMap::<[u32;2],u32> = HashMap::new();

    for i in 0..data.len() {
        'outer : for j in (i+1)..data.len() {
            for k in 0..2 {
                if (data[i][0][k]== data[i][1][k]) && (data[j][0][k] == data[j][1][k]) && (data[i][0][k] == data[j][0][k]) {
                    println!("bro");
                    let mut inter = [0u32;2];
                    let mut values = vec![data[i][0][inv(k)],data[i][1][inv(k)],data[j][0][inv(k)],data[j][1][inv(k)]]; 
                    values.sort();
                    for l in values[1]..values[2]+1 {
                        inter[k] = data[i][0][k];
                        inter[inv(k)] = l;
                        *overlaps.entry(inter).or_insert(0) += 1;
                    }
                    continue 'outer;
                } else{
                    
                }
            }
        }
    }
    println!("{:?}",overlaps);
    println!("{}",overlaps.len());
}
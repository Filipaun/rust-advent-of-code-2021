use std::fs;

pub fn day9_fn(_debug: bool) {
    let input_values = read_file(false);

    //day9_part1(input_values);
    day9_part2(input_values);
}

fn read_file(_debug: bool) -> Vec<Vec<u32>> {
    let contents =
        if _debug == true {
            fs::read_to_string("src/days/inputs/day9_test_input.txt")
                                    .expect("Unable to read file")
        } else {
            fs::read_to_string("src/days/inputs/day9_input.txt")
                                    .expect("Unable to read file")
        };

    //println!("{}",contents);
    let values: Vec<Vec<u32>> =  contents.split("\n")
                                          .map(|s| s.trim().chars()
                                                .filter_map(|c| c.to_digit(10))
                                                .collect())
                                          .collect();
    return values;
}


fn shifted_coords(pos: (usize,usize), neighbour : usize) -> (usize,usize) {
    match neighbour {
        0 => return (pos.0-1,pos.1),
        1 => return (pos.0+1,pos.1),
        2 => return (pos.0,pos.1-1),
        3 => return (pos.0,pos.1+1),
        _ => return (0,0),
    }
}

fn relative_location(i: usize) -> usize {
    match i {
        0 => return 1,
        1 => return 0,
        2 => return 3,
        3 => return 2,
        _ => return 4,
    }
}
#[derive(Copy, Clone,Debug)]
struct Node {
    pos : (usize,usize),
    value : u32,
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.pos == other.pos
    }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(pos: {:?}, val: {})", self.pos, self.value)
    }
}

struct Heightmap {
    values : Vec<Vec<u32>>,
    size_y : usize,
    size_x : usize,
    //local_minima : Vec<( u32,(usize,usize))>
}

impl Heightmap {
    fn get_value(&self, x: usize, y: usize) -> u32 {

            //println!("GETVALUE x: {},y : {}", x, y);
            return self.values[y][x];
    }

    fn get_neighbour_values(&self, pos: (usize,usize)) -> Vec<u32> {
        let mut neighbour_values = vec![10,10,10,10];

        //println!("x: {}, y: {}",x,y);
        
        if pos.0 == 0 {
            neighbour_values[1] = self.get_value(pos.0+1,pos.1);
        } else if pos.0 == self.size_x - 1 {
            neighbour_values[0] = self.get_value(pos.0-1,pos.1);
        } else {
            neighbour_values[0] = self.get_value(pos.0-1,pos.1);
            neighbour_values[1] = self.get_value(pos.0+1,pos.1);
        };

        if pos.1 == 0 {
            neighbour_values[3] = self.get_value(pos.0,pos.1+1);
        } else if pos.1 == self.size_y - 1 {
            neighbour_values[2] = self.get_value(pos.0,pos.1-1);
        } else {
            neighbour_values[2] = self.get_value(pos.0,pos.1-1);
            neighbour_values[3] = self.get_value(pos.0,pos.1+1);
        };
        return neighbour_values;
    }

    fn get_risk_level_sum(&self,local_minima: Vec<Node>) -> u32 {
        let mut risk_level_sum : u32 = 0;

        for node in local_minima {
            risk_level_sum += node.value+1;
        }

        return risk_level_sum;
    }

    fn get_local_minima(&self) -> Vec<Node> {

        let mut local_minima : Vec<Node> = Vec::new();
        let mut neighbour_values: Vec<u32>;

        for (y,sub_array) in self.values.iter().enumerate() {
            'outer: for (x,value) in sub_array.iter().enumerate() {
                neighbour_values = self.get_neighbour_values((x,y));

                for n_value  in neighbour_values.iter() {
                    if n_value <= value {
                        continue 'outer;
                    }
                }
                //println!("local minima, x: {},y: {}, value: {}, neighbours : {:?}", x, y, value, neighbour_values);
                local_minima.push(Node{pos:(x,y),value:*value});
            }
        }
        return local_minima;
    }
    fn determine_basin(&self, local_minimum : Node) -> u32 {
        let mut basin_size : u32  = 0;
        let mut current_loc : Node;
        let mut basin_frontier : Vec<Node> = vec![local_minimum];
        let mut basin_nodes : Vec<Node> = Vec::new();
        let mut neighbour_values : Vec<u32>;
        let mut basin_bool : bool;

        loop {
            if basin_frontier.len() == 0 {
                break;
            }
            current_loc = basin_frontier.remove(0);
            basin_bool = true;

            //println!("x,y: {:?}", current_loc.pos);

            neighbour_values = self.get_neighbour_values(current_loc.pos);

            for (i,n_value) in neighbour_values.iter().enumerate() {
                if !(*n_value > current_loc.value || (*n_value < 10)) {
                    basin_bool = false;
                    //println!("neighbour pos: {:?}, current basin:  {:?}",shifted_coords(current_loc.pos, i),basin_nodes);
                }
            }
            if basin_bool {
                basin_size += 1;
                //println!("basin");
                for i in 0..4 {
                    if neighbour_values[i] < 9 &&  basin_nodes.iter().find(|x| x.pos == shifted_coords(current_loc.pos, i)) == None {
                        let new_loc = Node{pos: shifted_coords(current_loc.pos,i),value : neighbour_values[i]};
                        if basin_frontier.iter().find(|x| x == &&new_loc) == None {
                            basin_frontier.push(new_loc); 
                        }
                    }
                    
                }
                basin_nodes.push(current_loc);   
            }
            
        }
        //println!("{}",basin_size);
        return basin_size;
    }
}



fn day9_part1(input_values : Vec<Vec<u32>>) {

    let size_w = input_values[0].len();
    let size_h = input_values.len();

    let heightmap = Heightmap {
        values : input_values,
        size_y : size_h,
        size_x : size_w,
    };

    let local_minima = heightmap.get_local_minima();
    let risk = heightmap.get_risk_level_sum(local_minima);
    println!("{}", risk);
}

fn day9_part2(input_values : Vec<Vec<u32>>) {

    let size_w = input_values[0].len();
    let size_h = input_values.len();

    let heightmap = Heightmap {
        values : input_values,
        size_y : size_h,
        size_x : size_w,
    };

    let local_minima = heightmap.get_local_minima();
    let mut basin_sizes = vec![0;local_minima.len()];

    for (i,local_minimum) in local_minima.iter().enumerate() {
        basin_sizes[i] = heightmap.determine_basin(*local_minimum);
    }
    basin_sizes.sort();
    let three_big_prod = basin_sizes[basin_sizes.len()-1]*basin_sizes[basin_sizes.len()-2]*basin_sizes[basin_sizes.len()-3];
    println!("basins: {:?}, 3 largest product: {}",basin_sizes,three_big_prod);
}
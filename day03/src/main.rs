use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input.txt file");
    let part_one_res = part_one(&input);
    println!("Part one solution: {:?}", part_one_res);

    let part_two_res = part_two(&input);
    println!("Part two solution: {:?}", part_two_res);
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn part_one(input: &str) -> i32 {
    let wires: Vec<Vec<&str>> = input.lines().map(|line| line.split(",").collect()).collect();
    let mut points = HashMap::new();
    let mut curr_wire_id: u32 = 0;
    let mut min_distance: i32 = 0;

    for wire in wires {
        curr_wire_id += 1;
        let mut wire_point = Point { x: 0, y: 0, };
        for movement in wire {
            let distance = *&movement[1..].parse::<i32>().unwrap();
            for _ in 0..distance {
                match &movement[0..1] {
                    "R" => wire_point.x += 1,
                    "L" => wire_point.x -= 1,
                    "U" => wire_point.y += 1,
                    "D" => wire_point.y -= 1,
                    _ => panic!("Invalid movement: {:?}", &movement[0..1])
                }

                if let Some(wire_id) = points.get(&wire_point) {
                    let distance = i32::abs(wire_point.y) + i32::abs(wire_point.x);
                    if *wire_id != curr_wire_id && (distance < min_distance || min_distance == 0) {
                        min_distance = distance;
                    }
                }

                points.insert(wire_point.clone(), curr_wire_id);
            }
        }
    } 
    return min_distance;
}


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct WireInfo {
    travel_distance: u32,
    wire_id: u32
}

impl WireInfo {
    pub fn new(wire_id: u32) -> WireInfo {
        WireInfo { travel_distance: 0, wire_id: wire_id, }
    }
}

fn part_two(input: &str) -> u32 {
    let wires: Vec<Vec<&str>> = input.lines().map(|line| line.split(",").collect()).collect();
    let mut points: HashMap<Point, WireInfo> = HashMap::new();
    let mut curr_wire_id: u32 = 0;
    let mut min_length: u32 = 0;

    for wire in wires {
        curr_wire_id += 1;
        let mut curr_wire_info = WireInfo::new(curr_wire_id);
        let mut wire_point = Point { x: 0, y: 0, };
        for movement in wire {
            let distance = *&movement[1..].parse::<i32>().unwrap();
            for _ in 0..distance {
                match &movement[0..1] {
                    "R" => wire_point.x += 1,
                    "L" => wire_point.x -= 1,
                    "U" => wire_point.y += 1,
                    "D" => wire_point.y -= 1,
                    _ => panic!("Invalid movement: {:?}", &movement[0..1])
                }

                curr_wire_info.travel_distance += 1;
                if let Some(wire_info) = points.get(&wire_point) {
                    if (*wire_info).wire_id != curr_wire_id {
                            let total_length = (*wire_info).travel_distance + curr_wire_info.travel_distance;
                            if total_length < min_length || min_length == 0 {
                            min_length = total_length;
                        }
                    }
                }
                points.insert(wire_point.clone(), curr_wire_info.clone());
            }
        }
    } 
    return min_length;
}
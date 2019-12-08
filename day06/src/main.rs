use std::fs;
use std::collections::HashMap;
use std::error::Error;

fn main() {
    let input = fs::read_to_string("input.txt").expect("error reading input.txt");
    let part_one_solution = part_one(&input); //Array brute force (very bad solution...) O(n!)
    println!("Part one solution {:?}", part_one_solution);

    let part_two_solution = part_two(&input); //Hash Map list (much better solution!) O(2n) 
    println!("Part two solution {:?}", part_two_solution);
}

#[derive(Debug)]
struct Planet<'a> {
    name: &'a str,
    orbiting: &'a str
}

impl<'a> Planet<'a> {
    pub fn new(str_planet: &'a str) -> Self {
        let arr_str_planet: Vec<&str> = str_planet.split(")").collect();
        Self {
            name: arr_str_planet[1],
            orbiting: arr_str_planet[0]
        }
    }
}

//naive array brute force solution (see part_two for the better, much faster, implementation)
fn part_one(input: &str) -> u32 {
    let planets: Vec<Planet> = input.lines()
                                    .map(|str_planet| Planet::new(str_planet))
                                    .collect();
    let mut total = 0;
    for target in &planets {
        let mut curr_planet = target;
        while curr_planet.orbiting != "COM" {
            //find the next planet that the current one is orbiting
            for planet in &planets {
                if planet.name == curr_planet.orbiting {
                    curr_planet = planet;
                    total += 1;
                    break;
                }
            }
        }
        //Increment here because COM also counts as a traverse
        total += 1;
    }
    return total;
}

//Solution using hash maps
fn part_two(input: &str) -> Result<i32, &str> {
    let planets: Vec<Planet> = input.lines()
                                    .map(|str_planet| Planet::new(str_planet))
                                    .collect();

    let mut hashmap_planets: HashMap<&str, &str> = HashMap::new();
    for planet in &planets { 
        hashmap_planets.insert(planet.name, planet.orbiting);
    }
    
    //-1 Because The distance from YOU/SAN to next planet does not count as a traverse
    let mut total: i32 = -1; 
    let mut santa_positions: HashMap<&str, i32> = HashMap::new(); //k: planet, v: depth 
    for mut curr_planet in ["SAN", "YOU"].iter() {
        while *hashmap_planets.get(curr_planet).unwrap() != "COM" {
            total += 1;
            curr_planet = hashmap_planets.get(curr_planet).unwrap();
            if santa_positions.contains_key(curr_planet) {
                return Result::Ok(santa_positions.get(curr_planet).unwrap() + total);
            }
            //Push our position each time to a hash map
            santa_positions.insert(curr_planet, total);
        }
        total = -1;
    }
    return Err(&"Could not find a path from YOU to SAN");
}


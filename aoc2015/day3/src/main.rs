use std::fs;
use std::collections::HashSet;
use clap::{Arg,App};

struct House {
    x: i32,
    y: i32,
}

impl House {
    fn new_house(_x: i32, _y:i32) -> House {
        House {
            x: _x,
            y: _y
        }
    }

    fn next_house(&mut self, direction: char) {
        match direction {
            '^' => self.y = self.y + 1,
            '>' => self.x = self.x + 1,
            'v' => self.y = self.y - 1,
            '<' => self.x = self.x - 1,
             _  => println!("weird entry!"),
        };
    }

    fn print_house(&self) {
        println!("location ({}, {})", self.x, self.y);
    }   
}

fn main() {
    let argument_set = App::new("AOC: Day 3")
                            .arg(Arg::with_name("input")
                                .short("f")
                                .long("input-file")
                                .takes_value(true)
                                .required(true))
                            .get_matches();
    let input = argument_set.value_of("input").unwrap();
    let code: String = fs::read_to_string(input)
                          .expect("unable to open");
    //println!("{}", code);

    // assume Santa starts at the origin
    let mut current_house = House::new_house(0, 0);
    current_house.print_house();
    
    // let's build a HashSet of all houses Santa is going to visit:
    let mut visited_houses = HashSet::new();
    visited_houses.insert((0,0));

    for direction in code.chars() {
        println!("Direction is {}", direction);
        current_house.next_house(direction);
        current_house.print_house();
        visited_houses.insert((current_house.x, current_house.y));
    }
    println!("Santa visited the following houses: {:?}, a total of {} houses", visited_houses, visited_houses.len());

    // part 2: we need to document houses for 2 Santas now
    let mut current_house_santa = House::new_house(0,0);
    let mut current_house_robosanta = House::new_house(0,0);

    let mut visited_houses_both = HashSet::new();
    visited_houses_both.insert((0,0));

    let mut counter:u32 = 0;
    for direction in code.chars() {
        counter = counter + 1;
        match counter % 2 {
            1 => {
                    current_house_santa.next_house(direction);
                    println!("Santa is at:");
                    current_house_santa.print_house();
                    visited_houses_both.insert((current_house_santa.x, current_house_santa.y));
                 }
            _ => {
                    current_house_robosanta.next_house(direction);
                    println!("Robo Santa is at:");
                    current_house_robosanta.print_house();
                    visited_houses_both.insert((current_house_robosanta.x, 
                                                     current_house_robosanta.y));                 
                 }
        }
    }
    println!("Total houses visited: {}", visited_houses_both.len()); 
}

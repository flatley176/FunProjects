use std::fs;
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
            //'^' => House { x: self.x, y: self.y + 1},
            //'>' => House { x: self.x + 1, y: self.y},
            //'v' => House { x: self.x, y: self.y - 1},
            //'<' => House { x: self.x - 1, y: self.y},
            // _  => House { x: -100, y: -100},
            '^' => self.y = self.y + 1,
            '>' => self.x = self.x + 1,
            'v' => self.y = self.y - 1,
            '<' => self.x = self.x - 1,
             _  => println!("weird entry!"),
        };
        
    }

    fn print_house(&self) {
        println!("Santa's now at ({}, {})", self.x, self.y);
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

    for direction in code.chars() {
        println!("Direction is {}", direction);
        current_house.next_house(direction);
        current_house.print_house();
    }
}

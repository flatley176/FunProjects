use std::fs;
use clap::{Arg,App};

fn main() {
    let argument_set = App::new("AOC: Day 1")
                            .arg(Arg::with_name("input")
                                .short("f")
                                .long("input-file")
                                .takes_value(true)
                                .required(true))
                            .get_matches();

    let input = argument_set.value_of("input").unwrap();
    let code: String = fs::read_to_string(input)
                          .expect("unable to open");
    let cur_floor = get_current_floor(code); 
    println!("Floor is {}", cur_floor); // this cannot be formatted with default formatter if the
                                        // return type is not explicitly defined in the function 
}

fn get_current_floor(code : String) -> i32 {
    let mut floor: i32 = 0; // floor has to be mut to increment or decrement
    for c in code.chars() { // maybe there's a select case way to do this that I don't know about
        if c == '(' {
            floor = floor + 1;
        } else if c == ')' {
            floor = floor - 1;
        }
    }
   return floor;
}

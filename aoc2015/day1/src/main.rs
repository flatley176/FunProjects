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
    let (cur_floor, first_negative_floor_index) = get_current_floor(code); 
    println!("Floor is {}; first negative floor index is {}", 
                cur_floor, 
                first_negative_floor_index); // this cannot be formatted with default formatter if 
                                        // return type is not explicitly defined in the function 
}

fn get_current_floor(code : String) -> (i32, i32) {
    let mut final_floor: i32 = 0; // floor has to be mut to increment or decrement
    let mut current_floor_index: i32 = 0;
    let mut first_negative_floor_index: i32 = 0;
    for c in code.chars() { // maybe there's a select case way to do this that I don't know about
        current_floor_index = current_floor_index + 1;
        if c == '(' {
            final_floor = final_floor + 1;
        } else if c == ')' {
            final_floor = final_floor - 1;
        }
        if final_floor < 0 && first_negative_floor_index == 0 {
            first_negative_floor_index = current_floor_index
        }
    }
   return (final_floor, first_negative_floor_index);
}

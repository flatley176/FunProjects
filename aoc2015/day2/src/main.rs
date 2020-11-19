use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use clap::{Arg,App};


struct ElfBox {
    sides: [u32; 3]
}

impl ElfBox {
    // static method, initializing with a sorted dimension array
    fn new_box(a: u32, b: u32, c: u32) -> ElfBox {
        let mut _tempbox = ElfBox { 
            sides: [a, b, c]
        };
        _tempbox.sides.sort();
        _tempbox
    }

    fn get_total_surface_area(&self) -> u32 {
        let surface_area: u32 = 2 * (self.sides[0] * self.sides[1] +
                                     self.sides[1] * self.sides[2] +
                                     self.sides[2] * self.sides[0]);   
       surface_area 
    }

    fn get_volume(&self) -> u32 {
        let volume: u32 = self.sides[0] * self.sides[1] * self.sides[2];
        volume
    }
    
    // sorting => the first two values are always going to be the smallest 2D/3D units
    fn get_smallest_area(&self) -> u32 {
        let smallest_area: u32 = self.sides[0] * self.sides[1];
        smallest_area
    }

    fn get_smallest_perimeter(&self) -> u32 {
        let smallest_perimeter: u32 = 2 * (self.sides[0] + self.sides[1]);
        smallest_perimeter
    }
}

fn main() {
    let argument_set = App::new("AOC: Day 2")
                            .arg(Arg::with_name("input")
                                .short("f")
                                .long("input-file")
                                .takes_value(true)
                                .required(true))
                            .get_matches();
    let input_file: &str = argument_set.value_of("input").unwrap();
    let file_handle = File::open(input_file).expect("unable to open");
    let buffered_file = BufReader::new(file_handle);
    let mut running_sum_paper: u32 = 0;  
    let mut running_sum_ribbon: u32 = 0;  
    for line in buffered_file.lines() {
        let dim: String = line.unwrap();
        let dimensions: Vec<&str> = dim.split("x").collect();
        let b = ElfBox::new_box(
                    dimensions[0].parse().unwrap(),
                    dimensions[1].parse().unwrap(), 
                    dimensions[2].parse().unwrap());
        let current_box_paper_surface_area: u32 = b.get_total_surface_area();
        let current_box_paper_slack: u32 = b.get_smallest_area();
        let current_box_bow_needs: u32 = b.get_volume();
        let current_box_ribbon_needs: u32 = b.get_smallest_perimeter();
        running_sum_paper = running_sum_paper + current_box_paper_surface_area + current_box_paper_slack; 
        running_sum_ribbon = running_sum_ribbon + current_box_bow_needs + current_box_ribbon_needs;       
        println!("Working on box {}, running paper needs up to {}, total ribbon needs up to {}", dim, running_sum_paper, running_sum_ribbon);
    }
}


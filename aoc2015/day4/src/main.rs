use md5;

fn main() {
    //let input = "yzbqklnj".to_string();
    //let mut integer = 609043.to_string();
    let mut integer = 0; 
    loop {
        integer = integer + 1;
        //format macro
        let input_string = "yzbqklnj".to_string() + &integer.to_string();
        //println!("{}", input_string);
        let hash = md5::compute(input_string);
        
        // md5 values are 16 bytes; so we're looking for the first 2.5 bytes
        // slicing can give us the first two bytes
        let slice_1 = &hash[..2];
        // we're going to rightshift the 3rd byte by 4 bits to get that last nibble
        let slice_2 = &hash[2] >> 4;
        let slice_3 = &hash[..3];
        
        // logic for "00000"
        // the use of i32 seems to be crucial here, I got an 
        // 'attempt' to add with overflow as each slice is by default a u8
        /*let sum_5  = slice_1[0] as i32 + slice_1[1] as i32 + slice_2 as i32;
         if sum_5 == 0 {
            break;
         }*/    

        // similar logic for "000000"
        // cargo run --  50.02s user 0.24s system 99% cpu 50.406 total
        let sum_6 = slice_3[0] as i32 + slice_3[1] as i32 + slice_3[2] as i32;
        if sum_6 == 0 {
            break;
        }
         
        // head scracher: why does this not work?
        //let sum_1: i32 = &slice_1[0].into();
        //let sum_2: i32 = &slice_1[1].into();
        //let sum_3: i32 = &slice_2;
        //if sum_1 + sum_2 + sum_3 == 0 { break; }
    }
    println!("{}", &integer);
}

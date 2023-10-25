use std::fs::{self};
// use std::io::{self, BufRead};
// use std::path::Path;
mod day1;



fn main() {

    let contents = fs::read_to_string("src/calories.txt")
        .expect("Cannot open the file");
    
    println!("{}", day1::get_max_fast(&contents));
    println!("{}", day1::three_most_caloric(&contents));
    
    

      
}

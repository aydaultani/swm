use crate::instructions::*;


pub fn run(debug: bool) {
    if debug {
        print!("Initializing vm... ");
        println!("DONE");
        print!("Initializing stack... ");
    }
    let mut s: Vec<i32> = vec![];
    let stack = &mut s;
    println!("DONE");
    println!("Ready to go.")
}
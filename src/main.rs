#![allow(unused)]

use std::io;
use rand::Rng; 
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let rnd_num: i32 = rand::thread_rng().gen_range(1..4);
    match rnd_num {
        1 => println!("You rolled a {}", rnd_num)
    }
}
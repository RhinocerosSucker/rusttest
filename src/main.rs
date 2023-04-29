#![allow(unused)]

use std::io;
use rand::Rng; 
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let str = String::from("a s w q d f g j u t f v n m j k u i o p l");
    let mut v1: Vec<char> = str.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char)
    }
    endloop();
}





fn endloop() {
    let mut ynans = String::new();
    println!("Do you want to try again?");
    io::stdin().read_line(&mut ynans);

    if ynans.to_lowercase().trim() == "yes" {
    main();
    }else if ynans.to_lowercase().trim() == "no" {
        println!("Goodbye!");
    } else  {
        print!("Invalid input");
        endloop();
    }
}
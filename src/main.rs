#![allow(unused)]

use std::io;
use rand::Rng; 
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    
    let mut str1 = String::new();

    str1.push('A');
    str1.push_str(" is a very important letter."); 

    for words in str1.split_whitespace() {
        println!("{}", words.replace('A', "Aloha"));
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
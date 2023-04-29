#![allow(unused)]

use std::io;
use rand::Rng; 
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    enum MyFam {
        Zeger,
        Reine,
        Jacobien,
        Stijn
    }
    impl MyFam {
        fn howManyInFamily(&self) -> u8 {
            return MyFam.len();
        }
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
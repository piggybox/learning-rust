#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let mut name = String::new();
    let greeting = "Nice to meet you";

    io::stdin()
        .read_line(&mut name)
        .expect("Didn't receive input");

    println!("Hello, {}! {}", name.trim_end(), greeting);
}

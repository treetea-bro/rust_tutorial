#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {}

// fn main() {
//     // 소수점 6자리 수까지만 정확
//     let num1: f32 = 1.111111111111111;
//     println!("f32 : {}", num1 + 0.111111111111111);
//     // 소수점 14자리 수까지만 정확
//     let num2: f64 = 1.111111111111111;
//     println!("f64 : {}", num2 + 0.111111111111111);
// }

// fn main() {
//     println!("Max u32 : {}", u32::MAX);
//     println!("Max u64 : {}", u64::MAX);
//     println!("Max usize : {}", usize::MAX);
//     println!("Max u128 : {}", u128::MAX);
//
//     println!("Max f32 : {}", f32::MAX);
//     println!("Max f64 : {}", f64::MAX);
// }

// fn main() {
//     const ONE_MIL: u32 = 1_000_000;
//     const PI: f64 = 3.141592;
//
//     let age = "47";
//     let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
//     age += 1;
//     println!("I'm {} and I want ${}", age, ONE_MIL);
// }

// fn main() {
//     println!("What your name?");
//     let mut name: String = String::new();
//     let greeting = "nice to meet you";
//     io::stdin()
//         .read_line(&mut name)
//         .expect("Didn't Receive Input");
//     println!("Hello {}!, {}", name.trim_end(), greeting);
// }

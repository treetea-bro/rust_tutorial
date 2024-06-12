#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {}

// fn main() {
//     let vec1: Vec<i32> = Vec::new();
//     let mut vec2 = vec![1, 2, 3, 4];
//     vec2.push(5);
//     println!("1st : {}", vec2[0]);
//     let second: &i32 = &vec2[1];
//     match vec2.get(1) {
//         Some(second) => println!("2nd : {}", second),
//         None => println!("No 2nd value"),
//     }
//     for i in &mut vec2 {
//         *i *= 2;
//     }
//     for i in &vec2 {
//         println!("{}", i);
//     }
//     println!("Vec Length {}", vec2.len());
//     println!("Pop : {:?}", vec2.pop());
//     println!("Pop : {:?}", vec2.pop());
//     println!("Pop : {:?}", vec2.pop());
//     println!("Pop : {:?}", vec2.pop());
//     println!("Pop : {:?}", vec2.pop());
//     println!("Pop : {:?}", vec2.pop());
//     println!("Pop : {:?}", vec2.pop());
// }

// fn main() {
//     enum Day {
//         Monday,
//         Tuesday,
//         Wendnesday,
//         Thursday,
//         Friday,
//         Saturday,
//         Sunday,
//     }
//     impl Day {
//         fn is_weekend(&self) -> bool {
//             match self {
//                 Day::Saturday | Day::Sunday => true,
//                 _ => false,
//             }
//         }
//     }
//     let today: Day = Day::Sunday;
//     match today {
//         Day::Monday => println!("Everyone hates Monday"),
//         _ => println!("Ok"),
//     }
//
//     println!("Is today the weekend {}", today.is_weekend());
// }

// fn main() {
//     let int_u8: u8 = 5;
//     let int2_u8: u8 = 4;
//     let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
// }

// fn main() {
//     let st3 = String::from("x r t b h k k a m c");
//     let mut v1: Vec<char> = st3.chars().collect();
//     v1.sort();
//     v1.dedup();
//     for char in v1 {
//         println!("{}", char);
//     }
//     let st4 = "Random string";
//     let mut st5 = st4.to_string();
//     println!("{}", st5);
//     let st6 = &st5[0..6];
//     println!("String length : {}", st6.len());
//     st5.clear();
// }

// fn main() {
//     let mut st1 = String::new();
//     st1.push('A');
//     st1.push_str(" word");
//     for word in st1.split_whitespace() {
//         println!("{}", word);
//     }
//
//     let st2 = st1.replace("A", "Another");
//     println!("{}", st2);
//
//     for c in "abcdef".chars() {
//         println!("{}", c);
//     }
// }

// fn main() {
//     let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_00.00);
//
//     println!("Name : {}", my_tuple.1);
//     let (v1, v2, v3) = my_tuple;
//     println!("Age : {}", v1);
// }

// fn compare_values(a: i32, b: i32) {
//     match a.cmp(&b) {
//         Ordering::Less => println!("{} is less than {}", a, b),
//         Ordering::Equal => println!("{} is equal to {}", a, b),
//         Ordering::Greater => println!("{} is greater than {}", a, b),
//     }
// }

// fn main() { let x = 10;
//     let mut y = 20;
//
//     compare_values(x, y); // "10 is less than 20" 출력
//     compare_values(y, x); // "20 is greater than 10" 출력
//     compare_values(x, x); // "10 is equal to 10" 출력
// }

// fn main() {
//     let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
//     println!("1st: {}", arr[0]);
//     println!("Length: {}", arr.len());
//
//     let mut loop_idx = 0;
//     loop {
//         if arr[loop_idx] % 2 == 0 {
//             loop_idx += 1;
//             continue;
//         }
//         if arr[loop_idx] == 9 {
//             break;
//         }
//         println!("Val : {}", arr[loop_idx]);
//         loop_idx += 1;
//     }
//
//     println!("");
//
//     loop_idx = 0;
//     while loop_idx < arr.len() {
//         println!("Arr : {}", arr[loop_idx]);
//         loop_idx += 1;
//     }
//
//     println!("");
//
//     for val in arr.iter() {
//         println!("Val : {}", val);
//     }
// }

// fn main() {
//     println!("input the age");
//     let mut my_age = String::new();
//     io::stdin()
//         .read_line(&mut my_age)
//         .expect("Didn't receive input");
//     let my_age: i32 = my_age.trim().parse().expect("Can't parse");
//     let voting_age = 18;
//     match my_age.cmp(&voting_age) {
//         Ordering::Less => println!("Can't Vote"),
//         Ordering::Greater => println!("Can Vote"),
//         Ordering::Equal => println!("You gained the right to vote"),
//     };
// }

// fn main() {
//     let age2 = 8;
//     match age2 {
//         1..=18 => println!("Important Birthday"),
//         21 | 50 => println!("Important Birthday"),
//         65..=i32::MAX => println!("Important Birthday"),
//         _ => println!("Not an Important Birthday"),
//     }
// }

// fn main() {
//     let mut my_age = 47;
//     let can_vote = if my_age >= 18 { true } else { false };
//     println!("Can Vote : {}", can_vote);
// }

// fn main() {
//     let age = 8;
//     if (age >= 1) && (age <= 18) {
//         println!("Important Birthday");
//     } else if (age == 21) || (age == 50) {
//         println!("Important Birthday");
//     } else if age >= 65 {
//         println!("Important Birthday");
//     } else {
//         println!("Not an Important Birthday");
//     }
// }

// fn main() {
//     let random_num = rand::thread_rng().gen_range(1..101);
//     println!("Random : {}", random_num);
// }

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

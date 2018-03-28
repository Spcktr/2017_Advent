extern crate regex;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate indoc;
extern crate num;

mod helper;

mod day01;
mod day02;
mod day03;

use std::env;
use std::process::exit;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 3 {
    println!("Usage: {} day no", &args[0]);
    exit(1);
  }

  if let Some(day) = args[1].parse::<i32>().ok() {
    if let Some(no) = args[2].parse::<i32>().ok() {
      match (day, no) {
        (01, 1) => println!("{}", day01::no1()),
        (01, 2) => println!("{}", day01::no2()),
        (02, 1) => println!("{}", day02::no1()),
        (02, 2) => println!("{}", day02::no2()),
        (03, 1) => println!("{}", day03::no1()),
        (03, 2) => println!("{}", day03::no2()),
        _ => println!("Day {} number {} not found", day, no),
      }
    } else {
      println!("No is invalid");
      exit(1);
    }
  } else {
    println!("Day is invalid");
    exit(1);
  }
}

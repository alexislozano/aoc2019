pub mod exercises;

use exercises::ex01::{ex011, ex012};
use exercises::ex02::{ex021, ex022};

fn main() {
    println!("Exercise 01.1: {}", ex011());
    println!("Exercise 01.2: {}", ex012());
    println!("Exercise 02.1: {}", ex021());
    println!("Exercise 02.2: {}", ex022());
}

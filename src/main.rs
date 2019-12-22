extern crate num;

pub mod exercises;
pub mod helpers;

use std::env;

use exercises::ex01::ex01;
use exercises::ex02::ex02;
use exercises::ex03::ex03;
use exercises::ex04::ex04;
use exercises::ex05::ex05;
use exercises::ex06::ex06;
use exercises::ex07::ex07;
use exercises::ex08::ex08;
use exercises::ex09::ex09;
use exercises::ex10::ex10;
use exercises::ex11::ex11;
use exercises::ex12::ex12;
use exercises::ex13::ex13;
use exercises::ex14::ex14;
use exercises::ex15::ex15;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter an exercise number");
    } else {
        match args[1].parse::<u32>() {
            Ok(n) => match n {
                1 => ex01(),
                2 => ex02(),
                3 => ex03(),
                4 => ex04(),
                5 => ex05(),
                6 => ex06(),
                7 => ex07(),
                8 => ex08(),
                9 => ex09(),
                10 => ex10(),
                11 => ex11(),
                12 => ex12(),
                13 => ex13(),
                14 => ex14(),
                15 => ex15(),
                _ => println!("This exercise does not exist")
            },
            _ => println!("Error reading exercise number")
        }
    }
}

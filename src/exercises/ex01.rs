use std::fs;

pub fn ex011() -> i32 {
    match fs::read_to_string("src/inputs/ex01.txt") {
        Ok(s) => s.split('\n').map(|mass| 
            match mass.parse::<i32>() {
                Ok(m) => fuel(m),
                _ => 0 
            }
        ).sum(),
        _ => 0
    }
}

pub fn ex012() -> i32 {
    match fs::read_to_string("src/inputs/ex01.txt") {
        Ok(s) => s.split('\n').map(|mass| 
            match mass.parse::<i32>() {
                Ok(m) => rec_fuel(m),
                _ => 0 
            }
        ).sum(),
        _ => 0
    }
}

fn fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn rec_fuel(mass: i32) -> i32 {
    let f = fuel(mass);
    if f <= 0 {
        0
    } else { 
        f + rec_fuel(f)
    }
}
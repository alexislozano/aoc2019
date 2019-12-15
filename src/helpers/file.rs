use std::fs;

pub fn read(e: &str) -> String {
    match fs::read_to_string(format!("src/inputs/ex{}.txt", e)) {
        Ok(s) => s.to_string(),
        _ => panic!("Cannot read ex{}.txt", e),
    }
}

pub fn write(e: &str, s1: &str, s2: &str) {
    match fs::write(
        format!("src/outputs/ex{}.txt", e),
        format!("{}\n----\n{}", s1, s2),
    ) {
        Ok(_) => println!("ex{} ok", e),
        _ => panic!("Cannot write in ex{}.txt", e),
    }
}

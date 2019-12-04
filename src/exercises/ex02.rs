use std::fs;

pub fn ex02() {
    match fs::read_to_string("src/inputs/ex02.txt") {
        Ok(s) => println!(
            "Ex 02.1: {} | Ex 02.2: {}",
            sub1(&s, Some((12, 2))),
            sub2(&s)
        ),
        _ => println!("Cannot read ex02.txt"),
    }
}

pub fn sub1(s: &str, args: Option<(i32, i32)>) -> i32 {
    let mut program = s
        .split(',')
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>();
    run(&mut program, args);
    program[0]
}

pub fn sub2(s: &str) -> i32 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>();
    let mut noun = 0;
    let mut verb = 0;
    'noun: for n in 0..99 {
        for v in 0..99 {
            let mut test = program.clone();
            run(&mut test, Some((n, v)));
            if test[0] == 19_690_720 {
                noun = n;
                verb = v;
                break 'noun;
            }
        }
    }
    100 * noun + verb
}

fn run(program: &mut Vec<i32>, args: Option<(i32, i32)>) {
    if let Some((n, v)) = args {
        program[1] = n;
        program[2] = v;
    }

    let mut step = 0;
    let mut current_line: [i32; 4] = Default::default();
    let mut opcode = program[step * 4];

    loop {
        let value = if opcode == 99 {
            break;
        } else {
            current_line.copy_from_slice(&program[step * 4..(step + 1) * 4]);
            if opcode == 1 {
                program[current_line[1] as usize]
                    + program[current_line[2] as usize]
            } else if opcode == 2 {
                program[current_line[1] as usize]
                    * program[current_line[2] as usize]
            } else {
                0
            }
        };

        program[current_line[3] as usize] = value;
        step += 1;
        opcode = program[step * 4];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("1,9,10,3,2,3,11,0,99,30,40,50", None), 3500);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("1,0,0,0,99", None), 2);
    }

    #[test]
    fn sub13() {
        assert_eq!(sub1("2,3,0,3,99", None), 2);
    }

    #[test]
    fn sub14() {
        assert_eq!(sub1("2,4,4,5,99,0", None), 2);
    }

    #[test]
    fn sub15() {
        assert_eq!(sub1("1,1,1,4,99,5,6,0,99", None), 30);
    }
}

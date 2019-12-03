use std::fs;

pub fn ex021() -> i32 {
    match fs::read_to_string("src/inputs/ex02.txt") {
        Ok(s) => {
            let mut program = s.split(',').map(|x| 
                x.parse::<i32>().unwrap_or(0)
            ).collect::<Vec<i32>>();
            run(&mut program, 12, 2);
            program[0]
        },
        _ => 0
    }
}

pub fn ex022() -> i32 {
    match fs::read_to_string("src/inputs/ex02.txt") {
        Ok(s) => {
            let program = s.split(',').map(|x| 
                x.parse::<i32>().unwrap_or(0)
            ).collect::<Vec<i32>>();
            let mut noun = 0;
            let mut verb = 0;
            'noun: for n in 0..99 {
                for v in 0..99 {
                    let mut test = program.clone();
                    run(&mut test, n, v);
                    if test[0] == 19_690_720 {
                        noun = n;
                        verb = v;
                        break 'noun;
                    }
                }
            }
            100 * noun + verb      
        },
        _ => 0
    }
}

fn run(program: &mut Vec<i32>, noun: i32, verb: i32) {
    program[1] = noun;
    program[2] = verb;
    
    let mut step = 0;
    let mut current_line: [i32; 4] = Default::default(); 
    current_line.copy_from_slice(&program[step*4..(step+1)*4]);
    let mut opcode = current_line[0];

    loop {
        let value = match opcode {
            1 => (
                program[current_line[1] as usize] + 
                program[current_line[2] as usize]
            ),
            2 => (
                program[current_line[1] as usize] * 
                program[current_line[2] as usize]
            ),
            99 => break,
            _ => 0
        };

        program[current_line[3] as usize] = value;
        
        step += 1;
        current_line.copy_from_slice(&program[step*4..(step+1)*4]);
        opcode = current_line[0];
    }
}
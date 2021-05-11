use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

// [43, 45, 62, 60, 91, 93, 44, 46]
//  +   -   >   <   [   ]   ,   .

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Interpreting Code...");

    let code: Vec<u8> = comment_removal(fs::read(&args[1]).expect("Error failed to read file."));
    let bracket_map: HashMap<usize, usize> = gen_bracket_map(&code);

    let mut ptr: usize = 0;
    let mut tape: Vec<i128> = Vec::new();
    tape.push(0);

    let mut pos: usize = 0;

    loop {
        if code[pos] == 43 {
            tape[ptr] = tape[ptr] + 1;
        } else if code[pos] == 45 {
            tape[ptr] = tape[ptr] - 1;
        } else if code[pos] == 62 {
            ptr = ptr + 1;
            if ptr > tape.len() - 1 {
                tape.push(0);
            }
        } else if code[pos] == 60 {
            ptr = ptr - 1;
        } else if code[pos] == 46 {
            print!("{}", tape[ptr] as u8 as char);
            io::stdout().flush().unwrap();
        } else if code[pos] == 44 {
            let mut tmp = String::new();
            io::stdin()
                .read_line(&mut tmp)
                .expect("Error: String Stuff");
            tape[ptr] = tmp.as_bytes()[0] as i128;
        } else if code[pos] == 91 && tape[ptr] == 0 {
            pos = bracket_map
                .get(&pos)
                .copied()
                .expect("Error: Invalid Brain Fuck");
        } else if code[pos] == 93 && tape[ptr] != 0 {
            pos = bracket_map
                .get(&pos)
                .copied()
                .expect("Error: Invalid Brain Fuck");
        }
        if pos == code.len() - 1 {
            break;
        }
        pos += 1;
    }
}

fn comment_removal(chars: Vec<u8>) -> Vec<u8> {
    let commands: Vec<u8> = vec![43, 45, 62, 60, 91, 93, 44, 46];
    let mut tmp: Vec<u8> = Vec::new();
    for c in chars {
        for cmd in &commands {
            if cmd == &c {
                tmp.push(c);
                break;
            }
        }
    }

    return tmp;
}

fn gen_bracket_map(chars: &Vec<u8>) -> HashMap<usize, usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut left_positions: Vec<usize> = Vec::new();

    let mut position: usize = 0;
    for c in chars {
        if c == &91 {
            left_positions.push(position);
        } else if c == &93 {
            let left = left_positions
                .pop()
                .expect("Error: Invalid Brain Fuck Code");
            map.insert(left, position);
            map.insert(position, left);
        }

        position += 1;
    }
    return map;
}

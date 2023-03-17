use clap::Parser;
use std::io;
use std::path::PathBuf;
use std::{fs, process};

// [43, 45, 62, 60, 91, 93, 44, 46]
//  +   -   >   <   [   ]   ,   .

#[derive(Parser)]
#[command(name = "bif")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Brainfuck Source File
    #[arg(name="source file", value_parser = clap::value_parser!(PathBuf), required=true, )]
    file: PathBuf,

    /// Input to the program
    #[arg(short = 'i', long = "input", default_value = "")]
    input: Option<String>,

    /// Tape Capacity
    #[arg(short = 'c', long = "capacity", default_value = "30000")]
    tape_cap: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    let code = read_file(&cli.file);
    let mut code_ptr: usize = 0;

    let mut tape: Vec<u8> = Vec::new();
    let mut tape_ptr: usize = 0;

    let mut bracket_stack: Vec<usize> = Vec::new();

    let cap = match cli.tape_cap {
        Some(cap) => cap,
        None => 30000,
    };

    let mut input_ptr: usize = 0;
    let input = match cli.input {
        Some(input) => input,
        None => String::new(),
    };

    for _ in 0..cap {
        tape.push(0);
    }

    loop {
        if code_ptr == code.len() - 1 || code.len() == 0 {
            break;
        }
        match code[code_ptr] {
            43 => tape[tape_ptr] = tape[tape_ptr].wrapping_add(1),
            45 => tape[tape_ptr] = tape[tape_ptr].wrapping_sub(1),
            62 => {
                tape_ptr += 1;
                if tape_ptr == tape.len() {
                    tape_ptr = 0;
                }
            }
            60 => {
                if tape_ptr == 0 {
                    tape_ptr = tape.len() - 1;
                }
                tape_ptr -= 1;
            }
            46 => print!("{}", tape[tape_ptr] as char),
            44 => {
                let mut next_in = String::new();
                if input.len() == 0 {
                    next_in = match io::stdin().read_line(&mut next_in) {
                        Ok(_) => next_in,
                        Err(_) => String::new(),
                    };
                    tape[tape_ptr] = next_in.as_bytes()[0];
                } else {
                    if input_ptr == input.len() {
                        break;
                    }

                    tape[tape_ptr] = input.as_bytes()[input_ptr];
                    input_ptr += 1;
                }
            }
            91 => bracket_stack.push(code_ptr),
            93 => {
                if tape[tape_ptr] != 0 {
                    code_ptr = bracket_stack[bracket_stack.len() - 1];
                } else {
                    bracket_stack.pop();
                }
            }
            _ => (),
        }

        code_ptr += 1;
    }
}

fn read_file(file: &PathBuf) -> Vec<u8> {
    return match fs::read(file) {
        Ok(code_buf) => code_buf,
        Err(err) => {
            let error_kind = err.kind();

            if error_kind == io::ErrorKind::NotFound {
                println!("error: file not found: {:?}", file);
            } else if error_kind == io::ErrorKind::PermissionDenied {
                println!("error: permission denied reading file: {:?}", file);
            } else {
                println!("error: unknown error reading file: {:?}", file);
            }
            process::exit(1);
        }
    };
}

use std::{char, fs};
use std::{env, process::exit};

fn main() {
    let mut strip = [0u8; 30_000];
    let mut pointer = 0usize;
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(i) => i,
        None => exit(0),
    };

    let contents: Vec<char> = fs::read_to_string(file_path)
        .expect("file not found")
        .chars()
        .collect();
    let mut not_done = true;
    let mut strip_pointer = 0;
    let strip_length = contents.len();

    while not_done {
        match contents[strip_pointer] {
            '<' => {
                if pointer == 0 {
                    pointer = 29_999;
                } else {
                    pointer -= 1;
                }
            }
            '>' => {
                if pointer == 29_999 {
                    pointer = 0;
                } else {
                    pointer += 1;
                }
            }
            '+' => {
                if strip[pointer] == 0xFF {
                    strip[pointer] = 0u8;
                } else {
                    strip[pointer] += 1;
                }
            }
            '-' => {
                if strip[pointer] == 0u8 {
                    strip[pointer] = 0xFF;
                } else {
                    strip[pointer] -= 1;
                }
            }
            '.' => {
                print!("{}", strip[pointer] as char);
            }
            ',' => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .ok()
                    .expect("Failed to read line");
                strip[pointer] = input.bytes().nth(0).expect("no byte read");
            }
            '[' => {
                if strip[pointer] == 0 {
                    let mut counter = 0;
                    for i in strip_pointer..strip_length {
                        if contents[i] == '[' {
                            counter += 1;
                        }
                        if contents[i] == ']' {
                            if counter == 0 {
                                strip_pointer = i - 1;
                                break;
                            }
                            counter -= 1;
                        }
                    }
                }
            }
            ']' => {
                if strip[pointer] != 0 {
                    let mut counter = 0;
                    for i in (0..strip_pointer).rev() {
                        if contents[i] == ']' {
                            counter += 1;
                        }
                        if contents[i] == '[' {
                            if counter == 0 {
                                strip_pointer = i - 1;
                                break;
                            }
                            counter -= 1;
                        }
                    }
                }
            }
            _ => {}
        }
        strip_pointer += 1;
        if strip_pointer >= strip_length {
            not_done = false;
        }
    }
}

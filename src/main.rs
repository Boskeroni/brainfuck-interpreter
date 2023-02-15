use std::{io::Write, fs, env};

fn try_input() -> i8 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return (input.chars().nth(0).unwrap())as i8;
}

fn handle_input(arr: &mut Vec<i8>, mut ptr: usize, code: &str) -> (Vec<i8>, usize) {
    let iter_code = code.as_bytes();
    let mut code_index = 0;
    while code_index != code.len() {
        let command = iter_code[code_index] as char;
        match command {
            '+' => arr[ptr] += 1,
            '-' => arr[ptr] -= 1,
            '>' => ptr += 1,
            '<' => ptr -= 1,
            ',' => arr[ptr] = try_input(),
            '.' => {
                print!("{}", arr[ptr] as u8 as char);
                std::io::stdout().flush().unwrap();
            }
            '[' => {
                let mut count = 1;

                // eventually becomes the location of the matching ']'
                let mut temp_index = code_index;
                while count != 0 {
                    temp_index += 1;
                    let c = iter_code[temp_index] as char;
                    if c == '[' {
                        count += 1;
                    } else if c == ']' {
                        count -= 1;
                    }
                }
                let inner_code = &code[code_index+1..temp_index];
                while arr[ptr] != 0 {
                    (*arr, ptr) = handle_input(arr, ptr, inner_code);
                }
                code_index = temp_index;
            }
            _ => {}
        }
        code_index += 1;
    }
    (arr.to_vec(), ptr)
}

fn main() {
    // all our values
    let mut array: Vec<i8> = vec![0; 30000];
    let path = env::args().nth(1).unwrap();
    let code =  fs::read_to_string(path).unwrap();
    let ptr = 0;

    handle_input(&mut array, ptr, &code);
}

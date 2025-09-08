use std::{arch::asm, collections::HashMap, io::Write};

use crate::try_input;

fn get_jump_table(code: &Vec<char>) -> HashMap<usize, usize> {
    let mut jump_table = HashMap::new();

    let mut starts = Vec::new();
    for i in 0..code.len() {
        if code[i] == '[' {
            starts.push(i);
        }
        else if code[i] == ']' {
            jump_table.insert(starts.pop().unwrap(), i+1);
        }
    }

    return jump_table;
}

pub fn interpret(code: &Vec<char>) {
    let mut stack: Vec<usize> = Vec::new();
    let mut arr: Vec<u8> = vec![0; 30000];
    let mut arr_ptr = 0;
    let mut code_ptr = 0;

    let jump_table = get_jump_table(code);

    // reached the end of the file
    while code_ptr != code.len() {
        let keyword = code[code_ptr];
        code_ptr += 1;

        unsafe {
            match keyword {
                '+' => arr[arr_ptr] = arr[arr_ptr].wrapping_add(1),
                '-' => arr[arr_ptr] = arr[arr_ptr].wrapping_sub(1),
                '>' => asm!(
                    "add {0}, {number}", 
                    inout(reg) arr_ptr,
                    number = const 1,
                ), //arr_ptr += 1,
                '<' => asm!(
                    "sub {0}, {number}", 
                    inout(reg) arr_ptr,
                    number = const 1,
                ), //arr_ptr += 1,
                ',' => arr[arr_ptr] = try_input(),
                '.' => {
                    print!("{}", arr[arr_ptr] as char); 
                    std::io::stdout().flush().unwrap(); 
                }
                '[' => {
                    if arr[arr_ptr] != 0 {
                        stack.push(code_ptr);
                        continue;
                    }
                    // skip it
                    code_ptr = *jump_table.get(&(code_ptr - 1)).unwrap(); 
                },
                ']' => {
                    if arr[arr_ptr] != 0 {
                        code_ptr = *stack.last().unwrap();
                        continue;
                    }
                    stack.pop();
                }
                _ => unreachable!(),
            }
        }

    }
}
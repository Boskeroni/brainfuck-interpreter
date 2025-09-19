use std::{env, fs};

mod jit;
mod backends;

// just making sure the code provided is valid
fn sanitize(code: &str) -> Vec<char> {
    let mut brackets = 0;
    let mut sanitized = Vec::new();

    for keyword in code.as_bytes().iter().map(|&c| c as char) {
        match keyword {
            '+'|'-'|'>'|'<'|','|'.' => sanitized.push(keyword),
            '[' => {
                brackets += 1;
                sanitized.push(keyword);
            },
            ']' => {
                brackets -= 1;
                sanitized.push(keyword);
            },
            _ => {},
        }
    }

    if brackets != 0 {
        println!("invalid brackets provided! =>");
        if brackets > 0 {
            eprintln!("too many opening brackets");
        } else {
            eprintln!("too few closing brackets");
        }
        panic!()
    }

    return sanitized;
}

fn main() {
    // all our values
    let path = env::args().nth(1).unwrap();
    let unsanitized_code = fs::read_to_string(path).unwrap();
    let code = sanitize(&unsanitized_code);

    // interpret::interpret(&code);
    jit::jit_compile(code);
}

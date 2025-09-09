#[derive(Debug, Clone)]
pub enum Tokens {
    Inc(u8),
    Dec(u8),
    MovUp(u8),
    MovDown(u8),
    LoopStart,
    LoopEnd,
    Print,
    Read,
}

pub fn tokenize(code: Vec<char>) -> Vec<Tokens> {
    let mut tokens = Vec::new();
    let mut repeat_count = 0;

    use Tokens::*;
    for i in 0..code.len() {
        let keyword = code[i];

        match keyword {
            '[' => tokens.push(LoopStart),
            ']' => tokens.push(LoopEnd),
            '.' => tokens.push(Print),
            ',' => tokens.push(Read),
            _ => {}
        }

        // handled so move onto next
        if [']', '[', '.', ','].contains(&keyword) {
            repeat_count = 0;
            continue;
        }

        // optimisation to reduce instruction count
        // handle edge case when i = len
        if i != code.len() - 1 {
            if keyword == code[i + 1] {
                repeat_count += 1;
                continue;
            }
        }

        // to account for the current keyword
        repeat_count += 1;
        match keyword {
            '+' => tokens.push(Inc(repeat_count)),
            '-' => tokens.push(Dec(repeat_count)),
            '>' => tokens.push(MovUp(repeat_count)),
            '<' => tokens.push(MovDown(repeat_count)),
            _ => unreachable!(),
        }
        repeat_count = 0;
    }

    return tokens
}
use std::{iter::Peekable, slice::Iter};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    Inc,
    Dec,
    MovUp,
    MovDown,
    LoopStart,
    LoopEnd,
    Print,
    Read,
}

pub fn tokenize(code: Vec<char>) -> Vec<Token> {
    let mut tokens = Vec::new();

    use Token::*;
    for i in 0..code.len() {
        let keyword = code[i];

        match keyword {
            '[' => tokens.push(LoopStart),
            ']' => tokens.push(LoopEnd),
            '.' => tokens.push(Print),
            ',' => tokens.push(Read),
            '+' => tokens.push(Inc),
            '-' => tokens.push(Dec),
            '>' => tokens.push(MovUp),
            '<' => tokens.push(MovDown),
            _ => {}
        }
    }

    return tokens
}

#[derive(Debug, Clone, Copy)]
pub enum SyntaxTreeInst {
    Add(u8),
    AddRelative(i16, i8, i8),
    Sub(u8),
    Shiftup(u8),
    ShiftDown(u8),
    Loop,
    Print,
    Read,
    LoopEnd,
    Clear,
}

#[derive(Debug)]
pub struct SyntaxTree {
    pub instructions: Vec<SyntaxTreeInst>,
    pub loops: Vec<SyntaxTree>,
}

pub fn build_ast(code: Vec<char>) -> SyntaxTree {
    let tokens = tokenize(code);
    let mut iter = tokens.iter().peekable();

    return build_ast_recursive(&mut iter);
}

fn build_ast_recursive(src: &mut Peekable<Iter<'_, Token>>) -> SyntaxTree {
    let mut root = SyntaxTree {
        instructions: Vec::new(),
        loops: Vec::new(),
    };

    let mut repeats = 1;
    while let Some(code) = src.next() {
        match code {
            Token::LoopStart => {
                root.loops.push(build_ast_recursive(src));
                root.instructions.push(SyntaxTreeInst::Loop);
                continue;
            }
            Token::LoopEnd => break,
            Token::Read => {
                root.instructions.push(SyntaxTreeInst::Read);
                continue;
            }
            Token::Print => root.instructions.push(SyntaxTreeInst::Print),
            _ => {}
        }

        if let Some(next_code) = src.peek() {
            if *next_code == code {
                repeats += 1;
                continue;
            }
        }

        root.instructions.push(match code {
            Token::Inc => SyntaxTreeInst::Add(repeats),
            Token::Dec => SyntaxTreeInst::Sub(repeats),
            Token::MovUp => SyntaxTreeInst::Shiftup(repeats),
            Token::MovDown => SyntaxTreeInst::ShiftDown(repeats),
            _ => unreachable!("{code:?}"),
        });
        repeats = 1;
    }

    return root;
}
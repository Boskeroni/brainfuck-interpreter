mod ast;
mod optimiser;

use std::{mem, ptr};
use libc::*;
use crate::{backends::*, jit::ast::{SyntaxTree, SyntaxTreeInst}};

fn flatten_ast(ast: &SyntaxTree) -> Vec<SyntaxTreeInst> {
    let mut instructions = Vec::new();

    let mut loop_index = 0;
    for instruction in &ast.instructions {
        if let SyntaxTreeInst::Loop = instruction {
            instructions.extend(flatten_ast(&ast.loops[loop_index]));
            instructions.push(SyntaxTreeInst::LoopEnd);
            loop_index += 1;
            continue;
        }

        instructions.push(*instruction);
    }

    return instructions;
}

fn compile(insts: Vec<SyntaxTreeInst>) -> Vec<u8> {
    let mut asm = Vec::new();
    let mut stack = Vec::new();

    setup_pointer(&mut asm);

    for inst in insts {
        match inst {
            SyntaxTreeInst::Shiftup(amount) => shift_up(&mut asm, amount),
            SyntaxTreeInst::ShiftDown(amount) => shift_down(&mut asm, amount),
            SyntaxTreeInst::Add(amount) => mem_inc(&mut asm, amount),
            SyntaxTreeInst::Sub(amount) => mem_dec(&mut asm, amount),
            SyntaxTreeInst::Loop => {
                start_loop(&mut asm);
                stack.push(asm.len()); // save the spot so we can write to it later
            }
            SyntaxTreeInst::LoopEnd => {
                let beginning = stack.pop().unwrap();

                // make it jump to the beginning, and then 5 more
                let offset = beginning.wrapping_sub(asm.len()) as i32 - 16;
                end_loop(&mut asm, offset as u32);

                // replace the temp value from before
                let jump_to = asm.len();
                let offset = jump_to - beginning;
                let replace_addr = beginning - 4;

                asm[replace_addr + 3] = (offset >> 24) as u8;
                asm[replace_addr + 2] = (offset >> 16) as u8;
                asm[replace_addr + 1] = (offset >> 8) as u8;
                asm[replace_addr + 0] = offset as u8;
            }
            SyntaxTreeInst::Print => print_asm(&mut asm),
            SyntaxTreeInst::Read => read_asm(&mut asm),
            SyntaxTreeInst::AddRelative(offset, change, per) => add_relative_asm(&mut asm, offset, change, per),
            SyntaxTreeInst::Clear => mov_0_asm(&mut asm),
        }
    }

    ret(&mut asm);
    return asm;
}

pub fn jit_compile(src: Vec<char>) {
    let mut ast = ast::build_ast(src);
    optimiser::optimise(&mut ast);

    let instructions = flatten_ast(&ast);
    let code = compile(instructions);
    let size = code.len();

    let pagesize = unsafe {sysconf(_SC_PAGESIZE) as usize};
    let rounded = (size + pagesize - 1) & !(pagesize - 1);
    let flags = MAP_PRIVATE | MAP_ANON;

    let p = unsafe {
        mmap(
            ptr::null_mut(),
            rounded,
            PROT_READ | PROT_WRITE | PROT_EXEC,
            flags,
            -1,
            0,
        )
    };

    if p == MAP_FAILED {
        panic!("mmap failed");
    }
    
    // area has been allocated
    let buffer = unsafe {
        std::slice::from_raw_parts_mut(p as _, size)
    };
    buffer[..code.len()].copy_from_slice(&code);
    
    unsafe {
        if mprotect(p as _, rounded, libc::PROT_READ | libc::PROT_EXEC) != 0 {
            panic!("mprotext RX failed")
        }
    }

    // get the array
    let arr_size = std::u16::MAX as usize;
    let array_addr = unsafe {
        let arr = mmap(ptr::null_mut(), arr_size, PROT_READ | PROT_WRITE, flags, -1, 0);
        if !arr.is_null() {
            std::ptr::write_bytes(arr, 0, arr_size);
        } else { panic!("couldnt create array "); }
        arr
    };

    let f: extern "C" fn(*mut u8) -> u64 = unsafe { mem::transmute(p) };
    println!("{array_addr:?}");
    let _r = f(array_addr as _);
    println!("");
}
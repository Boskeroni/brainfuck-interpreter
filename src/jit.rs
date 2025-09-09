use std::{mem, ptr};
use libc::*;

use crate::tokenize::{self, Tokens};

// for now just handle each keyword as its own instruction
// eventually move onto optimizing it
fn compile(tokens: Vec<Tokens>) -> Vec<u8> {
    let mut asm = Vec::new();

    asm.extend([0x48, 0x89, 0xf9]);
    let mut stack = Vec::new();

    use Tokens::*;
    for token in tokens {
        match token {
            MovUp(i) => asm.extend([0x48, 0x81, 0xc1, i, 0x00, 0x00, 0x00]),
            MovDown(i) => asm.extend([0x48, 0x81, 0xe9, i, 0x00, 0x00, 0x00]),
            Inc(i) => asm.extend([0x80, 0x01, i]),
            Dec(i) => asm.extend([0x80, 0x29, i]),
            LoopStart => {
                asm.extend([0x80, 0x39, 0x00]); // cmp [rcx], 0
                asm.extend([0x0f, 0x84]); // je (no offset specified yet)
                asm.extend([0x00, 0x00, 0x00, 0x00]); // (temp)

                stack.push(asm.len()); // save the spot so we can write to it later
            }
            LoopEnd => {
                let beginning = stack.pop().unwrap();

                // make it jump to the beginning, and then 5 more
                let offset = beginning.wrapping_sub(asm.len()) as i32 - 14;
                
                // jmp backwards first
                let (a, b, c, d) = ((offset >> 24) as u8, (offset >> 16) as u8, (offset >> 8) as u8, offset as u8);
                asm.extend([0xE9, d, c, b, a]);

                // replace the temp value from before
                let jump_to = asm.len();
                let offset = jump_to - beginning;
                let replace_addr = beginning - 4;

                asm[replace_addr + 3] = (offset >> 24) as u8;
                asm[replace_addr + 2] = (offset >> 16) as u8;
                asm[replace_addr + 1] = (offset >> 8) as u8;
                asm[replace_addr + 0] = offset as u8;
            }
            Print => {
                // since rcx gets changed, we need to store it temporarily
                asm.extend([0x49, 0x89, 0xcd]); // mov r13, rcx

                asm.extend([0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00]); // mov rax, 1
                asm.extend([0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00]); // mov rdi, 1
                asm.extend([0x48, 0x89, 0xce]); // mov rsi, rcx
                asm.extend([0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00]); // mov rdx, 1
                asm.extend([0x0f, 0x05]); // syscall

                // load it back
                asm.extend([0x4c, 0x89, 0xe9]); // mov rcx, r13
            }
            Read => {
                asm.extend([0x49, 0x89, 0xcd]);

                asm.extend([0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00]);
                asm.extend([0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00]);
                asm.extend([0x48, 0x89, 0xce]); // mov rsi, rcx
                asm.extend([0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00,]);
                asm.extend([0x0f, 0x05]);

                asm.extend([0x4c, 0x89, 0xe9]);
            }
        }
    }

    // for debugging purposes, move rax, rcx
    asm.extend([0x48, 0x89, 0xc8]);
    asm.push(0xc3);

    return asm;
}

pub fn jit_compile(src: Vec<char>) {
    let ast = tokenize::tokenize(src);
    println!("{ast:?}");
    let code = compile(ast);
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
    let array_addr = unsafe {
        let arr = mmap(ptr::null_mut(), 30000, PROT_READ | PROT_WRITE, flags, -1, 0);
        if !arr.is_null() {
            std::ptr::write_bytes(arr, 0, 30000);
        } else { panic!("couldnt create array "); }
        arr
    };

    let f: extern "C" fn(*mut u8) -> i64 = unsafe { mem::transmute(p) };
    let r = f(array_addr as _);
    println!("{r:X} {:X?}", array_addr as *mut u8);
}
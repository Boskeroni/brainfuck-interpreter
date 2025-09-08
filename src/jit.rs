use std::mem;
use windows::Win32::System::Memory::*;

// for now just handle each keyword as its own instruction
// eventually move onto optimizing it
fn process_src(src: Vec<char>) -> Vec<u8> {
    let mut asm = Vec::new();

    // no need for this, just gonna use rcx
    //asm.push(0x48); asm.push(0x89); asm.push(0xc8);
    let mut stack = Vec::new();

    for keyword in src {
        match keyword {
            '>' => asm.extend([0x48, 0xff, 0xc1]),
            '<' => asm.extend([0x48, 0xff, 0xc9]),
            '+' => asm.extend([0xfe, 0x01]),
            '-' => asm.extend([0xfe, 0x09]),
            '[' => {
                asm.extend([0x80, 0x39, 0x00]); // cmp [rcx], 0
                asm.extend([0x0f, 0x84]); // je (no offset specified yet)
                asm.extend([0x00, 0x00, 0x00, 0x00]); // (temp)

                stack.push(asm.len()); // save the spot so we can write to it later
            }
            ']' => {
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
            ',' => {}
            '.' => {
                asm.extend([0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00]); // mov rax, 1
                asm.extend([0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00]); // mov rdi, 1
                asm.extend([0x48, 0x89, 0xce]); // mov rsi, 1
                asm.extend([0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00]); // mov rdx, 1
                asm.extend([0x0f, 0x05]); // syscall
            }
            _ => unreachable!(),
        }
    }

    // for debugging purposes, move rax, rcx
    asm.extend([0x48, 0x89, 0xc8]);
    asm.push(0xc3);

    println!("{asm:02X?}");
    return asm;
}

pub fn compile(src: Vec<char>) {
    let code = process_src(src);
    let size = code.len();

    let code_addr = unsafe { 
        VirtualAlloc(None, size, MEM_COMMIT|MEM_RESERVE, PAGE_EXECUTE_READWRITE) 
    };
    if code_addr.is_null() {
        panic!("yeah this just hasnt been allocated");
    }
    
    // area has been allocated
    let buffer = unsafe {
        std::slice::from_raw_parts_mut(code_addr as _, size)
    };
    buffer[..code.len()].copy_from_slice(&code);
    
    unsafe {
        let mut old = PAGE_EXECUTE_READWRITE;
        VirtualProtect(code_addr, size, PAGE_EXECUTE_READWRITE, &mut old).unwrap();
    }

    // get the array
    let array_addr = unsafe {
        VirtualAlloc(None, 30000, MEM_COMMIT|MEM_RESERVE, PAGE_READWRITE)
    };
    unsafe {
        let mut old = PAGE_READWRITE;
        VirtualProtect(array_addr, 30000, PAGE_READWRITE, &mut old).unwrap();
    }
    if array_addr.is_null() {
        panic!("yeah this also just hasnt been allocated");
    }

    let f: extern "C" fn(*mut u8) -> i64 = unsafe { mem::transmute(code_addr) };
    let r = f(array_addr as _);
    println!("{r:X} {:X?}", array_addr as *mut u8);
}
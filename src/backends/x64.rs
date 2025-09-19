#[inline]
fn split_lil_endian(a: u32) -> (u8, u8, u8, u8) {
    (a as u8, (a >> 8) as u8, (a >> 16) as u8, (a >> 24) as u8)
}

fn asm_write(asm: &mut Vec<u8>, ext: Vec<u8>) {
    asm.extend(ext);
}
fn add_rcx_asm(asm: &mut Vec<u8>, add: u32) {
    let (a, b, c, d) = split_lil_endian(add);
    asm_write(asm, vec![0x48, 0x81, 0xc1, a, b, c, d]); // add rcx, add
}
fn sub_rcx_asm(asm: &mut Vec<u8>, sub: u32) {
    let (a, b, c, d) = split_lil_endian(sub);
    asm_write(asm, vec![0x48, 0x81, 0xe9, a, b, c, d]); // sub rcx, sub
}
fn add_mem_asm(asm: &mut Vec<u8>, inc: u8) {
    asm_write(asm, vec![0x42, 0x80, 0x04, 0x21, inc]); // add [rcx+r12], inc
}
fn dec_mem_asm(asm: &mut Vec<u8>, dec: u8) {
    asm_write(asm, vec![0x42, 0x80, 0x2c, 0x21, dec]); // sub [rcx+r12], dec
}

fn cmp_rcx_asm(asm: &mut Vec<u8>, cmp: u32) {
    let (a, b, c, d) = split_lil_endian(cmp);
    asm_write(asm, vec![0x48, 0x81, 0xf9, a, b, c, d]);
}
fn cmp_rcx_r12_mem_asm(asm: &mut Vec<u8>, cmp: u8) {
    asm_write(asm, vec![0x42, 0x80, 0x3c, 0x21, cmp]); // cmp [rcx+r12], 0
}
fn jump_less_asm(asm: &mut Vec<u8>, amount: u32) {
    let (a, b, c, d) = split_lil_endian(amount);
    asm_write(asm, vec![0x0f, 0x8c, a, b, c, d]); // jl
}



pub fn shift_up(asm: &mut Vec<u8>, add: u8) {
    add_rcx_asm(asm, add as u32);
    cmp_rcx_asm(asm, 0x10000);

    jump_less_asm(asm, 1); // just skip over the return instruction
    ret(asm);
}
pub fn shift_down(asm: &mut Vec<u8>, down: u8) {
    sub_rcx_asm(asm, down as u32);
    cmp_rcx_asm(asm, 0x10000);

    jump_less_asm(asm, 1); // no need to compare as the sub will update flags
    ret(asm);
}
pub fn mem_inc(asm: &mut Vec<u8>, inc: u8) {
    add_mem_asm(asm, inc);
}
pub fn mem_dec(asm: &mut Vec<u8>, dec: u8) {
    dec_mem_asm(asm, dec);
}
pub fn start_loop(asm: &mut Vec<u8>) {
    cmp_rcx_r12_mem_asm(asm, 0);
    asm_write(asm, vec![0x0f, 0x84]); // je
    asm_write(asm, vec![0x00, 0x00, 0x00, 0x00]); // (temp)
}
pub fn end_loop(asm: &mut Vec<u8>, offset: u32) {
    let (a, b, c, d) = split_lil_endian(offset);
    asm_write(asm, vec![0xe9, a, b, c, d]);
}
pub fn print_asm(asm: &mut Vec<u8>) {
    // since rcx gets changed, we need to store it temporarily
    asm_write(asm, vec![0x49, 0x89, 0xcd]); // mov r13, rcx

    asm_write(asm, vec![0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00]); // mov rax, 1
    asm_write(asm, vec![0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00]); // mov rdi, 1

    asm_write(asm, vec![0x48, 0x89, 0xce]); // mov rsi, rcx
    asm_write(asm, vec![0x4c, 0x01, 0xe6]); // add rsi, r12

    asm_write(asm, vec![0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00]); // mov rdx, 1

    asm_write(asm, vec![0x0f, 0x05]); // syscall

    // load it back
    asm_write(asm, vec![0x4c, 0x89, 0xe9]); // mov rcx, r13
}
pub fn read_asm(asm: &mut Vec<u8>) {
    asm_write(asm, vec![0x49, 0x89, 0xcd]); 
    asm_write(asm, vec![0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00]);
    asm_write(asm, vec![0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00]);

    asm_write(asm, vec![0x48, 0x89, 0xce]); // mov rsi, rcx
    asm_write(asm, vec![0x4c, 0x01, 0xe6]); // add rsi, r12

    asm_write(asm, vec![0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00,]);
    asm_write(asm, vec![0x0f, 0x05]);

    asm_write(asm, vec![0x4c, 0x89, 0xe9]);
}
pub fn ret(asm: &mut Vec<u8>) {
    asm_write(asm, vec![0xc3]);
}
pub fn setup_pointer(asm: &mut Vec<u8>) {
    // mov r12, rcx -> stores the actual location in memory
    // mov rcx, 0 -> the array pointer, relative to r12
    asm_write(asm, vec![0x48, 0x89, 0xf9]);
    asm_write(asm, vec![0x49, 0x89, 0xcc]);
    asm_write(asm, vec![0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00]);
}  
pub fn add_relative_asm(asm: &mut Vec<u8>, offset: i16, change: i8, per: i8) {
    
}
pub fn mov_0_asm(asm: &mut Vec<u8>) {
    asm_write(asm, vec![0x42, 0xc6, 0x04, 0x21, 0x00]);
}
fn asm_write(asm: &mut Vec<u8>, ext: Vec<u8>) {
    asm.extend(ext);
}

pub fn shift_up(asm: &mut Vec<u8>, add: u8) {
    asm_write(asm, vec![add, 0x10, 0x81, 0xe2]); // add r1, add
}
pub fn shift_down(asm: &mut Vec<u8>, sub: u8) {
    asm_write(asm, vec![sub, 0x10, 0x41, 0xe2]); // sub r1, sub
}
pub fn mem_inc(asm: &mut Vec<u8>, add: u8) {
    asm_write(asm, vec![0x00, 0x00, 0x91, 0xe5]); // ldr r0, [r1]
    asm_write(asm, vec![add, 0x00, 0x80, 0xe2]);  // add r0, add
    asm_write(asm, vec![0x00, 0x00, 0x81, 0xe5]); // str r0, [r1]
}
pub fn mem_dec(asm: &mut Vec<u8>, sub: u8) {
    asm_write(asm, vec![0x00, 0x00, 0x91, 0xe5]); // ldr r0, [r1]
    asm_write(asm, vec![add, 0x00, 0x40, 0xe2]);  // sub r0, sub
    asm_write(asm, vec![0x00, 0x00, 0x81, 0xe5]); // str r0, [r1]
}
pub fn start_loop(asm: &mut Vec<u8>, ext: Vec<u8>) {
    asm_write(asm, vec![0x00, 0x00, 0x91, 0xe5]); // ldr r0, [r1]
    asm_write(asm, vec![0x00, 0x00, 0x50, 0xe3]); // cmp r0, 0
    asm_write(asm, vec![0x00, 0x00, 0x00, 0x0a]); // beq (temp)
}
pub fn end_loop(asm: &mut Vec<u8>, ext: Vec<u8>) {

}
pub fn print_asm(asm: &mut Vec<u8>) {
    asm_write(asm, vec![0x03, 0x70, 0xa0, 0xe3]); // mov r7, 4
    asm_write(asm, vec![0x01, 0x00, 0xa0, 0xe3]); // mov r0, 1
    asm_write(asm, vec![0x01, 0x20, 0xa0, 0xe3]); // mov r2, 1
    asm_write(asm, vec![0x00, 0x00, 0x00, 0xef]); // swi 0
}
pub fn read_asm(asm: &mut Vec<u8>) {
    asm_write(asm, vec![0x04, 0x70, 0xa0, 0xe3]); // mov r7, 3
    asm_write(asm, vec![0x01, 0x00, 0xa0, 0xe3]); // mov r0, 1
    asm_write(asm, vec![0x01, 0x20, 0xa0, 0xe3]); // mov r2, 1
    asm_write(asm, vec![0x00, 0x00, 0x00, 0xef]); // swi 0
}
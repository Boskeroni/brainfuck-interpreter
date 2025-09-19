fn asm_write(asm: &mut Vec<u8>, ext: Vec<u8>) {
    asm.extend(ext);
}

pub fn shift_up(asm: &mut Vec<u8>, add: u8) {
    asm_write(asm, vec![0x83, 0xc1, add, 0x00, 0x00, 0x00])
}
pub fn shift_down(asm: &mut Vec<u8>, sub: u8) {
    asm_write(asm, vec![0x81, 0xe9, sub, 0x00, 0x00, 0x00])
}
pub fn mem_inc(asm: &mut Vec<u8>, add: u8) {
    asm_write(asm, vec![0x80, 0x01, add])
}
pub fn mem_dec(asm: &mut Vec<u8>, sub: u8) {
    asm_write(asm, vec![0x80, 0x29, sub])
}
pub fn start_loop(asm: &mut Vec<u8>) {
    asm_write(asm, [0x80, 0x3d, 0x00, 0x00, 0x00, 0x00, 0x00]);
    asm_write(asm, [0x0f, 0x84, 0x00, 0x00, 0x00, 0x00]);
}
pub fn end_loop(asm: &mut Vec<u8>, offset: u32) {
    let (a, b, c, d) = ((offset >> 24) as u8, (offset >> 16) as u8, (offset >> 8) as u8, offset as u8);
    asm_write(asm, vec![0xe9, d, c, b, a]);
}
pub fn print_asm(asm: &mut Vec<u8>) {
    asm_write(asm, [0xb8, 0x04, 0x00, 0x00, 0x00]); // mov eax 1
    asm_write(asm, [0xbb, 0x01, 0x00, 0x00, 0x00]); // mov ebx 1
    asm_write(asm, [0xba, 0x01, 0x00, 0x00, 0x00]); // mov edx 1
    asm_write(asm, [0x0f, 0x05]); // syscall
}
pub fn read_asm(asm: &mut Vec<u8>) {
    asm_write(asm, [0xb8, 0x03, 0x00, 0x00, 0x00]); // mov eax 0
    asm_write(asm, [0xbb, 0x01, 0x00, 0x00, 0x00]); // mov ebx 1
    asm_write(asm, [0xba, 0x01, 0x00, 0x00, 0x00]); // mov edx 1
    asm_write(asm, [0x0f, 0x05]); // syscall
}
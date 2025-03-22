use iced_x86::{IcedError, code_asm::*};
pub const GLINK_SIZE: i32 = 8 + 8;

pub fn emit_gestalt(a: &mut CodeAssembler, root_pc: u64) -> Result<(), IcedError> {
    a.push(r15)?;
    a.sub(r15, 8 + GLINK_SIZE)?;
    a.pop(r15 + 0)?;
    a.push(0u32)?;
    a.push(0u32)?;
    a.pop(r15 + 8)?;
    let mut t0 = a.create_label();
    a.call(t0)?;
    a.set_label(&mut t0)?;
    a.xchg(r15 + 16, r14)?;
    a.push(r12)?;
    a.mov(r12, root_pc)?;
    a.xchg(r15 + 8, r13)?;
    a.cmp(r13, 0)?;
    a.cmove(r13, r12)?;
    a.xchg(r15 + 8, r13)?;
    a.pop(r12)?;
    Ok(())
}
pub fn emit_prologue(
    a: &mut CodeAssembler,
    pc: u64,
) -> Result<impl FnOnce(&mut CodeAssembler) -> Result<(), IcedError> + use<>, IcedError> {
    a.push(r13)?;
    a.mov(r13, pc)?;
    a.cmp(r15 + 8, r13)?;
    a.pop(r13)?;
    let mut l = a.create_label();
    a.jne(l)?;
    Ok(move |a: &mut CodeAssembler| {
        a.jmp(r14)?;
        a.set_label(&mut l)?;

        Ok(())
    })
}
pub fn emit_gestalt_end(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut ret = a.create_label();
    let mut done = a.create_label();
    a.cmp(r15 + 8, 0)?;
    a.je(ret)?;
    a.mov(r15, r15 + 0)?;
    a.jmp(done)?;
    a.set_label(&mut ret)?;
    a.mov(r15 + 16, r14)?;
    a.pop(r14)?;
    a.push(r14)?;
    a.set_label(&mut done)?;
    a.ret()?;
    Ok(())
}

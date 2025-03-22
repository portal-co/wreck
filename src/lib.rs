#![no_std]
#[macro_use]
extern crate alloc;
use embedded_io::{ErrorType, Write};
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Flow {
    CanContinue,
    MaybeUninit,
}
pub trait Target: Write {
    fn pc(&self) -> u64;
}
pub trait Translator: ErrorType {
    fn translate<'a, E: From<Self::Error>>(
        &mut self,
        a: &'a [u8],
        pc: u64,
        res: &mut impl Target<Error = E>,
    ) -> Result<(&'a [u8], Flow), E>;
}
#[cfg(feature = "iced-x86")]
pub fn iced_coded<E, T>(
    a: &mut impl Target<Error = E>,
    mut mapper: impl FnMut(iced_x86::IcedError) -> E,
    bitness: u32,
    go: impl FnOnce(&mut iced_x86::code_asm::CodeAssembler) -> Result<T, E>,
) -> Result<T, E> {
    let mut i = iced_x86::code_asm::CodeAssembler::new(bitness).map_err(&mut mapper)?;
    let v = go(&mut i)?;
    a.write(&{
        let v = i.assemble(a.pc()).map_err(&mut mapper)?;
        // *rpc = rpc.wrapping_add(v.len() as u64);
        v
    })?;
    Ok(v)
}
#[cfg(feature = "iced-x86")]
pub mod x86_64_gestalt;
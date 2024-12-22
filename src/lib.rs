mod memory;
mod binary;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::NonNull;
use crate::memory::{Memory, MemPages};
use crate::binary::loadbin;

#[no_mangle]
pub extern "C" fn loadbin_dpi(fname: *const c_char) -> NonNull<MemPages> {
    let mut memory = MemPages::new();
    let fname_cstr = unsafe{ CStr::from_ptr(fname) };
    let fname_str  = fname_cstr.to_str().unwrap();

    let size = loadbin(fname_str, &mut memory);

    let memobj = Box::new(memory);
    let obj = Box::<MemPages>::into_raw(memobj);
    NonNull::new(obj).unwrap()
}

#[no_mangle]
pub extern "C" fn init_mem() -> NonNull<MemPages> {
    let memobj = Box::new(MemPages::new());
    let obj = Box::<MemPages>::into_raw(memobj);
    NonNull::new(obj).unwrap()
}

#[no_mangle]
pub extern "C" fn mem_read32(ad: u32, ptr: NonNull<MemPages>) -> u32 {
    let memory = unsafe { ptr.as_ref() };
    memory.read32(ad as usize)
}

#[no_mangle]
pub extern "C" fn mem_read16(ad: u32, ptr: NonNull<MemPages>) -> u16 {
    let memory = unsafe { ptr.as_ref() };
    memory.read16(ad as usize)
}

#[no_mangle]
pub extern "C" fn mem_read8(ad: u32, ptr: NonNull<MemPages>) -> u8 {
    let memory = unsafe { ptr.as_ref() };
    memory.read8(ad as usize)
}

#[no_mangle]
pub extern "C" fn mem_write32(ad: u32, dt: u32, mut ptr: NonNull<MemPages>) {
    let memory = unsafe { ptr.as_mut() };
    memory.write32(ad as usize, dt);
}

#[no_mangle]
pub extern "C" fn mem_write16(ad: u32, dt: u16, mut ptr: NonNull<MemPages>) {
    let memory = unsafe { ptr.as_mut() };
    memory.write16(ad as usize, dt);
}

#[no_mangle]
pub extern "C" fn mem_write8(ad: u32, dt: u8, mut ptr: NonNull<MemPages>) {
    let memory = unsafe { ptr.as_mut() };
    memory.write8(ad as usize, dt);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::*;
        use std::ffi::CString;
        use std::os::raw::c_char;

        let fname = "./test.bin";

        let mut memory = MemPages::new();
        let ret = loadbin(fname, &mut memory);
        for i in 0..16 {
            assert_eq!(memory.read8(i), i as u8);
        }
    }
}

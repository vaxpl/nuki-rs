#![allow(dead_code)]

extern crate alloc;

use super::ALIGNMENT;
use nuki_sys::{nk_handle, nk_size};

use std::alloc::Layout;
use std::mem;
use std::os::raw::c_void;

pub unsafe extern "C" fn alloc(_: nk_handle, _: *mut c_void, size: nk_size) -> *mut c_void {
    trace!("allocating {} bytes", size);

    let size_size = mem::size_of::<nk_size>();
    let size = size + size_size as nk_size;

    let memory = std::alloc::alloc(Layout::from_size_align(size as usize, ALIGNMENT).unwrap());
    trace!("allocating {} / {} bytes", size_size, size);

    *(memory as *mut nk_size) = size;
    trace!("allocated {} bytes at {:p}", size, memory);
    memory.offset(size_size as isize) as *mut c_void
}

pub unsafe extern "C" fn free(_: nk_handle, old: *mut c_void) {
    if old.is_null() {
        return;
    }

    let size_size = mem::size_of::<nk_size>();

    let old = old as *mut u8;
    let old = old.offset(-(size_size as isize));
    let old_size = *(old as *const nk_size);

    trace!("deallocating {} bytes from {:p}", old_size, old);

    std::alloc::dealloc(
        old,
        Layout::from_size_align(old_size as usize, ALIGNMENT).unwrap(),
    );
}

mod all_types;

use std::{ffi::c_void, mem::transmute, ops::Deref, slice::from_raw_parts};

use all_types::Conversions;
use bindings::{AllTypesC, Buffer};

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use fuzz_models::AllTypes;

pub fn read_addressbook(buffer: &mut [u8], packed: bool) -> i32 {
    unsafe {
        let len = buffer.len() as i32;
        bindings::read_addressbook(buffer.as_mut_ptr() as *mut c_void, len, packed)
    }
}

pub fn write_addressbook(buffer: &mut [u8], packed: bool) -> i32 {
    unsafe {
        let len = buffer.len() as i32;
        bindings::write_addressbook(buffer.as_mut_ptr() as *mut c_void, len, packed)
    }
}

pub fn serialize_all_types(all_types: &AllTypes, packed: bool) -> BufferWrapper {
    let (all_types_c, _do_not_drop) = AllTypesC::from_rust_struct(all_types);
    let buffer = unsafe {
        bindings::serialize_all_types(&all_types_c as *const _ as *mut AllTypesC, packed)
    };

    BufferWrapper {
        buffer,
        _do_not_drop,
    }
}

#[derive(Debug)]
pub struct BufferWrapper {
    buffer: Buffer,
    _do_not_drop: Conversions,
}

impl Drop for BufferWrapper {
    fn drop(&mut self) {
        unsafe { bindings::drop_buffer(transmute(&self.buffer)) };
    }
}

impl Deref for BufferWrapper {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        let size = unsafe { bindings::get_size(transmute(&self.buffer)) as usize };
        let raw = unsafe { bindings::get_raw_buffer(transmute(&self.buffer)) };
        unsafe { from_raw_parts(raw, size) }
    }
}

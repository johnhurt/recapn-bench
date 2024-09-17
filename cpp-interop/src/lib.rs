use std::ffi::c_void;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

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

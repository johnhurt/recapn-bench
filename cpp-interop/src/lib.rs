use std::ffi::c_void;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn read(buffer: &mut [u8]) -> i32 {
    unsafe {
        let len = buffer.len() as i32;
        bindings::read(buffer.as_mut_ptr() as *mut c_void, len)
    }
}

pub fn write(buffer: &mut [u8]) -> i32 {
    unsafe {
        let len = buffer.len() as i32;
        bindings::write(buffer.as_mut_ptr() as *mut c_void, len)
    }
}

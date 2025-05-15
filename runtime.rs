#[repr(C)]
pub struct RuntimeVec {
    ptr: *const i64,
    len: usize,
}

impl RuntimeVec {
    pub fn from_vec(vec: Vec<u8>) -> Self {
        let boxed = vec.into_boxed_slice();
        let len = boxed.len();
        let ptr = Box::into_raw(boxed) as *const i64;
        RuntimeVec { ptr, len }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn to_chars(ptr: *const u8, len: usize) -> RuntimeVec {
    println!("to_chars called with ptr: {:?}, len: {}", ptr, len);
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    RuntimeVec::from_vec(slice.to_vec()) // your vec wrapper
}

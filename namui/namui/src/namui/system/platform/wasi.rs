use std::ffi::c_void;

const ALIGN: usize = 4;
#[no_mangle]
pub extern "C" fn _malloc(size: usize) -> *mut c_void {
    // make sure, result should be power of 2
    unsafe {
        let aligned_size = (size + (ALIGN - 1)) & !(ALIGN - 1); // round up to nearest multiple of align
        let layout = std::alloc::Layout::from_size_align(aligned_size, ALIGN).unwrap();
        let buf = std::alloc::alloc(layout);
        buf as *mut c_void
    }
}

#[no_mangle]
pub extern "C" fn _free(ptr: *mut c_void) {
    println!("free: {:?}", ptr);
    unsafe {
        std::alloc::dealloc(
            ptr as *mut u8,
            std::alloc::Layout::from_size_align(0, ALIGN).unwrap(),
        );
    }
}

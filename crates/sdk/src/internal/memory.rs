use std::alloc::{alloc, Layout};
use std::mem;

pub struct Slice {
    pub ptr: u32,
    pub len: u32,
}

/// Allows to allocate memory space for a given size. Used when transmitting data from the host to
/// a guest module.
#[no_mangle]
pub extern "C" fn __hbindgen_mem_alloc(size: usize) -> *mut u8 {
    let align = mem::align_of::<usize>();
    if let Ok(layout) = Layout::from_size_align(size, align) {
        unsafe {
            if layout.size() > 0 {
                let ptr = alloc(layout);
                if !ptr.is_null() {
                    return ptr;
                }
            } else {
                return align as *mut u8;
            }
        }
    }

    std::process::abort();
}

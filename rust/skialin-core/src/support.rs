/// Heap-allocates `T` and constructs it in place via `ctor`
pub(crate) fn new_boxed<T>(ctor: unsafe extern "C" fn(*mut T)) -> Box<T> {
    let layout = std::alloc::Layout::new::<T>();
    let ptr = unsafe { std::alloc::alloc(layout) } as *mut T;
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    unsafe { ctor(ptr) };
    unsafe { Box::from_raw(ptr) }
}

/// Heap-allocates `T` and copy-constructs it in place from `src` via `ctor`
/// (a bindgen-generated `T(const T&)` copy constructor).
pub(crate) fn new_boxed_copy<T>(ctor: unsafe extern "C" fn(*mut T, *const T), src: *const T) -> Box<T> {
    let layout = std::alloc::Layout::new::<T>();
    let ptr = unsafe { std::alloc::alloc(layout) } as *mut T;
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    unsafe { ctor(ptr, src) };
    unsafe { Box::from_raw(ptr) }
}

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

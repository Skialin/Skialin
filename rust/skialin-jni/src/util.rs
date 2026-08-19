pub(crate) fn box_ptr<T>(value: T) -> i64 {
    Box::into_raw(Box::new(value)) as i64
}

/// # Safety
/// `ptr` must have come from `box_ptr::<T>` and not have been freed yet.
pub(crate) unsafe fn borrow<'a, T>(ptr: i64) -> &'a T {
    &*(ptr as *mut T)
}

/// # Safety
/// `ptr` must have come from `box_ptr::<T>` and not have been freed yet.
pub(crate) unsafe fn borrow_mut<'a, T>(ptr: i64) -> &'a mut T {
    &mut *(ptr as *mut T)
}

/// # Safety
/// `ptr` must have come from `box_ptr::<T>` and must not be used again after.
pub(crate) unsafe fn drop_ptr<T>(ptr: i64) {
    drop(Box::from_raw(ptr as *mut T));
}

/// # Safety
/// `ptr` must have come from `box_ptr::<T>` and must not be used again after.
pub(crate) unsafe fn take_ptr<T>(ptr: i64) -> T {
    *Box::from_raw(ptr as *mut T)
}

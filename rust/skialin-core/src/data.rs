use crate::sys;

/// An immutable (except via [`Data::writable_bytes`]), ref-counted buffer.
/// Mirrors Skia's `SkData`.
pub struct Data(pub(crate) *mut sys::SkData);

impl Data {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkData) -> Option<Self> {
        (!ptr.is_null()).then_some(Data(ptr))
    }

    pub fn empty() -> Self {
        Data(unsafe { sys::skialin_bridge_Data_makeEmpty() })
    }

    pub fn with_copy(bytes: &[u8]) -> Self {
        Data(unsafe { sys::skialin_bridge_Data_makeWithCopy(bytes.as_ptr().cast(), bytes.len()) })
    }

    /// Uninitialized contents; write through [`Data::writable_bytes`] before
    /// any other reference to this data is taken.
    pub fn uninitialized(length: usize) -> Self {
        Data(unsafe { sys::skialin_bridge_Data_makeUninitialized(length) })
    }

    pub fn zero_initialized(length: usize) -> Self {
        Data(unsafe { sys::skialin_bridge_Data_makeZeroInitialized(length) })
    }

    pub fn from_file(path: &str) -> Option<Self> {
        let path = std::ffi::CString::new(path).ok()?;
        unsafe { Self::from_raw(sys::skialin_bridge_Data_makeFromFileName(path.as_ptr())) }
    }

    pub fn size(&self) -> usize {
        unsafe { (*self.0).size() }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { (*self.0).empty() }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let ptr = (*self.0).bytes();
            std::slice::from_raw_parts(ptr, self.size())
        }
    }

    /// # Safety
    /// The caller must ensure no other reference to this `Data` is used
    /// concurrently, per `SkData::writable_data`'s own caveat.
    pub fn writable_bytes(&mut self) -> &mut [u8] {
        let len = self.size();
        unsafe {
            let ptr = (*self.0).writable_data() as *mut u8;
            std::slice::from_raw_parts_mut(ptr, len)
        }
    }

    pub fn copy_range(&self, offset: usize, length: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; length];
        let copied = unsafe { (*self.0).copyRange(offset, length, buffer.as_mut_ptr().cast()) };
        buffer.truncate(copied);
        buffer
    }

    /// A deep copy of `[offset, offset + length)`, or `None` if out of range.
    pub fn copy_subset(&self, offset: usize, length: usize) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_Data_copySubset(self.0, offset, length)) }
    }

    /// A reference to a subset of this data, sharing the same backing
    /// storage. `None` if out of range.
    pub fn share_subset(&self, offset: usize, length: usize) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_Data_shareSubset(self.0, offset, length)) }
    }

    pub fn equals(&self, other: &Data) -> bool {
        unsafe { (*self.0).equals(other.0) }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Data_unref(self.0) };
    }
}

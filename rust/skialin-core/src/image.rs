use crate::sys;

pub struct Image(pub(crate) *mut sys::SkImage);

impl Image {
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_Image_MakeFromEncoded(bytes.as_ptr(), bytes.len()) };
        (!ptr.is_null()).then_some(Image(ptr))
    }

    pub fn width(&self) -> i32 {
        unsafe { (*self.0).width() }
    }

    pub fn height(&self) -> i32 {
        unsafe { (*self.0).height() }
    }

    pub fn encode_to_png(&self) -> Option<Vec<u8>> {
        let data = unsafe { sys::skialin_bridge_Image_encodeToData(self.0) };
        if data.is_null() {
            return None;
        }
        let bytes = unsafe {
            let ptr = sys::SkData_bytes(data);
            let len = sys::SkData_size(data);
            std::slice::from_raw_parts(ptr, len).to_vec()
        };
        unsafe { sys::skialin_bridge_Data_unref(data) };
        Some(bytes)
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Image_unref(self.0) };
    }
}

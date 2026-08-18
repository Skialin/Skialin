use crate::{sys, Data};

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

    pub fn encode_to_data(&self) -> Option<Data> {
        unsafe { Data::from_raw(sys::skialin_bridge_Image_encodeToData(self.0)) }
    }

    pub fn encode_to_png(&self) -> Option<Vec<u8>> {
        self.encode_to_data().map(|data| data.as_bytes().to_vec())
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Image_unref(self.0) };
    }
}

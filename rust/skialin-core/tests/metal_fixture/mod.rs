//! Metal device/queue/texture creation for the Metal tests, via the plain C entry points of
//! Metal.framework and the Objective-C runtime (no objc crate). Test scaffolding, not part of the
//! shim. Lives in a subdirectory so cargo doesn't build it as its own test target.
#![allow(dead_code)]

use std::ffi::{c_char, c_void};

type Id = *mut c_void;
type Sel = *mut c_void;

#[link(name = "Metal", kind = "framework")]
extern "C" {
    fn MTLCreateSystemDefaultDevice() -> Id;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRelease(object: *const c_void);
}

#[link(name = "objc")]
extern "C" {
    fn objc_getClass(name: *const c_char) -> Id;
    fn sel_registerName(name: *const c_char) -> Sel;
    // Declared untyped: every call site transmutes it to the exact signature of the method it
    // sends, which is what arm64 requires (no variadic objc_msgSend).
    fn objc_msgSend();
}

// MTLPixelFormatBGRA8Unorm, and MTLTextureUsageShaderRead | ShaderWrite | RenderTarget.
const MTL_PIXEL_FORMAT_BGRA8_UNORM: usize = 80;
const MTL_TEXTURE_USAGE_READ_WRITE_RENDER_TARGET: usize = 0x1 | 0x2 | 0x4;

unsafe fn sel(name: &str) -> Sel {
    let name = std::ffi::CString::new(name).unwrap();
    sel_registerName(name.as_ptr())
}

/// Owns one +1 reference to a Metal object, released on drop.
pub struct Retained(pub Id);

impl Drop for Retained {
    fn drop(&mut self) {
        unsafe { CFRelease(self.0) };
    }
}

/// Fields drop in declaration order: the queue goes before the device it came from.
pub struct MetalFixture {
    pub queue: Retained,
    pub device: Retained,
}

impl MetalFixture {
    /// None (skip) when the machine has no Metal device, e.g. a VM without GPU passthrough.
    pub fn new() -> Option<Self> {
        unsafe {
            let device = MTLCreateSystemDefaultDevice();
            if device.is_null() {
                return None;
            }
            let device = Retained(device);
            let new_command_queue: unsafe extern "C" fn(Id, Sel) -> Id = std::mem::transmute(objc_msgSend as unsafe extern "C" fn());
            let queue = new_command_queue(device.0, sel("newCommandQueue"));
            if queue.is_null() {
                return None;
            }
            Some(MetalFixture { device, queue: Retained(queue) })
        }
    }

    /// A BGRA8 render-target-capable 2D texture, owned by the returned `Retained`.
    pub fn create_texture(&self, width: usize, height: usize) -> Option<Retained> {
        unsafe {
            let descriptor_class = objc_getClass(c"MTLTextureDescriptor".as_ptr());
            let make_descriptor: unsafe extern "C" fn(Id, Sel, usize, usize, usize, bool) -> Id =
                std::mem::transmute(objc_msgSend as unsafe extern "C" fn());
            // Autoreleased; there's no pool here, so it simply leaks for the test's lifetime.
            let descriptor = make_descriptor(
                descriptor_class,
                sel("texture2DDescriptorWithPixelFormat:width:height:mipmapped:"),
                MTL_PIXEL_FORMAT_BGRA8_UNORM,
                width,
                height,
                false,
            );
            if descriptor.is_null() {
                return None;
            }
            let set_usage: unsafe extern "C" fn(Id, Sel, usize) = std::mem::transmute(objc_msgSend as unsafe extern "C" fn());
            set_usage(descriptor, sel("setUsage:"), MTL_TEXTURE_USAGE_READ_WRITE_RENDER_TARGET);
            let new_texture: unsafe extern "C" fn(Id, Sel, Id) -> Id = std::mem::transmute(objc_msgSend as unsafe extern "C" fn());
            let texture = new_texture(self.device.0, sel("newTextureWithDescriptor:"), descriptor);
            (!texture.is_null()).then_some(Retained(texture))
        }
    }
}

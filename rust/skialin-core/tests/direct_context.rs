//! Ganesh + OpenGL smoke test. WGL context creation here is Windows-only
//! test scaffolding, not part of the shim; other platforms need their own
//! equivalent (GLX/EGL/CGL) test, or a GLFW-based one like the Kotlin test.
#![cfg(windows)]

use skialin_core::{sys, AlphaType, BackendTexture, ColorType, DirectContext, Image, ImageInfo, Surface, SurfaceOrigin};
use std::ptr;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::Graphics::Gdi::{ReleaseDC, HDC};
use windows_sys::Win32::Graphics::OpenGL::{
    wglCreateContext, wglDeleteContext, wglMakeCurrent, ChoosePixelFormat, SetPixelFormat, HGLRC,
    PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW, PFD_SUPPORT_OPENGL, PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassW, CW_USEDEFAULT, WNDCLASSW,
    WS_OVERLAPPEDWINDOW,
};

/// A hidden window purely to obtain an HDC for WGL; never shown or presented to.
struct GlWindow {
    hwnd: HWND,
    hdc: HDC,
    hglrc: HGLRC,
}

impl GlWindow {
    fn new() -> Self {
        unsafe {
            let class_name: Vec<u16> = "SkialinTestGlWindow\0".encode_utf16().collect();
            let hinstance = GetModuleHandleW(ptr::null());

            let wc = WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(DefWindowProcW),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: hinstance,
                hIcon: ptr::null_mut(),
                hCursor: ptr::null_mut(),
                hbrBackground: ptr::null_mut(),
                lpszMenuName: ptr::null(),
                lpszClassName: class_name.as_ptr(),
            };
            RegisterClassW(&wc);

            let hwnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                class_name.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                1,
                1,
                ptr::null_mut(),
                ptr::null_mut(),
                hinstance,
                ptr::null(),
            );
            assert!(!hwnd.is_null(), "CreateWindowExW failed");

            let hdc = windows_sys::Win32::Graphics::Gdi::GetDC(hwnd);
            assert!(!hdc.is_null(), "GetDC failed");

            let mut pfd: PIXELFORMATDESCRIPTOR = std::mem::zeroed();
            pfd.nSize = std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16;
            pfd.nVersion = 1;
            pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
            pfd.iPixelType = PFD_TYPE_RGBA;
            pfd.cColorBits = 32;
            pfd.cDepthBits = 24;
            pfd.cStencilBits = 8;

            let pixel_format = ChoosePixelFormat(hdc, &pfd);
            assert!(pixel_format != 0, "ChoosePixelFormat failed");
            assert!(SetPixelFormat(hdc, pixel_format, &pfd) != 0, "SetPixelFormat failed");

            let hglrc = wglCreateContext(hdc);
            assert!(!hglrc.is_null(), "wglCreateContext failed");
            assert!(wglMakeCurrent(hdc, hglrc) != 0, "wglMakeCurrent failed");

            GlWindow { hwnd, hdc, hglrc }
        }
    }
}

impl Drop for GlWindow {
    fn drop(&mut self) {
        unsafe {
            wglMakeCurrent(ptr::null_mut(), ptr::null_mut());
            wglDeleteContext(self.hglrc);
            ReleaseDC(self.hwnd, self.hdc);
            DestroyWindow(self.hwnd);
        }
    }
}

#[test]
fn render_target_round_trip() {
    // Dropped last (reverse declaration order): GrDirectContext teardown
    // needs the GL context to still be valid.
    let _window = GlWindow::new();

    let mut context = DirectContext::new_gl().expect("DirectContext::new_gl failed -- no GL driver current?");

    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let mut surface = Surface::new_render_target(&mut context, false, &info, 0, SurfaceOrigin::TopLeft, None, false, false)
        .expect("failed to create GPU render-target surface");

    surface.canvas.clear(0xFFFF0000);
    context.flush();
    context.submit(true);

    let image = surface.image_snapshot().unwrap();
    assert!(image.is_texture_backed());

    let mut pixels = vec![0u8; 16 * 16 * 4];
    let ok = unsafe { image.read_pixels(&info, pixels.as_mut_ptr(), 16 * 4, 0, 0) };
    assert!(ok, "read_pixels failed");

    // ColorType::N32 is Bgra8888: opaque red is B=0, G=0, R=255, A=255.
    assert_eq!(&pixels[0..4], &[0, 0, 255, 255]);
    assert_eq!(&pixels[pixels.len() - 4..], &[0, 0, 255, 255]);
}

// GL 1.1 core functions, always exported by opengl32.dll -- no
// wglGetProcAddress loading needed, unlike GL 1.2+/extension functions.
#[allow(non_snake_case)]
mod gl11 {
    #[link(name = "opengl32")]
    extern "system" {
        pub fn glGenTextures(n: i32, textures: *mut u32);
        pub fn glBindTexture(target: u32, texture: u32);
        pub fn glTexImage2D(target: u32, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: u32, type_: u32, pixels: *const std::ffi::c_void);
        pub fn glTexParameteri(target: u32, pname: u32, param: i32);
    }
}

const GL_TEXTURE_2D: u32 = 0x0DE1;
const GL_RGBA8: i32 = 0x8058;
const GL_RGBA: u32 = 0x1908;
const GL_UNSIGNED_BYTE: u32 = 0x1401;
const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
const GL_NEAREST: i32 = 0x2600;

#[test]
fn adopt_gl_backend_texture_matches_pixels() {
    let _window = GlWindow::new();
    let mut context = DirectContext::new_gl().expect("DirectContext::new_gl failed -- no GL driver current?");

    // Opaque red RGBA8 pixel data, uploaded directly via glTexImage2D --
    // this texture is populated on the CPU side, not drawn into by Skia,
    // to keep this test independent of the render-target path above.
    let pixels = vec![255u8, 0, 0, 255].repeat(16 * 16);
    let mut texture_id = 0u32;
    unsafe {
        gl11::glGenTextures(1, &mut texture_id);
        gl11::glBindTexture(GL_TEXTURE_2D, texture_id);
        gl11::glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
        gl11::glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
        gl11::glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA8, 16, 16, 0, GL_RGBA, GL_UNSIGNED_BYTE, pixels.as_ptr() as *const _);
    }

    let gl_info = sys::GrGLTextureInfo { fTarget: GL_TEXTURE_2D, fID: texture_id, fFormat: GL_RGBA8 as u32, ..Default::default() };
    let backend_texture = BackendTexture::new_gl(16, 16, false, &gl_info, "skialin-gl-adopt-test");
    assert!(backend_texture.is_valid());

    let image = Image::adopt_texture_from(&mut context, &backend_texture, SurfaceOrigin::TopLeft, ColorType::Rgba8888, AlphaType::Premul, None)
        .expect("adopt_texture_from failed");
    assert_eq!(image.width(), 16);
    assert_eq!(image.height(), 16);
    assert!(image.is_texture_backed());

    let info = ImageInfo::new(16, 16, ColorType::Rgba8888, AlphaType::Premul);
    let mut out_pixels = vec![0u8; 16 * 16 * 4];
    let ok = unsafe { image.read_pixels(&info, out_pixels.as_mut_ptr(), 16 * 4, 0, 0) };
    assert!(ok, "read_pixels failed");
    // RGBA8888: opaque red is R=255, G=0, B=0, A=255.
    assert_eq!(&out_pixels[0..4], &[255, 0, 0, 255]);

    // adopt_texture_from takes ownership (Skia calls glDeleteTextures once
    // the image is dropped), so this test must not delete texture_id itself.
    drop(image);
    context.flush();
    context.submit(true);
}

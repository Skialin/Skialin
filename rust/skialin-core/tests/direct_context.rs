//! Ganesh + OpenGL smoke test. Creates a minimal native WGL context (test-only
//! scaffolding, not part of the shim -- production callers create their GL
//! context via LWJGL/GLFW instead), wraps it in a `DirectContext`, draws to a
//! GPU-backed render-target `Surface`, and reads the pixels back.

use skialin_core::{AlphaType, ColorType, DirectContext, ImageInfo, Surface, SurfaceOrigin};
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
    // Declared first so it's dropped last: GL context deletion (and, transitively,
    // GrDirectContext's own GPU resource teardown, since Drop order is
    // last-declared-drops-first) must happen while this window's context is
    // still valid.
    let _window = GlWindow::new();

    let mut context = DirectContext::new_gl().expect("DirectContext::new_gl failed -- no GL driver current?");

    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let mut surface = Surface::new_render_target(&mut context, false, &info, 0, SurfaceOrigin::TopLeft, false, false)
        .expect("failed to create GPU render-target surface");

    surface.canvas().clear(0xFFFF0000);
    context.flush();
    context.submit(true);

    let image = surface.image_snapshot().unwrap();
    assert!(image.is_texture_backed());

    let mut pixels = vec![0u8; 16 * 16 * 4];
    let ok = unsafe { image.read_pixels(&info, pixels.as_mut_ptr(), 16 * 4, 0, 0) };
    assert!(ok, "read_pixels failed");

    // ColorType::N32 is Bgra8888 on this platform: bytes are B, G, R, A.
    // Premul opaque red -> B=0, G=0, R=255, A=255.
    assert_eq!(&pixels[0..4], &[0, 0, 255, 255]);
    assert_eq!(&pixels[pixels.len() - 4..], &[0, 0, 255, 255]);
}

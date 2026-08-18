//! Ganesh + Vulkan smoke test. Instance/device/queue creation uses `ash`
//! (plain Vulkan bindings, not part of the shim) since it's the same on
//! every platform -- unlike the GL test, this isn't Windows-specific.

use ash::vk;
use skialin_core::sys;
use skialin_core::{AlphaType, ColorType, DirectContext, ImageInfo, Surface, SurfaceOrigin};
use std::ffi::{c_char, c_void};

// Raw-pointer signatures (rather than ash's typed `vk::Instance`/`vk::Device`)
// so a VK_NULL_HANDLE argument -- used for global functions like
// vkEnumerateInstanceVersion -- stays a plain null pointer instead of
// risking UB constructing a NonZero-backed ash handle from zero.
type RawGetInstanceProcAddr = unsafe extern "system" fn(*mut c_void, *const c_char) -> sys::PFN_vkVoidFunction;
type RawGetDeviceProcAddr = unsafe extern "system" fn(*mut c_void, *const c_char) -> sys::PFN_vkVoidFunction;

struct ProcAddrs {
    get_instance_proc_addr: RawGetInstanceProcAddr,
    get_device_proc_addr: RawGetDeviceProcAddr,
}

struct VulkanFixture {
    _entry: ash::Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    queue: vk::Queue,
    queue_family_index: u32,
    proc_addrs: ProcAddrs,
}

impl VulkanFixture {
    fn new() -> Option<Self> {
        let entry = unsafe { ash::Entry::load() }.ok()?;
        let get_instance_proc_addr: RawGetInstanceProcAddr = unsafe { std::mem::transmute(entry.static_fn().get_instance_proc_addr) };

        let app_info = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_1);
        let instance_info = vk::InstanceCreateInfo::default().application_info(&app_info);
        let instance = unsafe { entry.create_instance(&instance_info, None) }.ok()?;
        let get_device_proc_addr: RawGetDeviceProcAddr = unsafe { std::mem::transmute(instance.fp_v1_0().get_device_proc_addr) };

        let physical_device = *unsafe { instance.enumerate_physical_devices() }.ok()?.first()?;

        let queue_family_index = unsafe { instance.get_physical_device_queue_family_properties(physical_device) }
            .iter()
            .position(|props| props.queue_flags.contains(vk::QueueFlags::GRAPHICS))?
            as u32;

        let queue_priorities = [1.0f32];
        let queue_info = vk::DeviceQueueCreateInfo::default().queue_family_index(queue_family_index).queue_priorities(&queue_priorities);
        let device_info = vk::DeviceCreateInfo::default().queue_create_infos(std::slice::from_ref(&queue_info));
        let device = unsafe { instance.create_device(physical_device, &device_info, None) }.ok()?;

        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        Some(VulkanFixture {
            _entry: entry,
            instance,
            physical_device,
            device,
            queue,
            queue_family_index,
            proc_addrs: ProcAddrs { get_instance_proc_addr, get_device_proc_addr },
        })
    }

    fn direct_context(&self, physical_device: vk::PhysicalDevice) -> Option<DirectContext> {
        unsafe extern "C" fn get_proc(ctx: *mut c_void, name: *const c_char, instance: sys::VkInstance, device: sys::VkDevice) -> sys::PFN_vkVoidFunction {
            let proc_addrs = &*(ctx as *const ProcAddrs);
            if !device.is_null() {
                (proc_addrs.get_device_proc_addr)(device as *mut c_void, name)
            } else {
                (proc_addrs.get_instance_proc_addr)(instance as *mut c_void, name)
            }
        }

        unsafe {
            DirectContext::new_vulkan(
                std::mem::transmute(self.instance.handle()),
                std::mem::transmute(physical_device),
                std::mem::transmute(self.device.handle()),
                std::mem::transmute(self.queue),
                self.queue_family_index,
                vk::API_VERSION_1_1,
                &self.proc_addrs as *const ProcAddrs as *mut c_void,
                Some(get_proc),
                false,
            )
        }
    }
}

impl Drop for VulkanFixture {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

#[test]
fn render_target_round_trip() {
    let Some(fixture) = VulkanFixture::new() else {
        eprintln!("skipping: no Vulkan runtime/driver available on this machine");
        return;
    };
    let mut context = fixture.direct_context(fixture.physical_device).expect("DirectContext::new_vulkan failed");

    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let mut surface = Surface::new_render_target(&mut context, false, &info, 0, SurfaceOrigin::TopLeft, None, false, false)
        .expect("failed to create GPU render-target surface");

    surface.canvas().clear(0xFFFF0000);
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

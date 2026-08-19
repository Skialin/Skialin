//! Ganesh + Vulkan smoke test. Instance/device/queue creation uses `ash`
//! (plain Vulkan bindings, not part of the shim) since it's the same on
//! every platform -- unlike the GL test, this isn't Windows-specific.

use ash::vk;
use skialin_core::sys;
use skialin_core::{AlphaType, BackendTexture, ColorType, DirectContext, Image, ImageInfo, Surface, SurfaceOrigin};
use std::ffi::{c_char, c_void};

// Raw-pointer signatures (rather than ash's typed `vk::Instance`/`vk::Device`)
// so a VK_NULL_HANDLE argument -- used for global functions like
// vkEnumerateInstanceVersion -- stays a plain null pointer instead of
// risking UB constructing a NonZero-backed ash handle from zero.
type RawGetInstanceProcAddr = unsafe extern "system" fn(*mut c_void, *const c_char) -> sys::PFN_vkVoidFunction;
type RawGetDeviceProcAddr = unsafe extern "system" fn(*mut c_void, *const c_char) -> sys::PFN_vkVoidFunction;

#[derive(Clone, Copy)]
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

    /// A caller-owned VkImage + backing memory, the same shape a real
    /// embedder (e.g. a windowing/compositor integration) would hand Skia
    /// to render into. Skia neither allocates nor frees these.
    fn create_image(&self, width: u32, height: u32, format: vk::Format) -> Option<(vk::Image, vk::DeviceMemory)> {
        let image_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(format)
            .extent(vk::Extent3D { width, height, depth: 1 })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED);
        let image = unsafe { self.device.create_image(&image_info, None) }.ok()?;

        let requirements = unsafe { self.device.get_image_memory_requirements(image) };
        let memory_properties = unsafe { self.instance.get_physical_device_memory_properties(self.physical_device) };
        let memory_type_index = (0..memory_properties.memory_type_count).find(|&i| requirements.memory_type_bits & (1 << i) != 0)?;

        let alloc_info = vk::MemoryAllocateInfo::default().allocation_size(requirements.size).memory_type_index(memory_type_index);
        let memory = unsafe { self.device.allocate_memory(&alloc_info, None) }.ok()?;
        unsafe { self.device.bind_image_memory(image, memory, 0) }.ok()?;

        Some((image, memory))
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
                Box::new(self.proc_addrs),
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

#[test]
fn wrap_backend_texture_round_trip() {
    let Some(fixture) = VulkanFixture::new() else {
        eprintln!("skipping: no Vulkan runtime/driver available on this machine");
        return;
    };
    let mut context = fixture.direct_context(fixture.physical_device).expect("DirectContext::new_vulkan failed");

    let format = vk::Format::B8G8R8A8_UNORM;
    let (image, memory) = fixture.create_image(16, 16, format).expect("create_image failed");

    let vk_image_info = sys::GrVkImageInfo {
        fImage: unsafe { std::mem::transmute(image) },
        fImageTiling: sys::VkImageTiling_VK_IMAGE_TILING_OPTIMAL,
        fImageLayout: sys::VkImageLayout_VK_IMAGE_LAYOUT_UNDEFINED,
        fFormat: sys::VkFormat_VK_FORMAT_B8G8R8A8_UNORM,
        fImageUsageFlags: (vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED).as_raw(),
        fSampleCount: 1,
        fLevelCount: 1,
        fCurrentQueueFamily: sys::VK_QUEUE_FAMILY_IGNORED as u32,
        fSharingMode: sys::VkSharingMode_VK_SHARING_MODE_EXCLUSIVE,
        ..Default::default()
    };
    let backend_texture = BackendTexture::new_vk(16, 16, &vk_image_info, "skialin-test-texture");
    assert!(backend_texture.is_valid());

    let mut surface = Surface::wrap_backend_texture(&mut context, &backend_texture, SurfaceOrigin::TopLeft, 0, ColorType::N32, None, None)
        .expect("wrap_backend_texture failed");

    surface.canvas.clear(0xFFFF0000);
    context.flush();
    context.submit(true);

    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let image_snapshot = surface.image_snapshot().unwrap();
    assert!(image_snapshot.is_texture_backed());

    let mut pixels = vec![0u8; 16 * 16 * 4];
    let ok = unsafe { image_snapshot.read_pixels(&info, pixels.as_mut_ptr(), 16 * 4, 0, 0) };
    assert!(ok, "read_pixels failed");
    assert_eq!(&pixels[0..4], &[0, 0, 255, 255]);
    assert_eq!(&pixels[pixels.len() - 4..], &[0, 0, 255, 255]);

    drop(surface);
    drop(image_snapshot);
    drop(backend_texture);
    context.flush();
    context.submit(true);
    unsafe {
        fixture.device.destroy_image(image, None);
        fixture.device.free_memory(memory, None);
    }
}

#[test]
fn adopt_texture_from_wraps_gpu_image() {
    let Some(fixture) = VulkanFixture::new() else {
        eprintln!("skipping: no Vulkan runtime/driver available on this machine");
        return;
    };
    let mut context = fixture.direct_context(fixture.physical_device).expect("DirectContext::new_vulkan failed");

    let format = vk::Format::B8G8R8A8_UNORM;
    let (image, memory) = fixture.create_image(16, 16, format).expect("create_image failed");
    let requirements = unsafe { fixture.device.get_image_memory_requirements(image) };

    // check_image_info (GrVkGpu.cpp) requires fAlloc.fMemory whenever
    // ownership is adopted, since Skia needs it to free the memory once it
    // destroys the image -- unlike wrap_backend_texture_round_trip, which
    // borrows and so can leave fAlloc unset.
    let vk_image_info = sys::GrVkImageInfo {
        fImage: unsafe { std::mem::transmute(image) },
        fAlloc: sys::skgpu::VulkanAlloc {
            fMemory: unsafe { std::mem::transmute(memory) },
            fOffset: 0,
            fSize: requirements.size,
            fFlags: 0,
            fBackendMemory: 0,
            fUsesSystemHeap: false,
        },
        fImageTiling: sys::VkImageTiling_VK_IMAGE_TILING_OPTIMAL,
        fImageLayout: sys::VkImageLayout_VK_IMAGE_LAYOUT_UNDEFINED,
        fFormat: sys::VkFormat_VK_FORMAT_B8G8R8A8_UNORM,
        fImageUsageFlags: (vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED).as_raw(),
        fSampleCount: 1,
        fLevelCount: 1,
        fCurrentQueueFamily: sys::VK_QUEUE_FAMILY_IGNORED as u32,
        fSharingMode: sys::VkSharingMode_VK_SHARING_MODE_EXCLUSIVE,
        ..Default::default()
    };
    let backend_texture = BackendTexture::new_vk(16, 16, &vk_image_info, "skialin-adopt-test");
    assert!(backend_texture.is_valid());

    // AdoptTextureFrom takes ownership of both the VkImage and (per fAlloc
    // above) the VkDeviceMemory: Skia will vkDestroyImage and free the
    // memory itself once the returned Image is dropped, so this test must
    // not destroy either manually afterward.
    let image = Image::adopt_texture_from(&mut context, &backend_texture, SurfaceOrigin::TopLeft, ColorType::N32, AlphaType::Premul, None)
        .expect("adopt_texture_from failed");
    assert_eq!(image.width(), 16);
    assert_eq!(image.height(), 16);
    assert!(image.is_texture_backed());

    drop(image);
    context.flush();
    context.submit(true);
}

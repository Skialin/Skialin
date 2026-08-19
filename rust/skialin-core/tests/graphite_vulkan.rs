//! Graphite + Vulkan smoke test. Mirrors direct_context_vulkan.rs's ash-based
//! instance/device setup.

use ash::vk;
use skialin_core::sys;
use skialin_core::{AlphaType, ColorType, GraphiteBackendTexture, GraphiteContext, Image, ImageInfo, SurfaceOrigin};
use std::ffi::{c_char, c_void};

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

    fn graphite_context(&self) -> Option<GraphiteContext> {
        unsafe extern "C" fn get_proc(ctx: *mut c_void, name: *const c_char, instance: sys::VkInstance, device: sys::VkDevice) -> sys::PFN_vkVoidFunction {
            let proc_addrs = &*(ctx as *const ProcAddrs);
            if !device.is_null() {
                (proc_addrs.get_device_proc_addr)(device as *mut c_void, name)
            } else {
                (proc_addrs.get_instance_proc_addr)(instance as *mut c_void, name)
            }
        }

        unsafe {
            GraphiteContext::new_vulkan(
                std::mem::transmute(self.instance.handle()),
                std::mem::transmute(self.physical_device),
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
    let mut context = fixture.graphite_context().expect("GraphiteContext::new_vulkan failed");
    let mut recorder = context.make_recorder().expect("make_recorder failed");

    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let mut surface = skialin_core::Surface::new_graphite_render_target(&mut recorder, &info, false, None).expect("failed to create Graphite render-target surface");

    surface.canvas.clear(0xFFFF0000);

    let mut recording = recorder.snap().expect("snap failed");
    let status = context.insert_recording(&mut recording, &mut surface);
    assert_eq!(status, 0, "insertRecording failed with status {status}");
    assert!(context.submit(true), "submit failed");

    // Graphite deprecated the synchronous SkImage::readPixels (and
    // makeNonTextureImage) path for GPU-backed images entirely, in favor of
    // Context::asyncRescaleAndReadPixels (not yet bridged) -- both reliably
    // fail here even though the draw/insertRecording/submit pipeline above
    // succeeded. So this test only verifies that pipeline, not pixel
    // contents; see direct_context_vulkan.rs's Ganesh test for a full
    // pixel-value round trip.
    let image = surface.image_snapshot().unwrap();
    assert!(image.is_texture_backed());
}

#[test]
fn wrap_texture_creates_valid_image() {
    let Some(fixture) = VulkanFixture::new() else {
        eprintln!("skipping: no Vulkan runtime/driver available on this machine");
        return;
    };
    let mut context = fixture.graphite_context().expect("GraphiteContext::new_vulkan failed");
    let mut recorder = context.make_recorder().expect("make_recorder failed");

    let format = vk::Format::B8G8R8A8_UNORM;
    let (image, memory) = fixture.create_image(16, 16, format).expect("create_image failed");

    let backend_texture = GraphiteBackendTexture::new_vk(
        16,
        16,
        1,
        false,
        0,
        sys::VkFormat_VK_FORMAT_B8G8R8A8_UNORM,
        sys::VkImageTiling_VK_IMAGE_TILING_OPTIMAL,
        (vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED).as_raw(),
        sys::VkSharingMode_VK_SHARING_MODE_EXCLUSIVE,
        vk::ImageAspectFlags::COLOR.as_raw(),
        sys::VkImageLayout_VK_IMAGE_LAYOUT_UNDEFINED,
        sys::VK_QUEUE_FAMILY_IGNORED as u32,
        unsafe { std::mem::transmute(image) },
        std::ptr::null_mut(),
        0,
        0,
        0,
    );
    assert!(backend_texture.is_valid());

    // WrapTexture doesn't take ownership (unlike Ganesh's AdoptTextureFrom),
    // so the caller destroys the image/memory itself once done.
    let wrapped = Image::wrap_graphite_texture(&mut recorder, &backend_texture, AlphaType::Premul, None, SurfaceOrigin::TopLeft, false)
        .expect("wrap_graphite_texture failed");
    assert_eq!(wrapped.width(), 16);
    assert_eq!(wrapped.height(), 16);
    assert!(wrapped.is_texture_backed());

    drop(wrapped);
    drop(backend_texture);
    context.submit(true);
    unsafe {
        fixture.device.destroy_image(image, None);
        fixture.device.free_memory(memory, None);
    }
}

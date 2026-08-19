package org.skialin

import org.lwjgl.system.MemoryStack
import org.lwjgl.vulkan.VK10.VK_QUEUE_GRAPHICS_BIT
import org.lwjgl.vulkan.VK10.VK_STRUCTURE_TYPE_APPLICATION_INFO
import org.lwjgl.vulkan.VK10.VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO
import org.lwjgl.vulkan.VK10.VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO
import org.lwjgl.vulkan.VK10.VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO
import org.lwjgl.vulkan.VK10.VK_SUCCESS
import org.lwjgl.vulkan.VK10.vkCreateDevice
import org.lwjgl.vulkan.VK10.vkCreateInstance
import org.lwjgl.vulkan.VK10.vkDestroyDevice
import org.lwjgl.vulkan.VK10.vkDestroyInstance
import org.lwjgl.vulkan.VK10.vkEnumeratePhysicalDevices
import org.lwjgl.vulkan.VK10.vkGetDeviceQueue
import org.lwjgl.vulkan.VK10.vkGetPhysicalDeviceQueueFamilyProperties
import org.lwjgl.vulkan.VK11.VK_API_VERSION_1_1
import org.lwjgl.vulkan.VkApplicationInfo
import org.lwjgl.vulkan.VkDevice
import org.lwjgl.vulkan.VkDeviceCreateInfo
import org.lwjgl.vulkan.VkDeviceQueueCreateInfo
import org.lwjgl.vulkan.VkInstance
import org.lwjgl.vulkan.VkInstanceCreateInfo
import org.lwjgl.vulkan.VkPhysicalDevice
import org.lwjgl.vulkan.VkQueueFamilyProperties
import java.nio.ByteBuffer
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlin.test.fail

/** Ganesh + Vulkan smoke test. LWJGL's Vulkan bindings provide the native
 * instance/device/queue that [DirectContext.makeVulkan] wraps. */
class DirectContextVulkanTest {
    @Test
    fun renderTargetRoundTrip() {
        MemoryStack.stackPush().use { stack ->
            val appInfo = VkApplicationInfo.calloc(stack).sType(VK_STRUCTURE_TYPE_APPLICATION_INFO).apiVersion(VK_API_VERSION_1_1)
            val instanceInfo = VkInstanceCreateInfo.calloc(stack).sType(VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO).pApplicationInfo(appInfo)

            val pInstance = stack.mallocPointer(1)
            val created =
                try {
                    vkCreateInstance(instanceInfo, null, pInstance)
                } catch (e: LinkageError) {
                    println("skipping: Vulkan natives unavailable on this machine ($e)")
                    return
                }
            if (created != VK_SUCCESS) {
                println("skipping: no Vulkan runtime/driver available on this machine")
                return
            }
            val instance = VkInstance(pInstance[0], instanceInfo)
            try {
                val pCount = stack.mallocInt(1)
                vkEnumeratePhysicalDevices(instance, pCount, null)
                if (pCount[0] == 0) fail("no Vulkan physical devices")
                val pDevices = stack.mallocPointer(pCount[0])
                vkEnumeratePhysicalDevices(instance, pCount, pDevices)
                val physicalDevice = VkPhysicalDevice(pDevices[0], instance)

                val pFamilyCount = stack.mallocInt(1)
                vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, pFamilyCount, null)
                val families = VkQueueFamilyProperties.calloc(pFamilyCount[0], stack)
                vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, pFamilyCount, families)
                val graphicsQueueFamilyIndex =
                    (0 until families.capacity()).firstOrNull { families[it].queueFlags() and VK_QUEUE_GRAPHICS_BIT != 0 }
                        ?: fail("no graphics queue family")

                val queueInfo = VkDeviceQueueCreateInfo.calloc(1, stack)
                queueInfo[0]
                    .sType(VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO)
                    .queueFamilyIndex(graphicsQueueFamilyIndex)
                    .pQueuePriorities(stack.floats(1.0f))

                val deviceInfo = VkDeviceCreateInfo.calloc(stack).sType(VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO).pQueueCreateInfos(queueInfo)

                val pDevice = stack.mallocPointer(1)
                if (vkCreateDevice(physicalDevice, deviceInfo, null, pDevice) != VK_SUCCESS) fail("vkCreateDevice failed")
                val device = VkDevice(pDevice[0], physicalDevice, deviceInfo)
                try {
                    val pQueue = stack.mallocPointer(1)
                    vkGetDeviceQueue(device, graphicsQueueFamilyIndex, 0, pQueue)

                    val context =
                        DirectContext.makeVulkan(
                            instance.address(),
                            physicalDevice.address(),
                            device.address(),
                            pQueue[0],
                            graphicsQueueFamilyIndex,
                            VK_API_VERSION_1_1,
                        ) ?: fail("DirectContext.makeVulkan failed")

                    context.use {
                        val info = ImageInfo.make(16, 16, ColorType.N32, AlphaType.PREMUL)
                        val surface =
                            Surface.makeRenderTarget(
                                context,
                                budgeted = false,
                                info = info,
                                sampleCount = 0,
                                surfaceOrigin = SurfaceOrigin.TOP_LEFT,
                            ) ?: fail("makeRenderTarget failed")
                        surface.use {
                            surface.canvas.clear(Colors.RED)
                            context.flush()
                            context.submit(syncCpu = true)

                            surface.makeImageSnapshot()!!.use { image ->
                                assertTrue(image.isTextureBacked)
                                val buffer = ByteBuffer.allocateDirect(16 * 16 * 4)
                                assertTrue(image.readPixels(info, buffer, 16L * 4))
                                // ColorType.N32 is BGRA_8888: opaque red -> B=0, G=0, R=255, A=255.
                                assertEquals(0, buffer.get(0).toInt() and 0xFF)
                                assertEquals(0, buffer.get(1).toInt() and 0xFF)
                                assertEquals(255, buffer.get(2).toInt() and 0xFF)
                                assertEquals(255, buffer.get(3).toInt() and 0xFF)
                            }
                        }
                    }
                } finally {
                    vkDestroyDevice(device, null)
                }
            } finally {
                vkDestroyInstance(instance, null)
            }
        }
    }
}

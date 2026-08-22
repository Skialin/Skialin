package org.skialin

/**
 * Resolves Vulkan entry points for a Skia context, in place of the platform
 * loader Skia would otherwise use. Passed to [DirectContext.makeVulkan] /
 * [GraphiteContext.makeVulkan].
 *
 * Skia calls this with either a non-zero `instance` (instance-level lookup,
 * i.e. `vkGetInstanceProcAddr`) or a non-zero `device` (device-level lookup,
 * i.e. `vkGetDeviceProcAddr`) -- never both. Once Skia has a device it stops
 * passing the instance, so a device-level name must not be re-derived from
 * the instance argument.
 *
 * Called from whichever thread Skia happens to be on, including during
 * context construction and at arbitrary later points, for as long as the
 * context is alive -- so implementations must be thread-safe and must not
 * capture anything shorter-lived than the context. Return 0 for an
 * unresolvable name; throwing is reported to stderr and treated as 0.
 */
fun interface VulkanGetProc {
    /** @return the native address of `name`, or 0 if it can't be resolved. */
    fun getProc(
        name: String,
        instance: Long,
        device: Long,
    ): Long
}

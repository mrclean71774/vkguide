use std::fmt;

// Error handling will be different from the tutorial because Rust.
#[derive(Debug)]
pub enum Error {
  FromGltf(gltf::Error),       // map_err from gltf
  FromVkcboot(vkcboot::Error), // map_err from vkcboot
  FromIO(std::io::Error),      // map_err from std::io::Error
  Str(&'static str),           // error with &str message
  String(String),              // error with String message
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::FromGltf(e) => fmt::Display::fmt(&e, f),
      Error::FromVkcboot(e) => fmt::Display::fmt(&e, f),
      Error::FromIO(e) => fmt::Display::fmt(&e, f),
      Error::Str(s) => fmt::Display::fmt(&s, f),
      Error::String(s) => fmt::Display::fmt(&s, f),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Error::FromGltf(e) => Some(e),
      Error::FromVkcboot(e) => Some(e),
      Error::FromIO(e) => Some(e),
      Error::Str(_) => None,
      Error::String(_) => None,
    }
  }
}

// Expanding on the tutorials macro by showing what the error code means.
#[macro_export]
macro_rules! VK_CHECK {
  ($func:expr) => {
    let err: i32 = $func;
    if err != vkcapi::core::v1_0::VK_SUCCESS {
      let s: &str;
      match err {
        VK_NOT_READY => s = "VK_NOT_READY",
        VK_TIMEOUT => s = "VK_TIMEOUT",
        VK_EVENT_SET => s = "VK_EVENT_SET",
        VK_EVENT_RESET => s = "VK_EVENT_RESET",
        VK_INCOMPLETE => s = "VK_INCOMPLETE",
        VK_ERROR_OUT_OF_HOST_MEMORY => s = "VK_ERROR_OUT_OF_HOST_MEMORY",
        VK_ERROR_OUT_OF_DEVICE_MEMORY => s = "VK_ERROR_OUT_OF_DEVICE_MEMORY",
        VK_ERROR_INITIALIZATION_FAILED => s = "VK_ERROR_INITIALIZATION_FAILED",
        VK_ERROR_DEVICE_LOST => s = "VK_ERROR_DEVICE_LOST",
        VK_ERROR_MEMORY_MAP_FAILED => s = "VK_ERROR_MEMORY_MAP_FAILED",
        VK_ERROR_LAYER_NOT_PRESENT => s = "VK_ERROR_LAYER_NOT_PRESENT",
        VK_ERROR_EXTENSION_NOT_PRESENT => s = "VK_ERROR_EXTENSION_NOT_PRESENT",
        VK_ERROR_FEATURE_NOT_PRESENT => s = "VK_ERROR_FEATURE_NOT_PRESENT",
        VK_ERROR_INCOMPATIBLE_DRIVER => s = "VK_ERROR_INCOMPATIBLE_DRIVER",
        VK_ERROR_TOO_MANY_OBJECTS => s = "VK_ERROR_TOO_MANY_OBJECTS",
        VK_ERROR_FORMAT_NOT_SUPPORTED => s = "VK_ERROR_FORMAT_NOT_SUPPORTED",
        VK_ERROR_FRAGMENTED_POOL => s = "VK_ERROR_FRAGMENTED_POOL",
        VK_ERROR_UNKNOWN => s = "VK_ERROR_UNKNOWN",
        VK_ERROR_OUT_OF_POOL_MEMORY => s = "VK_ERROR_OUT_OF_POOL_MEMORY",
        VK_ERROR_INVALID_EXTERNAL_HANDLE => s = "VK_ERROR_INVALID_EXTERNAL_HANDLE",
        VK_ERROR_FRAGMENTATION => s = "VK_ERROR_FRAGMENTATION",
        VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS => s = "VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS",
        VK_ERROR_SURFACE_LOST_KHR => s = "VK_ERROR_SURFACE_LOST_KHR",
        VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => s = "VK_ERROR_NATIVE_WINDOW_IN_USE_KHR",
        VK_SUBOPTIMAL_KHR => s = "VK_SUBOPTIMAL_KHR",
        VK_ERROR_OUT_OF_DATE_KHR => s = "VK_ERROR_OUT_OF_DATE_KHR",
        VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => s = "VK_ERROR_INCOMPATIBLE_DISPLAY_KHR",
        VK_ERROR_VALIDATION_FAILED_EXT => s = "VK_ERROR_VALIDATION_FAILED_EXT",
        VK_ERROR_INVALID_SHADER_NV => s = "VK_ERROR_INVALID_SHADER_NV",
        VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT => {
          s = "VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT"
        }
        VK_ERROR_NOT_PERMITTED_EXT => s = "VK_ERROR_NOT_PERMITTED_EXT",
        VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => {
          s = "VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT"
        }
        VK_THREAD_IDLE_KHR => s = "VK_THREAD_IDLE_KHR",
        VK_THREAD_DONE_KHR => s = "VK_THREAD_DONE_KHR",
        VK_OPERATION_DEFERRED_KHR => s = "VK_OPERATION_DEFERRED_KHR",
        VK_OPERATION_NOT_DEFERRED_KHR => s = "VK_OPERATION_NOT_DEFERRED_KHR",
        VK_PIPELINE_COMPILE_REQUIRED_EXT => s = "VK_PIPELINE_COMPILE_REQUIRED_EXT",
        _ => s = "This should never happen!?!?",
      }
      panic!("Vulkan error: {}", s);
    }
  };
}

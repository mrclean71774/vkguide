use {
  crate::{error::Error, VK_CHECK},
  sdl2::*,
  std::{
    mem::zeroed,
    ptr::{null, null_mut},
  },
  vkcapi::{
    core::v1_0::*,
    ext::{vk_khr_surface::*, vk_khr_swapchain::*},
  },
};

#[cfg(feature = "validation")]
use vkcapi::ext::vk_ext_debug_utils::*;

pub struct VulkanEngine {
  is_initialized: bool,
  frame_number: i32,

  window_extent: VkExtent2D,
  window: *mut SDL_Window,

  instance: vkcapi::core::v1_0::VkInstance, // Vulkan library handle
  #[cfg(feature = "validation")]
  debug_messenger: VkDebugUtilsMessengerEXT, // Vulkan debug output handle

  chosen_gpu: VkPhysicalDevice, // GPU chosen as the default device
  device: VkDevice,             // Vulkan device for commands
  surface: vkcapi::ext::vk_khr_surface::VkSurfaceKHR, // Vulkan window surface

  graphics_queue: VkQueue,   // Queue for graphics commands
  graphics_queue_index: u32, // index of graphics queue
  present_queue: VkQueue,    // Queue for presentation to surface
  present_queue_index: u32,  // index of presentation queue

  swapchain: VkSwapchainKHR,
  swapchain_format: VkFormat, // image format expected by windowing system
  swapchain_images: Vec<VkImage>, // array of images from the swapchain
  swapchain_image_views: Vec<VkImageView>, // array of image-views from the swapchain
}

impl VulkanEngine {
  pub fn new() -> VulkanEngine {
    VulkanEngine {
      is_initialized: false,
      frame_number: 0,

      window_extent: VkExtent2D {
        width: 1700,
        height: 900,
      },
      window: null_mut(),

      instance: null(),
      #[cfg(feature = "validation")]
      debug_messenger: null(),

      chosen_gpu: null(),
      device: null(),
      surface: null(),

      graphics_queue: null(),
      graphics_queue_index: u32::MAX, // zero is an actual queue index
      present_queue: null(),
      present_queue_index: u32::MAX, // max seems like a reasonable value for un-init

      swapchain: null(),
      swapchain_format: unsafe { zeroed() },
      swapchain_images: Vec::new(),
      swapchain_image_views: Vec::new(),
    }
  }

  // initializes everything in the engine
  pub fn init(&mut self) -> Result<(), Error> {
    // We initialize SDL and create a window with it.
    unsafe {
      SDL_Init(SDL_INIT_VIDEO);
      let window_flags = SDL_WINDOW_VULKAN;

      // create blank window for our application
      self.window = SDL_CreateWindow(
        b"Vulkan Engine\0".as_ptr() as *const i8, // window title
        SDL_WINDOWPOS_UNDEFINED_MASK as i32,      // window position x (don't care)
        SDL_WINDOWPOS_UNDEFINED_MASK as i32,      // window position y (don't care)
        self.window_extent.width as i32,          // window width in pixels
        self.window_extent.height as i32,         // window height in pixels
        window_flags,
      );
    }

    // load the core Vulkan structures
    self.init_vulkan()?;

    // create the swapchain
    self.init_swapchain()?;

    // everything went fine
    self.is_initialized = true;

    Ok(())
  }

  // shuts down the engine
  pub fn cleanup(&mut self) {
    if self.is_initialized {
      unsafe {
        vkDestroySwapchainKHR(self.device, self.swapchain, null());

        self
          .swapchain_image_views
          .iter()
          .for_each(|iv| vkDestroyImageView(self.device, *iv, null()));

        vkDestroyDevice(self.device, null());
        vkDestroySurfaceKHR(self.instance, self.surface, null());

        #[cfg(feature = "validation")]
        vkDestroyDebugUtilsMessengerEXT(self.instance, self.debug_messenger, null());

        vkDestroyInstance(self.instance, null());

        SDL_DestroyWindow(self.window);
        SDL_Quit(); // why doesn't the tutorial do this?
      }
    }
  }

  // draw loop
  fn draw(&mut self) {
    // nothing yet
  }

  // run main loop
  pub fn run(&mut self) {
    let mut e: SDL_Event = unsafe { zeroed() };
    let mut b_quit: bool = false;

    // main loop
    while !b_quit {
      // Handle events on queue
      while unsafe { SDL_PollEvent(&mut e) } != 0 {
        // close the window when user clicks the X button or alt-f4s
        if unsafe { e.type_ } == SDL_QUIT {
          b_quit = true;
        }
      }
      self.draw();
    }
  }

  fn init_vulkan(&mut self) -> Result<(), Error> {
    // vkcboot is somewhat different and probably inferior to vk_bootstrap
    // but it works for the purpose of this turorial. It is based on code from
    // https://vulkan-tutorial.com
    self.instance = vkcboot::InstanceBuilder::new(self.window)
      .with_version(1, 1)
      .build()
      .map_err(|e| Error::FromVkcboot(e))?;
    // using validation feature to turn validation layers on/off same as vkcboot
    #[cfg(feature = "validation")]
    {
      self.debug_messenger =
        vkcboot::DebugMessenger::new(self.instance).map_err(|e| Error::FromVkcboot(e))?;
    }

    // vkcboot uses sdl2 to get surface
    self.surface = vkcboot::Surface::new(self.window, self.instance);

    let device = vkcboot::DeviceBuilder::new(self.instance, self.surface)
      .with_version(1, 1)
      .build()
      .map_err(|e| Error::FromVkcboot(e))?;

    self.chosen_gpu = device.physical_device;
    self.device = device.device;

    // we get the queues a little earlier than the tutorial
    self.graphics_queue = device.graphics_queue;
    self.graphics_queue_index = device.graphics_queue_index;

    self.present_queue = device.present_queue;
    self.present_queue_index = device.present_queue_index;
    Ok(())
  }

  fn init_swapchain(&mut self) -> Result<(), Error> {
    // VK_PRESENT_MODE_FIFO is vkcboot default when preferred_present_mode isn't called
    let swapchain = vkcboot::SwapchainBuilder::new(
      self.window,
      self.surface,
      self.chosen_gpu,
      self.device,
      self.graphics_queue_index,
      self.present_queue_index,
    )
    .build()
    .map_err(|e| Error::FromVkcboot(e))?;

    // store the swapchain and it's related stuffs
    self.swapchain = swapchain.swapchain;
    self.swapchain_images = swapchain.images;
    self.window_extent = swapchain.extent;
    self.swapchain_format = swapchain.format;
    self.swapchain_image_views = swapchain.image_views;

    Ok(())
  }
}

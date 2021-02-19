use {
  sdl2::*,
  std::{mem::zeroed, ptr::null_mut},
  vkcapi::core::v1_0::*,
};

pub struct VulkanEngine {
  is_initialized: bool,
  frame_number: i32,

  window_extent: VkExtent2D,
  window: *mut SDL_Window,
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
    }
  }

  // initializes everything in the engine
  pub fn init(&mut self) {
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

    // everything went fine
    self.is_initialized = true;
  }

  // shuts down the engine
  pub fn cleanup(&mut self) {
    if self.is_initialized {
      unsafe {
        SDL_DestroyWindow(self.window);
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
}

use {
  crate::{error::Error, VK_CHECK},
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

  pub fn init(&mut self) {
    unsafe {
      SDL_Init(SDL_INIT_VIDEO);
      let window_flags = SDL_WINDOW_VULKAN;

      self.window = SDL_CreateWindow(
        b"Vulkan Engine\0".as_ptr() as *const i8,
        SDL_WINDOWPOS_UNDEFINED_MASK as i32,
        SDL_WINDOWPOS_UNDEFINED_MASK as i32,
        self.window_extent.width as i32,
        self.window_extent.height as i32,
        window_flags,
      );
    }
  }

  pub fn cleanup(&mut self) {
    if self.is_initialized {
      unsafe {
        SDL_DestroyWindow(self.window);
      }
    }
  }

  fn draw(&mut self) {}

  pub fn run(&mut self) {
    let mut e: SDL_Event = unsafe { zeroed() };
    let mut b_quit: bool = false;

    while !b_quit {
      while unsafe { SDL_PollEvent(&mut e) } != 0 {
        if unsafe { e.type_ } == SDL_QUIT {
          b_quit = true;
        }
      }
      self.draw();
    }
  }
}

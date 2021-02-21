use {
  std::ptr::null,
  {vkcapi::core::v1_0::*, vma::*},
};

#[derive(Clone, Copy)]
pub struct AllocatedBuffer {
  pub buffer: VkBuffer,
  pub allocation: VmaAllocation,
}

impl AllocatedBuffer {
  pub fn null() -> AllocatedBuffer {
    AllocatedBuffer {
      buffer: null(),
      allocation: null(),
    }
  }
}

#[derive(Clone, Copy)]
pub struct AllocatedImage {
  pub image: VkImage,
  pub allocation: VmaAllocation,
}

impl AllocatedImage {
  pub fn null() -> AllocatedImage {
    AllocatedImage {
      image: null(),
      allocation: null(),
    }
  }
}

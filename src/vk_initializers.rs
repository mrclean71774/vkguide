use {std::mem::zeroed, vkcapi::core::v1_0::*};

pub fn command_pool_create_info(
  queue_family_index: u32,
  flags: Option<VkCommandPoolCreateFlags>,
) -> VkCommandPoolCreateInfo {
  let mut info: VkCommandPoolCreateInfo = unsafe { zeroed() };
  info.sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
  info.queueFamilyIndex = queue_family_index;

  if flags.is_some() {
    info.flags = flags.unwrap();
  }
  info
}

pub fn command_buffer_allocate_info(
  pool: VkCommandPool,
  count: u32,
  level: Option<VkCommandBufferLevel>,
) -> VkCommandBufferAllocateInfo {
  let mut info: VkCommandBufferAllocateInfo = unsafe { zeroed() };
  info.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
  info.commandPool = pool;
  info.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
  info.commandBufferCount = count;

  if level.is_some() {
    info.level = level.unwrap();
  }
  info
}

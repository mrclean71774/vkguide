use {
  std::{mem::zeroed, ptr::null},
  vkcapi::core::v1_0::*,
};

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

pub fn clear_value_f32(r: f32, g: f32, b: f32, a: f32) -> VkClearValue {
  VkClearValue {
    color: VkClearColorValue {
      float32: [r, g, b, a],
    },
  }
}

pub fn rect_2d(x: i32, y: i32, width: u32, height: u32) -> VkRect2D {
  VkRect2D {
    offset: VkOffset2D { x, y },
    extent: VkExtent2D { width, height },
  }
}

pub fn pipeline_shader_stage_create_info(
  stage: VkShaderStageFlagBits,
  shader_module: VkShaderModule,
) -> VkPipelineShaderStageCreateInfo {
  let info = VkPipelineShaderStageCreateInfo {
    sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
    pNext: null(),
    flags: 0,
    stage: stage,                           // shader stage
    module: shader_module,                  // module containing code for this shader stage
    pName: b"main\0".as_ptr() as *const i8, // entry point of shader
    pSpecializationInfo: null(),
  };
  info
}

pub fn vertex_input_state_create_info(
  vertex_binding_description_count: Option<u32>,
  p_vertex_binding_descriptions: Option<*const VkVertexInputBindingDescription>,
  vertex_attribute_description_count: Option<u32>,
  p_vertex_attribute_descriptions: Option<*const VkVertexInputAttributeDescription>,
) -> VkPipelineVertexInputStateCreateInfo {
  let mut info = VkPipelineVertexInputStateCreateInfo {
    sType: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
    pNext: null(),
    flags: 0,
    // no vertex bindings or attributes
    vertexBindingDescriptionCount: 0,
    pVertexBindingDescriptions: null(),
    vertexAttributeDescriptionCount: 0,
    pVertexAttributeDescriptions: null(),
  };
  if vertex_binding_description_count.is_some() {
    info.vertexBindingDescriptionCount = vertex_binding_description_count.unwrap();
  }
  if p_vertex_binding_descriptions.is_some() {
    info.pVertexBindingDescriptions = p_vertex_binding_descriptions.unwrap();
  }
  if vertex_attribute_description_count.is_some() {
    info.vertexAttributeDescriptionCount = vertex_attribute_description_count.unwrap();
  }
  if p_vertex_attribute_descriptions.is_some() {
    info.pVertexAttributeDescriptions = p_vertex_attribute_descriptions.unwrap();
  }
  info
}

pub fn input_assembly_state_create_info(
  topology: VkPrimitiveTopology,
) -> VkPipelineInputAssemblyStateCreateInfo {
  let info = VkPipelineInputAssemblyStateCreateInfo {
    sType: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
    pNext: null(),
    flags: 0,
    topology,
    // not going to use primitive restart in tutorial
    primitiveRestartEnable: VK_FALSE,
  };
  info
}

pub fn rasterization_state_create_info(
  polygon_mode: VkPolygonMode,
) -> VkPipelineRasterizationStateCreateInfo {
  let info = VkPipelineRasterizationStateCreateInfo {
    sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
    pNext: null(),
    flags: 0,
    depthClampEnable: VK_FALSE,
    // discards all primitive before rasterization if enabled which we don't want
    rasterizerDiscardEnable: VK_FALSE,
    polygonMode: polygon_mode,
    // no backface cull
    cullMode: VK_CULL_MODE_NONE,
    frontFace: VK_FRONT_FACE_CLOCKWISE,
    // no depth bias
    depthBiasEnable: VK_FALSE,
    depthBiasConstantFactor: 0.0,
    depthBiasClamp: 0.0,
    depthBiasSlopeFactor: 0.0,
    lineWidth: 1.0,
  };
  info
}

pub fn multisampling_state_create_info() -> VkPipelineMultisampleStateCreateInfo {
  let info = VkPipelineMultisampleStateCreateInfo {
    sType: VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
    pNext: null(),
    flags: 0,
    // multisampling defaulted to no multisampling (1 sample per pixel)
    rasterizationSamples: VK_SAMPLE_COUNT_1_BIT,
    sampleShadingEnable: VK_FALSE,
    minSampleShading: 1.0,
    pSampleMask: null(),
    alphaToCoverageEnable: VK_FALSE,
    alphaToOneEnable: VK_FALSE,
  };
  info
}

pub fn color_blend_attachment_state() -> VkPipelineColorBlendAttachmentState {
  let color_blend_attachment = VkPipelineColorBlendAttachmentState {
    blendEnable: VK_FALSE,
    srcColorBlendFactor: VK_BLEND_FACTOR_ZERO,
    dstColorBlendFactor: VK_BLEND_FACTOR_ZERO,
    colorBlendOp: VK_BLEND_OP_ADD,
    srcAlphaBlendFactor: VK_BLEND_FACTOR_ZERO,
    dstAlphaBlendFactor: VK_BLEND_FACTOR_ZERO,
    alphaBlendOp: VK_BLEND_OP_ADD,
    colorWriteMask: VK_COLOR_COMPONENT_R_BIT
      | VK_COLOR_COMPONENT_G_BIT
      | VK_COLOR_COMPONENT_B_BIT
      | VK_COLOR_COMPONENT_A_BIT,
  };
  color_blend_attachment
}

pub fn depth_stencil_create_info(
  b_depth_test: bool,
  b_depth_write: bool,
  compare_op: VkCompareOp,
) -> VkPipelineDepthStencilStateCreateInfo {
  VkPipelineDepthStencilStateCreateInfo {
    sType: VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
    pNext: null(),
    flags: 0,
    depthTestEnable: if b_depth_test { VK_TRUE } else { VK_FALSE },
    depthWriteEnable: if b_depth_write { VK_TRUE } else { VK_FALSE },
    depthCompareOp: if b_depth_test {
      compare_op
    } else {
      VK_COMPARE_OP_ALWAYS
    },
    depthBoundsTestEnable: VK_FALSE,
    stencilTestEnable: VK_FALSE,
    front: unsafe { zeroed() },
    back: unsafe { zeroed() },
    minDepthBounds: 0.0,
    maxDepthBounds: 1.0,
  }
}

pub fn pipeline_layout_create_info() -> VkPipelineLayoutCreateInfo {
  VkPipelineLayoutCreateInfo {
    sType: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
    pNext: null(),
    flags: 0,
    setLayoutCount: 0,
    pSetLayouts: null(),
    pushConstantRangeCount: 0,
    pPushConstantRanges: null(),
  }
}

pub fn viewport(
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  min_depth: f32,
  max_depth: f32,
) -> VkViewport {
  VkViewport {
    x,
    y,
    width,
    height,
    minDepth: min_depth,
    maxDepth: max_depth,
  }
}

pub fn image_create_info(
  format: VkFormat,
  usage_flags: VkImageUsageFlags,
  extent: VkExtent3D,
) -> VkImageCreateInfo {
  VkImageCreateInfo {
    sType: VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
    pNext: null(),
    flags: 0,
    imageType: VK_IMAGE_TYPE_2D,
    format: format,
    extent: extent,
    mipLevels: 1,
    arrayLayers: 1,
    samples: VK_SAMPLE_COUNT_1_BIT,
    tiling: VK_IMAGE_TILING_OPTIMAL,
    usage: usage_flags,
    sharingMode: 0,
    queueFamilyIndexCount: 0,
    pQueueFamilyIndices: null(),
    initialLayout: 0,
  }
}

pub fn imageview_create_info(
  format: VkFormat,
  image: VkImage,
  aspect_flags: VkImageAspectFlags,
) -> VkImageViewCreateInfo {
  VkImageViewCreateInfo {
    sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
    pNext: null(),
    flags: 0,
    image,
    viewType: VK_IMAGE_VIEW_TYPE_2D,
    format,
    components: unsafe { zeroed() },
    subresourceRange: VkImageSubresourceRange {
      aspectMask: aspect_flags,
      baseMipLevel: 0,
      levelCount: 1,
      baseArrayLayer: 0,
      layerCount: 1,
    },
  }
}

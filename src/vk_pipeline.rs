use {crate::error::Error, std::ptr::null, vkcapi::core::v1_0::*};

pub struct PipelineBuilder {
  shader_stages: Option<Vec<VkPipelineShaderStageCreateInfo>>,
  vertex_input_info: Option<VkPipelineVertexInputStateCreateInfo>,
  input_assembly: Option<VkPipelineInputAssemblyStateCreateInfo>,
  viewport: Option<VkViewport>,
  scissor: Option<VkRect2D>,
  rasterizer: Option<VkPipelineRasterizationStateCreateInfo>,
  color_blend_attachment: Option<VkPipelineColorBlendAttachmentState>,
  multisampling: Option<VkPipelineMultisampleStateCreateInfo>,
  pipeline_layout: Option<VkPipelineLayout>,
}

impl PipelineBuilder {
  pub fn new() -> PipelineBuilder {
    PipelineBuilder {
      shader_stages: None,
      vertex_input_info: None,
      input_assembly: None,
      viewport: None,
      scissor: None,
      rasterizer: None,
      color_blend_attachment: None,
      multisampling: None,
      pipeline_layout: None,
    }
  }

  pub fn push_shader_stage(&mut self, stage: VkPipelineShaderStageCreateInfo) -> &mut Self {
    if self.shader_stages.is_none() {
      self.shader_stages = Some(Vec::new());
      self.shader_stages.as_mut().unwrap().push(stage);
    } else {
      self.shader_stages.as_mut().unwrap().push(stage);
    }
    self
  }

  pub fn vertex_input_info(&mut self, info: VkPipelineVertexInputStateCreateInfo) -> &mut Self {
    self.vertex_input_info = Some(info);
    self
  }

  pub fn input_assembly(&mut self, info: VkPipelineInputAssemblyStateCreateInfo) -> &mut Self {
    self.input_assembly = Some(info);
    self
  }

  pub fn viewport(&mut self, viewport: VkViewport) -> &mut Self {
    self.viewport = Some(viewport);
    self
  }

  pub fn scissor(&mut self, scissor: VkRect2D) -> &mut Self {
    self.scissor = Some(scissor);
    self
  }

  pub fn rasterizer(&mut self, rasterizer: VkPipelineRasterizationStateCreateInfo) -> &mut Self {
    self.rasterizer = Some(rasterizer);
    self
  }

  pub fn color_blend_attachment(
    &mut self,
    color_blend_attachment: VkPipelineColorBlendAttachmentState,
  ) -> &mut Self {
    self.color_blend_attachment = Some(color_blend_attachment);
    self
  }

  pub fn multisampling(
    &mut self,
    multisampling: VkPipelineMultisampleStateCreateInfo,
  ) -> &mut Self {
    self.multisampling = Some(multisampling);
    self
  }

  pub fn pipeline_layout(&mut self, pipeline_layout: VkPipelineLayout) -> &mut Self {
    self.pipeline_layout = Some(pipeline_layout);
    self
  }

  pub fn build(&self, device: VkDevice, pass: VkRenderPass) -> Result<VkPipeline, Error> {
    // make viewport state from our stored viewport and scissor.
    // at the moment we won't support multiple viewports or scissors
    let viewport_state = VkPipelineViewportStateCreateInfo {
      sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
      pNext: null(),
      flags: 0,
      viewportCount: 1,
      pViewports: self.viewport.as_ref().unwrap(),
      scissorCount: 1,
      pScissors: self.scissor.as_ref().unwrap(),
    };

    // setup dummy color blending. We aren't using transparent objects yet
    // the blending is just "no blend", but we do write to the color attachment
    let color_blending = VkPipelineColorBlendStateCreateInfo {
      sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
      pNext: null(),
      flags: 0,
      logicOpEnable: VK_FALSE,
      logicOp: VK_LOGIC_OP_COPY,
      attachmentCount: 1,
      pAttachments: self.color_blend_attachment.as_ref().unwrap(),
      blendConstants: [0.0, 0.0, 0.0, 0.0],
    };

    // build the actual pipeline
    // we now use all of the info structs we have been writing into
    // into this one to create the pipeline
    let pipeline_info = VkGraphicsPipelineCreateInfo {
      sType: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
      pNext: null(),
      flags: 0,
      stageCount: self.shader_stages.as_ref().unwrap().len() as u32,
      pStages: self.shader_stages.as_ref().unwrap().as_ptr(),
      pVertexInputState: self.vertex_input_info.as_ref().unwrap(),
      pInputAssemblyState: self.input_assembly.as_ref().unwrap(),
      pTessellationState: null(),
      pViewportState: &viewport_state,
      pRasterizationState: self.rasterizer.as_ref().unwrap(),
      pMultisampleState: self.multisampling.as_ref().unwrap(),
      pDepthStencilState: null(),
      pColorBlendState: &color_blending,
      pDynamicState: null(),
      layout: self.pipeline_layout.unwrap(),
      renderPass: pass,
      subpass: 0,
      basePipelineHandle: null(),
      basePipelineIndex: 0,
    };
    // it's easy to error out on create graphics pipeline,
    // so we handle it a bit better than VK_CHECK
    let mut pipeline: VkPipeline = null();
    unsafe {
      if vkCreateGraphicsPipelines(device, null(), 1, &pipeline_info, null(), &mut pipeline)
        != VK_SUCCESS
      {
        return Err(Error::Str("Failed vkCreateGraphicsPipelines"));
      }
    }
    Ok(pipeline)
  }
}

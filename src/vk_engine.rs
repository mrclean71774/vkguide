use {
  crate::{error::Error, vk_initializers as vkinit, vk_pipeline::PipelineBuilder, VK_CHECK},
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

  command_pool: VkCommandPool, // the command pool for our commands
  main_command_buffer: VkCommandBuffer, // the buffer we will record into

  render_pass: VkRenderPass,
  framebuffers: Vec<VkFramebuffer>,

  present_semaphore: VkSemaphore,
  render_semaphore: VkSemaphore,
  render_fence: VkFence,

  triangle_pipeline_layout: VkPipelineLayout,
  triangle_pipeline: VkPipeline,
  red_triangle_pipeline: VkPipeline,

  selected_shader: i32,
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

      command_pool: null(),
      main_command_buffer: null(),

      render_pass: null(),
      framebuffers: Vec::new(),

      present_semaphore: null(),
      render_semaphore: null(),
      render_fence: null(),

      triangle_pipeline_layout: null(),
      triangle_pipeline: null(),
      red_triangle_pipeline: null(),

      selected_shader: 0,
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

    self.init_commands()?;

    self.init_default_renderpass()?;

    self.init_framebuffers()?;

    self.init_sync_structures()?;

    self.init_pipelines()?;

    // everything went fine
    self.is_initialized = true;

    Ok(())
  }

  // shuts down the engine
  pub fn cleanup(&mut self) {
    if self.is_initialized {
      unsafe {
        vkDestroyPipeline(self.device, self.red_triangle_pipeline, null());
        vkDestroyPipeline(self.device, self.triangle_pipeline, null());
        vkDestroyPipelineLayout(self.device, self.triangle_pipeline_layout, null());
        // tutorial is missing the cleanup of sync structures
        vkDestroyFence(self.device, self.render_fence, null());
        vkDestroySemaphore(self.device, self.render_semaphore, null());
        vkDestroySemaphore(self.device, self.present_semaphore, null());

        vkDestroyCommandPool(self.device, self.command_pool, null());
        vkDestroySwapchainKHR(self.device, self.swapchain, null());

        // destroy the main render_pass
        vkDestroyRenderPass(self.device, self.render_pass, null());

        // destroy swapchain resources
        self
          .framebuffers
          .iter()
          .for_each(|fb| vkDestroyFramebuffer(self.device, *fb, null()));

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
    // wait until the GPU has finished rendering the last frame. Timeout of 1 second
    unsafe {
      VK_CHECK!(vkWaitForFences(
        self.device,
        1,
        &self.render_fence,
        VK_TRUE, // true is not an int in rust
        1_000_000_000
      ));
      VK_CHECK!(vkResetFences(self.device, 1, &self.render_fence));

      // request image from the swapchain, one second timeout
      let mut swapchain_image_index = 0;
      VK_CHECK!(vkAcquireNextImageKHR(
        self.device,
        self.swapchain,
        1_000_000_000,
        self.present_semaphore,
        null(),
        &mut swapchain_image_index
      ));

      // now that we are sure that the commands finished executing,
      // we can safely reset the command buffer to begin recording again.
      VK_CHECK!(vkResetCommandBuffer(self.main_command_buffer, 0));

      // naming it cmd for shorter writing
      let cmd = self.main_command_buffer;

      // begin the command buffer recording. We will use this command buffer
      // exactly once, so we want to let Vulkan know that
      let cmd_begin_info = VkCommandBufferBeginInfo {
        sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        pNext: null(),
        flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
        pInheritanceInfo: null(),
      };
      VK_CHECK!(vkBeginCommandBuffer(cmd, &cmd_begin_info));

      // make a clear-color from frame number. This will flash with a 120*pi frame period.
      let flash = f32::abs(f32::sin(self.frame_number as f32 / 120.0));
      let clear_value = vkinit::clear_value_f32(0.0, 0.0, flash, 1.0);

      // start the main renderpass. We will use the clear color from above,
      // and the framebuffer of the index the swapchain gave us.
      let rp_info = VkRenderPassBeginInfo {
        sType: VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
        pNext: null(),
        renderPass: self.render_pass,
        framebuffer: self.framebuffers[swapchain_image_index as usize],
        renderArea: vkinit::rect_2d(0, 0, self.window_extent.width, self.window_extent.height),
        clearValueCount: 1,
        pClearValues: &clear_value,
      };
      vkCmdBeginRenderPass(cmd, &rp_info, VK_SUBPASS_CONTENTS_INLINE);

      if self.selected_shader == 0 {
        vkCmdBindPipeline(cmd, VK_PIPELINE_BIND_POINT_GRAPHICS, self.triangle_pipeline);
      } else {
        vkCmdBindPipeline(
          cmd,
          VK_PIPELINE_BIND_POINT_GRAPHICS,
          self.red_triangle_pipeline,
        );
      }
      vkCmdDraw(cmd, 3, 1, 0, 0);

      // finalize the render render_pass
      vkCmdEndRenderPass(cmd);
      // finalize the command buffer (we can no longer add commands, but it can be executed)
      VK_CHECK!(vkEndCommandBuffer(cmd));

      // prepare the submission to the queue. We want to wait on the present_semaphore,
      // as that is signaled when the swapchain is ready.
      // We will signal the render_semaphore, to signal that rendering is finished.
      let submit = VkSubmitInfo {
        sType: VK_STRUCTURE_TYPE_SUBMIT_INFO,
        pNext: null(),
        waitSemaphoreCount: 1,
        pWaitSemaphores: &self.present_semaphore,
        pWaitDstStageMask: &VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
        commandBufferCount: 1,
        pCommandBuffers: &cmd,
        signalSemaphoreCount: 1,
        pSignalSemaphores: &self.render_semaphore,
      };
      // submit command buffer to the queue and execute it.
      // render_fence will now block until the graphic commands finish execution
      VK_CHECK!(vkQueueSubmit(
        self.graphics_queue,
        1,
        &submit,
        self.render_fence
      ));

      // this will put the image we just rendered into the visible window.
      // we want to wait on the render_semaphore for that, as it's necessary
      // that drawing commands have finished before the image is displayed to the user.
      let present_info = VkPresentInfoKHR {
        sType: VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
        pNext: null(),
        waitSemaphoreCount: 1,
        pWaitSemaphores: &self.render_semaphore,
        swapchainCount: 1,
        pSwapchains: &self.swapchain,
        pImageIndices: &swapchain_image_index,
        pResults: null_mut(),
      };
      VK_CHECK!(vkQueuePresentKHR(self.graphics_queue, &present_info));

      // increase the number of frames drawn
      self.frame_number += 1;
    }
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
        match unsafe { e.type_ } {
          SDL_QUIT => b_quit = true,
          SDL_KEYDOWN => match unsafe { e.key.keysym.sym as u32 } {
            SDLK_SPACE => {
              self.selected_shader += 1;
              if self.selected_shader > 1 {
                self.selected_shader = 0;
              }
            }
            SDLK_ESCAPE => b_quit = true,
            _ => {}
          },
          _ => {}
        }
      }
      self.draw();
    }
    unsafe {
      // we need to wait for rendering to finish before starting cleanup
      vkQueueWaitIdle(self.graphics_queue);
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

    // we have a separate queue handle for presentation even thought they might
    // refer to the same queue family. On my machine they are the same but I don't
    // think they have to be on all devices.
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

  fn init_commands(&mut self) -> Result<(), Error> {
    // create a command pool for commands submitted to the graphics queue
    let command_pool_info = vkinit::command_pool_create_info(
      // the command pool will be the one that can submit graphics commands
      self.graphics_queue_index,
      // we also want the pool to allow for resetting of individual command buffers
      Some(VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT),
    );
    unsafe {
      VK_CHECK!(vkCreateCommandPool(
        self.device,
        &command_pool_info,
        null(),
        &mut self.command_pool
      ));
    }

    // allocate the default command buffer that we will use for rendering
    let cmd_alloc_info = vkinit::command_buffer_allocate_info(
      self.command_pool, // commands will be made from our command pool
      1,                 // we will allocate 1 command buffer
      None,              // primary is the default level
    );
    unsafe {
      VK_CHECK!(vkAllocateCommandBuffers(
        self.device,
        &cmd_alloc_info,
        &mut self.main_command_buffer
      ));
    }
    Ok(())
  }

  fn init_default_renderpass(&mut self) -> Result<(), Error> {
    // the renderpass will use this color attachment
    let color_attachment = VkAttachmentDescription {
      flags: 0,
      // the attachment will have the format needed by the swapchain
      format: self.swapchain_format,
      // 1 sample, we won't be doing MSAA
      samples: VK_SAMPLE_COUNT_1_BIT,
      // we Clear when this attachment is loaded
      loadOp: VK_ATTACHMENT_LOAD_OP_CLEAR,
      // we keep the attachment stored when the renderpass ends
      storeOp: VK_ATTACHMENT_STORE_OP_STORE,
      stencilLoadOp: VK_ATTACHMENT_LOAD_OP_DONT_CARE,
      stencilStoreOp: VK_ATTACHMENT_STORE_OP_DONT_CARE,
      // we don't know or care about the starting layout of the attachment
      initialLayout: VK_IMAGE_LAYOUT_UNDEFINED,
      // after the renderpass ends, the image has to be on a layout ready for display
      finalLayout: VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
    };

    let color_attachment_ref = VkAttachmentReference {
      // attachment number will index into the pAttachments array in the parent renderpass
      attachment: 0,
      layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
    };

    // we are going to create 1 subpass, which is the minimum you can do
    let subpass = VkSubpassDescription {
      flags: 0,
      pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
      inputAttachmentCount: 0,
      pInputAttachments: null(),
      colorAttachmentCount: 1,
      pColorAttachments: &color_attachment_ref,
      pResolveAttachments: null(),
      pDepthStencilAttachment: null(),
      preserveAttachmentCount: 0,
      pPreserveAttachments: null(),
    };

    let render_pass_info = VkRenderPassCreateInfo {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
      pNext: null(),
      flags: 0,
      // connect the color attachment to the info
      attachmentCount: 1,
      pAttachments: &color_attachment,
      // conntect the subpass to the info
      subpassCount: 1,
      pSubpasses: &subpass,
      dependencyCount: 0,
      pDependencies: null(),
    };

    unsafe {
      VK_CHECK!(vkCreateRenderPass(
        self.device,
        &render_pass_info,
        null(),
        &mut self.render_pass
      ));
    }
    Ok(())
  }

  fn init_framebuffers(&mut self) -> Result<(), Error> {
    // create the framebuffers for the swapchain images. This will connect
    // the render-pass to the images for rendering
    let mut fb_info = VkFramebufferCreateInfo {
      sType: VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
      pNext: null(),
      flags: 0,
      renderPass: self.render_pass,
      attachmentCount: 1,
      pAttachments: null(),
      width: self.window_extent.width,
      height: self.window_extent.height,
      layers: 1,
    };

    // grab how many images we have in the swapchain
    self
      .framebuffers
      .resize(self.swapchain_images.len(), null());

    // create framebuffers for each of the swapchain image views
    for i in 0..self.swapchain_image_views.len() {
      fb_info.pAttachments = &self.swapchain_image_views[i];
      unsafe {
        VK_CHECK!(vkCreateFramebuffer(
          self.device,
          &fb_info,
          null(),
          &mut self.framebuffers[i]
        ));
      }
    }
    Ok(())
  }

  fn init_sync_structures(&mut self) -> Result<(), Error> {
    // create synchronization structures
    let fence_create_info = VkFenceCreateInfo {
      sType: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
      pNext: null(),
      // we want to create the fence with the Create Signaled flag,
      // so we can wait on it before using it on a GPU command (for the first frame)
      flags: VK_FENCE_CREATE_SIGNALED_BIT,
    };
    unsafe {
      VK_CHECK!(vkCreateFence(
        self.device,
        &fence_create_info,
        null(),
        &mut self.render_fence
      ));
    }

    // for the semaphores we don't need any flags
    let semaphore_create_info = VkSemaphoreCreateInfo {
      sType: VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
      pNext: null(),
      flags: 0,
    };
    unsafe {
      VK_CHECK!(vkCreateSemaphore(
        self.device,
        &semaphore_create_info,
        null(),
        &mut self.render_semaphore
      ));
      VK_CHECK!(vkCreateSemaphore(
        self.device,
        &semaphore_create_info,
        null(),
        &mut self.present_semaphore
      ));
    }
    Ok(())
  }

  fn create_shader_module(&self, path: &str) -> Result<(bool, VkShaderModule), Error> {
    // Rust has nice things to load file
    let source = std::fs::read(path).map_err(|e| Error::FromIO(e))?;

    let create_info = VkShaderModuleCreateInfo {
      sType: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
      pNext: null(),
      flags: 0,
      codeSize: source.len(),
      pCode: source.as_ptr() as *const u32,
    };

    // check that the creation goes well
    let mut shader_module = null();
    if unsafe { vkCreateShaderModule(self.device, &create_info, null(), &mut shader_module) }
      != VK_SUCCESS
    {
      Ok((false, shader_module))
    } else {
      Ok((true, shader_module))
    }
  }

  fn init_pipelines(&mut self) -> Result<(), Error> {
    // a little different than the tutorial, we will be silent if all is well and return
    // an error &str with the offending file name if there was a problem.
    let (ok, triangle_vert_shader) =
      self.create_shader_module("shaders/colored_triangle.vert.spv")?;
    if !ok {
      return Err(Error::Str("Error when building colored_triangle.vert.spv"));
    }
    let (ok, triangle_frag_shader) =
      self.create_shader_module("shaders/colored_triangle.frag.spv")?;
    if !ok {
      return Err(Error::Str("Error when building colored_triangle.frag.spv"));
    }

    let (ok, red_triangle_vert_shader) = self.create_shader_module("shaders/triangle.vert.spv")?;
    if !ok {
      return Err(Error::Str("Error when building triangle.vert.spv"));
    }
    let (ok, red_triangle_frag_shader) = self.create_shader_module("shaders/triangle.frag.spv")?;
    if !ok {
      return Err(Error::Str("Error when building triangle.frag.spv"));
    }

    // build the pipeline layout that controls the inputs/outputs of the shader
    // we are not using descriptor sets or other system yet so no need to use
    // anything other than the empty default.
    let pipeline_layout_info = vkinit::pipeline_layout_create_info();
    unsafe {
      VK_CHECK!(vkCreatePipelineLayout(
        self.device,
        &pipeline_layout_info,
        null(),
        &mut self.triangle_pipeline_layout
      ));
    }

    self.triangle_pipeline = PipelineBuilder::new()
      // build the stage-create-info for both vertex and fragment stages.
      // This lets the pipeline know the shader modules per stage
      .push_shader_stage(vkinit::pipeline_shader_stage_create_info(
        VK_SHADER_STAGE_VERTEX_BIT,
        triangle_vert_shader,
      ))
      .push_shader_stage(vkinit::pipeline_shader_stage_create_info(
        VK_SHADER_STAGE_FRAGMENT_BIT,
        triangle_frag_shader,
      ))
      // vertex input controls how to read vertices from vertes buffers. We aren't using it yet
      .vertex_input_info(vkinit::vertex_input_state_create_info())
      // input assembly is the configuration for drawing triangle lists, strips, or individual
      // points. We are just going to draw triangle list.
      .input_assembly(vkinit::input_assembly_state_create_info(
        VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
      ))
      // vuild viewport and scissor from the swapchain extents
      .viewport(vkinit::viewport(
        0.0,
        0.0,
        self.window_extent.width as f32,
        self.window_extent.height as f32,
        0.0,
        1.0,
      ))
      .scissor(vkinit::rect_2d(
        0,
        0,
        self.window_extent.width,
        self.window_extent.height,
      ))
      // configure the rasterizer to draw filled triangles
      .rasterizer(vkinit::rasterization_state_create_info(
        VK_POLYGON_MODE_FILL,
      ))
      // we don't use multisampling, so just run the default one
      .multisampling(vkinit::multisampling_state_create_info())
      // a single blend attachment with no blending and writing to RGBA
      .color_blend_attachment(vkinit::color_blend_attachment_state())
      // use the triangle layout we created
      .pipeline_layout(self.triangle_pipeline_layout)
      // finally build the pipeline
      .build(self.device, self.render_pass)?;

    self.red_triangle_pipeline = PipelineBuilder::new()
      .push_shader_stage(vkinit::pipeline_shader_stage_create_info(
        VK_SHADER_STAGE_VERTEX_BIT,
        red_triangle_vert_shader,
      ))
      .push_shader_stage(vkinit::pipeline_shader_stage_create_info(
        VK_SHADER_STAGE_FRAGMENT_BIT,
        red_triangle_frag_shader,
      ))
      .vertex_input_info(vkinit::vertex_input_state_create_info())
      .input_assembly(vkinit::input_assembly_state_create_info(
        VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
      ))
      .viewport(vkinit::viewport(
        0.0,
        0.0,
        self.window_extent.width as f32,
        self.window_extent.height as f32,
        0.0,
        1.0,
      ))
      .scissor(vkinit::rect_2d(
        0,
        0,
        self.window_extent.width,
        self.window_extent.height,
      ))
      .rasterizer(vkinit::rasterization_state_create_info(
        VK_POLYGON_MODE_FILL,
      ))
      .multisampling(vkinit::multisampling_state_create_info())
      .color_blend_attachment(vkinit::color_blend_attachment_state())
      .pipeline_layout(self.triangle_pipeline_layout)
      .build(self.device, self.render_pass)?;

    unsafe {
      vkDestroyShaderModule(self.device, triangle_vert_shader, null());
      vkDestroyShaderModule(self.device, triangle_frag_shader, null());

      vkDestroyShaderModule(self.device, red_triangle_vert_shader, null());
      vkDestroyShaderModule(self.device, red_triangle_frag_shader, null());
    }
    Ok(())
  }
}

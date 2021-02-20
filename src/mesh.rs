use {
  crate::vk_types::AllocatedBuffer, lina::vec3::Vec3, std::mem::size_of, vkcapi::core::v1_0::*,
};

#[derive(Clone)]
pub struct VertexInputDescription {
  pub bindings: Vec<VkVertexInputBindingDescription>,
  pub attributes: Vec<VkVertexInputAttributeDescription>,
  pub flags: VkPipelineVertexInputStateCreateFlags,
}

impl VertexInputDescription {
  pub fn new() -> VertexInputDescription {
    VertexInputDescription {
      bindings: Vec::new(),
      attributes: Vec::new(),
      flags: 0,
    }
  }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
  pub position: Vec3,
  pub normal: Vec3,
  pub color: Vec3,
}

impl Vertex {
  pub fn get_vertex_description() -> VertexInputDescription {
    let mut description = VertexInputDescription::new();

    // we will have just 1 vertex buffer bindin, with a per-vertex rate
    let main_binding = VkVertexInputBindingDescription {
      binding: 0,
      stride: size_of::<Vertex>() as u32,
      inputRate: VK_VERTEX_INPUT_RATE_VERTEX,
    };

    description.bindings.push(main_binding);

    // position will be stored at Location 0
    let position_attribute = VkVertexInputAttributeDescription {
      location: 0,
      binding: 0,
      format: VK_FORMAT_R32G32B32_SFLOAT,
      offset: 0,
    };

    // normal will be stored at Location 1
    let normal_attribute = VkVertexInputAttributeDescription {
      location: 1,
      binding: 0,
      format: VK_FORMAT_R32G32B32_SFLOAT,
      offset: 12,
    };

    // color will be stored at Location 2
    let color_attribute = VkVertexInputAttributeDescription {
      location: 2,
      binding: 0,
      format: VK_FORMAT_R32G32B32_SFLOAT,
      offset: 24,
    };
    description.attributes.push(position_attribute);
    description.attributes.push(normal_attribute);
    description.attributes.push(color_attribute);

    description
  }
}

#[derive(Clone)]
pub struct Mesh {
  pub vertices: Vec<Vertex>,
  pub vertex_buffer: AllocatedBuffer,
}

impl Mesh {
  pub fn new() -> Mesh {
    Mesh {
      vertices: Vec::new(),
      vertex_buffer: AllocatedBuffer::null(),
    }
  }
}

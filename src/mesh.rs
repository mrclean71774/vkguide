use {
  crate::{error::Error, vk_types::AllocatedBuffer},
  lina::vec3::Vec3,
  std::mem::size_of,
  vkcapi::core::v1_0::*,
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
  pub fn new3v3(position: Vec3, normal: Vec3, color: Vec3) -> Vertex {
    Vertex {
      position,
      normal,
      color,
    }
  }
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

  // we are using gltf instead of obj like the tutorial because I think it's a better
  // format and there is a library available to load it.
  pub fn load_gltf(filename: &str) -> Result<Mesh, Error> {
    let (document, buffers, _images) = gltf::import(filename).map_err(|e| Error::FromGltf(e))?;

    let mut result = Mesh::new();
    // we are depending on the fact that we created the gltf file and know
    // that it contains 1 mesh and nothing else

    // get the first mesh or panic if there is no mesh in file
    let mesh = document.meshes().next().unwrap();
    for primitive in mesh.primitives() {
      // we are taking an idexed buffer and turning it into a non indexed buffer because
      // we haven't done indexed drawing in the tutorial yet.

      let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
      let positions: Vec<Vec3> = reader
        .read_positions()
        .unwrap()
        .map(|v| Vec3::new(v[0], v[1], v[2]))
        .collect();
      let normals: Vec<Vec3> = reader
        .read_normals()
        .unwrap()
        .map(|n| Vec3::new(n[0], n[1], n[2]))
        .collect();
      let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();

      for index in indices {
        result.vertices.push(Vertex::new3v3(
          positions[index as usize],
          normals[index as usize],
          normals[index as usize],
        ));
      }
    }
    Ok(result)
  }
}

mod error;
mod vk_engine;
mod vk_initializers;
mod vk_types;

use {error::Error, vk_engine::VulkanEngine};

fn main() -> Result<(), Error> {
  let mut engine = VulkanEngine::new();

  engine.init()?;

  engine.run();

  engine.cleanup();

  Ok(())
}

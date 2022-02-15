use serde::Serialize;
use serde::Deserialize;

pub mod file_format;
pub mod world_header;
mod utils;

/// A Terraria world file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct World {
    pub file_format: file_format::FileFormat,
    pub world_header: world_header::WorldHeader,
}

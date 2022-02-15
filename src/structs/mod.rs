use serde::Serialize;
use serde::Deserialize;

pub mod file_format;

/// A Terraria world file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct World {
    pub file_format: file_format::FileFormat,
}

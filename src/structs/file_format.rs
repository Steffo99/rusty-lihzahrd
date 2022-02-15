use serde::Serialize;
use serde::Deserialize;
use serde_big_array::BigArray;

/// The first section of a Terraria world file.
/// Contains information on how the file is structured.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileFormat {
    /// The game version this file was last saved with.
    /// Different file versions have different contents.
    pub version: i32,

    /// The string `"relogic"`.
    pub relogic: [u8; 7],

    /// The format of the save file. Should be `2` for all versions following Terraria 1.2.
    pub save_type: u8,

    /// The number of times this world was saved.
    pub revision: u32,

    /// Whether the world has been favourited, or not.
    pub favourite: u64,

    /// Pointers to the other sections of the file.
    pub pointers: FileSectionPointers,

    pub tileframeimportant: TileFrameImportant,
}

/// Container for the file section pointers.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileSectionPointers {
    /// The number of pointers in the list. Should always be `11`.
    pub size: u16,

    /// Array of file section pointers.
    pub contents: [u32; 11],
}

impl FileSectionPointers {
    pub fn world_header(&self) -> u32 {
        self.contents[0]
    }

    pub fn world_tiles(&self) -> u32 {
        self.contents[1]
    }

    pub fn chests(&self) -> u32 {
        self.contents[2]
    }

    pub fn signs(&self) -> u32 {
        self.contents[3]
    }

    pub fn npcs(&self) -> u32 {
        self.contents[4]
    }

    pub fn tile_entities(&self) -> u32 {
        self.contents[5]
    }

    pub fn pressure_plates(&self) -> u32 {
        self.contents[6]
    }

    pub fn town_manager(&self) -> u32 {
        self.contents[7]
    }

    pub fn bestiary(&self) -> u32 {
        self.contents[8]
    }

    pub fn journey_powers(&self) -> u32 {
        self.contents[9]
    }

    pub fn footer(&self) -> u32 {
        self.contents[10]
    }
}

/// Container for the tiles' FrameImportant data.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TileFrameImportant {
    /// The number of tiles in the list. Should always be `625`.
    pub size: u16,

    /// Array of flags determining whether if each individual tile was FrameImportant or not at the moment of the last save.
    #[serde(with = "BigArray")]
    pub contents: [u8; 79]
}


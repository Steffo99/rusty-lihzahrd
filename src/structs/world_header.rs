use crate::structs::utils::Coordinates;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorldHeader {
    pub name: String,
    pub generator: Generator,
    pub uuid: u128,
    pub id: i32,
    pub bounds: Bounds,
    pub world_size: Coordinates,
    pub game_mode: GameMode,
    pub is_drunk_world: bool,
    pub is_good_world: bool,
    pub is_anniversary_world: bool,
    pub is_dontstarve_world: bool,
    pub is_notthebees_world: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Generator {
    pub name: String,
    pub seed: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bounds {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

#[repr(i32)]
enum GameMode {
    Normal = 0_i32,
    Expert = 1_i32,
    Master = 2_i32,
}
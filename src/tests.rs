#[cfg(test)]

use crate::structs::World;

#[test]
fn parse_rusty_tests() {
    let mut file = std::fs::File::open("worlds/Rusty_Tests.wld").expect("Could not find test world");
    let world = serde_altar::from_reader::<std::fs::File, World>(&mut file).expect("Could not deserialize world");

    let ff = &world.file_format;
    assert_eq!(ff.version, 244_i32);
    assert_eq!(ff.relogic, [114, 101, 108, 111, 103, 105, 99]);
    assert_eq!(ff.save_type, 2_u8);
    assert_eq!(ff.revision, 1_u32);
    assert_eq!(ff.favourite, 0_u64);
    assert_eq!(ff.pointers.size, 11_u16);
    assert_eq!(ff.pointers.contents, [151, 3295, 2917671, 2938549, 2938551, 2938615, 2938619, 2938623, 2938627, 2938639, 2938670]);
    assert_eq!(ff.pointers.world_header(), 151_u32);
    assert_eq!(ff.pointers.world_tiles(), 3295_u32);
    assert_eq!(ff.pointers.chests(), 2917671_u32);
    assert_eq!(ff.pointers.signs(), 2938549_u32);
    assert_eq!(ff.pointers.npcs(), 2938551_u32);
    assert_eq!(ff.pointers.tile_entities(), 2938615_u32);
    assert_eq!(ff.pointers.pressure_plates(), 2938619_u32);
    assert_eq!(ff.pointers.town_manager(), 2938623_u32);
    assert_eq!(ff.pointers.bestiary(), 2938627_u32);
    assert_eq!(ff.pointers.journey_powers(), 2938639_u32);
    assert_eq!(ff.pointers.footer(), 2938670_u32);
    assert_eq!(ff.tileframeimportant.size, 625_u16);
    assert_ne!(ff.tileframeimportant.contents, [0; 79]);
}
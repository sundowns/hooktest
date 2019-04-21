use num_derive::FromPrimitive;

pub struct Tile {
    pub _type: TileType,
    pub position: [u32; 2],
}

// NOTE: The indicies of this enum should match their definitions in tilesheet.ron
#[derive(FromPrimitive)]
pub enum TileType {
    Rock = 0,
}

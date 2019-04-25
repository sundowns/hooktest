use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use num_traits::FromPrimitive;

use crate::components::{Tile, TileType};
use crate::util::GameAssets;

#[derive(Debug, Clone, Copy)]
struct TileDataUnit {
    id: usize,
    position: (u32, u32),
}

#[derive(Debug, Clone)]
pub struct TileData {
    tiles: Vec<Option<TileDataUnit>>,
    pub cols: u32,
    pub rows: u32,
    pub cell_dimensions: (f32, f32),
}

impl TileData {
    fn get_tile(&self, x: u32, y: u32) -> Option<TileDataUnit> {
        self.tiles[((y * self.cols) + x) as usize]
    }
}

fn tile_id_to_type(id: usize) -> TileType {
    match FromPrimitive::from_usize(id) {
        Some(TileType::Rock) => TileType::Rock,
        _ => {
            panic!("Unknown type type id: {:?}", id);
        }
    }
}

pub fn populate_world(world: &mut World, tile_data: TileData, game_assets: GameAssets) {
    // lets create tile entities with sprites and locations

    for _cell in tile_data.tiles.iter() {
        if let Some(_tile) = _cell {
            let tile_type = tile_id_to_type(_tile.id);

            let mut local_transform = Transform::default();

            local_transform.set_xyz(
                tile_data.cell_dimensions.0 * _tile.position.0 as f32,
                (tile_data.cell_dimensions.1 * tile_data.rows as f32)
                    - tile_data.cell_dimensions.1 * _tile.position.1 as f32,
                0.0,
            );

            world
                .create_entity()
                .with(Tile { _type: tile_type })
                .with(game_assets.tile_sprite(_tile.id))
                .with(local_transform)
                .build();
        };
    }

    world.add_resource(tile_data);
}

pub fn load_tile_data(cell_dimensions: (f32, f32)) -> Option<TileData> {
    // TODO: Derive from the loaded file
    let cols: u32 = 32;
    let rows: u32 = 18;

    let mut data = TileData {
        tiles: Vec::new(),
        cols,
        rows,
        cell_dimensions,
    };

    let y = rows - 1;

    for i in 0..cols * rows {
        // TODO: place tiles based on a file (just on the bottom row for now)
        if i >= cols * y {
            data.tiles.push(Some(TileDataUnit {
                id: 0,
                position: ((i - (cols * y)), y),
            }));
        } else {
            data.tiles.push(None);
        }
    }

    Some(data)
}

use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use num_traits::FromPrimitive;

use crate::util::GameAssets;

#[derive(Debug, Clone, Copy)]
struct TileDataUnit {
    id: usize,
}

#[derive(Debug)]
pub struct TileData {
    tiles: Vec<Option<TileDataUnit>>,
    cols: usize,
    rows: usize,
}

impl TileData {
    fn get_tile(&self, x: usize, y: usize) -> Option<TileDataUnit> {
        self.tiles[(y * self.cols) + x]
    }
}

use crate::components::{Tile, TileType};

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

            //TODO: need to figure out deriving their on-screen positions from the grid pos
            // TODO: May not actually be required, can we just draw them based on tile size & indices?
            // local_transform.set_xyz(
            //     _tile.position[0] as f32 * tile_width,
            //     _tile.position[1] as f32 * tile_height,
            //     0.0,
            // );

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

pub fn load_tile_data(cols: usize, rows: usize) -> Option<TileData> {
    let mut data = TileData {
        tiles: Vec::new(),
        cols: cols,
        rows: rows,
    };

    for i in 0..cols * rows {
        // TODO: place tiles based on a file (just on the bottom row for now)
        if i >= cols * (rows - 1) {
            data.tiles.push(Some(TileDataUnit { id: 0 }));
        }
    }

    Some(data)
}

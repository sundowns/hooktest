use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use num_traits::FromPrimitive;

use crate::util::GameAssets;

#[derive(Debug)]
struct TileDataUnit {
    id: usize,
    position: [u32; 2],
}

#[derive(Debug)]
pub struct TileData {
    tiles: Vec<Vec<Option<TileDataUnit>>>,
    cols: usize,
    rows: usize,
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

    for _row in tile_data.tiles.iter() {
        for _cell in _row.iter() {
            if let Some(_tile) = _cell {
                let tile_type = tile_id_to_type(_tile.id);

                let mut local_transform = Transform::default();

                //TODO: need to figure out deriving their on-screen positions from the grid pos
                // local_transform.set_xyz(
                //     _tile.position[0] as f32 * tile_width,
                //     _tile.position[1] as f32 * tile_height,
                //     0.0,
                // );

                world
                    .create_entity()
                    .with(Tile {
                        _type: tile_type,
                        position: _tile.position,
                    })
                    .with(game_assets.tile_sprite(_tile.id))
                    .with(local_transform)
                    .build();

                println!("ur getting an entity");
            };
        }
    }
}

pub fn load_tile_data(cols: usize, rows: usize) -> Option<TileData> {
    let mut data = TileData {
        tiles: Vec::new(),
        cols: cols,
        rows: rows,
    };

    for x in 0..cols {
        data.tiles.push(Vec::new());
        for y in 0..rows {
            // TODO: place tiles based on a file (just on the bottom row for now)
            if y == rows - 1 {
                data.tiles[x].push(Some(TileDataUnit {
                    id: 0,
                    position: [x as u32, y as u32],
                }));
            }
        }
    }

    Some(data)
}

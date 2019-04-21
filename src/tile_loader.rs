#[derive(Debug)]
struct Tile {
    id: TileId,
}

#[derive(Debug)]
struct TileId(u32);

#[derive(Debug)]
pub struct Map {
    data: Vec<Vec<Option<Tile>>>,
    tile_width: f32,
    tile_height: f32,
}

impl Default for Map {
    fn default() -> Map {
        Map {
            data: Vec::new(),
            tile_height: 1.0,
            tile_width: 1.0,
        }
    }
}

pub fn initialise(cols: usize, rows: usize) -> Option<Map> {
    let mut map = Map::default();
    // TODO: divide the screen up and preserve tile width/height

    for x in 0..cols {
        map.data.push(Vec::new());
        for _ in 0..rows {
            // TODO: place tiles in a specific place for now (this will fill every square atm)
            map.data[x].push(Some(Tile { id: TileId(1) }));
        }
    }

    Some(map)
}

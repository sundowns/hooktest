use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

#[derive(Clone)]
pub struct GameAssets {
    pub entities_sprite_sheet: SpriteSheetHandle,
}

impl GameAssets {
    pub fn entity_sprite(&self, sprite_number: usize) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.entities_sprite_sheet.clone(),
            sprite_number,
        }
    }
}

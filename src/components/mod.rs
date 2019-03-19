use amethyst::ecs::prelude::{Component, DenseVecStorage};

mod hook;
mod player;

pub use self::hook::Hook;
pub use self::player::Player;

impl Component for Hook {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

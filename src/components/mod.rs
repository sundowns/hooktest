use amethyst::ecs::prelude::{Component, DenseVecStorage};

mod gravity;
mod hook;
mod player;

pub use self::{gravity::Gravity, hook::Hook, player::Player};

impl Component for Hook {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Gravity {
    type Storage = DenseVecStorage<Self>;
}

use amethyst::ecs::prelude::{Component, DenseVecStorage};

mod extending;
mod gravity;
mod hook;
mod hook_fired;
mod jump;
mod player;

pub use self::{
    extending::Extending, gravity::Gravity, hook::Hook, hook_fired::HookFired, jump::Jump,
    player::Player,
};

impl Component for HookFired {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Hook {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Gravity {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Extending {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Jump {
    type Storage = DenseVecStorage<Self>;
}

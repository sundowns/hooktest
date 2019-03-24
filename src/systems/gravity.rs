extern crate amethyst;

use amethyst::core::{Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::Gravity;

pub const GRAVITY: f32 = 20.0;

pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Gravity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, gravities, time): Self::SystemData) {
        for (_gravity, _transform) in (&gravities, &mut transforms).join() {
            _transform.set_y(_transform.translation().y - (GRAVITY * time.delta_seconds()));
        }
    }
}

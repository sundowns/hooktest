use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::Hook;

pub struct MoveHookSystem;

impl<'s> System<'s> for MoveHookSystem {
    type SystemData = (
        ReadStorage<'s, Hook>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (hooks, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        for (hook, local) in (&hooks, &mut locals).join() {
            local.translate_x(hook.velocity[0] * time.delta_seconds());
            local.translate_y(hook.velocity[1] * time.delta_seconds());
        }
    }
}

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::Extending;
use crate::components::Hook;

pub struct MoveHookSystem;

impl<'s> System<'s> for MoveHookSystem {
    type SystemData = (
        ReadStorage<'s, Hook>,
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Extending>,
    );

    fn run(&mut self, (hooks, time, mut locals, mut is_extending): Self::SystemData) {
        // Move every hook according to its speed, and the time passed.
        for (_hook, local, _is_extending) in (&hooks, &mut locals, &mut is_extending).join() {
            local.translate_x(_hook.velocity[0] * time.delta_seconds());
            local.translate_y(_hook.velocity[1] * time.delta_seconds());

            _is_extending.distance_traveled =
                _is_extending.distance_traveled + (_hook.speed * time.delta_seconds());

            if _is_extending.distance_traveled > _hook.max_distance {
                println!("max distance reached, destroy/stop me!"); // TODO: new struct to reference is_extending components and destroy them out of the loop
            };
        }
    }
}

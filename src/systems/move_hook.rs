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
        // Move every ball according to its speed, and the time passed.
        for (_hook, local) in (&hooks, &mut locals).join() {
            local.translate_x(_hook.velocity[0] * time.delta_seconds());
            local.translate_y(_hook.velocity[1] * time.delta_seconds());
        }

        for (_is_extending, _hook) in (&mut is_extending, &hooks).join() {
            // TODO: calc distance and increment ix_extending counter.
            // TODO:  when counter == max_distance -> reset hook velocity & remove Extending component

            // _is_extending.distance_traveled = _is_extending.distance_traveled + _hook.velocity.x
        }
    }
}

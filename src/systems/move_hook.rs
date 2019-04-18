use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, System, WriteStorage},
    ecs::{Entities, Entity},
};

use crate::components::Extending;
use crate::components::Hook;
use crate::components::HookFired;

struct Deletion {
    pub hook: Entity,
    pub player: Entity,
}

pub struct MoveHookSystem;

impl<'s> System<'s> for MoveHookSystem {
    type SystemData = (
        WriteStorage<'s, Hook>,
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Extending>,
        WriteStorage<'s, HookFired>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut hooks, time, mut locals, mut is_extending, mut hook_fired, entities): Self::SystemData,
    ) {
        let mut deletions: Vec<Deletion> = vec![];

        // Move every hook according to its speed, and the time passed.
        for (_hook, local, _is_extending, _entity) in
            (&mut hooks, &mut locals, &mut is_extending, &*entities).join()
        {
            local.translate_x(_hook.velocity[0] * time.delta_seconds());
            local.translate_y(_hook.velocity[1] * time.delta_seconds());

            _is_extending.distance_traveled =
                _is_extending.distance_traveled + (_hook.speed * time.delta_seconds());

            if _is_extending.distance_traveled > _hook.max_distance {
                _hook.velocity = [0.0, 0.0];
                deletions.push(Deletion {
                    hook: _entity,
                    player: _hook.owner,
                });
            };
        }

        for _deletion in deletions.iter() {
            match entities.delete(_deletion.hook) {
                Err(_e) => {
                    panic!("failed to delete entity."); // TODO: handle failures
                }
                _ => {}
            }
            hook_fired.remove(_deletion.player);
        }
    }
}

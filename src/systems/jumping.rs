use amethyst::{
    core::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

use crate::components::Jump;
use crate::components::Player;

const JUMP_VELOCITY: f32 = 350.0;

pub struct JumpingSystem;

impl<'s> System<'s> for JumpingSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
        WriteStorage<'s, Jump>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut players, input, mut jumps, transforms): Self::SystemData) {
        for (_player, _jump, _transform) in (&mut players, &mut jumps, &transforms).join() {
            if !_jump.is_jumping {
                match input.action_is_down("jump") {
                    Some(_is_down) => {
                        if _is_down {
                            _player.velocity[1] = JUMP_VELOCITY;
                            _jump.is_jumping = true;
                        }
                    }
                    _ => {}
                }
            } else {
                if _transform.translation().y <= 0.0 {
                    // TODO: real collisions resolve this
                    _jump.is_jumping = false;
                }
            }
        }
    }
}

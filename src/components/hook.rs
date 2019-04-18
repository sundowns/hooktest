use amethyst::ecs::Entity;

pub struct Hook {
    pub velocity: [f32; 2],
    pub speed: f32,
    pub radius: f32,
    pub max_distance: f32,
    pub owner: Entity,
}

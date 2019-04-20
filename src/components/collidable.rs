pub struct Collidable {
    hitbox: Hitbox,
}

struct Hitbox {
    origin: [f32; 2],
    width: f32,
    height: f32,
}

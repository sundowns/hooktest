pub struct Hook {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Hook {
    type Storage = DenseVecStorage<Self>;
}
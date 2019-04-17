pub struct Jump {
    pub is_jumping: bool,
}

impl Default for Jump {
    fn default() -> Jump {
        Jump { is_jumping: false }
    }
}

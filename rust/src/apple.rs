// apple.rs
use godot::classes::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Apple {
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Apple {
    fn init(base: Base<Area2D>) -> Self {
        Self { base }
    }
}
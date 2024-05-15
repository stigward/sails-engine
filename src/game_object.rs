use raylib::prelude::*;

pub struct Position {
    pub x: i32, 
    pub y: i32
}

pub struct Velocity {
    pub x: i32, 
    pub y: i32
}

pub struct GameObject {
    pub position: Position,
    pub velocity: Velocity,
    pub texture: Texture2D,
    pub rigid_body: bool
}

impl GameObject {
    pub fn new(position: Position, 
               texture: Texture2D, 
               rigid_body: bool) -> Self {
        GameObject {
            position,
            texture,
            rigid_body,
            velocity: Velocity { x: 0, y: 0 }
        }
    }
}


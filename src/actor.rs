use raylib::prelude::*;

use crate::game_object::{GameObject};



pub struct Actor {
    pub object: GameObject,
    isPlayer: bool,
}

impl Actor {
    pub fn new(object: GameObject, isPlayer: bool) -> Self {
        Actor {
            object,
            isPlayer
        }
    }

}

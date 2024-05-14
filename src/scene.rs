use raylib::prelude::*;
use crate::game_object::GameObject;

pub struct Scene {
    game_objects: Vec<GameObject>,
    tilemap: Texture2D 
}

impl Scene {
    pub fn new(tilemap: Texture2D) -> Self {
        Scene {
            game_objects: Vec::new(),
            tilemap
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        for game_object in &self.game_objects {
            // TODO - game_object.draw(d);
        }

        d.draw_texture(&self.tilemap, 0, 0, Color::WHITE);

    }

}

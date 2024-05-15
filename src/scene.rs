use raylib::prelude::*;
use crate::game_object::{GameObject, Position};
use crate::graphics::GraphicsManager;

pub struct Scene {
    // TODO - we can abstract this to an 
    // interface so that it holds both 
    // Actors and Game Objects
    
    game_objects: Vec<GameObject>,
    tilemap: Texture2D 
}

impl Scene {
    pub fn new(tilemap_path: &str, gm: &mut GraphicsManager) -> Self {
        let texture = gm.load_texture(tilemap_path);
        Scene {
            game_objects: Vec::new(),
            tilemap: texture
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_texture(&self.tilemap, 0, 0, Color::WHITE);

        for game_object in &self.game_objects {
            d.draw_texture(&game_object.texture, game_object.position.x, game_object.position.y, Color::WHITE);
        }
    }

    pub fn create_game_object(&mut self, texture_path: &str, rigid_body: bool, gm: &mut GraphicsManager, position: Position) {
        let object = GameObject::new(position, gm.load_texture(texture_path), rigid_body);
        self.game_objects.push(object);
    }

    pub fn add_object(&mut self, game_object: GameObject) {
        self.game_objects.push(game_object);
    }



}

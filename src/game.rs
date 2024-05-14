use raylib::prelude::*;

use crate::graphics::GraphicsManager;
use crate::scene::Scene;

pub struct Game {
    pub graphics_manager: Option<GraphicsManager>,
    pub current_scene: Option<Scene>,
    background_texture: Option<Texture2D>
}

impl Game {
    pub fn new() -> Self {
        Game {
            graphics_manager: None,
            current_scene: None,
            background_texture: None
        }
    }
    pub fn add_window(&mut self, width: i32, height: i32, title: &str) {
        self.graphics_manager = Some(GraphicsManager::new(width, height, title));

    }
    
    pub fn add_scene(&mut self, tilemap_path: &str) {
        let gm: &mut GraphicsManager = self.graphics_manager.as_mut().unwrap();
        let texture = gm.load_texture(tilemap_path);
        
        self.current_scene = Some(Scene::new(texture));
    }


    pub fn run(&mut self) {
        // TODO - handle this better
        let gm = self.graphics_manager.as_mut().unwrap();
        while !gm.window_should_close() {
            let mut d = gm.rl.begin_drawing(&gm.thread);
            self.current_scene.as_mut().unwrap().draw(&mut d);
        }
    }
}

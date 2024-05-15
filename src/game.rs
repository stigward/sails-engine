use raylib::prelude::*;

use crate::graphics::GraphicsManager;
use crate::scene::Scene;

pub struct Game {
    pub graphics_manager: GraphicsManager,
    pub current_scene: Option<Scene>,
    background_texture: Option<Texture2D>
}

impl Game {
    pub fn new(width: i32, height: i32, title: &str) -> Self {
        Game {
            graphics_manager: GraphicsManager::new(width, height, title),
            current_scene: None,
            background_texture: None
        }
    }
    pub fn add_window(&mut self, width: i32, height: i32, title: &str) {
        self.graphics_manager = GraphicsManager::new(width, height, title)
    }
    
    pub fn add_scene(&mut self, scene: Scene) {
        self.current_scene = Some(scene); 
    }


    pub fn run(&mut self) {
        // TODO - handle this better
        while !self.graphics_manager.window_should_close() {
            let mut d = self.graphics_manager.rl.begin_drawing(&self.graphics_manager.thread);
            self.current_scene.as_mut().unwrap().draw(&mut d);
        }
    }
}

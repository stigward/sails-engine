use raylib::prelude::*;

pub struct GameObject {
    position: Position
}

pub struct Position(i32, i32);

pub struct Game {
    graphics_manager: Option<GraphicsManager>,
    current_scene: Option<Scene>,
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
    
    //TODO - add params (background)
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

// TODO - Add tilemap

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

pub struct GraphicsManager {
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl GraphicsManager {
    pub fn new(width: i32, height: i32, title: &str) -> Self {
        let (rl, thread) = raylib::init()
            .size(width, height)
            .title(title)
            .build();
        GraphicsManager { rl, thread }       
    }

    pub fn window_should_close(&self) -> bool {
        self.rl.window_should_close()
    }

    pub fn load_texture(&mut self, image_path: &str) -> Texture2D {
        let image = Image::load_image(image_path).expect("Failed to load image");
        self.rl.load_texture_from_image(&self.thread, &image).expect("Failed to load texture")
    }


}

use raylib::prelude::*;

pub struct GameObject {
    position: Position

}

pub struct Position(i32, i32);

pub struct Engine {
    game_objects: Vec<GameObject>,
    graphics_manager: Option<GraphicsManager>
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            game_objects: Vec::new(),
            graphics_manager: None
        }
    }
    pub fn add_window(&mut self, width: i32, height: i32, title: &str) {
        self.graphics_manager = Some(GraphicsManager::new(width, height, title));

    }

    pub fn run(&mut self) {
        // TODO - handle this better
        let mut gm = self.graphics_manager.as_mut().unwrap();
        while !gm.window_should_close() {
            let mut d = gm.rl.begin_drawing(&gm.thread);

            d.clear_background(Color::WHITE);
            d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        }
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


}

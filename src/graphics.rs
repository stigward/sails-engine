use raylib::prelude::*;
 
pub struct GraphicsManager {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
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

extern crate sails_engine;

fn main() {
    println!("Hello, world!");
    let mut new_game = sails_engine::Game::new();
    new_game.add_window(500, 500, "Hello World");
    new_game.add_scene("./assets/tilemap.png");
    new_game.run();
}

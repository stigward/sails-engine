extern crate sails_engine;

fn main() {
    println!("Hello, world!");
    let mut new_game = sails_engine::Game::new();
    new_game.add_window(500, 500, "Hello World");
    new_game.add_scene("./assets/tilemap.png");
    new_game.current_scene
        .as_mut()
        .unwrap()
        .add_game_object("./assets/character.png", true, new_game.graphics_manager.as_mut().unwrap(), sails_engine::game_object::Position{x: 0, y: 0});
    new_game.run();
}

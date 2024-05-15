extern crate sails_engine;

fn main() {
    println!("Hello, world!");
    let mut new_game = sails_engine::Game::new(500, 500, "Sails Engine");
    // TODO - don't like this, should create the scene, and then add it, not initialize it this way
    // TODO - graphics manager should not be optional

    let mut scene = sails_engine::Scene::new("./assets/tilemap.png", &mut new_game.graphics_manager);
    scene.add_game_object("./assets/character.png", 
                          true, 
                          &mut new_game.graphics_manager, 
                          sails_engine::game_object::Position {x: 0, y: 0}
                          );
    new_game.add_scene(scene); 

    new_game.run();
}

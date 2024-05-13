extern crate sails_engine;

fn main() {
    println!("Hello, world!");
    let mut sails_engine = sails_engine::Engine::new();
    sails_engine.add_window(500, 500, "Hello World");
    sails_engine.run();

}

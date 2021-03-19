// Thanks! (https://stackoverflow.com/questions/29763647/how-to-make-a-program-that-does-not-display-the-console-window)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use ggez::conf::{WindowMode};
use ggez::{ContextBuilder, GameResult, Context};
use ggez::event;
extern crate nalgebra as na;

struct Game {}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {}
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}


pub fn main() -> GameResult {
    let w_dim = nalgebra::Vector2::new(960.0, 640.0);

    let (context, event_loop) = &mut ContextBuilder::new("batman-hash", "Lucas")
        .window_setup(ggez::conf::WindowSetup::default().title("batman-hash"))
        .window_mode(WindowMode::default().dimensions(w_dim.x, w_dim.y).max_dimensions(w_dim.x, w_dim.y).min_dimensions(w_dim.x, w_dim.y),)
        .build()
            .expect("Could not populate GGEZ context");
    
    let game = &mut Game::new(context);

    match event::run(context, event_loop, game) {
        Ok(_) => println!("Exited properly :)"),
        Err(e) => println!("The game had some problems: {}", e)
    }

    Ok(())
}

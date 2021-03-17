use ggez::*;
use ggez::{ContextBuilder, Context, GameResult};
use ggez::event::{EventHandler};

struct Game {}

const TARGET_FPS: u32 = 60;

impl Game {
    pub fn new(_context: &mut Context) -> Game {
        Game {}
    }
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}


pub fn main() {
    let (mut context, mut event_loop) = ContextBuilder::new
    ("kiwigrape-matchmaking", "Lucas")
    .build()
    .expect("Problem fetching GGEZlib context!");

    let mut game = Game::new(&mut context);

    match event::run(&mut context, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited properly :)"),
        Err(e) => println!("The game had some problems: {}", e)
    }
}

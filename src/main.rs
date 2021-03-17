use ggez::*;
use ggez::{ContextBuilder, Context, GameResult, event};
use ggez::event::{EventHandler};

struct Game {}

impl Game {
    pub fn new(_context: &mut Context) -> Game {
        Game {}
    }
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, context: &mut Context) -> GameResult {
        Ok(())
    }
}


pub fn main() {
    let c = conf::Conf::new();
    let (mut context, mut event_loop) = ContextBuilder::new("kiwigrape-matchmaking", "Lucas")
        .conf(c)
        .build()
        .unwrap();
    
    let mut game = Game::new(&mut context);    
    
    match event::run(&mut context, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited properly :)"),
        Err(e) => println!("The game had some problems: {}", e)
    }
}

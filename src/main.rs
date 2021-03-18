use ggez::conf::WindowMode;
use ggez::{ContextBuilder, GameResult, Context};
use ggez::event;

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

    let window_mode = WindowMode::default()
        .dimensions(1920.0, 1080.0)
        .resizable(true);
    let (context, event_loop) = &mut ContextBuilder::new("kiwigrape-matchmaking", "Lucas")
        .window_mode(window_mode)
        .build()?;
    
    let game = &mut Game::new(context);

    match event::run(context, event_loop, game) {
        Ok(_) => println!("Exited properly :)"),
        Err(e) => println!("The game had some problems: {}", e)
    }

    Ok(())
}

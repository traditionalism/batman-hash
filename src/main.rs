use ggez::{ContextBuilder, GameResult, Context};
use ggez::event;

pub const WIDTH: i32 = 960;
pub const HEIGHT: i32 = 540;
pub const TL: i32 = 32;
pub const WIDTH_TL: i32 = WIDTH / TL;
pub const HEIGHT_TL: i32 = HEIGHT / TL;

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
    let (context, event_loop) = &mut ContextBuilder::new("batman-hash", "Lucas")
        .window_setup(ggez::conf::WindowSetup::default().title("batman-hash"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WIDTH as f32, HEIGHT as f32))
        .build()
            .expect("Could not populate GGEZ context");
    
    let game = &mut Game::new(context);

    match event::run(context, event_loop, game) {
        Ok(_) => println!("Exited properly :)"),
        Err(e) => println!("The game had some problems: {}", e)
    }

    Ok(())
}

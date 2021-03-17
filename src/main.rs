use ggez::conf::WindowMode;
use ggez::{Context, GameResult};
use ggez::{event, ContextBuilder};

const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;

struct Game {}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {}
    }
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}


pub fn main() {
    let mut window_mode: WindowMode = Default::default();
        window_mode.width = WIDTH;
        window_mode.height = HEIGHT;
    let (mut ctx, mut event_loop) = ContextBuilder::new("kiwigrape-matchmaking", "Lucas")
        .window_mode(window_mode)
        .build()
        .unwrap();

    let mut game = Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited properly :)"),
        Err(e) => println!("The game had some problems: {}", e)
    }
}
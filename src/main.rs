use ggez::*;

struct State {}

pub fn main() {
    let state = &mut State {};
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("kiwigrape-matchmaking", "Lucas")
    .conf(c)
    .build()
    .unwrap();
    event::run(ctx, event_loop, state).unwrap();
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
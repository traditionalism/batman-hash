// Thanks! (https://stackoverflow.com/questions/29763647/how-to-make-a-program-that-does-not-display-the-console-window)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::imgui_wrapper::ImGuiWrapper;
use crate::utils::{fix_path};
use ggez::conf::{WindowMode};
use ggez::{GameResult, Context};
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;

use std::env;
use log::{info};

mod utils;
mod imgui_wrapper;


struct GameState {
    pos_x: f32,
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
}

impl GameState {
    fn new(mut ctx: &mut Context, hidpi_factor: f32) -> GameResult<GameState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);
        let s = GameState {
            pos_x: 0.0,
            imgui_wrapper,
            hidpi_factor,
        };
        Ok(s)
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        {
            self.imgui_wrapper.render(ctx, self.hidpi_factor);
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down(button);
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_up(button);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        self.imgui_wrapper.update_key_down(keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.imgui_wrapper.update_key_up(keycode, keymods);
    }

    fn text_input_event(&mut self, _ctx: &mut Context, val: char) {
        self.imgui_wrapper.update_text(val);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.imgui_wrapper.update_scroll(x, y);
    }
}


pub fn main() -> GameResult {
    #[cfg(debug_assertions)]
    {
        use log::LevelFilter;

        env_logger::Builder::new()
            .filter(None, LevelFilter::Info)
            .filter(Some("gfx_device_gl"), LevelFilter::Error)
            .init();
    }

    let w_dim = nalgebra::Vector2::new(960.0, 640.0);

    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("resources");

    let mut cb = ggez::ContextBuilder::new("batman-hash", "Lucas")
        .window_setup(ggez::conf::WindowSetup::default().title("batman-hash"))
        .window_mode(WindowMode::default().dimensions(w_dim.x, w_dim.y).max_dimensions(w_dim.x, w_dim.y).min_dimensions(w_dim.x, w_dim.y));


    #[cfg(debug_assertions)]
    {
        info!("Use resource in {:?}", current_dir);
        cb = cb.add_resource_path(current_dir);
    }

    #[cfg(not(debug_assertions))]
    {
        info!("Use resource archive");
        cb = cb.add_zipfile_bytes(include_bytes!("../resources/zip").to_vec());
    }

    let (ctx, event_loop) = &mut cb.build().unwrap();

    graphics::set_window_icon(ctx, Some(fix_path(ctx, "128x128.ico"))).unwrap();

    let hidpi_factor = event_loop.get_primary_monitor().get_hidpi_factor() as f32;

    let state = &mut GameState::new(ctx, hidpi_factor)?;

    event::run(ctx, event_loop, state)
}

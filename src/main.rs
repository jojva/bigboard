extern crate ggez;

mod color;
mod grid;

use color::ColorWheel;
use grid::{Cursor, Direction, Grid, GridPosition};

use ggez::event::Keycode;
use ggez::{event, graphics, Context, GameResult};
use std::time::{Duration, Instant};

const UPDATES_PER_SECOND: f32 = 8.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

struct GameState<'a> {
    color_wheel: ColorWheel<'a>,
    cursor: Cursor,
    grid: Grid,
    last_update: Instant,
}

impl<'a> GameState<'a> {
    pub fn new() -> Self {
        let cursor_pos = GridPosition::new(10, 10);

        GameState {
            color_wheel: ColorWheel::new(),
            cursor: Cursor::new(cursor_pos),
            grid: Grid::new(),
            last_update: Instant::now(),
        }
    }
}

impl<'a> event::EventHandler for GameState<'a> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // graphics::clear(ctx);
        self.grid.draw(ctx)?;
        self.cursor.draw(ctx)?;
        graphics::present(ctx);
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: event::Mod,
        _repeat: bool,
    ) {
        if let Some(dir) = Direction::from_keycode(keycode) {
            self.cursor.move_to(dir);
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: i32, y: i32) {
        if y > 0 {
            self.color_wheel.backward();
        } else if y < 0 {
            self.color_wheel.forward();
        }
        self.cursor.set_color(self.color_wheel.get_color());
    }
}

fn main() {
    let ctx = &mut ggez::ContextBuilder::new("bigboard", "Joris Valette")
        .window_setup(ggez::conf::WindowSetup::default().title("Big Board"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(grid::SCREEN_SIZE.0, grid::SCREEN_SIZE.1))
        .build()
        .expect("Failed to build ggez context");

    graphics::set_background_color(ctx, [1.0, 0.41, 0.70, 1.0].into());
    let state = &mut GameState::new();
    match event::run(ctx, state) {
        Err(e) => println!("Error encountered running game: {}", e),
        Ok(_) => println!("Game exited cleanly!"),
    }
}

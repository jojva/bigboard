extern crate ggez;

use ggez::event::Keycode;
use ggez::{event, graphics, Context, GameResult};

use std::time::{Duration, Instant};


const GRID_SIZE: (i32, i32) = (30, 20);
const GRID_CELL_SIZE: (i32, i32) = (32, 32);

const SCREEN_SIZE: (u32, u32) = (
    GRID_SIZE.0 as u32 * GRID_CELL_SIZE.0 as u32,
    GRID_SIZE.1 as u32 * GRID_CELL_SIZE.1 as u32,
);

const UPDATES_PER_SECOND: f32 = 8.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i32,
    y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        GridPosition { x, y }
    }

    pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
        match dir {
            Direction::Up => GridPosition::new(pos.x, pos.y - 1),
            Direction::Down => GridPosition::new(pos.x, pos.y + 1),
            Direction::Left => GridPosition::new(pos.x - 1, pos.y),
            Direction::Right => GridPosition::new(pos.x + 1, pos.y),
        }
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl From<(i32, i32)> for GridPosition {
    fn from(pos: (i32, i32)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_keycode(key: Keycode) -> Option<Direction> {
        match key {
            Keycode::Up => Some(Direction::Up),
            Keycode::Down => Some(Direction::Down),
            Keycode::Left => Some(Direction::Left),
            Keycode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

struct Cursor {
    pos: GridPosition,
}

impl Cursor {
    pub fn new(pos: GridPosition) -> Self {
        Cursor { pos }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, [0.0, 0.0, 1.0, 1.0].into())?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, self.pos.into())
    }
}

struct GameState {
    cursor: Cursor,
    last_update: Instant,
}

impl GameState {
    pub fn new() -> Self {
        let cursor_pos = GridPosition::new(10, 10);

        GameState {
            cursor: Cursor::new(cursor_pos),
            last_update: Instant::now(),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
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
            self.cursor.pos = GridPosition::new_from_move(self.cursor.pos, dir);
        }
    }
}

fn main() {
    let ctx = &mut ggez::ContextBuilder::new("bigboard", "Joris Valette")
        .window_setup(ggez::conf::WindowSetup::default().title("Big Board"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build().expect("Failed to build ggez context");

    graphics::set_background_color(ctx, [1.0, 1.0, 1.0, 1.0].into());
    let state = &mut GameState::new();
    match event::run(ctx, state) {
        Err(e) => println!("Error encountered running game: {}", e),
        Ok(_) => println!("Game exited cleanly!"),
    }
}

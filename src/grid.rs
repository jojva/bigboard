use color;

use ggez::event::Keycode;
use ggez::graphics::{Color, DrawMode};
use ggez::{graphics, Context, GameResult};

pub const GRID_SIZE: (u32, u32) = (30, 20);
pub const GRID_CELL_SIZE: (u32, u32) = (32, 32);

pub const SCREEN_SIZE: (u32, u32) = (
    GRID_SIZE.0 * GRID_CELL_SIZE.0,
    GRID_SIZE.1 * GRID_CELL_SIZE.1,
);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
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

impl From<(usize, usize)> for GridPosition {
    fn from(pos: (usize, usize)) -> Self {
        GridPosition {
            x: pos.0 as i32,
            y: pos.1 as i32,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
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

#[derive(Clone, Copy)]
pub struct Cell {
    color: Color,
    draw_mode: graphics::DrawMode,
    dirty: bool,
}

impl Cell {
    pub fn new(color: Color, draw_mode: graphics::DrawMode) -> Self {
        Cell {
            color,
            draw_mode,
            dirty: true,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.dirty = true;
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn draw(&mut self, ctx: &mut Context, pos: GridPosition) -> GameResult<()> {
        if self.dirty {
            graphics::set_color(ctx, self.color)?;
            graphics::rectangle(ctx, self.draw_mode, pos.into())?;
            self.dirty = false;
        }
        Ok(())
    }
}

pub struct Cursor {
    pos: GridPosition,
    cell: Cell,
}

impl Cursor {
    pub fn new(pos: GridPosition) -> Self {
        Cursor {
            pos: pos,
            cell: Cell::new(color::BLACK, graphics::DrawMode::Line(5.0)),
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.cell.set_color(color);
    }

    pub fn move_to(&mut self, dir: Direction) {
        self.pos = GridPosition::new_from_move(self.pos, dir);
        self.cell.set_dirty();
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.cell.draw(ctx, self.pos)
    }
}

pub struct Grid {
    cells: [[Cell; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            cells: [[Cell { color: color::WHITE, draw_mode: DrawMode::Fill, dirty: true }; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize],
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for (row, row_item) in self.cells.iter_mut().enumerate() {
            for (col, col_item) in row_item.iter_mut().enumerate() {
                let grid_position: GridPosition = (row, col).into();
                col_item.draw(ctx, grid_position)?;
            }
        }
        Ok(())
    }
}

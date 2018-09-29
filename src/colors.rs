extern crate ggez;

use ggez::graphics;
use ggez::graphics::Color;

use std::iter::{Cycle, Peekable};
use std::slice::Iter;

// Black and white are already defined in ggez
pub const BLACK: Color = graphics::BLACK;
pub const WHITE: Color = graphics::WHITE;

pub const DARK_GRAY: Color = Color {
    r: 0.34,
    g: 0.34,
    b: 0.34,
    a: 1.0,
};

pub const LIGHT_GRAY: Color = Color {
    r: 0.63,
    g: 0.63,
    b: 0.63,
    a: 1.0,
};

pub const BLUE: Color = Color {
    r: 0.16,
    g: 0.29,
    b: 0.84,
    a: 1.0,
};

pub const LIGHT_BLUE: Color = Color {
    r: 0.61,
    g: 0.68,
    b: 1.0,
    a: 1.0,
};

pub const CYAN: Color = Color {
    r: 0.16,
    g: 0.82,
    b: 0.82,
    a: 1.0,
};

pub const GREEN: Color = Color {
    r: 0.11,
    g: 0.41,
    b: 0.0,
    a: 1.0,
};

pub const LIGHT_GREEN: Color = Color {
    r: 0.51,
    g: 0.77,
    b: 0.03,
    a: 1.0,
};

pub const YELLOW: Color = Color {
    r: 1.00,
    g: 0.93,
    b: 0.01,
    a: 1.0,
};

pub const BROWN: Color = Color {
    r: 0.51,
    g: 0.29,
    b: 0.0,
    a: 1.0,
};

pub const TAN: Color = Color {
    r: 0.91,
    g: 0.87,
    b: 0.04,
    a: 1.0,
};

pub const ORANGE: Color = Color {
    r: 1.0,
    g: 0.57,
    b: 0.01,
    a: 1.0,
};

pub const PURPLE: Color = Color {
    r: 0.51,
    g: 0.15,
    b: 0.05,
    a: 1.0,
};

pub const RED: Color = Color {
    r: 0.68,
    g: 0.14,
    b: 0.01,
    a: 1.0,
};

pub const PINK: Color = Color {
    r: 1.0,
    g: 0.80,
    b: 0.06,
    a: 1.0,
};

const COLORWHEEL: [Color; 16] = [
    BLACK,
    WHITE,
    DARK_GRAY,
    LIGHT_GRAY,
    BLUE,
    LIGHT_BLUE,
    CYAN,
    GREEN,
    LIGHT_GREEN,
    YELLOW,
    BROWN,
    TAN,
    ORANGE,
    PURPLE,
    RED,
    PINK,
];

pub struct ColorWheel<'a> {
    color: Peekable<Cycle<Iter<'a, Color>>>,
}

impl<'a> ColorWheel<'a> {
    pub fn new() -> Self {
        ColorWheel {
            color: COLORWHEEL.iter().cycle().peekable(),
        }
    }

    pub fn forward(&mut self) {
        self.color.next();
    }

    pub fn backward(&mut self) {
        // Won't do anything until I figure out how to iterate backwards in Rust
    }

    pub fn get_color(&mut self) -> Color {
        **self.color.peek().unwrap()
    }
}

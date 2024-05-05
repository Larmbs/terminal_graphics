
mod screen;
mod canvas;

pub use screen::Screen;
pub use canvas::Canvas;

use std::io;

use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};

fn clear_screen() -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::All))?;
    Ok(())
}

fn set_cursor_to_start() -> io::Result<()> {
    execute!(io::stdout(), MoveTo(0, 0))?;
    Ok(())
}

pub fn init() {
    clear_screen().unwrap();
}

pub trait PixelScreen {
    fn get_size(&self) -> (u8, u8);
    fn set_pixel(&mut self, x: usize, y: usize, color: u8);
    fn fill(&mut self, color: u8);
    fn clear_screen(&self) {
        set_cursor_to_start().unwrap();
    }
    fn display(&self);
}

fn char_for_pixel(pixel: u8) -> char {
    match pixel {
        0..=51 => ' ',
        52..=102 => '.',
        103..=153 => ':',
        154..=204 => '-',
        205..=255 => '#',
    }
}

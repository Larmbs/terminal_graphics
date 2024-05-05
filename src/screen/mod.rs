
mod screen;
mod canvas;

pub use screen::Screen;
pub use canvas::Canvas;


pub trait PixelScreen {
    fn get_size(&self) -> (u8, u8);
    fn set_pixel(&mut self, x: usize, y: usize, color: u8);
    fn fill(&mut self, color: u8);
    fn clear_screen(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
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

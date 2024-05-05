use std::io::{self, Write};

fn char_for_pixel(pixel: u8) -> char {
    // For example, you could use a gradient from darkest to lightest character
    match pixel {
        0..=51 => ' ',
        52..=102 => '.',
        103..=153 => ':',
        154..=204 => '-',
        205..=255 => '#',
    }
}

pub struct Screen {
    height: u8,
    width: u8,
    pixel_grid: Vec<Vec<u8>>,
}

impl Screen {
    pub fn new(width: u8, height: u8, color: u8) -> Self {
        let pixel_grid = vec![vec![color; width as usize]; height as usize];
        
        Screen { width, height, pixel_grid: pixel_grid }
    }

    pub fn get_size(&self) -> (u8, u8) {
        (self.width, self.height)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        if let Some(row) = self.pixel_grid.get_mut(y as usize) {
            if let Some(pixel) = row.get_mut(x as usize) {
                *pixel = color;
            }
        }   
    }

    pub fn clear_screen() {
        print!("\x1B[2J");
        io::stdout().flush().expect("Failed to flush stdout");
    }

    pub fn fill(&mut self, color: u8) {
        self.pixel_grid = vec![vec![color; self.width as usize]; self.height as usize];
    }

    pub fn display(&self) {
        Screen::clear_screen();
        for row in &*self.pixel_grid {
            for &pixel in row {
                print!("{} ", char_for_pixel(pixel));
            }
            println!(); // Move to the next line after printing a row
        }
    }
}

pub struct Canvas {
    screen: Screen,
}

impl Canvas {
    pub fn new(width: u8, height: u8, color: u8) -> Self {
        let screen = Screen::new(width, height, color);
        Canvas { screen }
    }

    pub fn get_size(&self) -> (u8, u8) {
        self.screen.get_size()
    }

    pub fn display(&self) {
        self.screen.display();
    }

    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u8) {
        let mut x1 = x1 as isize;
        let mut y1 = y1 as isize;
        let mut x2 = x2 as isize;
        let mut y2 = y2 as isize;

        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            self.screen.set_pixel(x1 as usize, y1 as usize, color);

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x1 += sx;
            }
            if e2 <= dx {
                err += dx;
                y1 += sy;
            }
        }
    }

    pub fn draw_circle(&mut self, x: usize, y: usize, radius: usize, color: u8) {
        let mut x0 = radius as isize;
        let mut y0 = 0;
        let mut err = 0;

        while x0 >= y0 {
            self.screen.set_pixel((x as isize + x0) as usize, (y as isize + y0) as usize, color);
            self.screen.set_pixel((x as isize + y0) as usize, (y as isize + x0) as usize, color);
            self.screen.set_pixel((x as isize - y0) as usize, (y as isize + x0) as usize, color);
            self.screen.set_pixel((x as isize - x0) as usize, (y as isize + y0) as usize, color);
            self.screen.set_pixel((x as isize - x0) as usize, (y as isize - y0) as usize, color);
            self.screen.set_pixel((x as isize - y0) as usize, (y as isize - x0) as usize, color);
            self.screen.set_pixel((x as isize + y0) as usize, (y as isize - x0) as usize, color);
            self.screen.set_pixel((x as isize + x0) as usize, (y as isize - y0) as usize, color);

            y0 += 1;
            err += 1 + 2 * y0;
            if 2 * (err - x0) + 1 > 0 {
                x0 -= 1;
                err += 1 - 2 * x0;
            }
        }
    }
}
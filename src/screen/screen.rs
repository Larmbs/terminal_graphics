use super::{char_for_pixel, PixelScreen};


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
}

impl PixelScreen for Screen {
    fn get_size(&self) -> (u8, u8) {
        (self.width, self.height)
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        if let Some(row) = self.pixel_grid.get_mut(y as usize) {
            if let Some(pixel) = row.get_mut(x as usize) {
                *pixel = color;
            }
        }   
    }

    fn fill(&mut self, color: u8) {
        self.pixel_grid = vec![vec![color; self.width as usize]; self.height as usize];
    }

    fn display(&self) {
        self.clear_screen();
        for row in &*self.pixel_grid {
            for &pixel in row {
                print!("{} ", char_for_pixel(pixel));
            }
            println!(); // Move to the next line after printing a row
        }
    }
}


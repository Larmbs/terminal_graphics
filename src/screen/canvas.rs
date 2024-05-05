use super::{PixelScreen, Screen};


pub struct Canvas {
    screen: Screen,
}

/// Forwarding PixelScreens
impl PixelScreen for Canvas {
    fn get_size(&self) -> (u8, u8) {
        self.screen.get_size()
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        self.screen.set_pixel(x, y, color);
    }

    fn fill(&mut self, color: u8) {
        self.screen.fill(color);
    }

    fn display(&self) {
        self.screen.display();
    }
}
impl Canvas {
    pub fn new(width: u8, height: u8, color: u8) -> Self {
        let screen = Screen::new(width, height, color);
        Canvas { screen }
    }
}

#[allow(unused)]
impl Canvas {
    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u8) {
        let mut x1 = x1 as isize;
        let mut y1 = y1 as isize;
        let x2 = x2 as isize;
        let y2 = y2 as isize;

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

    pub fn draw_rect(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u8) {
        // Draw top and bottom edges
        self.draw_line(x1, y1, x2, y1, color);
        self.draw_line(x1, y2, x2, y2, color);
    
        // Draw left and right edges
        self.draw_line(x1, y1, x1, y2, color);
        self.draw_line(x2, y1, x2, y2, color);
    }

    pub fn draw_rect_filled(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u8) {
        // Calculate the top-left and bottom-right corners of the rectangle
        let x_min = x1.min(x2);
        let y_min = y1.min(y2);
        let x_max = x1.max(x2);
        let y_max = y1.max(y2);
    
        // Iterate over each row of the rectangle and draw a horizontal line
        for y in y_min..=y_max {
            self.draw_line(x_min, y, x_max, y, color);
        }
    }
}

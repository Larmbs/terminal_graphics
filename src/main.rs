use screen::PixelScreen;
use std::thread;
use std::time::Duration;
mod screen;

fn main() {
    screen::init();
    let mut screen = screen::Canvas::new(40, 30, 250);
    screen.draw_line(0, 0, 50, 20, 0);
    screen.draw_circle(10, 10, 10, 100);
    screen.display();
    let mut x = 0;

    loop {
        screen.fill(250);
        x += 2;

        x %= 41;

        screen.draw_line(x, 30, 0, 0, 0);
        screen.draw_circle(x/2, 15, 7, 60);
        screen.display();
        thread::sleep(Duration::from_millis(30));
    }
}

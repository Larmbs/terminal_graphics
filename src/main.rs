use screen::PixelScreen;

mod screen;

fn main() {
    let mut screen = screen::Canvas::new(80, 30, 250);
    screen.draw_line(0, 0, 50, 20, 0);
    screen.draw_circle(10, 10, 10, 100);
    screen.display();
    let mut x = 0;
    loop {
        x += 1;
        screen.draw_line(x, 0, 50 + x, 20, 0);
        screen.display();
    }
}

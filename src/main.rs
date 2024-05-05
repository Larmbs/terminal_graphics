mod screen;

fn main() {
    let mut screen = screen::Canvas::new(80, 30, 250);
    screen.draw_line(0, 0, 50, 20, 0);
    screen.draw_circle(10, 10, 10, 100);
    screen.display();
}

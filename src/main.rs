use terminal_graphics::{
    init_display,
    Canvas,
    PixelScreen,
    Clock,
    Duration,
};

fn main() {
    init_display();
    let mut screen = Canvas::new(40, 30, 250);
   
    let mut x = 0;

    let mut clock = Clock::new();

    loop {
        screen.fill(250);
        x += 1;

        x %= 41;

        screen.draw_line(x, 30, 0, 0, 0);
        screen.draw_circle(x/2 as usize, 15, 7, 60);

        screen.display();

        clock.tick(Duration::from_millis(17))
    }
}

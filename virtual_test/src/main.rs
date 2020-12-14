use opengl_graphics::{GlGraphics, OpenGL};
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Virtual Test", [512, 342])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        println!("{:#x?}", e);
        //if let Some(r) = e.render_args() {
        //    app.render(&r);
        //}

        //if let Some(u) = e.update_args() {
        //    app.update(&u);
        //}

        //if let Some(b) = e.press_args() {
        //    app.press(&b);
        //}

        //if let Some(b) = e.release_args() {
        //    app.release(&b);
        //}
    }
}

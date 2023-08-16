use fltk::{app, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Clipr");
    wind.end();
    wind.show();
    match app.run() {
        Ok(_) => println!("Running app!"),
        Err(e) => panic!("Error running app! {:?}", e)
    }
}
use std::io;

mod app;
mod buffer;
mod screen;
mod playground;
mod tetro;
mod units;

use app::App;
use std::panic;

fn main() -> Result<(), io::Error> {
    // panic::set_hook(Box::new(|_info| { }));

    let mut app = App::new().unwrap();
    app.run().unwrap();
    Ok(())
}

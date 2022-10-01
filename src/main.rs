mod app;

use app::create_app;
use cursive::CursiveExt;

fn main() {
    let mut app = create_app();

    app.run();
}


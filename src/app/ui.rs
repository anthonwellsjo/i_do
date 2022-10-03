use cursive::Cursive;
use cursive::views::{ResizedView, NamedView, SelectView};
use cursive::view::{Nameable, Resizable};


pub fn get_select_view(on_submit: fn(s: &mut Cursive, name: &str)) -> ResizedView<NamedView<SelectView>> {
     SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size((10, 5))
}

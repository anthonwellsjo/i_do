use cursive::view::{Nameable, Resizable};
use cursive::views::{Button, Dialog, DummyView, LinearLayout, NamedView, ResizedView, SelectView};
use cursive::Cursive;

pub fn get_select_view(
    on_submit: fn(s: &mut Cursive, name: &str),
) -> ResizedView<NamedView<SelectView>> {
    SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size((10, 5))
}

pub fn get_buttons(
    add_name: fn(s: &mut Cursive),
    delete_name: fn(s: &mut Cursive),
) -> LinearLayout {
    LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit))
}

pub fn compose_app_ui(
    s: &mut Cursive,
    select: ResizedView<NamedView<SelectView>>,
    buttons: LinearLayout,
) {
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons),
        )
        .title("Select a user"),
    )
}

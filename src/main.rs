use cursive::{
    traits::{Nameable, Resizable},
    views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView},
    Cursive, CursiveExt,
};

fn main() {
    let mut siv = Cursive::default();
    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size((10, 5));

    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons),
        )
        .title("Select a user"),
    );

    siv.run();
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(format!("{}s infor", name))
            .title("User info")
            .button("Close", Cursive::quit),
    );
}

fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name);
        });
        s.pop_layer();
    }
    s.add_layer(
        Dialog::around(EditView::new().with_name("user_name").fixed_width(10))
            .title("Add user")
            .button("OK", |s| {
                let name = s
                    .call_on_name("user_name", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                ok(s, &name);
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}

fn delete_name(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No user to delete.")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

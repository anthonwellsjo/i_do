mod ui;
mod db;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, SelectView};
use cursive::Cursive;

pub fn create_app() -> Cursive {
    let mut siv: Cursive = Cursive::default();
    let select = ui::get_select_view(on_submit);
    let buttons = ui::get_buttons(add_name, delete_name);
    ui::compose_app_ui(&mut siv, select, buttons);
    return siv;
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
                    .call_on_name("user_name", |view: &mut EditView| view.get_content())
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

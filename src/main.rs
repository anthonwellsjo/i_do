use menu;

fn main() {
    println!("Hello, world!");
    let menuez = menu::Menu::new(vec!["Add I DO", "Remove I DO"]);
    menuez.print_items();
}

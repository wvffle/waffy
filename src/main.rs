use gtk::*;
use gtk_layer_shell_rs::*;
use gdk::*;

const GTK_STYLE_PROVIDER_PRIORITY_USER: i32 = 800;

fn main() {
    open_config();
    let desktop_entries = find_desktop_entries();

    gtk::init().expect("Could not init GTK!");

    let window = Window::new(WindowType::Toplevel);
    init_for_window(&window);
    set_layer(&window, Layer::Top);
    set_anchor(&window, Edge::Top, true);
    set_anchor(&window, Edge::Left, true);
    set_anchor(&window, Edge::Right, true);
    set_anchor(&window, Edge::Bottom, true);

    window.set_resizable(false);
    window.set_decorated(false);

    let layout = Box::new(Orientation::Vertical, 0);
    window.add(&layout);

    let search_box = Box::new(Orientation::Horizontal, 0);
    layout.pack_start(&search_box, false, false, 0);

    let grid_width = 270 * config_columns + (config_columns + 2) * COL_PADDING;
    let spacer = Box::new(Orientation::Horizontal, 0);
    let width = get_monitor_width() - grid_width / 2;
    spacer.set_size_request(width, 1);

    search_box.pack_start(&spacer, false, false, 0);

    let search_label = Label::new(None);
    search_label.set_markup(config_prompt);
    search_box.pack_start(&search_label, false, false, 0);

    search_label.add_class("textview-label");

    let search_input = TextView::new();
    search_input.set_name("search");
    search_box.pack_start(&search_input, true, true, 0);

    let app_grid = Grid::new();
    layout.pack_start(&app_grid, true, true, 0);

    for i in 0..config_columns {
        app_grid.insert_column(i);
    }

    app_grid.set_column_spacing(COL_PADDING);
    app_grid.set_row_spacing(COL_PADDING);
    app_grid.set_name("apps");

    app_grid.set_halign(Align::Center);

    update_apps(&desktop_entries);

    let css_provider = CssProvider::new();

    if config_wal == 1 {
        window.add_class("pywal");
    }

    let css = get_css();

    if let Ok(_) = css_provider.load_from_data(css.as_ref()) {
        css_provider.add_provider_for_screen(&window, css, GTK_STYLE_PROVIDER_PRIORITY_USER);
    }

    if let Some(display) = window.get_display() {
        let arrow = Cursor::new_from_name(&display, "default")
            .expect("Could not create 'default' cursor!");

        let pointer = Cursor::new_from_name(&display, "pointer")
            .expect("Could not create 'pointer' cursor!");
    }

    window.set_title("Waffy");
    window.show_all();

    gtk::main();
}

struct DesktopEntry {

}

fn get_css() -> String {

}

fn update_apps(apps: &Vec<DesktopEntry>) {

}

fn get_monitor_width() -> i32 {

}

fn find_desktop_entries() -> Vec<DesktopEntry> {

}

fn open_config() {

}

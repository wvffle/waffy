use crate::grid::GridButton;
use std::cell::RefCell;
use std::rc::Rc;

use gdk::*;
use gtk::{
    BoxExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Inhibit, Label, LabelExt,
    Orientation, StyleContext, StyleContextExt, TextView, WidgetExt,
};
use gtk_layer_shell_rs::*;

mod config;
mod grid;
mod resource;
mod style;

mod desktop_entry;
use desktop_entry::DesktopEntry;

use config::Config;

fn main() {
    Config::create_dir();

    let config = Config::get();
    let desktop_entries = DesktopEntry::get_all();

    gtk::init().expect("Could not init GTK!");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    init_for_window(&window);
    set_layer(&window, Layer::Top);
    set_anchor(&window, Edge::Top, true);
    set_anchor(&window, Edge::Left, true);
    set_anchor(&window, Edge::Right, true);
    set_anchor(&window, Edge::Bottom, true);
    set_keyboard_interactivity(&window, true);


    window.connect_key_release_event(|window, key| {
        use gdk::enums::key;

        match key.get_keyval() {
            key::Escape => {
                window.close();
                std::process::exit(0)
            }
            _ => Inhibit(true),
        }
    });

    window.set_resizable(false);
    window.set_decorated(false);

    let layout = gtk::Box::new(Orientation::Vertical, 0);
    window.add(&layout);

    let search_box = gtk::Box::new(Orientation::Horizontal, 0);
    layout.pack_start(&search_box, false, false, 0);

    let grid_width = 270 * 4 + (4 + -1) * 17;
    let spacer = gtk::Box::new(Orientation::Horizontal, 0);
    let width = get_monitor_width() / 2 - (grid_width as i32) / 2;
    spacer.set_size_request(width, 1);

    search_box.pack_start(&spacer, false, false, 0);

    let search_label = Label::new(None);
    search_label.set_markup(config.search_prompt.as_str());
    search_box.pack_start(&search_label, false, false, 0);

    add_class(&search_label, "textview-label");

    let search_input = TextView::new();
    //    search_input.set_name("search");
    search_box.pack_start(&search_input, true, true, 0);

    let buttons = desktop_entries
        .into_iter()
        .map(|entry| entry as Rc<RefCell<dyn GridButton>>)
        .collect::<Vec<_>>();

    let mut app_grid = grid::Grid::new(
        buttons,
        grid::SHOW_ICON | grid::SHOW_LABEL,
        Rc::new(|entry| {
            println!("{:?}", entry.borrow().label());
        }),
    );

    layout.pack_start(&app_grid.window, true, true, 0);

    let css_provider = CssProvider::new();

    if config.enable_pywal == true {
        add_class(&window, "pywal");
    }

    let css = style::get_css();

    if let Ok(_) = css_provider.load_from_data(css.as_ref()) {
        StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing css provider"),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );
    }

    if let Some(display) = window.get_display() {
        let arrow =
            Cursor::new_from_name(&display, "default").expect("Could not create 'default' cursor!");

        let pointer =
            Cursor::new_from_name(&display, "pointer").expect("Could not create 'pointer' cursor!");
    }

    window.set_title("waffy");
    window.show_all();
    //    app_grid.update();

    gtk::main();
}

fn get_monitor_width() -> i32 {
    return 1920;
}

fn add_class<W: gtk::prelude::WidgetExt>(widget: &W, class_name: &str) {
    let ctx = widget.get_style_context();
    ctx.add_class(class_name);
}

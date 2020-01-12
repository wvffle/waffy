use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use gdk::*;
use gtk::{
    Align, BoxExt, ContainerExt, CssProvider, CssProviderExt, Grid, GridExt, GtkWindowExt, Label,
    LabelExt, Orientation, StyleContext, StyleContextExt, TextView, WidgetExt,
};
use gtk_layer_shell_rs::*;

use rust_embed::RustEmbed;
use serde::Deserialize;

fn main() {
    const COL_PADDING: u32 = 17;

    let config_wal = 0;
    let config_columns = 4;
    let config_prompt = "run:";

    open_config();
    let desktop_entries = find_desktop_entries();

    gtk::init().expect("Could not init GTK!");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    init_for_window(&window);
    set_layer(&window, Layer::Top);
    set_anchor(&window, Edge::Top, true);
    set_anchor(&window, Edge::Left, true);
    set_anchor(&window, Edge::Right, true);
    set_anchor(&window, Edge::Bottom, true);

    window.set_resizable(false);
    window.set_decorated(false);

    let layout = gtk::Box::new(Orientation::Vertical, 0);
    window.add(&layout);

    let search_box = gtk::Box::new(Orientation::Horizontal, 0);
    layout.pack_start(&search_box, false, false, 0);

    let grid_width = 270 * config_columns + (config_columns + 2) * COL_PADDING;
    let spacer = gtk::Box::new(Orientation::Horizontal, 0);
    let width = get_monitor_width() - (grid_width as i32) / 2;
    spacer.set_size_request(width, 1);

    search_box.pack_start(&spacer, false, false, 0);

    let search_label = Label::new(None);
    search_label.set_markup(config_prompt);
    search_box.pack_start(&search_label, false, false, 0);

    add_class(&search_label, "textview-label");

    let search_input = TextView::new();
    //    search_input.set_name("search");
    search_box.pack_start(&search_input, true, true, 0);

    let app_grid = Grid::new();
    layout.pack_start(&app_grid, true, true, 0);

    for i in 0..config_columns {
        app_grid.insert_column(i as i32);
    }

    app_grid.set_column_spacing(COL_PADDING);
    app_grid.set_row_spacing(COL_PADDING);
    //    app_grid.set_name("apps");

    app_grid.set_halign(Align::Center);

    update_apps(&desktop_entries);

    let css_provider = CssProvider::new();

    if config_wal == 1 {
        add_class(&window, "pywal");
    }

    let css = get_css();

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

    window.set_title("Waffy");
    window.show_all();

    gtk::main();
}

#[derive(RustEmbed)]
#[folder = "res/"]
struct Resource;

#[derive(Deserialize)]
struct Config {
    columns: u8,
    search_prompt: String,
    enable_pywal: bool,
}

fn get_css() -> String {
    return String::from("window { background: alpha(#000, .7) }");
}

fn update_apps(apps: &Vec<DesktopEntry>) {}

fn get_monitor_width() -> i32 {
    return 1920;
}

#[derive(Debug)]
struct DesktopEntry {
    name: Option<String>,
    icon: Option<String>,
}

impl DesktopEntry {
    pub fn empty() -> Self {
        Self {
            name: None,
            icon: None,
        }
    }

    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = Some(name.into());
    }

    pub fn set_icon<S: Into<String>>(&mut self, icon: S) {
        self.icon = Some(icon.into());
    }
}

fn find_desktop_entries() -> Vec<DesktopEntry> {
    let mut desktop_entry_dirs: Vec<PathBuf> = vec!["/usr/share/applications".into(), "/usr/local/share/applications".into()];

    if let Some(user_dir) = dirs::home_dir() {
        desktop_entry_dirs.push(user_dir.join(".local/share/applications/"));
    }

    let mut desktop_entries: Vec<DesktopEntry> = Vec::with_capacity(10);

    for dir in desktop_entry_dirs {
        if let Ok(read_dir) = fs::read_dir(dir) {
            for entry in read_dir {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let extension = path.extension();

                    match extension {
                        Some(extension) => {
                            if extension != "desktop" {
                                continue;
                            }
                        }
                        None => continue,
                    }

                    if let Ok(desktop_entry) = parse_desktop_file(path) {
                        if let Some(desktop_entry) = desktop_entry {
                            desktop_entries.push(desktop_entry);
                        }
                    }
                }
            }
        }
    }

    desktop_entries
}

fn parse_desktop_file(path: PathBuf) -> io::Result<Option<DesktopEntry>> {
    let content = fs::read_to_string(path)?;

    let mut entry = DesktopEntry::empty();
    let mut entry_section = false;

    for line in content.lines() {
        let mut chars = line.chars();

        if let Some(first_char) = chars.nth(0) {
            if first_char == '\0' || first_char == '#' {
                continue;
            }
        }

        if entry_section {
            if let Some(first_char) = chars.nth(0) {
                if first_char == '[' {
                    break;
                }
            }

            let split = line.splitn(2, "=").map(String::from).collect::<Vec<_>>();

            let key = match split.get(0) {
                Some(key) => key,
                None => continue,
            };
            let value = match split.get(1) {
                Some(value) => value,
                None => continue,
            };

            if key == "NoDisplay" && value == "true" {
                return Ok(None);
            }

            if key == "Name" {
                entry.set_name(value);
                continue;
            }

            if value == "Icon" {
                entry.set_icon(value);
                continue;
            }
        } else if line == "[Desktop Entry]" {
            entry_section = true;
        }
    }

    Ok(Some(entry))
}

fn open_config() -> Option<Config> {
    if let Some(mut config_path) = dirs::config_dir() {
        config_path.push("waffy");
        config_path.push("config");

        if !config_path.exists() {
            let file = Resource::get("default_config.json5").unwrap();
            let content = std::str::from_utf8(file.as_ref()).expect("Cannot read default config");
            let config =
                json5::from_str::<Config>(&content).expect("Cannot parse default config");

            fs::write(config_path, content);
            return Some(config);
        }

        let content = fs::read_to_string(config_path).expect("Could not read config");
        let config = json5::from_str::<Config>(&content).expect("Could not parse config");
        return Some(config);
    }

    None
}

fn add_class<W: gtk::prelude::WidgetExt>(widget: &W, class_name: &str) {
    let ctx = widget.get_style_context();
    ctx.add_class(class_name);
}

use gtk::{Grid as GtkGrid, ScrolledWindow as GtkWindow, Viewport as GtkViewport, Widget as GtkWidget, GridExt, ViewportExt, ScrolledWindowExt, ContainerExt, WidgetExt, Adjustment};
use sublime_fuzzy::{best_match as fuzzy_match, format_simple as fuzzy_format};

use super::Config;

pub trait GridButton {
    fn default_icon(&self) -> String;
    fn label(&self) -> String;
    fn display_label(&self) -> String;
    fn icon(&self) -> Option<String>;
    fn set_display_label(&self, label: String) -> ();
    fn gtk_widget(&self) -> &GtkWidget;
}

pub struct Grid<T> {
    items: Vec<T>,
    filter_string: String,
    flags: u8,
    click_callback: Box<Fn(&T)>,
    window: GtkWindow,
    grid: GtkGrid,
}

impl<T: GridButton> Grid<T> {
    pub fn new (items: Vec<T>, flags: u8, click_callback: Box<Fn(&T)>) -> Self {
        let adjustment = None::<&gtk::Adjustment>;
        let window = GtkWindow::new(adjustment, adjustment);
        let viewport = GtkViewport::new(adjustment, adjustment);
        let grid = GtkGrid::new();

        let config = Config::get();

        // Initialize columns
        for i in 0..(config.columns - 1) {
            grid.insert_column(i as i32);
        }

        window.add(&viewport);
        viewport.add(&grid);

        Self { items, filter_string: String::from(""), flags, click_callback, window, grid }
    }

    pub fn filter (&mut self, needle: String) {
        self.filter_string = needle;
        self.update();
    }

    pub fn update (&self) {
        self.grid.foreach(|child| self.grid.remove(child));

        let config = Config::get();
        let columns = config.columns as i32;

        let mut filtered: Vec<&T> = Vec::new();
        for item in self.items.iter() {
            let label = item.label();
            if let Some(res) = fuzzy_match(self.filter_string.as_str(), label.as_str()) {
                item.set_display_label(fuzzy_format(&res, label.as_str(), "<span>", "</span>"));
                filtered.push(item);
            }
        }

        for (i, item) in filtered.into_iter().enumerate() {
            let col = i as i32 % columns;
            let row = i as i32 / columns;
            self.grid.attach(item.gtk_widget(), col, row, 1, 1);
        }

        self.grid.show_all();
    }
}

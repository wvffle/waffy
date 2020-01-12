use gtk::{
    Grid as GtkGrid, ScrolledWindow as GtkWindow, Viewport as GtkViewport,
    GridExt, ContainerExt, WidgetExt, LabelExt, ButtonExt,
};
use sublime_fuzzy::{
    best_match as fuzzy_match,
    // format_simple as fuzzy_format
};

use super::Config;

pub const SHOW_ICON: u32  = 0b01;
pub const SHOW_LABEL: u32 = 0b10;

pub trait GridButton {
    fn label(&self) -> &String;
    fn display_label(&self) -> gtk::Label;
    fn icon(&self) -> gtk::Image;
    fn set_display_label(&mut self, label: String);
}

pub struct Grid<T> {
    items: Vec<T>,
    filter_string: String,
    flags: u32,
    click_callback: Box<dyn Fn(&T)>,
    pub window: GtkWindow,
    grid: GtkGrid,
}

impl<T: GridButton> Grid<T> {
    pub fn new (items: Vec<T>, flags: u32, click_callback: Box<dyn Fn(&T)>) -> Self {
        let adjustment = None::<&gtk::Adjustment>;
        let window = GtkWindow::new(adjustment, adjustment);
        let viewport = GtkViewport::new(adjustment, adjustment);
        let grid = GtkGrid::new();

        let config = Config::get();

        // Initialize columns
        for i in 0..(config.columns - 1) {
            grid.insert_column(i as i32);
        }

        grid.set_column_spacing(17);
        grid.set_row_spacing(17);
        grid.set_halign(gtk::Align::Center);

        window.add(&viewport);
        viewport.add(&grid);

        Self { items, filter_string: String::from(""), flags, click_callback, window, grid }
    }

    pub fn filter (&mut self, needle: String) {
        self.filter_string = needle;
        self.update();
    }

    pub fn update (&mut self) {
        self.grid.foreach(|child| self.grid.remove(child));

        let config = Config::get();
        let columns = config.columns as i32;

        let mut filtered: Vec<&T> = Vec::new();
        for item in self.items.iter() {
            if self.filter_string == "" {
                filtered.push(item);
                continue;
            }

            let label = item.label();
            if let Some(res) = fuzzy_match(self.filter_string.as_str(), label.as_str()) {
//                item.set_display_label(fuzzy_format(&res, label.as_str(), "<span>", "</span>"));
                filtered.push(item);
            }
        }

        for (i, item) in filtered.into_iter().enumerate() {
            let col = i as i32 % columns;
            let row = i as i32 / columns;

            let widget = gtk::Button::new();
            let content = gtk::Grid::new();

            content.set_column_spacing(17);

            if self.flags & SHOW_ICON > 0 {
                content.insert_column(0);
                content.attach(&item.icon(), 0, 0, 1, 1);
            }

            if self.flags & SHOW_LABEL > 0 {
                let label = item.display_label();
                content.insert_column(1);
                content.attach(&label, 1, 0, 1, 1);

                label.set_max_width_chars(16);
                label.set_ellipsize(pango::EllipsizeMode::End);
            }

            widget.connect_clicked(|_| {
                (self.click_callback)(&item);
            });

            widget.add(&content);
            self.grid.attach(&widget, col, row, 1, 1);
        }

        self.grid.show_all();
    }
}

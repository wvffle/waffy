use std::cell::RefCell;
use std::rc::Rc;

use gtk::{ButtonExt, ContainerExt, Grid as GtkGrid, GridExt, LabelExt, ScrolledWindow as GtkWindow, StyleContextExt, Viewport as GtkViewport, WidgetExt, Label};
use glib::object::Cast;

use sublime_fuzzy::{
    best_match as fuzzy_match,
    format_simple as fuzzy_format,
};

use super::Config;

pub const SHOW_ICON: u32 = 0b01;
pub const SHOW_LABEL: u32 = 0b10;

type GridButtonRc = Rc<RefCell<dyn GridButton>>;
type GridButtonCallback = dyn Fn(GridButtonRc);

pub trait GridButton {
    fn label(&self) -> &String;
    fn display_label(&self) -> gtk::Label;
    fn icon(&self) -> gtk::Image;
}

pub struct Grid {
    buttons: Vec<gtk::Button>,
    items: Vec<GridButtonRc>,
    pub window: GtkWindow,
    pub grid: GtkGrid,
    flags: u32,
}

impl Grid {
    pub fn new(
        items: Vec<GridButtonRc>,
        flags: u32,
        click_callback: Rc<GridButtonCallback>,
    ) -> Self {
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

        let mut buttons: Vec<gtk::Button> = Vec::new();

        let mut i = 0;
        let mut local_items = Vec::new();
        for item in items {
            local_items.push(item.clone());

            let col = i as i32 % config.columns as i32;
            let row = i as i32 / config.columns as i32;
            i += 1;

            let widget = gtk::Button::new();
            let content = gtk::Grid::new();

            content.set_column_spacing(17);

            if flags & SHOW_ICON > 0 {
                content.insert_column(0);
                content.attach(&item.borrow().icon(), 0, 0, 1, 1);
            }

            if flags & SHOW_LABEL > 0 {
                let label = item.borrow().display_label();
                content.insert_column(1);
                content.attach(&label, 1, 0, 1, 1);

                label.set_max_width_chars(16);
                label.set_ellipsize(pango::EllipsizeMode::End);
            }

            let callback = click_callback.clone();

            widget.connect_clicked(move |_| {
                (callback)(item.clone());
            });

            widget.connect_enter_notify_event(move |widget, _| {
                let ctx = widget.get_style_context();
                ctx.add_class("active");
                gtk::Inhibit(true)
            });

            widget.connect_leave_notify_event(move |widget, _| {
                let ctx = widget.get_style_context();
                ctx.remove_class("active");
                gtk::Inhibit(true)
            });

            widget.add(&content);
            grid.attach(&widget, col, row, 1, 1);
            buttons.push(widget);
        }

        Self {
            buttons,
            items: local_items,
            window,
            grid,
            flags
        }
    }

    pub fn filter(&self, needle: String) {
        self.update(needle);
    }

    pub fn update(&self, needle: String) {
        let mut sorted_entries = Vec::with_capacity(self.items.len());
        let columns = Config::get().columns as i32;
        self.grid.foreach(|widget| self.grid.remove(widget));

        for (i, item) in self.items.iter().enumerate() {
            let mut item = item.borrow_mut();
            let label = item.label().clone();
            let button = self.buttons.get(i).unwrap();

            if needle == "" {
                sorted_entries.push(true);
                button.set_label(&label);
                continue;
            }

            if let Some(res) = fuzzy_match(needle.as_str(), label.as_str()) {
                let container = button.get_children().get(0).as_ref().unwrap();
                let grid: gtk::Grid = container.downcast().unwrap();
                for child in grid.get_children() {
                    match child.downcast::<Label>() {
                        Ok(label_widget) => {
                            label_widget.set_markup(fuzzy_format(
                                &res,
                                label.as_str(),
                                "<span class=\"button-highlight\">[[",
                                "]]</span>"
                            ).as_str());
                        },
                        Err(_) => {}
                    }
                }

                sorted_entries.push(true);
                continue;
            }

            sorted_entries.push(false);
        }

        let mut idx = 0;
        for (i, visible) in sorted_entries.into_iter().enumerate() {
            let mut col = i as i32 % columns;
            let mut row = i as i32 / columns;
            let button = self.buttons.get(i).unwrap();

            if visible {
                col = idx as i32 % columns;
                row = idx as i32 / columns;

                self.grid.attach(button, col, row, 1, 1);
                idx += 1;
            }

        }

        self.grid.show();
    }
}

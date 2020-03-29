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

pub trait GridCursor {
    fn cursor_left(&mut self);
    fn cursor_right(&mut self);
    fn cursor_up(&mut self);
    fn cursor_down(&mut self);
    fn cursor_hide(&self);
    fn cursor_show(&self);
    fn cursor_set_pos(&mut self, x: usize, y: usize);
    fn cursor_set_index(&mut self, index: usize);
}

pub struct Grid {
    buttons: Vec<gtk::Button>,
    items: Vec<GridButtonRc>,
    pub window: GtkWindow,
    pub grid: GtkGrid,
    flags: u32,
    cursor: (usize, usize),
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
                label.set_use_markup(true);
            }

            let callback = click_callback.clone();

            widget.connect_clicked(move |_| {
                (callback)(item.clone());
            });

            widget.add(&content);
            grid.attach(&widget, col, row, 1, 1);
            buttons.push(widget);
        }

        let mut res = Self {
            buttons,
            items: local_items,
            window,
            grid,
            flags,
            cursor: (0, 0)
        };

        for (i, widget) in buttons.iter().enumerate() {
            widget.connect_enter_notify_event(move |widget, _| {
                res.cursor_set_index(i);
                gtk::Inhibit(true)
            });

            widget.connect_leave_notify_event(move |widget, _| {
                res.cursor_hide();
                gtk::Inhibit(true)
            });
        }

        res
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
                let name = fuzzy_format(
                    &res,
                    label.as_str(),
                    "[[",
                    "]]"
                );

                // TODO: Label is not setting for some weird reason
                item.display_label().set_label(name.as_str());
                println!("{} {}", item.display_label().get_label().unwrap(), name.as_str());

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

impl GridCursor for Grid {
    fn cursor_left(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor_set_pos(self.cursor.0 - 1, self.cursor.1)
        }
    }

    fn cursor_right(&mut self) {
        self.cursor_set_pos(self.cursor.0 + 1, self.cursor.1)
    }

    fn cursor_up(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor_set_pos(self.cursor.0, self.cursor.1 - 1)
        }
    }

    fn cursor_down(&mut self) {
        self.cursor_set_pos(self.cursor.0, self.cursor.1 + 1)
    }

    fn cursor_hide(&self) {
        let config = Config::get();
        if let Some(widget) = self.buttons.get(self.cursor.1 * config.columns as usize + self.cursor.0) {
            let ctx = widget.get_style_context();
            ctx.remove_class("active");
        }
    }

    fn cursor_show(&self) {
        let config = Config::get();
        if let Some(widget) = self.buttons.get(self.cursor.1 * config.columns as usize + self.cursor.0) {
            let ctx = widget.get_style_context();
            ctx.add_class("active");
        }
    }

    fn cursor_set_pos(&mut self, x: usize, y: usize) {
        let config = Config::get();

        let idx = y * config.columns as usize + x;

        let mut x_next = idx % config.columns as usize;
        let mut y_next = idx / config.columns as usize;

        let y_max = self.buttons.len() - 1;
        if y_next > y_max {
            y_next = y_max;
        }

        if self.cursor.0 != x_next && self.cursor.1 != y_next {
            self.cursor_hide();

            self.cursor.0 = x_next;
            self.cursor.1 = y_next;

            self.cursor_show();
        }
    }

    fn cursor_set_index(&mut self, i: usize) {
        let config = Config::get();

        let x = i % config.columns as usize;
        let y = i / config.columns as usize;

        self.cursor_set_pos(x, y);
    }
}


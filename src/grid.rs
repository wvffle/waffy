use std::cell::RefCell;
use std::rc::Rc;

use gtk::{
    ButtonExt, ContainerExt, Grid as GtkGrid, GridExt, LabelExt, ScrolledWindow as GtkWindow,
    Viewport as GtkViewport, WidgetExt, StyleContextExt
};
use sublime_fuzzy::best_match as fuzzy_match;

use super::Config;

pub const SHOW_ICON: u32 = 0b01;
pub const SHOW_LABEL: u32 = 0b10;

type GridButtonCallback = dyn Fn(Rc<RefCell<dyn GridButton>>);

pub trait GridButton {
    fn label(&self) -> &String;
    fn display_label(&self) -> gtk::Label;
    fn icon(&self) -> gtk::Image;
    fn set_display_label(&mut self, label: String);
}

pub struct Grid {
    items: Vec<gtk::Button>,
    filter_string: String,
    pub window: GtkWindow,
    grid: GtkGrid,
}

impl Grid {
    pub fn new(
        items: Vec<Rc<RefCell<dyn GridButton>>>,
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
        for item in items {
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
            items: buttons,
            filter_string: String::from(""),
            window,
            grid,
        }
    }

    pub fn filter(&mut self, needle: String) {
        self.filter_string = needle;
        self.update();
    }

    pub fn update(&mut self) {
        self.grid.foreach(|child| self.grid.remove(child));
        //
        //        for item in self.items.iter() {
        //            if self.filter_string == "" {
        //                filtered.push(item);
        //                continue;
        //            }
        //
        //            let label = item.label();
        //            if let Some(res) = fuzzy_match(self.filter_string.as_str(), label.as_str()) {
        ////                item.set_display_label(fuzzy_format(&res, label.as_str(), "<span>", "</span>"));
        //                filtered.push(item);
        //            }
        //        }
        //
        self.grid.show_all();
    }
}

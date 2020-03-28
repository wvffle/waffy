use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;
use std::rc::Rc;

use gtk::{
    ButtonExt, ContainerExt, Grid as GtkGrid, GridExt, LabelExt, ScrolledWindow as GtkWindow,
    StyleContextExt, Viewport as GtkViewport, WidgetExt,
};
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
    fn set_display_label(&mut self, label: String);
}

pub struct Grid {
//    buttons: Vec<gtk::Button>,
    items: Vec<GridButtonRc>,
    pub window: GtkWindow,
    pub grid: GtkGrid,
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
//            buttons,
            items: local_items,
            window,
            grid,
        }
    }

    pub fn filter(&self, needle: String) {
        self.update(needle);
    }

    pub fn update(&self, needle: String) {
        let mut sorted_entries = HashMap::with_capacity(self.items.len());

        for (i, item) in self.items.iter().enumerate() {
            let mut item = item.borrow_mut();
            let label = item.label().clone();

            if needle == "" {
                sorted_entries.insert(label, (item, true, i));
                continue;
            }

            if let Some(res) = fuzzy_match(needle.as_str(), label.as_str()) {
                item.set_display_label(fuzzy_format(&res, label.as_str(), "<span>", "</span>"));
                sorted_entries.insert(label, (item, true, i));
                continue;
            }

            sorted_entries.insert(label, (item, false, i));
        }

        // sorted.sort_by_key(|a| a.0.label().clone());
        // sorted.sort_by_key(|a| a.1);
        //
        // let mut items = HashMap::new();
        // for (i, item) in sorted.into_iter().enumerate() {
        //     items.insert(item.0.label().clone(), (item.0, item.1, i));
        // }

        let columns = Config::get().columns as i32;

        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();

        for (i, (key, value)) in sorted_entries.into_iter().enumerate() {
            if visited.get(&i).copied().unwrap_or(false) {
                continue;
            }

            queue.push_back((i, None));
            while !queue.is_empty() {
                let (i, button) = queue.pop_front().unwrap();
                visited.insert(i, true);

                let (_, show, next_idx) = value;

                let col = i as i32 % columns;
                let row = i as i32 / columns;
                let button = &button.unwrap_or(self.grid.get_child_at(col, row).unwrap());

                if !visited.get(&next_idx).unwrap_or(&false) {
                    let col = next_idx as i32 % columns;
                    let row = next_idx as i32 / columns;
                    queue.push_back((next_idx.clone(), Some(self.grid.get_child_at(col, row).unwrap())));
                }

                let col = next_idx as i32 % columns;
                let row = next_idx as i32 / columns;
                self.grid.attach(button, col, row, 1, 1);

                if show { button.show(); } else { button.hide(); }
            }
        }

        self.grid.show();
    }
}

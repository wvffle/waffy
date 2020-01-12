use super::AppButton;
use gtk::{Widget, ScrolledWindow, Viewport, Grid, GridExt, ContainerExt, WidgetExt};

pub struct AppGrid {
    filter: fn(&Vec<AppButton>) -> &Vec<AppButton>,
    buttons: Vec<AppButton>,
    grid: Grid,
    pub element: ScrolledWindow,
}

impl AppGrid {
    pub fn new (buttons: &Vec<AppButton>) -> Self {

        let window = ScrolledWindow::new(None, None);
        let viewport = Viewport::new(None, None);
        let grid = Grid::new();

        // Initialize columns
        let columns = 4; // TODO: Get from config
        for i in 0..(columns - 1) {
            grid.insert_column(i);
        }

        window.add(&viewport);
        viewport.add(&grid);

        Self {
            filter: |buttons| { buttons },
            buttons: buttons.to_vec(),
            grid,
            element: window,
        }
    }

    pub fn add (&mut self, button: &AppButton) {
        self.buttons.push(*button);
    }

    pub fn show_all (&self) {
        let buttons = (self.filter)(&self.buttons);
        let columns = 4; // TODO: Get from config

        self.grid.foreach(|child| {
            self.grid.remove(child);
        });

        for (i, button) in buttons.iter().enumerate() {
            self.grid.attach(button.element, i as i32 % columns, i as i32 / columns, 1, 1);
        }

        self.grid.show_all();
    }
}
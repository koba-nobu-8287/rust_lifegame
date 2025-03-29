/// cell_factory.rs
/// - Factory component for the cell.
use gtk::prelude::*;
use gtk::DrawingArea;
use cairo::Context;
use relm4:: {
     gtk,
     factory::{
        FactoryComponent,
        FactorySender,
        FactoryView,
        Position,
        DynamicIndex,
        positions::GridPosition,
     }
};
use std::rc::Rc;
use std::cell::RefCell;
use crate::model::Cell;

#[derive(Debug, Clone)]
pub struct CellModel {
    drawing_area: Option<DrawingArea>,
    cell_rc: Rc<RefCell<Cell>>,
}

impl Position<GridPosition, DynamicIndex> for CellModel {
    fn position(&self, _index: &DynamicIndex) -> GridPosition {
        let cell_ref = self.cell_rc.clone();
        let cell = cell_ref.borrow();
        let (x, y) = cell.get_position();
        GridPosition {
            column: x,
            row: y,
            width: 1,
            height: 1,
        }
    }
}

#[derive(Debug)]
pub enum CellMsg {
    NextGeneration(bool),
}

fn draw_cell(is_alive: bool, _area: &DrawingArea, cr: &Context, width: i32, height: i32) {
    // println!("alive: {}", is_alive);
    if is_alive {
        cr.set_source_rgb(0.0, 1.0, 0.8);
    } else {
        cr.set_source_rgb(0.5, 0.5, 0.5);
    }
    cr.rectangle(0.0, 0.0, width as f64, height as f64);
    cr.fill().expect("Failed to fill rectangle.");
}    

#[relm4::factory(pub)]
impl FactoryComponent for CellModel {
    type Init = (i32, i32, bool);
    type Input = CellMsg;
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Grid;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 5,
            #[name(drawing_area)]
            DrawingArea {
                set_content_width: 20,
                set_content_height: 20,
                set_hexpand: true,
                set_vexpand: true,
                set_draw_func: {
                    let model = self.cell_rc.clone();
                    move |area, cr, width, height| {
                        let model = model.borrow();
                        draw_cell(model.is_alive(), area, cr, width, height);
                    }
                },
            }
        }
    }

    /// Initialize the cell model with the given parameters.
    fn init_model((x, y, alive): Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        CellModel {
            drawing_area: None,
            cell_rc: Rc::new(RefCell::new(Cell::new(x, y, alive))),
        }
    }

    /// Initialize the widgets for the cell model.
    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>) -> Self::Widgets {
        let widgets = view_output!();
        self.drawing_area = Some(widgets.drawing_area.clone());
        widgets
    }

    /// Update the cell model with the given message.
    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            CellMsg::NextGeneration(alive) => {
                let cell_ref = self.cell_rc.clone();
                let mut cell = cell_ref.borrow_mut();
                cell.set_alive(alive);
                if let Some(drawing_area) = &self.drawing_area {
                    // let (x, y) = self.cell.get_position();
                    // println!("Drawing cell at ({}, {}) with state: {}", x, y, alive);
                    drawing_area.queue_draw();
                }
            }
        }
    }
}
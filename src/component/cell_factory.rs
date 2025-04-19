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
    is_mouse_pressed: bool,
    press_x: f64,
    press_y: f64,
    widget_width: f64,
    widget_height: f64,
    is_event_accept: bool,
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
    MousePressed { x: f64, y: f64 },
    MouseReleased { x: f64, y: f64 },
    MouseMoved { x: f64, y: f64 },
    ClickDetected { start_x: f64, start_y: f64, end_x: f64, end_y: f64 },
    ClickCanceled { start_x: f64, start_y: f64, end_x: f64, end_y: f64 },
    AcceptClick(bool),
}

#[derive(Debug)]
pub enum CellOutputMsg {
    StateChanged { column: i32, row: i32, alive: bool },
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
    type Output = CellOutputMsg;
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
            is_mouse_pressed: false,
            press_x: 0.0,
            press_y: 0.0,
            widget_width: 0.0,
            widget_height: 0.0,
            is_event_accept: true,
        }
    }

    /// Initialize the widgets for the cell model.
    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>) -> Self::Widgets {
        let widgets = view_output!();
        self.drawing_area = Some(widgets.drawing_area.clone());
        if let Some(drawing_area) = &self.drawing_area {
            self.setup_mouse_events(drawing_area, &sender);
        }
        widgets
    }

    /// Update the cell model with the given message.
    fn update(&mut self, msg: Self::Input, sender: FactorySender<Self>) {
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
            },
            CellMsg::MousePressed { x, y } => {
                self.is_mouse_pressed = true;
                self.press_x = x;
                self.press_y = y;
                #[cfg(debug_assertions)]
                println!("Mouse pressed at ({}, {})", x, y);
            },
            CellMsg::MouseReleased { x, y } => {
                if self.is_mouse_pressed {
                    self.is_mouse_pressed = false;
                    if x >= 0.0 && x <= self.widget_width && y >= 0.0 && y <= self.widget_height {
                        sender.input(CellMsg::ClickDetected {
                            start_x: self.press_x,
                            start_y: self.press_y,
                            end_x: x,
                            end_y: y,
                        });
                    } else {
                        sender.input(CellMsg::ClickCanceled {
                            start_x: self.press_x,
                            start_y: self.press_y,
                            end_x: x,
                            end_y: y, });
                    }
                }
                #[cfg(debug_assertions)]
                println!("Mouse released at ({}, {})", x, y);
            },
            CellMsg::MouseMoved { x, y } => {
                if x >= 0.0 && y >= 0.0 {
                    self.widget_width = self.drawing_area.as_ref().unwrap().width() as f64;
                    self.widget_height = self.drawing_area.as_ref().unwrap().height() as f64;
                }
            },
            CellMsg::ClickDetected { start_x, start_y, end_x, end_y } => {
                #[cfg(debug_assertions)]
                println!("Click detected from ({}, {}) to ({}, {})", start_x, start_y, end_x, end_y);
                if self.is_event_accept {
                    let cell_ref = self.cell_rc.clone();
                    let mut cell = cell_ref.borrow_mut();
                    let alive = cell.is_alive();
                    let (x, y) = cell.get_position();
                    cell.set_alive(!alive);
                    if let Some(drawing_area) = &self.drawing_area {
                        drawing_area.queue_draw();
                    }
                    _ = sender.output(CellOutputMsg::StateChanged {
                        column: x,
                        row: y,
                        alive: !alive,
                    });
                }
            },
            CellMsg::ClickCanceled { start_x, start_y, end_x, end_y } => {
                #[cfg(debug_assertions)]
                println!("Click canceled from ({}, {}) to ({}, {})", start_x, start_y, end_x, end_y);
            },
            CellMsg::AcceptClick(accept) => {
                self.is_event_accept = accept;
            }
        }
    }
}

impl CellModel {
    fn setup_mouse_events(&self, drawing_area: &DrawingArea, sender: &FactorySender<Self>) {
        let sender_clone = sender.clone();
        // Create EventController for button press and release
        let click_controller = gtk::GestureClick::new();
        // GestureClick is implments GestureSingle, so if we need to handle secondery button,
        // we can use set_button() method.
        click_controller.connect_pressed(move |_gesture, _n_press, x, y| {
            sender_clone.input(CellMsg::MousePressed { x, y });
        });

        let sender_clone = sender.clone();
        click_controller.connect_released(move |_gesture, _npress,x, y| {
            sender_clone.input(CellMsg::MouseReleased { x, y });
        });

        // Add the click controller to the DrawingArea
        drawing_area.add_controller(click_controller);

        // Create EventController for motion notify
        let sender_clone = sender.clone();
        let motion_controller = gtk::EventControllerMotion::new();
        motion_controller.connect_motion(move |_controller, x, y| {
            sender_clone.input(CellMsg::MouseMoved { x, y });
        });

        // Add the motion controller to the DrawingArea
        drawing_area.add_controller(motion_controller);
    }
}
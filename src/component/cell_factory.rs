/// cell_factory.rs
/// - Factory component for the cell.
use gtk::prelude::*;
use relm4:: {
     gtk,
     factory::{
        FactoryComponent,
        FactorySender,
        Position,
        DynamicIndex,
        positions::GridPosition,
     }
};
use crate::model::Cell;

impl Position<GridPosition, DynamicIndex> for Cell {
    fn position(&self, _index: &DynamicIndex) -> GridPosition {
        let (x, y) = self.get_position();
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

#[relm4::factory(pub)]
impl FactoryComponent for Cell {
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
            #[name(label)]
            gtk::Label {
                #[watch]
                set_label: if self.is_alive() { "O" } else { "." },
                set_hexpand: true,
                set_vexpand: true,
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,
            }
        }
    }
    fn init_model((x, y, alive): Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Cell::new(x, y, alive)
    }
    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            CellMsg::NextGeneration(alive) => {
                self.set_alive(alive);
            }
        }
    }
}
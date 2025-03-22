/// view.rs
/// - View of the life-game.
use gtk::prelude::*;
use relm4::{
    gtk,
    factory::FactoryVecDeque,
    ComponentSender,
    ComponentParts,
    SimpleComponent,
};
use tokio::time::{self, Duration};
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;
use tokio::select;

use crate::model::LifeGame;
use crate::model::Cell;

pub struct ViewModel {
    life_game: LifeGame,
    cell_widgets: FactoryVecDeque<Cell>,
    timer: bool,
    timer_handle: Option<Arc<Notify>>,
}

#[derive(Debug)]
pub enum LifeGameMsg {
    StartStop,
    NextGeneration,
}

#[relm4::component(pub)]
impl SimpleComponent for ViewModel {
    type Init = (usize, usize);
    type Input = LifeGameMsg;
    type Output = ();

    view! {
        #[root]
        gtk::Window {
            set_title: Some("Life Game"),
            set_default_size: (400, 400),
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                #[name(start_stop_button)]
                gtk::Button {
                    #[watch]
                    set_label: if model.timer { "Stop" } else { "Start" },
                    connect_clicked => LifeGameMsg::StartStop,
                },
                #[local_ref]
                game_grid -> gtk::Grid {
                    set_orientation: gtk::Orientation::Vertical,
                    set_column_spacing: 5,
                    set_row_spacing: 5,
                }
            }
        }
    }
    fn init(
        (width, height): Self::Init,
        root: Self::Root,
        _sender: ComponentSender<ViewModel>,
    ) -> ComponentParts<Self> {
        let cells = FactoryVecDeque::<Cell>::builder()
            .launch(gtk::Grid::default())
            .detach();
        let mut model = ViewModel {
            life_game: LifeGame::new(width, height),
            cell_widgets: cells,
            timer: false,
            timer_handle: None,
        };
        for y in 0 .. height {
            for x in 0 .. width {
                model.cell_widgets.guard().push_back((x as i32, y as i32, false));
            }
        }

        let game_grid = model.cell_widgets.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
    fn update(
        &mut self,
        msg: Self::Input,
        sender: ComponentSender<Self>,
    ) {
        match msg {
            LifeGameMsg::StartStop => {
                if !self.timer {
                    self.timer = true;
                    if self.timer_handle.is_none() {
                        let notify = Arc::new(Notify::new());
                        let notify_clone = notify.clone();
                        self.timer_handle = Some(notify);
                        tokio::spawn(async move {
                            let mut interval = time::interval(Duration::from_secs(1));
                            loop {
                                select! {
                                    _ = interval.tick() => {
                                        sender.input(LifeGameMsg::NextGeneration);
                                    }
                                    _ = notify_clone.notified() => {
                                        break;
                                    }
                                }
                            }
                        });
                    }
                } else {
                    self.timer = false;
                    if let Some(handle) = self.timer_handle.take() {
                        handle.notify_one();
                    }
                }
            }
            LifeGameMsg::NextGeneration => {
                self.life_game.next_generation();
            }
        }
    }
}
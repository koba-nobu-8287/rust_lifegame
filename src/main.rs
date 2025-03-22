use relm4::RelmApp;

mod model;
mod view;
mod component;

use view::ViewModel;

fn main() {
    let app = RelmApp::new("koba-nobu-8287.lifegame");
    app.run::<ViewModel>((8, 8));
}

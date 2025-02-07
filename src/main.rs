use eframe::egui::Context;
use eframe::{Frame, Storage};
use serde::{Deserialize, Serialize};
use crate::model::TypeSet;

mod model;

#[derive(Default)]
struct FMMApp {
    type_set: TypeSet,
}

impl FMMApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            type_set: cc.storage
        }
    }
}

impl eframe::App for FMMApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        storage.set_string("typeset", serde_json::to_string(&self.type_set).unwrap());
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        todo!()
    }
}

fn main() {

}

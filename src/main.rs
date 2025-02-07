use eframe::egui::Context;
use eframe::{Frame, Storage};
use egui::panel::TopBottomSide;
use egui::TopBottomPanel;
use crate::model::TypeSet;

pub mod model;

#[derive(Default)]
struct FMMApp {
    type_set: TypeSet,
}

impl FMMApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let type_set = if let Some(storage) = cc.storage {
            if let Some(string) = storage.get_string("typeset") {
                serde_json::from_str::<TypeSet>(string.as_str()).unwrap_or_default()
            } else {
                TypeSet::default()
            }
        } else {
            TypeSet::default()
        };

        Self {
            type_set,
        }
    }
}

impl eframe::App for FMMApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.
        });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        storage.set_string("typeset", serde_json::to_string(&self.type_set).unwrap());
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Factorio Mod Maker", native_options, Box::new(|cc| Ok(Box::new(FMMApp::new(cc)))))
}

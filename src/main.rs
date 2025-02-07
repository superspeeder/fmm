use crate::FMMTab::TypeSetEditor;
use crate::model::TypeSet;
use eframe::egui::Context;
use eframe::{Frame, Storage};
use eframe::epaint::CornerRadius;
use egui::panel::TopBottomSide;
use egui::{Margin, Stroke, TopBottomPanel, ViewportCommand};
use egui_dock::egui::{Ui, WidgetText};
use egui_dock::{DockArea, DockState, Style, TabViewer};
use indexmap::indexmap;

pub mod model;
mod widget;

#[derive(Eq, PartialEq, Clone, Debug)]
enum FMMTab {
    TypeSetEditor {},
    ProjectEditor { name: String },
}

struct FMMApp {
    type_set: TypeSet,
    is_project_open: bool,
    dock_state: DockState<FMMTab>,
}

struct FMMTabViewer;

impl TabViewer for FMMTabViewer {
    type Tab = FMMTab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        match tab {
            FMMTab::TypeSetEditor { .. } => WidgetText::from("TypeSet"),
            FMMTab::ProjectEditor { name, .. } => WidgetText::from(name.as_str()),
        }
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            FMMTab::TypeSetEditor { .. } => {}
            FMMTab::ProjectEditor { name, .. } => {}
        }
    }
}

impl Default for FMMApp {
    fn default() -> Self {
        Self {
            type_set: Default::default(),
            is_project_open: false,
            dock_state: DockState::new(vec![]),
        }
    }
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
            dock_state: DockState::new(vec![TypeSetEditor {}]),
            ..Default::default()
        }
    }
}

impl eframe::App for FMMApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {
                        // TODO: new projects
                    }

                    if ui.button("Open Project").clicked() {
                        // TODO: projects
                    }

                    if ui.button("Open Typeset").clicked() {
                        // TODO: Open Typesets
                    }

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                })
            })
        });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(ctx.style().as_ref())
                    .outer_margin(Margin::ZERO)
                    .inner_margin(Margin::ZERO),
            )
            .show(ctx, |ui| {
                let mut style = Style::from_egui(ui.style().as_ref());
                style.dock_area_padding = None;
                style.main_surface_border_rounding = CornerRadius::ZERO;
                style.main_surface_border_stroke = Stroke::NONE;
                style.tab_bar.corner_radius = CornerRadius::ZERO;
                style.tab.tab_body.corner_radius = CornerRadius::ZERO;
                style.tab.tab_body.stroke = Stroke::NONE;
                style.tab.focused.corner_radius = CornerRadius::ZERO;
                style.tab.active.corner_radius = CornerRadius::ZERO;
                style.tab.hovered.corner_radius = CornerRadius::ZERO;
                style.tab.inactive.corner_radius = CornerRadius::ZERO;
                style.tab.inactive_with_kb_focus.corner_radius = CornerRadius::ZERO;

                DockArea::new(&mut self.dock_state)
                    .style(style)
                    .show_leaf_close_all_buttons(false)
                    .show_leaf_collapse_buttons(false)
                    .show_inside(ui, &mut FMMTabViewer);
            });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        storage.set_string("typeset", serde_json::to_string(&self.type_set).unwrap());
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Factorio Mod Maker",
        native_options,
        Box::new(|cc| Ok(Box::new(FMMApp::new(cc)))),
    )
}

use std::str::FromStr;
use crate::FMMTab::TypeSetEditor;
use crate::model::TypeSet;
use eframe::egui::Context;
use eframe::{Frame, Storage};
use eframe::epaint::CornerRadius;
use egui::panel::TopBottomSide;
use egui::{Color32, Margin, Stroke, TopBottomPanel, ViewportCommand};
use egui_dock::egui::{Ui, WidgetText};
use egui_dock::{DockArea, DockState, Style, TabViewer};
use indexmap::indexmap;

pub mod model;
mod widget;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
enum FMMTab {
    TypeSetEditor {},
    ProjectEditor { name: String },
}

struct FMMApp {
    type_set: TypeSet,
    is_project_open: bool,
    dock_state: DockState<FMMTab>,
    scaling_factor: f32,
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
            scaling_factor: 1.0,
        }
    }
}

impl FMMApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut scaling_factor: f32 = 1.0;

        let type_set = if let Some(storage) = cc.storage {
            if let Some(scale_s) = storage.get_string("scaling_factor") {
                if let Ok(scale) = f32::from_str(scale_s.as_str()) {
                    cc.egui_ctx.set_zoom_factor(scale);
                    scaling_factor = scale;
                }
            }

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
            scaling_factor,
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
                });

                ui.menu_button("View", |ui| {
                    if ui.button("TypeSet Editor").clicked() {
                        let tab = TypeSetEditor {};
                        if let Some((surface_index, node_index, tab_index)) = self.dock_state.find_tab(&tab) {
                            self.dock_state.set_focused_node_and_surface((surface_index, node_index));
                            // self.dock_state.
                        } else {
                            self.dock_state.push_to_focused_leaf(tab);
                        }

                        ui.close_menu();
                    }

                    if ui.add_enabled(self.is_project_open, egui::Button::new("Project Editor")).clicked() {
                        ui.close_menu();
                    }

                    ui.menu_button("Scaling", |ui| {
                        if ui.selectable_label(self.scaling_factor == 1.0,"1x").clicked() {
                            self.scaling_factor = 1.0;
                            ctx.set_zoom_factor(self.scaling_factor);
                            ctx.request_discard("scaling changed");
                        }

                        if ui.selectable_label(self.scaling_factor == 1.25,"1.25x").clicked() {
                            self.scaling_factor = 1.25;
                            ctx.set_zoom_factor(self.scaling_factor);
                            ctx.request_discard("scaling changed");
                        }

                        if ui.selectable_label(self.scaling_factor == 1.5,"1.5x").clicked() {
                            self.scaling_factor = 1.5;
                            ctx.set_zoom_factor(self.scaling_factor);
                            ctx.request_discard("scaling changed");
                        }

                        if ui.selectable_label(self.scaling_factor == 1.75,"1.75x").clicked() {
                            self.scaling_factor = 1.75;
                            ctx.set_zoom_factor(self.scaling_factor);
                            ctx.request_discard("scaling changed");
                        }

                        if ui.selectable_label(self.scaling_factor == 2.0, "2x").clicked() {
                            self.scaling_factor = 2.0;
                            ctx.set_zoom_factor(self.scaling_factor);
                            ctx.request_discard("scaling changed");
                        }

                    })
                });
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
                style.tab_bar.hline_color = Color32::TRANSPARENT;
                style.tab_bar.corner_radius = CornerRadius::ZERO;

                style.tab.tab_body.corner_radius = CornerRadius::ZERO;
                style.tab.tab_body.stroke = Stroke::NONE;

                style.tab.focused.corner_radius = CornerRadius::ZERO;
                style.tab.active.corner_radius = CornerRadius::ZERO;
                style.tab.hovered.corner_radius = CornerRadius::ZERO;
                style.tab.inactive.corner_radius = CornerRadius::ZERO;
                style.tab.inactive_with_kb_focus.corner_radius = CornerRadius::ZERO;

                style.tab.focused.outline_color = Color32::TRANSPARENT;
                style.tab.active.outline_color = Color32::TRANSPARENT;
                style.tab.hovered.outline_color = Color32::TRANSPARENT;
                style.tab.inactive.outline_color = Color32::TRANSPARENT;
                style.tab.inactive_with_kb_focus.outline_color = Color32::TRANSPARENT;

                DockArea::new(&mut self.dock_state)
                    .style(style)
                    .show_leaf_close_all_buttons(false)
                    .show_leaf_collapse_buttons(false)
                    .show_inside(ui, &mut FMMTabViewer);
            });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        storage.set_string("typeset", serde_json::to_string(&self.type_set).unwrap());
        storage.set_string("scaling_factor", self.scaling_factor.to_string());
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

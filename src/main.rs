#![windows_subsystem = "windows"]
use eframe::egui::*;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fs;
use std::ops::DerefMut;
use std::u8;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Designation maker",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Debug, Deserialize, Serialize)]
struct MyApp {
    name: String,
    designation_number: String,
    prefix: Vec<Affix>,
    suffix: Vec<Affix>,
    temp_affix: Affix,
    affix_type: AffixType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Affix {
    name: String,
    pub selected: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum AffixType {
    Prefix,
    Suffix,
}

impl Affix {
    fn new(name: &str) -> Self {
        let name = name.to_owned();
        Self {
            name,
            selected: false,
        }
    }
    fn new_selected(name: &str) -> Self {
        let name = name.to_owned();
        Self {
            name,
            selected: true,
        }
    }
}

const PATH: &str = "text.ron";

impl Default for MyApp {
    fn default() -> Self {
        if fs::metadata(PATH).is_ok() {
            let string = fs::read_to_string(PATH).unwrap();
            let app = ron::from_str::<MyApp>(&string);
            if let Ok(i) = app {
                return i;
            }
        }
        Self {
            name: "name".to_owned(),
            designation_number: String::new(),

            prefix: vec![
                Affix::new_selected("Obedience"),
                Affix::new("Pleasure"),
                Affix::new("Administrative"),
                Affix::new("Conversion"),
                Affix::new("Denial"),
            ],

            suffix: vec![
                Affix::new("Hucow"),
                Affix::new("Toy"),
                Affix::new("Resouce"),
                Affix::new("Slave"),
                Affix::new("Kitty"),
            ],

            temp_affix: Affix::new(""),

            affix_type: AffixType::Prefix,
        }
    }
}

impl MyApp {
    fn remove(&mut self, affix: &Affix) {
        self.prefix = self
            .prefix
            .clone()
            .into_iter()
            .filter(|p| p.name != affix.name)
            .collect();
        self.suffix = self
            .suffix
            .clone()
            .into_iter()
            .filter(|p| p.name != affix.name)
            .collect();
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.designation_number = String::new();
        let mut i = 0;

        for character in self.name.chars() {
            self.designation_number +=
                &((character.to_ascii_lowercase() as u8 - 96) % 10).to_string();
            i += 1;
            if i >= 4 {
                break;
            }
        }
        while self.designation_number.len() < 4 {
            self.designation_number += "0";
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drone designation maker");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });

            ui.label(format!("Number: '{}'", self.designation_number));
            ui.label("Prefixes");
            for prefix in self.prefix.iter_mut() {
                ui.checkbox(&mut prefix.selected, prefix.name.clone());
            }
            ui.label("suffixes");
            for suffix in self.suffix.iter_mut() {
                ui.checkbox(&mut suffix.selected, suffix.name.clone());
            }
            let mut prefix: (String, u8) = (String::new(), 0);
            let mut suffix: (String, u8) = (String::new(), 0);

            for p in self.prefix.iter() {
                if p.selected && prefix.1 < 2 {
                    prefix.0 += &p.name.chars().collect::<Vec<char>>()[0].to_string();
                    prefix.1 += 1;
                }
            }

            for s in self.suffix.iter() {
                if s.selected && suffix.1 < 2 {
                    suffix.0 += &s.name.chars().collect::<Vec<char>>()[0].to_string();
                    suffix.1 += 1;
                }
            }

            let mut result = format!("{}-{}", prefix.0, self.designation_number);

            if suffix.1 != 0 {
                result += &format!("-{}", suffix.0)
            }
            ui.heading(result);
            ui.horizontal(|ui| {
                let add_button = ui.button("Add");
                let popup_id = Id::new("add");

                if add_button.clicked() {
                    ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                    self.temp_affix = Affix::new("")
                }

                popup_below_widget(
                    ui,
                    popup_id,
                    &add_button,
                    PopupCloseBehavior::CloseOnClickOutside,
                    |ui| {
                        ui.set_min_width(150.0);
                        ui.horizontal(|ui| {
                            let name_label = ui.label("Name: ");
                            ui.text_edit_singleline(&mut self.temp_affix.name)
                                .labelled_by(name_label.id)
                        });
                        ui.radio_value(&mut self.affix_type, AffixType::Prefix, "Prefix");
                        ui.radio_value(&mut self.affix_type, AffixType::Suffix, "Suffix");
                        if ui.button("Add").clicked() {
                            match self.affix_type {
                                AffixType::Prefix => self.prefix.push(self.temp_affix.clone()),
                                AffixType::Suffix => self.suffix.push(self.temp_affix.clone()),
                            }
                            ui.close_menu();
                        }
                    },
                );

                let remove_button = ui.button("remove");
                let remove_popup = Id::new("Remove");

                if remove_button.clicked() {
                    ui.memory_mut(|mem| mem.toggle_popup(remove_popup));
                }

                popup_below_widget(
                    ui,
                    remove_popup,
                    &remove_button,
                    PopupCloseBehavior::CloseOnClickOutside,
                    |ui| {
                        ui.set_min_width(100.0);
                        let mut to_be_removed: Option<Affix> = None;
                        for prefix in self.prefix.iter_mut() {
                            if ui.button(prefix.name.clone()).clicked() {
                                to_be_removed = Some(prefix.clone());
                            }
                        }
                        for suffix in self.suffix.iter_mut() {
                            if ui.button(suffix.name.clone()).clicked() {
                                to_be_removed = Some(suffix.clone());
                            }
                        }
                        if let Some(affix) = to_be_removed {
                            self.remove(&affix)
                        }
                    },
                );
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                fs::write(
                    PATH,
                    ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).unwrap(),
                );
            }
        });
    }
}

#![windows_subsystem = "windows"]

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::u8;
 
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 320.0]),
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
    obedience: bool,
    pleasure: bool,
    hucow: bool,
    administrative: bool,
    conversion: bool,
    resouce: bool,
    slave: bool,
    toy: bool,
    denial: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "a".to_owned(),
            designation_number: String::new(),

            obedience: true,
            pleasure: false,
            hucow: false,
            administrative: false,
            conversion: false,
            resouce: false,
            slave: false,
            toy: false,
            denial: false,
        }
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
            //prefix
            ui.checkbox(&mut self.administrative, "Administrative");
            ui.checkbox(&mut self.conversion, "Conversion");
            ui.checkbox(&mut self.denial, "Denial");
            ui.checkbox(&mut self.obedience, "Obedience");
            ui.checkbox(&mut self.pleasure, "Pleasure");
            //suffix
            ui.checkbox(&mut self.hucow, "Hucow");
            ui.checkbox(&mut self.resouce, "Resource");
            ui.checkbox(&mut self.toy, "Toy");
            ui.checkbox(&mut self.slave, "Slave");

            let mut prefix = String::new();
            let mut suffix = String::new();
            let mut prefix_count: u8 = 0;
            if self.administrative && prefix_count < 2 {
                prefix += "A";
                prefix_count += 1;
            }
            if self.conversion && prefix_count < 2 {
                prefix += "C";
                prefix_count += 1;
            }

            if self.pleasure && prefix_count < 2 {
                prefix += "P";
                prefix_count += 1;
            }
            if self.denial && prefix_count < 2 {
                prefix += "D";
                prefix_count += 1;
            }
            if self.obedience && prefix_count < 2 {
                prefix += "O";
            }

            let mut suffix_count: u8 = 0;
            if self.hucow && suffix_count < 2 {
                suffix += "H";
                suffix_count += 1
            }
            if self.resouce && suffix_count < 2 {
                suffix += "R";
                suffix_count += 1;
            }
            if self.toy && suffix_count < 2 {
                suffix += "T";
                suffix_count += 1;
            }
            if self.slave && suffix_count < 2 {
                suffix += "S";
            }
            let mut result = format!("{}-{}", prefix, self.designation_number);
            if suffix.len() > 0 {
                result += "-";
                result += &suffix
            }
            ui.heading(result);
        });
    }
}

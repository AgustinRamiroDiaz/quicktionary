#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use rand::seq::SliceRandom;
use std::{fs::File, path::Path};
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    let file_name = "prompts.json".to_string();

    // let fr = web_sys::FileReaderSync::new().unwrap();
    // fr.read_as_text(&web_sys::Blob::new().unwrap()).unwrap();

    let file_path = Path::new(&file_name);
    let file = File::open(&file_path).unwrap();

    let database = serde_json::from_reader(file).expect("Could not read values.");

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|_cc| Box::<Content>::new(Content::new(database))),
        )
        .await
        .expect("failed to start eframe");
    });
}

#[derive(serde::Deserialize)]
struct Database {
    A: Vec<String>,
    B: Vec<String>,
    C: Vec<String>,
}

impl Database {
    // fn new() -> Database {

    // Database {
    //     pile1: vec!["1".to_string(), "a".to_string(), "b".to_string()],
    //     pile2: vec!["2".to_string(), "a".to_string(), "b".to_string()],
    //     pile3: vec!["3".to_string(), "a".to_string(), "b".to_string()],
    // }
    // }

    fn random(&self) -> [String; 3] {
        let mut rng = rand::thread_rng();
        [
            self.A.choose(&mut rng).unwrap().to_string(),
            self.B.choose(&mut rng).unwrap().to_string(),
            self.C.choose(&mut rng).unwrap().to_string(),
        ]
    }
}

struct Content {
    card1: String,
    card2: String,
    card3: String,
    db: Database,
}

impl Content {
    fn new(db: Database) -> Self {
        let [card1, card2, card3] = db.random();
        Content {
            card1,
            card2,
            card3,
            db,
        }
    }
    fn randomize(&mut self) {
        [self.card1, self.card2, self.card3] = self.db.random();
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Quicktionary!");
            if ui.button("Randomize").clicked() {
                self.randomize();
            }
            ui.columns(3, |col| {
                col[0].label(&self.card1);
                col[1].label(&self.card2);
                col[2].label(&self.card3);
            });
        });
    }
}

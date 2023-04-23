#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use rand::seq::SliceRandom;

#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|_cc| Box::<Content>::default()),
        )
        .await
        .expect("failed to start eframe");
    });
}

struct Database {
    pile1: Vec<String>,
    pile2: Vec<String>,
    pile3: Vec<String>,
}

impl Database {
    fn new() -> Database {
        // TODO: populate database with data from file
        Database {
            pile1: vec!["1".to_string(), "a".to_string(), "b".to_string()],
            pile2: vec!["2".to_string(), "a".to_string(), "b".to_string()],
            pile3: vec!["3".to_string(), "a".to_string(), "b".to_string()],
        }
    }

    fn random(&self) -> [String; 3] {
        let mut rng = rand::thread_rng();
        [
            self.pile1.choose(&mut rng).unwrap().to_string(),
            self.pile2.choose(&mut rng).unwrap().to_string(),
            self.pile3.choose(&mut rng).unwrap().to_string(),
        ]
    }
}

struct Content {
    card1: String,
    card2: String,
    card3: String,
    db: Database,
}

impl Default for Content {
    fn default() -> Self {
        let db = Database::new();
        let [card1, card2, card3] = db.random();
        Content {
            card1,
            card2,
            card3,
            db,
        }
    }
}

impl Content {
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

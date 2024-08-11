// main.rs

use eframe::egui;
mod quantum_simulator;
use quantum_simulator::Qubit;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native("Quantum Computer Simulator", Default::default(), Box::new(|_cc| {
        Box::<dyn eframe::App>::new(MyApp::default())
    }))
}

#[derive(Default)]
struct MyApp {
    qubit: Qubit,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Quantum Computing Simulator");
            
            if ui.button("Initialize Qubit").clicked() {
                self.qubit = Qubit::new();
            }

            if ui.button("Apply Hadamard Gate").clicked() {
                self.qubit.apply_hadamard();
            }

            if ui.button("Apply Pauli-X Gate").clicked() {
                self.qubit.apply_pauli_x();
            }

            if ui.button("Measure Qubit").clicked() {
                let result = self.qubit.measure();
                ui.label(format!("Measurement Result: |{}⟩", result));
            }

            let (x, y, z) = self.qubit.bloch_coordinates();
            ui.label(format!("Bloch Coordinates: ({:.2}, {:.2}, {:.2})", x, y, z));
        });
    }
}

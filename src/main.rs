// main.rs

use eframe::{egui, epi};

mod quantum_simulator; // Assuming you have the previous code in quantum_simulator.rs

use quantum_simulator::Qubit;

struct QuantumSimulatorApp {
    qubit: Qubit,
}

impl Default for QuantumSimulatorApp {
    fn default() -> Self {
        Self {
            qubit: Qubit::new(),
        }
    }
}

impl epi::App for QuantumSimulatorApp {
    fn name(&self) -> &str {
        "Quantum Simulator"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Quantum Simulator");
            
            ui.horizontal(|ui| {
                if ui.button("Apply Hadamard").clicked() {
                    self.qubit.apply_hadamard();
                }
                if ui.button("Apply Pauli-X").clicked() {
                    self.qubit.apply_pauli_x();
                }
                if ui.button("Measure").clicked() {
                    let result = self.qubit.measure();
                    ui.label(format!("Measurement Result: |{}>", result));
                }
            });

            let (x, y, z) = self.qubit.bloch_coordinates();
            ui.label(format!("Bloch Sphere Coordinates: (x: {:.2}, y: {:.2}, z: {:.2})", x, y, z));
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(Box::<QuantumSimulatorApp>::default(), eframe::NativeOptions::default())
}

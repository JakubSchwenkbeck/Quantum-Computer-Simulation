use eframe::{egui, epi};

mod quantum_simulator; 

use quantum_simulator::Qubit;

struct QuantumSimulatorApp {
    qubit: Qubit,
    measurement_result: Option<u32>, // Store the measurement result
}

impl Default for QuantumSimulatorApp {
    fn default() -> Self {
        Self {
            qubit: Qubit::new(),
            measurement_result: None, // Initialize to None
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
            
            // Buttons to apply quantum gates and measure
            ui.horizontal(|ui| {
                if ui.button("Apply Hadamard").clicked() {
                    self.qubit.apply_hadamard();
                    self.measurement_result = Some(self.qubit.measure()); // Store the measurement result
                }
                if ui.button("Apply Pauli-X").clicked() {
                    self.qubit.apply_pauli_x();
                    self.measurement_result = Some(self.qubit.measure()); // Store the measurement result
                }
                if ui.button("Apply Pauli-Z").clicked() {
                    self.qubit.apply_pauli_z(); // New button for Pauli-Z gate
                    self.measurement_result = Some(self.qubit.measure()); // Store the measurement result
                }
                if ui.button("Measure Manually").clicked() {
                    self.measurement_result = Some(self.qubit.measure()); // Store the measurement result
                }
            });

            // Display Bloch Sphere coordinates
            let (x, y, z) = self.qubit.bloch_coordinates();
            ui.label(format!("Bloch Sphere Coordinates: (x: {:.2}, y: {:.2}, z: {:.2})", x, y, z));

            // Draw Bloch Sphere
            draw_bloch_sphere(ui, ctx,x, y);

            // Display probabilities for measuring |0> and |1>
            let (prob_zero, prob_one) = self.qubit.probabilities();
            ui.label(format!("Probability of |0>: {:.2}%", prob_zero * 100.0));
            ui.label(format!("Probability of |1>: {:.2}%", prob_one * 100.0));

            // Display current values of alpha and beta
            ui.label(format!("Current α (alpha): {:.2}", self.qubit.alpha));
            ui.label(format!("Current β (beta): {:.2}", self.qubit.beta));

            // Display the qubit state in Dirac notation
            ui.label(format!("Qubit State: {}", self.qubit.to_string()));

            // Display if the qubit is in a pure state
            ui.label(format!("Is Pure State: {}", self.qubit.is_pure()));

            // Display the measurement result if it exists
            if let Some(result) = self.measurement_result {
                ui.label(format!("Measurement Result: |{}>", result));
            }
        });
    }
}

// Function to draw the Bloch sphere in 2D

fn draw_bloch_sphere(ui: &mut egui::Ui, _ctx: &egui::Context, x: f32, y: f32) {
    // Set sphere properties
    let radius = 100.0; // Radius of the Bloch sphere
    let center = ui.max_rect().center(); // Center of the drawing area

    // Draw the circle representing the Bloch sphere
    ui.painter().circle_filled(center, radius, egui::Color32::from_black_alpha(50));
    ui.painter().circle_stroke(center, radius, egui::Stroke::new(1.0, egui::Color32::from_black_alpha(50)));

    // Draw axes
    let x_axis_end = (center.x + radius, center.y);
    let y_axis_end = (center.x, center.y - radius);
    let z_axis_end = (center.x, center.y + radius);

    ui.painter().line_segment([center, egui::pos2(x_axis_end.0, x_axis_end.1)], egui::Stroke::new(1.0, egui::Color32::from_rgb(250,0,0)));
    ui.painter().line_segment([center, egui::pos2(y_axis_end.0, y_axis_end.1)], egui::Stroke::new(1.0, egui::Color32::from_rgb(0,250,0)));
    ui.painter().line_segment([center, egui::pos2(z_axis_end.0, z_axis_end.1)], egui::Stroke::new(1.0, egui::Color32::from_rgb(0,0,250)));

    // Calculate the position of the qubit on the Bloch sphere
    let qubit_pos = egui::pos2(center.x + (x * radius), center.y - (y * radius)); // Note: Y-axis is inverted

    // Draw the qubit position on the Bloch sphere
    ui.painter().circle_filled(qubit_pos, 5.0, egui::Color32::from_rgb(50,50,50));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(Box::<QuantumSimulatorApp>::default(), eframe::NativeOptions::default())
}

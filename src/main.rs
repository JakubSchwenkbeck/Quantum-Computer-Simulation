use eframe::{egui, epi};
use rand;

#[derive(Debug, Clone)]
pub struct Qubit {
    pub alpha: f32,
    pub beta: f32,
}

impl Qubit {
    pub fn new() -> Self {
        Qubit { alpha: 1.0, beta: 0.0 }
    }

    pub fn apply_hadamard(&mut self) {
        let alpha = self.alpha;
        let beta = self.beta;
        self.alpha = (alpha + beta) / (2.0f32).sqrt();
        self.beta = (alpha - beta) / (2.0f32).sqrt();
        self.normalize();
    }

    pub fn apply_pauli_x(&mut self) {
        std::mem::swap(&mut self.alpha, &mut self.beta);
        self.normalize();
    }

    pub fn apply_pauli_z(&mut self) {
        self.beta = -self.beta;
        self.normalize();
    }

    pub fn apply_pauli_y(&mut self) {
        let alpha = self.alpha;
        self.alpha = -self.beta; // -i * beta
        self.beta = alpha;       // i * alpha
        self.normalize();
    }

    pub fn measure(&self) -> u32 {
        let probability = self.alpha.powi(2);
        if rand::random::<f32>() < probability {
            0 // |0> state
        } else {
            1 // |1> state
        }
    }

    pub fn normalize(&mut self) {
        let norm = (self.alpha.powi(2) + self.beta.powi(2)).sqrt();
        self.alpha /= norm;
        self.beta /= norm;
    }

    pub fn to_string(&self) -> String {
        format!("|ψ> = {:.2}|0> + {:.2}|1>", self.alpha, self.beta)
    }

    pub fn probabilities(&self) -> (f32, f32) {
        (self.alpha.powi(2), self.beta.powi(2))
    }

    pub fn is_pure(&self) -> bool {
        self.alpha == 1.0 || self.beta == 1.0
    }

    pub fn bloch_coordinates(&self) -> (f32, f32, f32) {
        let theta = 2.0 * (self.alpha.atan2(self.beta).cos() + self.beta.atan2(self.alpha).sin());
        let phi = (self.alpha.powi(2) - self.beta.powi(2)).atan2(2.0 * self.alpha * self.beta);
        (theta.cos(), theta.sin() * phi.cos(), theta.sin() * phi.sin())
    }
}

struct QuantumSimulatorApp {
    qubit: Qubit,
    measurement_result: Option<u32>, // Store the measurement result
    circuit: Vec<String>, // Store the sequence of gates
    tutorial_visible: bool, // Control visibility of the tutorial
}

impl Default for QuantumSimulatorApp {
    fn default() -> Self {
        Self {
            qubit: Qubit::new(),
            measurement_result: None,
            circuit: Vec::new(),
            tutorial_visible: false,
        }
    }
}

impl epi::App for QuantumSimulatorApp {
    fn name(&self) -> &str {
        "Quantum Simulator"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        ctx.set_visuals(egui::Visuals::light()); // Use light visuals for a white background

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Quantum Simulator");

            // Tutorial button
            if ui.button("Toggle Tutorial").clicked() {
                self.tutorial_visible = !self.tutorial_visible;
            }

            // Display tutorial if visible
            if self.tutorial_visible {
                ui.group(|ui| {
                    ui.label("Tutorial:");
                    ui.label("1. Use sliders to adjust the qubit state.");
                    ui.label("2. Apply quantum gates to see their effects.");
                    ui.label("3. Measure the qubit to see the result.");
                    ui.label("4. Add gates to the circuit and run them.");
                });
            }

            // Interactive controls for Alpha and Beta
            ui.label("Adjust Qubit State:");
            ui.horizontal(|ui| {
                ui.label("Alpha:");
                ui.add(egui::Slider::new(&mut self.qubit.alpha, -1.0..=1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Beta:");
                ui.add(egui::Slider::new(&mut self.qubit.beta, -1.0..=1.0));
            });

            // Buttons to apply quantum gates and measure
            ui.horizontal(|ui| {
                if ui.button("Apply Hadamard").clicked() {
                    self.qubit.apply_hadamard();
                    self.circuit.push("Hadamard".to_string());
                }
                if ui.button("Apply Pauli-X").clicked() {
                    self.qubit.apply_pauli_x();
                    self.circuit.push("Pauli-X".to_string());
                }
                if ui.button("Apply Pauli-Z").clicked() {
                    self.qubit.apply_pauli_z();
                    self.circuit.push("Pauli-Z".to_string());
                }
                if ui.button("Apply Pauli-Y").clicked() {
                    self.qubit.apply_pauli_y();
                    self.circuit.push("Pauli-Y".to_string());
                }
                if ui.button("Measure Manually").clicked() {
                    self.measurement_result = Some(self.qubit.measure());
                }
            });

            // Display the current quantum circuit
            ui.label("Current Circuit:");
            for gate in &self.circuit {
                ui.label(gate);
            }

            // Display Bloch Sphere coordinates
            let (x, y, z) = self.qubit.bloch_coordinates();
            ui.label(format!("Bloch Sphere Coordinates: (x: {:.2}, y: {:.2}, z: {:.2})", x, y, z));

            // Draw Bloch Sphere
            draw_bloch_sphere(ui, ctx, x, y);

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
    let radius = 100.0; // Radius of the Bloch sphere
    let center = ui.max_rect().center(); // Center of the drawing area

    // Draw the circle representing the Bloch sphere
    ui.painter().circle_filled(center, radius, egui::Color32::from_black_alpha(50));
    ui.painter().circle_stroke(center, radius, egui::Stroke::new(1.0, egui::Color32::from_black_alpha(50)));

    // Draw axes
    ui.painter().line_segment([center, egui::pos2(center.x + radius, center.y)], egui::Stroke::new(1.0, egui::Color32::from_rgb(250, 0, 0))); // X-axis
    ui.painter().line_segment([center, egui::pos2(center.x, center.y - radius)], egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 250, 0))); // Y-axis
    ui.painter().line_segment([center, egui::pos2(center.x, center.y + radius)], egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 0, 250))); // Z-axis

    // Calculate the position of the qubit on the Bloch sphere
    let qubit_pos = egui::pos2(center.x + (x * radius), center.y - (y * radius)); // Note: Y-axis is inverted

    // Draw the qubit position on the Bloch sphere
    ui.painter().circle_filled(qubit_pos, 5.0, egui::Color32::from_rgb(255, 255, 255));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(Box::<QuantumSimulatorApp>::default(), eframe::NativeOptions::default())
}


use eframe::{egui, epi};

mod quantum_simulator;

use quantum_simulator::Qubit;

struct QuantumSimulatorApp {
    qubits: Vec<Qubit>, // Multiple qubits
    measurement_results: Vec<Option<u32>>, // Store the measurement results for each qubit
    circuit: Vec<String>, // Store the sequence of gates
    tutorial_visible: bool, // Control visibility of the tutorial
    selected_qubit: usize, // Currently selected qubit to manipulate
}

impl Default for QuantumSimulatorApp {
    fn default() -> Self {
        Self {
            qubits: vec![Qubit::new()],
            measurement_results: vec![None],
            circuit: Vec::new(),
            tutorial_visible: false,
            selected_qubit: 0,
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

            // Select the qubit to manipulate
            ui.label("Select Qubit:");
            for i in 0..self.qubits.len() {
                if ui.button(format!("Qubit {}", i)).clicked() {
                    self.selected_qubit = i;
                }
            }

            // Interactive controls for Alpha and Beta of the selected qubit
            ui.label(format!("Adjust Qubit {} State:", self.selected_qubit));
            ui.horizontal(|ui| {
                ui.label("Alpha:");
                ui.add(egui::Slider::new(&mut self.qubits[self.selected_qubit].alpha, -1.0..=1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Beta:");
                ui.add(egui::Slider::new(&mut self.qubits[self.selected_qubit].beta, -1.0..=1.0));
            });

            // Buttons to apply quantum gates to the selected qubit
            ui.horizontal(|ui| {
                if ui.button("Apply Hadamard").clicked() {
                    self.qubits[self.selected_qubit].apply_hadamard();
                    self.circuit.push(format!("Qubit {}: Hadamard", self.selected_qubit));
                }
                if ui.button("Apply Pauli-X").clicked() {
                    self.qubits[self.selected_qubit].apply_pauli_x();
                    self.circuit.push(format!("Qubit {}: Pauli-X", self.selected_qubit));
                }
                if ui.button("Apply Pauli-Z").clicked() {
                    self.qubits[self.selected_qubit].apply_pauli_z();
                    self.circuit.push(format!("Qubit {}: Pauli-Z", self.selected_qubit));
                }
                if ui.button("Apply Pauli-Y").clicked() {
                    self.qubits[self.selected_qubit].apply_pauli_y();
                    self.circuit.push(format!("Qubit {}: Pauli-Y", self.selected_qubit));
                }
            });

            // Predefined circuits
            ui.label("Predefined Circuits:");
            if ui.button("Bell State Circuit").clicked() {
                self.circuit.clear(); // Clear previous circuit
                self.circuit.push("Qubit 0: Hadamard".to_string());
                self.circuit.push("Qubit 1: Pauli-X".to_string());
            }
            if ui.button("Quantum Fourier Transform (QFT)").clicked() {
                self.circuit.clear(); // Clear previous circuit
                self.circuit.push("Qubit 0: Hadamard".to_string());
                self.circuit.push("Qubit 1: Controlled Phase Shift".to_string());
            }

            // Add a qubit to the circuit
            if ui.button("Add Qubit").clicked() {
                self.qubits.push(Qubit::new());
                self.measurement_results.push(None);
            }

            // Clear the circuit
            if ui.button("Clear Circuit").clicked() {
                self.circuit.clear();
            }

            // Run the current circuit
            if ui.button("Run Circuit").clicked() {
                self.run_circuit();
            }

            // Display the current quantum circuit
            ui.label("Current Circuit:");
            for gate in &self.circuit {
                ui.label(gate);
            }
            ui.label("");

            // Display Bloch Sphere coordinates of the selected qubit
            let (x, y, z) = self.qubits[self.selected_qubit].bloch_coordinates();
            ui.label(format!(
                "Bloch Sphere Coordinates (Qubit {}): (x: {:.2}, y: {:.2}, z: {:.2})",
                self.selected_qubit, x, y, z
            ));

            // Draw Bloch Sphere
            draw_bloch_sphere(ui, ctx, x, y);

            // Display probabilities for measuring |0> and |1> for the selected qubit
            let (prob_zero, prob_one) = self.qubits[self.selected_qubit].probabilities();
            ui.label(format!("Probability of |0>: {:.2}%", prob_zero * 100.0));
            ui.label(format!("Probability of |1>: {:.2}%", prob_one * 100.0));

            // Display current values of alpha and beta of the selected qubit
            ui.label(format!("Current α (alpha): {:.2}", self.qubits[self.selected_qubit].alpha));
            ui.label(format!("Current β (beta): {:.2}", self.qubits[self.selected_qubit].beta));

            // Display the qubit state in Dirac notation
            ui.label(format!("Qubit State: {}", self.qubits[self.selected_qubit].to_string()));

            // Display if the qubit is in a pure state
            ui.label(format!("Is Pure State: {}", self.qubits[self.selected_qubit].is_pure()));

            // Display the measurement results for all qubits if they exist
            for (i, result) in self.measurement_results.iter().enumerate() {
                if let Some(result) = result {
                    ui.label(format!("Measurement Result (Qubit {}): |{}>", i, result));
                }
            }
        });
    }
}

impl QuantumSimulatorApp {
    // Function to run the current circuit
    fn run_circuit(&mut self) {
        for gate in &self.circuit {
            let parts: Vec<&str> = gate.split(": ").collect();
            let qubit_index = parts[0].replace("Qubit ", "").parse::<usize>().unwrap();
            let gate_name = parts[1];

            match gate_name {
                "Hadamard" => self.qubits[qubit_index].apply_hadamard(),
                "Pauli-X" => self.qubits[qubit_index].apply_pauli_x(),
                "Pauli-Z" => self.qubits[qubit_index].apply_pauli_z(),
                "Pauli-Y" => self.qubits[qubit_index].apply_pauli_y(),
                "Controlled Phase Shift" => self.qubits[qubit_index].apply_controlled_phase_shift(),
                _ => (),
            }
        }

        // Measure the result for each qubit
        for i in 0..self.qubits.len() {
            self.measurement_results[i] = Some(self.qubits[i].measure());
        }
    }
}

// Function to draw the Bloch sphere in 2D
fn draw_bloch_sphere(ui: &mut egui::Ui, _ctx: &egui::Context, x: f32, y: f32) {
    let radius = 100.0; // Radius of the Bloch sphere
    let mut center = ui.max_rect().center(); // Center of the drawing area

    // Shift the center to the right
    center.x += 75.0; // Adjust this value to move the sphere further right
    center.y += 25.0; // Adjust this value to move the sphere further right

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

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

            // Display all qubits with different colors
            ui.label("Qubits:");
            for (i, qubit) in self.qubits.iter().enumerate() {
                let color = egui::Color32::from_rgb(
                    100 + (i as u8 * 30) % 155,
                    100 + (i as u8 * 50) % 155,
                    250 - (i as u8 * 30) % 155,
                );

                // Button to select the qubit
                if ui.button(format!("Select Qubit {}", i)).clicked() {
                    self.selected_qubit = i;
                }

                // Display Bloch Sphere for each qubit
                let (x, y, z) = qubit.bloch_coordinates();
                ui.label(format!("Qubit {}:", i));
                ui.label(format!("Bloch Sphere Coordinates: (x: {:.2}, y: {:.2}, z: {:.2})", x, y, z));
                draw_bloch_sphere(ui, ctx, x, y, color);
                ui.add_space(20.0);
            }

            // Display and control the selected qubit
            ui.label(format!("Selected Qubit {}:", self.selected_qubit));
            let selected_qubit = &mut self.qubits[self.selected_qubit];
            let color = egui::Color32::from_rgb(
                100 + (self.selected_qubit as u8 * 30) % 155,
                100 + (self.selected_qubit as u8 * 50) % 155,
                250 - (self.selected_qubit as u8 * 30) % 155,
            );

            // Interactive controls for Alpha and Beta of the selected qubit
            ui.horizontal(|ui| {
                ui.label("Alpha:");
                ui.add(egui::Slider::new(&mut selected_qubit.alpha, -1.0..=1.0).text("α").text_color(color));
            });
            ui.horizontal(|ui| {
                ui.label("Beta:");
                ui.add(egui::Slider::new(&mut selected_qubit.beta, -1.0..=1.0).text("β").text_color(color));
            });

            // Buttons to apply quantum gates to the selected qubit
            ui.horizontal(|ui| {
                if ui.button("Apply Hadamard").clicked() {
                    selected_qubit.apply_hadamard();
                    self.circuit.push(format!("Qubit {}: Hadamard", self.selected_qubit));
                }
                if ui.button("Apply Pauli-X").clicked() {
                    selected_qubit.apply_pauli_x();
                    self.circuit.push(format!("Qubit {}: Pauli-X", self.selected_qubit));
                }
                if ui.button("Apply Pauli-Z").clicked() {
                    selected_qubit.apply_pauli_z();
                    self.circuit.push(format!("Qubit {}: Pauli-Z", self.selected_qubit));
                }
                if ui.button("Apply Pauli-Y").clicked() {
                    selected_qubit.apply_pauli_y();
                    self.circuit.push(format!("Qubit {}: Pauli-Y", self.selected_qubit));
                }
                if ui.button("Controlled Phase Shift").clicked() {
                    selected_qubit.apply_controlled_phase_shift();
                    self.circuit.push(format!("Qubit {}: Controlled Phase Shift", self.selected_qubit));
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
            if ui.button("Clear Qubits").clicked() {
                self.qubits.clear();
                self.qubits.push(Qubit::new());

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

// Function to draw the Bloch sphere in 2D with color
fn draw_bloch_sphere(ui: &mut egui::Ui, _ctx: &egui::Context, x: f32, y: f32, color: egui::Color32) {
    let radius = ui.max_rect().height()/(3 as f32); // Radius of the Bloch sphere
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

    // Draw the qubit position on the Bloch sphere with color
    ui.painter().circle_filled(qubit_pos, 5.0, color);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(Box::<QuantumSimulatorApp>::default(), eframe::NativeOptions::default())
}

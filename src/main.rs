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
        ctx.set_visuals(egui::Visuals::light()); // Use light visuals for a white background

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Quantum Simulator");
            
            // Buttons to apply quantum gates and measure
            ui.horizontal(|ui| {
                if ui.button("Apply Hadamard").clicked() {
                    self.qubit.apply_hadamard();
                 //   self.measurement_result = Some(self.qubit.measure()); // Store the measurement result
                }
                if ui.button("Apply Pauli-X").clicked() {
                    self.qubit.apply_pauli_x();
                   // self.measurement_result = Some(self.qubit.measure()); // Store the measurement result
                }
                if ui.button("Apply Pauli-Z").clicked() {
                    self.qubit.apply_pauli_z(); // New button for Pauli-Z gate
                  //  self.measurement_result = Some(self.qubit.measure()); // Store the measurement result
                }
                if ui.button("Apply Pauli-Y").clicked() {
                    self.qubit.apply_pauli_y(); // New button for Pauli-Z gate
                  //  self.measurement_result = Some(self.qubit.measure()); // Store the measurement result
                }
                if ui.button("Measure Manually").clicked() {
                    self.measurement_result = Some(self.qubit.measure()); // Store the measurement result
                }
            });

            // Display Bloch Sphere coordinates
            let (x, y, z) = self.qubit.bloch_coordinates();
            ui.label(format!("Bloch Sphere Coordinates: (x: {:.2}, y: {:.2}, z: {:.2})", x, y, z));

            // Draw Bloch Sphere
           // Define rotation parameters for the pseudo-3D effect
        // You might want to control these via UI sliders or another input mechanism
        let rotation_x = 0.5; // Example rotation angle around X-axis
        let rotation_y = 0.5; // Example rotation angle around Y-axis

        // Draw Bloch Sphere
        draw_bloch_sphere(ui, ctx, x, y);//, z, (rotation_x, rotation_y));
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
fn draw_bloch_sphere_3d(ui: &mut egui::Ui, _ctx: &egui::Context, x: f32, y: f32, z: f32, rotation: (f32, f32)) {
    let radius = 100.0;
    let center = ui.max_rect().center();

    // Rotation matrix for the X axis
    let (rot_x, rot_y) = rotation;
    let rotated_x = x * rot_y.cos() + z * rot_y.sin();
    let rotated_y = y;
    let rotated_z = -x * rot_y.sin() + z * rot_y.cos();

    // Convert 3D coordinates to 2D
    let projected_x = rotated_x * rot_x.cos() - rotated_y * rot_x.sin();
    let projected_y = rotated_x * rot_x.sin() + rotated_y * rot_x.cos();

    // Calculate the position of the qubit on the pseudo-3D Bloch sphere
    let qubit_pos = egui::pos2(center.x + (projected_x * radius), center.y - (projected_y * radius));

    // Draw the circle representing the Bloch sphere
    ui.painter().circle_filled(center, radius, egui::Color32::from_black_alpha(50));
    ui.painter().circle_stroke(center, radius, egui::Stroke::new(1.0, egui::Color32::from_black_alpha(50)));

    // Draw axes
    draw_axis(ui, center, radius, 1.0, egui::Color32::from_rgb(250, 0, 0), (rot_x, rot_y)); // X-axis
    draw_axis(ui, center, radius, 0.0, egui::Color32::from_rgb(0, 250, 0), (rot_x, rot_y)); // Y-axis
    draw_axis(ui, center, radius, 2.0, egui::Color32::from_rgb(0, 0, 250), (rot_x, rot_y)); // Z-axis

    // Draw the qubit position on the Bloch sphere
    ui.painter().circle_filled(qubit_pos, 5.0, egui::Color32::from_rgb(255, 255, 255));
}

// Function to draw a 3D axis
fn draw_axis(ui: &mut egui::Ui, center: egui::Pos2, radius: f32, axis_type: f32, color: egui::Color32, rotation: (f32, f32)) {
    let (rot_x, rot_y) = rotation;
    let length = radius * 1.5; // Length of the axis lines

    let (start, end) = match axis_type {
        0.0 => (center, egui::pos2(center.x, center.y - length)), // Y-axis
        1.0 => (center, egui::pos2(center.x + length, center.y)), // X-axis
        2.0 => (center, egui::pos2(center.x, center.y + length)), // Z-axis
        _ => (center, center),
    };

    // Apply rotation to the axis end points
    let rotated_end = egui::pos2(
        (end.x - center.x) * rot_x.cos() - (end.y - center.y) * rot_x.sin() + center.x,
        (end.x - center.x) * rot_x.sin() + (end.y - center.y) * rot_x.cos() + center.y
    );

    // Draw the axis line
    ui.painter().line_segment([center, rotated_end], egui::Stroke::new(1.0, color));
}


fn draw_bloch_sphere(ui: &mut egui::Ui, _ctx: &egui::Context, x: f32, y: f32) {
    // Set sphere properties
    let radius = 100.0; // Radius of the Bloch sphere
    let center = ui.max_rect().center(); // Center of the drawing area

    // Draw the circle representing the Bloch sphere
    ui.painter().circle_filled(center, radius, egui::Color32::from_black_alpha(50));
    ui.painter().circle_stroke(center, radius, egui::Stroke::new(1.0, egui::Color32::from_black_alpha(50)));
    // Position for the label
    let label_pos = egui::pos2(center.x, center.y - radius - 20.0); // Positioned above the Bloch sphere

    // Draw the label "Bloch sphere:"
    ui.painter().text(
        label_pos, 
        egui::Align2::CENTER_CENTER, 
        "Bloch sphere:", 
        egui::TextStyle::Heading, 
        egui::Color32::WHITE
    );
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
    ui.painter().circle_filled(qubit_pos, 5.0, egui::Color32::from_rgb(255,255,255));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(Box::<QuantumSimulatorApp>::default(), eframe::NativeOptions::default())
}

// quantum_simulator.rs

use std::f32::consts::PI;

// Define a struct to represent a qubit.
#[derive(Debug, Clone)]
pub struct Qubit {
    // Coefficients for the |0> and |1> states.
    pub alpha: f32,
    pub beta: f32,
}

impl Qubit {
    // Create a new qubit in the |0> state by default.
    pub fn new() -> Self {
        Qubit { alpha: 1.0, beta: 0.0 }
    }

    // Apply the Hadamard gate to the qubit.
    pub fn apply_hadamard(&mut self) {
        let alpha = self.alpha;
        let beta = self.beta;

        // The Hadamard gate transforms the qubit to a superposition state.
        self.alpha = (alpha + beta) / (2.0f32).sqrt();
        self.beta = (alpha - beta) / (2.0f32).sqrt();
    }

    // Apply a Pauli-X (NOT) gate to the qubit.
    pub fn apply_pauli_x(&mut self) {
        // Flips the state of the qubit.
        std::mem::swap(&mut self.alpha, &mut self.beta);
    }

    // Measure the qubit and collapse it to |0> or |1> based on probabilities.
    pub fn measure(&self) -> u32 {
        let probability = self.alpha.powi(2);
        // Randomly collapse the qubit state.
        if rand::random::<f32>() < probability {
            0 // |0> state
        } else {
            1 // |1> state
        }
    }

    // Get the Bloch sphere representation of the qubit state.
    pub fn bloch_coordinates(&self) -> (f32, f32, f32) {
        let theta = 2.0 * (self.alpha.atan2(self.beta).cos() + self.beta.atan2(self.alpha).sin());
        let phi = (self.alpha.powi(2) - self.beta.powi(2)).atan2(2.0 * self.alpha * self.beta);
        (theta.cos(), theta.sin() * phi.cos(), theta.sin() * phi.sin())
    }
}

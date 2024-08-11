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

    pub fn new_with_state(alpha: f32, beta: f32) -> Self {
        let mut qubit = Qubit { alpha, beta };
        qubit.normalize();
        qubit
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

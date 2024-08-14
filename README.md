
# **Quantum Simulator in Rust**
This project is a simple quantum computer simulator implemented in Rust. It allows users to explore basic quantum operations on qubits, such as applying **Hadamard**, **Pauli-X**, and **Pauli-Z** gates, and measuring **qubit** states. The application features a graphical interface built with eframe and egui, providing a visual representation of the qubit's state on the Bloch sphere.

**Features:**

- Qubit Management: Create and manipulate multiple qubits, selecting any qubit to adjust its state.
- Quantum Gates: Apply essential gates:

  - Hadamard: Creates superposition.
  - Pauli-X, Y, Z: Flip and phase operations.
  - Controlled Phase Shift: Entangle qubits.
  - Measurement: Measure qubit states with probabilistic outcomes for |0⟩ and |1⟩.

- Bloch Sphere Visualization: Visualize qubit states on the Bloch sphere, showing coordinates and measurement probabilities.

- Density Matrix and Histogram visualization

- Well known Alogrithms such as Grovers Search and Quantum Teleportation

- Dynamic Circuit Simulation: Create, modify, and run quantum circuits, tracking the sequence of applied gates.

- Interactive Sliders: Adjust qubit parameters (α and β) with sliders for real-time exploration.

- Tutorial: Access a built-in tutorial for easy onboarding to quantum concepts.

![image](https://github.com/user-attachments/assets/bef51d59-9df9-48f5-8ec9-2dba03421dac)


**Prerequisites:**
- Rust programming language
- Cargo package manager
  
**Getting Started:**
- Clone the repository.
- Navigate to the project directory.
- Run the application using cargo run.
Explore the fascinating world of quantum computing through this interactive simulator!

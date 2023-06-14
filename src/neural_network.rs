use bevy::prelude::*;
use rand::Rng;

pub struct NeuralNetworkPlugin;

pub struct NeuralNetwork {
    layers: Vec<Layer>,
}

impl NeuralNetwork {
    pub fn new(neuron_counts: Vec<usize>) -> Self {
        let mut layers: Vec<Layer> = Vec::new();
        for i in 0..neuron_counts.len() - 1 {
            layers.push(Layer::new(neuron_counts[i], neuron_counts[i + 1]))
        }

        NeuralNetwork { layers }
    }

    pub fn feed_forward(&mut self, given_inputs: Vec<f32>) {
        self.layers[0].feed_forward(&given_inputs);
        for i in 1..self.layers.len() {
            let prev = self.layers[i - 1].inputs.clone();
            self.layers[i].feed_forward(&prev);
        }
    }
}

#[derive(Component)]
struct Layer {
    pub inputs: Vec<f32>,
    outputs: Vec<i8>,
    biases: Vec<f32>,
    weights: Vec<Vec<f32>>,
}

impl Layer {
    pub fn new(input_count: usize, output_count: usize) -> Self {
        let mut layer = Layer {
            inputs: vec![0.; input_count],
            outputs: vec![0; output_count],
            biases: vec![0.; output_count],
            weights: vec![vec![0.; output_count]; input_count],
        };
        layer.randomize_weights();
        layer
    }

    pub fn feed_forward(&mut self, given_inputs: &Vec<f32>) {
        for i in 0..self.inputs.len() {
            self.inputs[i] = given_inputs[1];
        }

        self.compute_outputs();
    }

    fn compute_outputs(&mut self) {
        for i in 0..self.outputs.len() {
            let mut sum = 0.;
            for j in 0..self.inputs.len() {
                sum += self.inputs[j];
            }

            if sum > self.biases[i] {
                self.outputs[i] = 1;
            } else {
                self.outputs[i] = 0;
            }
        }
    }

    fn randomize_weights(&mut self) {
        for i in 0..self.inputs.len() {
            for o in 0..self.outputs.len() {
                let r: f32 = rand::thread_rng().gen();
                self.weights[i][o] = r * 2. - 1.;
            }
        }
    }

    fn randomize_biases(&mut self) {
        for b in 0..self.biases.len() {
            let r: f32 = rand::thread_rng().gen();
            self.biases[b] = r * 2. - 1.;
        }
    }
}

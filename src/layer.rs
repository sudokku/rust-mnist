use crate::{matrix::Matrix, activation::ActivationFunction};

#[derive(Debug)]
pub struct Layer{
    size: usize,
    pub neurons: Matrix,
    pub weights: Matrix,
    pub biases: Matrix,
    pub activator: ActivationFunction
}

impl Layer {
    pub fn new(size: usize, input_size: usize) -> Layer {
        let neurons = Matrix::random(1, size);
        let weights = Matrix::random(input_size, size);
        let biases = Matrix::random(1, size);

        Layer {
            size,
            neurons,
            weights,
            biases,
            activator: ActivationFunction::Sigmoid
        }
    }

    pub fn feed_from(&mut self, input: Vec<f64>) -> Vec<f64> {
        let mut temp = Matrix::from_data(vec![input]) * self.weights.clone() + self.biases.clone();
        temp = temp.map(|x| self.activator.function(x));
        self.neurons = temp.clone();
        temp.data[0].clone()
    }

    pub fn back_propagate(&mut self, next_layer_delta: Matrix, inputs: &Matrix, learning_rate: f64){
        let gradient = self.neurons.map(|x| self.activator.derivative(x));
        let layer_delta = next_layer_delta * self.weights.transpose() * gradient;

        
        let weights_adjustment = inputs.transpose() * layer_delta.clone();
        let biases_adjustment = layer_delta;

        self.weights = self.weights.clone() - weights_adjustment.map(|x| x * learning_rate);
        self.biases = self.biases.clone() - biases_adjustment.map(|x| x * learning_rate);
    }
}
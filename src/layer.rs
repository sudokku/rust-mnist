use crate::{matrix::Matrix, activation::{ ActivationFunction}};

#[derive(Debug)]
pub struct Layer{
    size: usize,
    pub neurons: Matrix,
    weights: Matrix,
    biases: Matrix,
    activator: ActivationFunction
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

    pub fn feed_from(&self, input: Vec<f64>) -> Vec<f64> {
        let mut temp = Matrix::from_data(vec![input]) * self.weights.clone() + self.biases.clone();
        // temp.map(self.activator. );

        // TODO: Update current Layer's neurons and return it as resulting data 


        vec![]
    }

    // pub fn feed_from(self, other: &Layer) -> Layer {
    //     let neurons = other.neurons * self.weights + self.biases;
    // }
}
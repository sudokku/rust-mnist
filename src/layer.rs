use crate::{matrix::Matrix, activation::{SIGMOID, ActivationFunction}};

pub struct Layer<'a>{
    size: usize,
    neurons: Matrix,
    weights: Matrix,
    biases: Matrix,
    activator: ActivationFunction<'a>
}

impl Layer<'_> {
    pub fn new<'a>(size: usize, input_size: usize) -> Layer<'a> {
        let neurons = Matrix::random(1, size);
        let weights = Matrix::random(input_size, size);
        let biases = Matrix::random(1, size);

        Layer {
            size,
            neurons,
            weights,
            biases,
            activator: SIGMOID
        }
    }

    pub fn feed_from(&self, input: Vec<f64>) -> Vec<f64> {
        let mut res = Matrix::from_data(vec![input]) * self.weights.clone() + self.biases.clone();

        // TODO: Update current Layer's neurons and return it as resulting data 


        vec![]
    }

    // pub fn feed_from(self, other: &Layer) -> Layer {
    //     let neurons = other.neurons * self.weights + self.biases;
    // }
}
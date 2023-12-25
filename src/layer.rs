use crate::{matrix::Matrix, activation::ActivationFunction};

#[derive(Debug)]
pub struct Layer{
    size: usize,
    neurons: Matrix,
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

    pub fn feed_from(&mut self, input: Vec<f64>) -> Vec<f64> {
        let mut temp = Matrix::from_data(vec![input]) * self.weights.clone() + self.biases.clone();
        temp = temp.map(|x| self.activator.function(x));
        self.neurons = temp.clone();
        temp.data[0].clone()
    }

    pub fn back_propagate(&mut self){
        
    }
}
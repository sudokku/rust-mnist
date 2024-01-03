use layer::Layer;
use matrix::Matrix;

 
mod matrix;
mod layer;
mod activation;

fn main() {
    // Initialize the network
    let mut network: Vec<Layer> = vec![
        Layer::new(2, 2),  // Hidden layer with 2 neurons, input size is 2
        Layer::new(1, 2)   // Output layer with 1 neuron, input size is 2
    ];

    // Inputs and expected outputs for XOR
    let inputs = vec![
        vec![1.0, 0.0],
        vec![1.0, 1.0],
        vec![0.0, 1.0],
        vec![0.0, 0.0]
    ];
    let expected_outputs = vec![
        vec![1.0],  // Expected output for [1.0, 0.0]
        vec![0.0],  // Expected output for [1.0, 1.0]
        vec![1.0],  // Expected output for [0.0, 1.0]
        vec![0.0]   // Expected output for [0.0, 0.0]
    ];

    // Training parameters
    let learning_rate = 0.1;
    let epochs = 10000;

    // Training loop
    for _ in 0..epochs {
        train_network(&mut network, inputs.clone(), expected_outputs.clone(), learning_rate);
    }

    // Testing the network
    for input in &inputs {
        let mut layer_input = input.clone();
        for layer in &mut network {
            layer_input = layer.feed_from(layer_input);
        }
        println!("Input: {:?}, Output: {:?}", input, layer_input);
    }
}

fn train_network(
    network: &mut Vec<Layer>, 
    inputs: Vec<Vec<f64>>, 
    expected_outputs: Vec<Vec<f64>>, 
    learning_rate: f64
) {
    for (input, expected) in inputs.iter().zip(expected_outputs.iter()) {
        // Forward pass
        let mut layer_input = input.clone();
        for layer in network.iter_mut() {
            layer_input = layer.feed_from(layer_input);
        }

        // Calculate the error for the output layer (assuming the last layer is the output layer)
        // This requires a suitable loss function derivative. For simplicity, we'll use mean squared error.
        let mut error = layer_input
            .iter()
            .zip(expected)
            .map(|(a, b)| a - b)
            .collect::<Vec<f64>>();
        let mut delta = Matrix::from_data(vec![error]);

        // Backward pass through the network
        for i in (0..network.len()).rev() {
            let input_for_layer = if i == 0 {
                Matrix::from_data(vec![input.clone()])
            } else {
                network[i - 1].neurons.clone()
            };

            network[i].back_propagate(delta.clone(), &input_for_layer, learning_rate);

            // Update delta for the next layer (previous in terms of backpropagation)
            if i > 0 {
                delta = compute_delta_for_previous_layer(&network[i], &delta);
            }
        }
    }
}

fn compute_delta_for_previous_layer(layer: &Layer, next_layer_delta: &Matrix) -> Matrix {
    // Compute the gradient of the activation function for the layer's output (neurons)
    let gradient = layer.neurons.map(|x| layer.activator.derivative(x));

    // Calculate delta for this layer
    println!("{:?}", next_layer_delta);
    println!("{:?}", layer.weights);
    println!("{:?}", gradient);

    next_layer_delta.clone() * layer.weights.transpose() * gradient
}

/*

NN
[
    Layer(n) -> Input
    Layer(m)
    ...
    Layer(k) -> Output
]

*/
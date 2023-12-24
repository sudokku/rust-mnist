use layer::Layer;

 
mod matrix;
mod layer;
mod activation;

fn main() {
    let hidden_layer: Layer = Layer::new(3, 2);
    let output_layer: Layer = Layer::new(1, 3);

    let input = vec![
        vec![1.0, 0.0],
        vec![1.0, 1.0],
        vec![0.0, 1.0],
        vec![0.0, 0.0]
    ];

    let answers = vec![1.0, 0.0, 1.0, 0.0];

    print!("{:#?}", hidden_layer);
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
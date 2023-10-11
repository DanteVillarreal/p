pub mod network{


    pub struct NetworkLayer {
        rows: usize,
        columns: usize,
        data: Vec<Vec<f64>>,
    }
    
    pub struct WeightLayer {
        rows: usize,
        columns: usize,
        data: Vec<Vec<f64>>,
    }
    
    pub struct BiasLayer {
        rows: usize,
        columns: usize,
        data: Vec<Vec<f64>>,
    }
    
    pub struct NeuralNetwork {
        layers: Vec<NetworkLayer>,
        weights: Vec<WeightLayer>,
        biases: Vec<BiasLayer>,
    }
    

    impl NeuralNetwork {
        pub fn feed_forward(&mut self) {
            
        }
    }

}




    







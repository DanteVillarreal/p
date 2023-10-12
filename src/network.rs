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
    
    impl NetworkLayer {
        pub fn print_network_layer(&mut self) {
            for i in 0..self.columns {
                println!("{:?}\n", self.data[i]);
            }
        }
    }

    impl WeightLayer {
        pub fn print_weight_layer(&mut self) {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    println!("{:?} ", self.data[i]);
                }
                println!("\n");
            }
        }
    }

    impl BiasLayer {
        pub fn print_bias_layer(&mut self) {
            for i in 0..self.columns {
                println!("{:?}\n", self.data[i]);
            }
        }
    }

    


    pub fn matrix_multiply(layer: Vec<Vec<f64>>, weights: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        
        //cant do layer.colummns because layer is not of type NetworkLayer (i think)

        let layer_columns = layer[0].len();     //basically, we just want the length of the 
                                                //  the first row because this tells us how 
                                                //  many columns there are.
                                                //  vec![1.0, 2.0, 3.0] means 3 rows
        let weights_rows = weights.len();       //we are looking for number of vectors in 
                                                //  the weights array.
                                                //  say weights is vec![
                                                //    vec![0.0, 1.0],   //row 0
                                                //    vec![2.0, 3.0],   //row 1
                                                //    vec![4.0, 5.0],   //row 2
                                                //    ];
                                                //  the len of weights is 3

        if layer_columns != weights_rows {
            panic!("columns {}, of network layer does not match the number of rows {}
                    in weight layer", layer_columns, weights_rows);
        }


		//vec![what_i_want_each_element_to_be ; number_of_elements];
		//the above creates a vector the number of elements being number_of_elements
		//all initialized to  what_i_want_each_element_to_be
		//so if I did this:
		//vec![  vec![what_i_want_each_element_to_be;number_of_elements] ; number];
		//	this would generate number number of vectors all initialized to 
		//	vec![what_i_want_each_element_to_be;number_of_elements]
		let sum = vec![vec![0.0; weights[0].len()] ; layer.len()];


		//NEED TO EXPLAIN THIS FIRST. GO BACK TO EDGE TO LEARN HOW THIS WORKS
		for i in 0..layer.len() {
			for j in 0..weights[0].len() {
				for k in 0..layer_columns {
					sum[i][j] += layer[i][k] * weights[k][j];
				}
			}
		}
	
		sum
    }



    impl NeuralNetwork {
        pub fn feed_forward(&mut self) {
            for i in 1..self.layers.len() {
                
                let previous_activations = &self.layers[i-1].data;
                let weights = &self.weights[i-1].data;
                let biases = &self.biases[i-1].data;

                self.layers[i].data = matrix_multiply(previous_activations, weights);
                self.layers[i].data = matrix_add(self.layers[i].data, biases);
                self.layers[i].data = apply_activation_function(self.layers[i].data);
                

            }
        }
    }

}




    







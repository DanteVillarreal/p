pub mod network{
	use rand::Rng;

	//STANDARD INITIALIZATION OF PARTS OF NEURAL NETWORK
	pub struct NetworkLayer {
		rows: usize,
		columns: usize,
		data: Vec<Vec<f64>>,        //need Vec<Vec   because we want the same format as WeightLayer   (i think)
	}
	
	pub struct WeightLayer {
		rows: usize,
		columns: usize,
		data: Vec<Vec<f64>>,
	}
	
	pub struct BiasLayer {
		rows: usize,
		columns: usize,
		data: Vec<Vec<f64>>,        //same as up top^^
	}
	
	pub struct NeuralNetwork {
		layers: Vec<NetworkLayer>,
		weights: Vec<WeightLayer>,
		biases: Vec<BiasLayer>,
	}
	
	impl NetworkLayer {
		pub fn print_network_layer( &self) {
			for i in 0..self.columns {
				println!("{:?}\n", &self.data[i]);
			}
		}
	}

	impl WeightLayer {
		pub fn print_weight_layer(&self) {
			for i in 0..self.rows {
				for j in 0..self.columns {
					println!("{:?} ", &self.data[i][j]);
				}
				println!("\n");
			}
		}
	}

	impl BiasLayer {
		pub fn print_bias_layer( &self) {
			for i in 0..self.columns {
				println!("{:?}\n", &self.data[i]);
			}
		}
	}

	
							  //  i JUST care about its value. 
							  //  |               no need to take ownership (i think)
	pub fn matrix_multiply(layer: &Vec<Vec<f64>>, weights: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
		
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
			//NEED TO DO *MUT* because i'm changing sum later.
			let mut sum = vec![vec![0.0; weights[0].len()] ; layer.len()];


			//computes the actual matrix multiplication
			//if confused. draw it out and do the calculations and you'll see it works
			for i in 0..layer.len() {
				for j in 0..weights[0].len() {
					for k in 0..layer_columns {
						sum[i][j] += layer[i][k] * weights[k][j];
					}
				}
			}
			sum //this is the value returned. aka the matrix returned
			
	}

		//IT SUCKS but i need to clone the layer right now. I'll optimize it after 1st prototype is done.
	pub fn matrix_add (layer: &Vec<Vec<f64>>, biases: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
		let mut resulting_matrix = layer.clone();
		for i in 0..layer.len() {
			for j in 0..layer[0].len() {
				resulting_matrix[i][j] += biases[0][j];
			}
		}
		resulting_matrix   //creates a matrix of 1 row
	}


	//this is our activation function. leaky_relu is better than relu because
	//  no vanishing gradient
	//and even relu is better than tanh and sigmoid. so only option is leaky_relu
	pub fn leaky_relu(x: f64) -> f64 {
		if x>= 0.0 {
			x
		}
		else {
			0.01*x
		}
	}

	//no return necessary because i'm passing in the "matrix" by mut reference
	//  so any changes I make will be refleted in original "matrix" i pass in
	//we are doing &mut because we want to be able to modify it w/o taking ownership
	//  if we take ownership, then the origianl variable will not be able to be used
	//  again after this function is done
	pub fn apply_activation_function(matrix: &mut Vec<Vec<f64>>) {
		for i in 0..matrix.len() {
			for j in 0..matrix[i].len() {
				matrix[i][j] = leaky_relu(matrix[i][j]);
			}
		}
	}

		//this determines if neural network will take random action or if it will take
		//	what it thinks is the best action.
		//a return of false means perform exploration
		//a return of true means perform exploitation
		//remember to check if epsilon is initialized in main and it is initialized to 1. 
	pub fn epsilon_greedy(epsilon: &mut f64) -> bool {
		let is_epsilon_bigger: bool;
		let p: f64 = rand::thread_rng().gen_range(0.0..=1.0);

		// *epsilon is used instead of just epsilon because in order to change epsilon
		//		i have to say: dereference epsilon to get the f64 value it points to
		//		and then subtract .0001 from it
		// just epsilon would be like saying, go to this address in memory, and 
		//		subtract 0.0001 from it. NANI?? ARE YOU SURE YOU WANT TO DO THAT
		*epsilon -= 0.0001;

		if p >= *epsilon {
			is_epsilon_bigger = false; //--> so explore
			is_epsilon_bigger	//return the bool
		}
		else {
			is_epsilon_bigger = true; //--> so exploit
			is_epsilon_bigger	//return the bool
		}
	}

	impl NeuralNetwork {
		
		pub fn feed_forward(&mut self) {
			for i in 1..self.layers.len() {

				//i REALLY NEED to understand this part more. I need to know what's being multiplied
				//		and when and what's being added and when.
				
				let previous_activations = &self.layers[i-1].data;
				let weights = &self.weights[i-1].data;
				let biases = &self.biases[i-1].data;

				self.layers[i].data = matrix_multiply(previous_activations, weights);
				self.layers[i].data = matrix_add(&self.layers[i].data, biases);

				//this fn below changes the layer itself. so it doesn't need to equal anything.
				//apply activation fn to all layers except for the output layer
				if i != self.layers.len()-1 {
				apply_activation_function(&mut self.layers[i].data);
				}
			}
		}

		pub fn exploration_or_exploitation(&mut self, epsilon: &mut f64) {
			let i = self.layers.len();	//i want to get to the output layer

			//then want to see if epsilon greedy returns true or not. I have to call it on epsilon
			let exploit_or_explore: bool = epsilon_greedy(epsilon);

			//if exploit, run tuple for-loop through output layer. 
			//establishes values to work with for-loop
			let mut index_of_largest_qvalue;
			let mut largest_qvalue_so_far = f64::MIN;

			if exploit_or_explore == true {
				//then choose top q value. this would then call another function that
				//		executes the task

				//run for loop for last ouput layer. I want the index and the number
				//		associated with it for each iteration
				//
				for (index, &qvalue_of_neuron) in self.layers[i-1].data.iter().enumerate() {
					if qvalue_of_neuron > largest_qvalue_so_far {
						largest_qvalue_so_far = qvalue_of_neuron;
						index_of_largest_qvalue = index;
					}
				}

				//i now have the index of the largest q value. I must then choose the
				//		next action based on which index it was. 
				//do I do a binary search to match the index to the action?
				//		if so, how would I do it.


				//
			}
			else {
				//choose one of the outputs randomly. the specific output would then
				//		call another function to execute said task
			}
		}
	}

}




	







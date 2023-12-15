pub mod network{
	use rand::Rng;
	use crate::action_functions::{/*insert all the action functions */};
	use rand_distr::{Normal, Distribution};

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
													//  vec![1.0, 2.0, 3.0] means 3 columns
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
	//  so any changes I make will be refleted in original "matrix" i pass in.
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
		//a return of false means perform exploration.
		//a return of true means perform exploitation.
		//remember to check if epsilon is initialized in main and it is initialized to 1. 
	pub fn epsilon_greedy(epsilon: &mut f64) -> bool {
		let is_epsilon_bigger: bool;
		let p: f64 = rand::thread_rng().gen_range(0.0..=1.0);

		// *epsilon is used instead of just epsilon because in order to change epsilon
		//		i have to say: dereference epsilon to get the f64 value it points to
		//		and then subtract .0001 from it.
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

	
	pub fn reward_function() -> f64 {
		//let new_balance = 
		//I need to figure out where I would get the balance from. Do I make an entire function just to return a balance
		//or can I return two f64 from 1 function


/*HOW I WILL STRUCTURE THIS FUNCTION*&*&*&*&(*(*(&*&*------------------------------------:
		this function will get information from the REST APIs of giver and recipient, 
			how_much_i_spent 	will equal how much I spent buying crypto from giver
			balance				will equal how much recipient wallet was at before crypto transfer
			new_balance			will equal how much recipient wallet was at after+  crypto transfer
			change				will equal new_balance minus balance
			updated_balance		will equal balance * (1.0 + change);
			then return updated_balance.ln()


			why 1.0 + change?	so that if change was .05, multiplying it by balance
				would mean losing money. I'm trying to find the gain here, so it would be 
				balance*1.05.
			why ln?				to account for greater loss.
					absolute value of  ln(1-x) is greater than ln(1+x). this is good so our
					DQN will weigh losses as heavier than "equivalent" gain
 */

		//the reason I made it type    Option<f64> is because I don't want to prematurely
        //    assign a value to it, so if somehow it never gets assigned a value,
        //    I can then handle the situation.
        let how_much_i_spent: Option<f64>;
        match how_much_i_spent {
            Some(_) => (),
            None => panic!("how_much_i_spent   is none"),
        }
        let how_much_2nd_wallet_changed: Option<f64>;           //same thing as above^^
        match how_much_2nd_wallet_changed {
            Some(_) => (),
            None => panic!("how_much_2nd_wallet_changed    is none"),
        }

        //can't directly subtract Option types. So I need to handle the possiblity that
        //how_much_i_spent    and/or    how_much_2nd_wallet_changed    could carry no value
        //adds another layer of redundancy, which is always good
        let total_gained = match(how_much_i_spent, how_much_2nd_wallet_changed) {
            (Some(spent), Some(gained)) => Some(gained-spent),
            _ => None,
        }.expect("No value found");



//-----------------MY ACTUAL REWARD FUNCITON------------------------------------------//
		//12/14/23: Im thinking of making this as my reward function but Im not sure what 
		//	the best course of action is.
		//nah, you know what, FUCK ITTTTTT. Im using this for now, and if "oh no, im losing
		//	money in sandbox mode", then Ill change it later

		//self needs to be changed to the porfolio's worth
		//need to REMOVE the parameters, and just input function calls into this function itself 
		//	that get the total portfolio size and 
		fn reward(&self, previous_value: f64) -> f64 {
			let multiplier = 1.3;
			let absolute_change = self.value - previous_value;
			let relative_change = absolute_change / previous_value;
			let reward = if absolute_change > 0.0 {
				absolute_change
			} else {
			//why 1 / (1-relative_change)?
			//so imagine I lost 1 dollars from 100 originally. So the relative change would be 0.01
			//		if I did just 1/relative chnage, I would have 1/.01 = 100
			//now imagine if I lost 10 dollars from 100. Relative change would be 0.1
			//		if I did just 1/relative change, I would have 1/.1 = 10.
			//so basically I'm having less penalty for a worse loss. This is why I'm doing 
			//		1 - relative change. Now let's do the same scenarios but with 1-relative_change
			//1 lost from 100. relative_change = 0.01. 1-relative_change = .99.
			//		 1/(1-relative_change)= 1.010101...
			//10 lost from 100. relative_change = 0.1. 1-relative_change = .9.
			//		 1/(1-relative_change) = 1.11.
			//so a larger weight for a larger loss.
			//why 1 / (1-relative_change)*absolute)_change?
			//The idea with that is so that the same loss is now weighed heavier than the same gain.
			//AND it's weight is scaled in accordance with how much you lost percentage wise.
			//But if I lost 10 percent of my entire portfolio, then I would want it to be worse than
			//		just weighted 1.11 times more
			//so why dont we multiply it by like 1.3 or some constant 
				-1.0 *multiplier* (1.0 / (1.0 - relative_change.abs())) * absolute_change.abs()
			};
			return reward;
		}







	}
	

	impl NeuralNetwork {
		
		pub fn feed_forward(&mut self) {
			for i in 1..self.layers.len() {

				//i REALLY NEED to understand this part more. I need to know what's being
				//	 multiplied and when and what's being added and when.
				
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



		//this will just return the index of the largest_q_value if exploit, or just a random
		//	 index if explore
		//12/15/23 update:
		//I want it to also return the actual q value so that we can use it to update our
		//		 "current Q-value estimate" in the "temporal difference error"
		pub fn exploration_or_exploitation(&self, epsilon: &mut f64) -> usize {
			
			// want to see if epsilon greedy returns true or not so that I explore or exploit
			let exploit_or_explore: bool = epsilon_greedy(epsilon);

			//True = exploit
			//establishes values to work with for-loop
			let mut index_of_largest_qvalue: Option<usize> = None;
			let mut largest_qvalue_so_far = f64::MIN;

			let mut indexx: usize = 0;		//this will function as the index in the for loop below

			if exploit_or_explore == true {
				//Below: I will choose the neuron with the top q value.
				//		this would then call another function that executes the specific task



				//let Some(last_layer) = self.layers.last()		means set last_layer 
				//		equal to self.layers.last() aka the last layer aka the output layer.
				//The reason we have the	"if"	is because we want to handle errors
				//		like if somehow there weren't any layers in the first place
				//		Then it would go to the corresponding else block and tell us the error
				//The more error checking the better
				if let Some(last_layer) = self.layers.last() {
					for value in &last_layer.data[0] {
						if value > &largest_qvalue_so_far {
							largest_qvalue_so_far = *value;		//just to document that we hit a new max
							index_of_largest_qvalue = Some(indexx);	//to know where the new max was
							indexx += 1;						//to iterate the index value

						}
						else {		//this block executes only if the value isn't bigger
									//		than the largest qvalue we have so far.
									//Because:	we dont care about storing the index 
									//		of a smaller q value,
									//		and instead we jsut want to show we visited 
									//		another value, then We do this by 
									//		just incrementing the index
							indexx+=1;		
						}
					}
				}
				else {
					panic!("last_layer.data is empty. this is in fn exploration_or_exploration when exploit_or_explore == true");
				}


				//this deals wtih returning the index_of_largest_qvalue value
				//basically "match" is saying "let's look at the value of 
				//		index_of_largest_qvalue, and do different things depending on what it is"
				//the 	Some(index) => index,	means if index_of_largest_qvalue contains
				//		 a usize value, (usize is the type of variable indexes are)
				//		then return the value that index_of_largest_qvalue holds.
				//the	None => panic!("index_of_largest_qvalue was never initialized"), 
				//		means: if there is no value inside index_of_largest_qvalue, then
				//		quit the program and display the following message.
				//We need to do a match because the variable
				//		index_of_largest_qvalue is of type Option<usize>
				return match index_of_largest_qvalue {
					Some(index) => index,
					None => panic!("index_of_largest_qvalue was never initialized"),
				}

				//in this point in the code, i now have the index of the largest q value.
				//This value is now returned.
				//In the next function or module, I must then choose the function
				//		that corresponds to said q value.
				//I will do that in another funciton. I might even make an entire 
				//		module just for that function
				
			}
			else {
				//choose one of the outputs randomly. the specific output would then
				//		call another function to execute said task

				//attaches a random value between 0 and the last neuron to index_of_random_qvalue
				//		because we want to return a random "neuron" because we're doing
				//		explore. explore means do some random shit, so we can then document
				//		if it was good or not
				let mut index_of_random_qvalue :Option<usize> = None;
				index_of_random_qvalue = Some(rand::thread_rng().gen_range(0..=indexx));

				return match index_of_random_qvalue {
					Some(index) => index,
					None => panic!("index_of_random_qvalue was never initialized"),
				}
			}
			
		}





		pub fn initialization(&mut self, input_size: usize, output_size: usize, number_of_hidden_layers: usize) {
			/*intiialization of weights and biases and what not */
			/*initialization rule I'm following:
        		The number of hidden neurons should be 2/3 the size of the input layer, 
					plus the size of the output layer.
			*/
			/*it will make the hidden layers each the same size.
			  NEED TO ADD ANOTHER PARAMETER FOR IF I WANT PYRAMID, REVERSE PYRAMID, OR NORMAL 
			*/

			/*hidden_size is usize because i cant have a fraction of a neuron, nor a negative size.*/
			let hidden_size = (2.0 / 3.0 * (input_size + output_size) as f64) as usize;
			/*this creates the random number generator */
    		let mut rng = rand::thread_rng();

			// Input layer
			/*
			The .push() is acutally creating a new NetworkLayer with the properties:
					rows: 1,
					columns: input_size,
					data: vec![vec![0.0; input_size]],
				and then appending the layer it creates, to the end of the .layers it already has
			 */
			self.layers.push(NetworkLayer {
				rows: 1,
				columns: input_size,
				data: vec![vec![0.0; input_size]],
			});

			//---------------------Hidden layers----------------------//

			//--first hidden layer--//
				//for loop removed because I'm only making one layer 
				
				/*pushhing NetworkLayer first because each layer needs to be initialized
					before establish weights and baises
				*/
				self.layers.push(NetworkLayer {
					rows: 1,
					columns: hidden_size,
					data: vec![vec![0.0; hidden_size]],
				});

				/*this creates the StandardNormal distribution itself */
				let normal_distr = Normal::new(0.0, 1.0).unwrap();


				/*
				(0..hidden_size).map(|_| {...}).collect()	this is creating a new Vec
						with hidden_size # of elements.
						For each element it applies this function: 
						normal_distr.sample(&mut rng) * (2.0 / (hidden_size as f64)).sqrt()
						.collect()	is returning these results into the new vector
						|_|			means we aren't using the values currently there, 
									if there are any
				why no .iter()? because the range itself:	(0..hidden_size) works as the iterator.
				why 2 layers of (0..hidden_size).map?		the inside layer creates each inside
																vec![1, 2, 3, ...]	.
										each iteration of the outer (0..hidden_size).map creates
											the outer vec![ ] that all the tiny vec![] of each
											hidden layer are in
				*/
				let weights: Vec<Vec<f64>> = (0..hidden_size).map(|_| {
					(0..hidden_size)
					.map(|_| normal_distr.sample(&mut rng) * (2.0 / (hidden_size as f64))
																.sqrt())
																.collect()
				}).collect();

				self.weights.push(WeightLayer {
					rows: input_size,
					columns: hidden_size,
					data: weights,
				});

				self.biases.push(BiasLayer {
					rows: 1,
					columns: hidden_size,
					data: vec![vec![0.01; hidden_size]],
				});
			

			//----rest of hidden layers ---//
				/*only difference is:
				self.weights.push(WeightLayer {
					rows: hidden_size,
					columns: hidden_size,
					data: weights,
				});
					instead of rows: input_size above
				*/

				//starting at 1 because we already established the first hidden layer
				for _ in 1..number_of_hidden_layers {
					self.layers.push(NetworkLayer {
						rows: 1,
						columns: hidden_size,
						data: vec![vec![0.0; hidden_size]],
					});
		
					let normal_distr = Normal::new(0.0, 1.0).unwrap();
					let weights: Vec<Vec<f64>> = (0..hidden_size).map(|_| {
						(0..hidden_size).map(|_| normal_distr.sample(&mut rng) * (2.0 / (hidden_size as f64))
																					.sqrt())
																					.collect()
					}).collect();

					//each layer contains this amount. 
					self.weights.push(WeightLayer {
						rows: hidden_size,
						columns: hidden_size,
						data: weights,
					});
		
					self.biases.push(BiasLayer {
						rows: 1,
						columns: hidden_size,
						data: vec![vec![0.01; hidden_size]],
					});
				}

			// Output layer
				//no for loop because just doing 1 layer
				self.layers.push(NetworkLayer {
					rows: 1,
					columns: output_size,		//only difference
					data: vec![vec![0.0; output_size]],
				});

				let normal_distr = Normal::new(0.0, 1.0).unwrap();
				let weights: Vec<Vec<f64>> = (0..hidden_size).map(|_| {
					(0..output_size).map(|_| normal_distr.sample(&mut rng) * (2.0 / (hidden_size as f64))
																				.sqrt())
																				.collect()
				}).collect();
				self.weights.push(WeightLayer {
					rows: hidden_size,
					columns: output_size,
					data: weights,
				});

				self.biases.push(BiasLayer {
					rows: 1,
					columns: output_size,
					data: vec![vec![0.01; output_size]],
				});
    	}










		pub fn update_input(&mut self, indices: &[usize], new_values: &[f64]) {
			// Check that indices and new_values have the same length
			//EXPLAIN THIS LATER
			assert_eq!(indices.len(), new_values.len());
	
			// Update the specified inputs
			//EXPLAIN THIS LATER
			for (index, &new_value) in indices.iter().zip(new_values) {
				self.layers[0].data[0][*index] = new_value;
			}
		}









		//prints out bias and weight layers
		//I DONT THINK THIS WORKS. IT PRODUCED AN ERROR WHEN I PUT IT IN TO RUST PLAYGROUND
		pub fn print_network(&self) {
			for i in 0..self.layers.len() {
				println!("Layer {}:", i+1);
				for j in 0..self.biases[i].data[0].len() {
					println!("Node {}: {:.2}", j+1, self.biases[i].data[0][j]);
				}
		
				if i < self.weights.len() {
					println!("Weights to next layer:");
					for row in &self.weights[i].data {
						let weights: Vec<String> = row.iter().map(|&x| format!("{:.2}", x))
													.collect();
						println!("{}", weights.join("\t"));
					}
				}
			}
		}


















	}
}





	







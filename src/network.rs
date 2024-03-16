
	use rand::Rng;															//to generate random numbers
	//use crate::action_functions::{/*insert all the action functions */};
	use rand_distr::{Normal, Distribution};									//to generate different dist. of random numbers
	use serde::{Serialize, Deserialize};									//to save/load my neural network
	use std::fs::File;														//to access files. for NeuralNet
	use std::io::{BufReader, BufWriter, Write};									//to save NeuralNet
	use chrono::Utc;														//for timestamp
	use std::path::Path;													//for help in representing file paths
	//use std::sync::Mutex;													//not async compat.
	//use rand::prelude::SliceRandom;												//for exp replay
	use	crate::action_functions;											//for action_functions
	use std::error::Error;													//for action_functions
	use std::fs;														//for replay_buffer
	//use tokio::sync::Mutex;                             // Use async Mutex from Tokio
	//use std::sync::Arc;  // Use Arc to share Mutex among multiple tasks
	//use std::collections::BinaryHeap;					//02/14/24 - added: to help delete replay buffers
	//use std::time::SystemTime;							//02/14/24 - added: to help delete replay buffers

	//STANDARD INITIALIZATION OF PARTS OF NEURAL NETWORK
	
	//why #[derive(Serialize, Deserialize)] ?
	//		this is an attribute in Rust that is used to automatically generate the ncessary
	//		 code to convert a data structure to and from a serialized format
	//why do we want a serialized format?
	//		So we can save the neural network and then load it
	//what is a serialized format?
	//		a format that allows for serialization
	//what is serialization?
	//		the process of converting data to a series of bytes. So below,
	//		 the structs are in terms of data I'm making called rows, 
	//		columns and data. Serialization will convert this to bytes. 
	//		Then deserialization will convert this back to the structs.
	//why is serialization necessary?
	//		Because for some reason you can't just save the data as is. 
	//		 You need to change it to a format that can be saved.
	#[derive(Debug)]
	#[derive(Serialize, Deserialize, Clone)]
	pub struct NetworkLayer {
		pub rows: usize,
		pub columns: usize,
		pub data: Vec<Vec<f64>>,        //need Vec<Vec   because we want the same format as WeightLayer   (i think)
	}
	#[derive(Debug)]
	#[derive(Serialize, Deserialize)]
	//01/18/24 - added to see if update weights is working
		#[derive(Clone, PartialEq)]
	pub struct WeightLayer {
		pub rows: usize,
		pub columns: usize,
		pub data: Vec<Vec<f64>>,
	}
	#[derive(Debug)]
	#[derive(Serialize, Deserialize)]
	pub struct BiasLayer {
		pub rows: usize,
		pub columns: usize,
		pub data: Vec<Vec<f64>>,        //same as up top^^
	}
	#[derive(Debug)]
	#[derive(Serialize, Deserialize)]
	pub struct NeuralNetwork {
		pub layers: Vec<NetworkLayer>,
		pub weights: Vec<WeightLayer>,
		pub biases: Vec<BiasLayer>,
		//01/19/24 - added
		pub gradients: GradientNetwork,
		//01/20/24 - added:
		pub replay_buffer: ReplayBuffer,
	}



	//01/19/24 - added
	#[derive(Debug)]
	#[derive(Serialize, Deserialize)]
	pub struct GradientLayer {
		pub rows: usize,
		pub columns: usize,
		pub data: Vec<Vec<f64>>,
	}
	//01/19/24 - added
	#[derive(Debug)]
	#[derive(Serialize, Deserialize)]
	pub struct GradientNetwork {
		pub layers: Vec<GradientLayer>,
	}











	//added 12/25/23
	#[derive(Debug)]
	#[derive(Serialize, Deserialize, Clone)]
	pub struct Transition {
		pub state: NetworkLayer,
		pub action: usize,
		pub reward: f64,
		pub next_state: NetworkLayer,
	}

	#[derive(Debug)]
	#[derive(Serialize, Deserialize)]
	pub struct ReplayBuffer {
		pub capacity: usize,
		pub buffer: Vec<Transition>,
	}

	impl ReplayBuffer {
		pub fn new(capacity: usize) -> ReplayBuffer {
			ReplayBuffer {
				capacity,
				buffer: Vec::with_capacity(capacity),
			}
		}

		pub fn push(&mut self, transition: Transition) {
			if self.buffer.len() == self.capacity {
				self.buffer.remove(0);
			}
			self.buffer.push(transition);
		}
		//12/28/23 CODE COMMENTED SO i CAN RUN IT
		pub fn sample(&self) -> &Transition {
			let mut rng = rand::thread_rng();
			let index = rng.gen_range(0..self.buffer.len());
			&self.buffer[index]
		}

		////added 12/26/23
		////why? because they would save to RAM instead if we didnt do this.
		//pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
		//	let file = File::create(filename)?;
		//	let writer = BufWriter::new(file);
		//	serde_json::to_writer(writer, &self)?;
		//	Ok(())
		//}
		pub fn save_replay_buffer_v2(&self) -> std::io::Result<()> {
			let base_path = "D:\\Downloads\\PxOmni\\rust_replay_buffers";
			let now = Utc::now();
			let timestamp = now.timestamp_millis().to_string();
			let file_path = Path::new(base_path).join(timestamp);
			let file = File::create(file_path)?;
			let writer = BufWriter::new(file);
			serde_json::to_writer(writer, &self)?;
			Ok(())
		}
		//added 12/26/23
		//why? to load from disk instead of clogging up ram
		pub fn load_from_file(filename: &str) -> std::io::Result<Self> {
			let file = File::open(filename)
				.expect(&format!("could not open file with filename:\n{}", filename));
			let reader = BufReader::new(file);
			let replay_buffer = serde_json::from_reader(reader)?;
			Ok(replay_buffer)
		}
		//added 01/11/24
		//this is to test if replay_buffer actually works
		pub fn load_most_recent() -> std::io::Result<Self> {
			let base_path = Path::new("D:\\Downloads\\PxOmni\\rust_replay_buffers");
			let mut most_recent_file = None;
			let mut most_recent_timestamp = 0;
	
			for entry in fs::read_dir(base_path)? {
				let entry = entry?;
				let path = entry.path();
				if path.is_file() {
					if let Some(filename) = path.file_name() {
						if let Some(filename_str) = filename.to_str() {
							if let Ok(timestamp) = filename_str.parse::<i64>() {
								if timestamp > most_recent_timestamp {
									most_recent_file = Some(path);
									most_recent_timestamp = timestamp;
								}
							}
						}
					}
				}
			}
	
			if let Some(most_recent_file) = most_recent_file {
				//01/24/24 - removed:
					//let file = fs::File::open(most_recent_file)?;
				//01/24/24 - added:
					//added file_path variable so I could do .expect with more information.
					//	 this may be costly in terms of time though.
					let file_path = most_recent_file.clone();
					let file = fs::File::open(file_path)
						.expect(&format!("could not open most recent file named:\n{:?}",
							&most_recent_file));
				let reader = BufReader::new(file);
				//01/24/24 - removed:
					//let replay_buffer = serde_json::from_reader(reader)?;
				//01/24/24 - added expect in this line
				let replay_buffer = serde_json::from_reader(reader)
					.expect("serde json reader didn't read");
				Ok(replay_buffer)
			} else {
				Err(std::io::Error::new(std::io::ErrorKind::Other, "No replay buffer found"))
			}
		}
		//added 01/11/24
		pub fn print_replay_buffer(&self) {
			println!("ReplayBuffer capacity: {}", self.capacity);
			println!("Number of transitions: {}", self.buffer.len());
			for (i, transition) in self.buffer.iter().enumerate() {
				println!("Transition {}: {:?}", i, transition);
			}
		}
		//01/20/24 added:
		//DEPRECATED. NEVER ONCE USED. DO NOT USE
		pub fn sample_random_replay_buffer(&self) -> std::io::Result<Transition> {
			let base_path = Path::new("D:\\Downloads\\PxOmni\\rust_replay_buffers");
			let mut rng = rand::thread_rng();
			let mut replay_buffers = Vec::new();
		
			// Collect all replay buffer files
			for entry in fs::read_dir(base_path)? {
				let entry = entry?;
				let path = entry.path();
				if path.is_file() {
					replay_buffers.push(path);
				}
			}
		
			// Randomly select a replay buffer file
			let random_index = rng.gen_range(0..replay_buffers.len());
			let random_file = &replay_buffers[random_index];
		
			// Load the selected replay buffer
			let file = fs::File::open(random_file)?;
			let reader = BufReader::new(file);
			//01/24/24 - added expect in this line
			let replay_buffer: ReplayBuffer = serde_json::from_reader(reader).expect("serde json reader didn't read");
		
			// Randomly select a transition from the replay buffer
			let random_index = rng.gen_range(0..replay_buffer.buffer.len());
			let random_transition = &replay_buffer.buffer[random_index];
		
			Ok(random_transition.clone())
		}	
	}
	//02/14/24 - added:
	pub fn delete_most_recent_replay_buffers( x: usize) -> std::io::Result<()> {
		let base_path = "D:\\Downloads\\PxOmni\\rust_replay_buffers";
		let mut heap = std::collections::BinaryHeap::new();
	
		for entry in std::fs::read_dir(base_path)? {
			let entry = entry?;
			let metadata = std::fs::metadata(entry.path())?;
			let timestamp = metadata.modified()?;
			heap.push((timestamp, entry.path()));
		}
	
		for _ in 0..x {
			if let Some((_, path)) = heap.pop() {
				fs::remove_file(&path)?;
				log::info!("Deleted replay_buffer: {:?}", &path);
			}
		}
	
		Ok(())
	}










	







	impl NetworkLayer {
		pub fn print_network_layer( &self) {
			for j in 0..self.rows {
				for i in 0..self.columns {
					println!("{:?} ", &self.data[j][i]);
				}
				println!("\n");
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
			for j in 0..self.rows {
				for i in 0..self.columns {
					println!("{:?} ", &self.data[j][i]);
				}
				println!("\n");
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
													//    vec![0.0, 1.0],   //row 0. these should be all the weights for the 1st/0th neuron
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
			//vec![  vec![what_i_want_each_element_to_be;number_of_elements] ; number_of_vectors];
			//	this would generate number_of_vectors number of vectors all initialized to 
			//	with each inner vector's number of elements being how many weights are connected to the first neuron.
			//why first neuron?
			//		because every neuron in the same layer has the same number of weights
			//	vec![what_i_want_each_element_to_be;number_of_elements]
			//NEED TO DO *MUT* because i'm changing sum later.
			let mut sum = vec![vec![0.0; weights[0].len()] ; layer.len()];


			//computes the actual matrix multiplication
			//confusing. I know, but best way to understand is to genuinely go 
			//	through the trouble of drawing it out and doing the calculations.
			//explanation is also below "sum"
			for i in 0..layer.len() {
				for j in 0..weights[0].len() {
					for k in 0..layer_columns {
						sum[i][j] += layer[i][k] * weights[k][j];
					}
				}
			}

			sum //this is the value returned. aka the matrix returned

			//previous neuron layer
			// vec![
  			//	vec![Na, Nb, Nc, ...]
  			// ]
			//weights:
			// vec![
  			//	vec![wa1, wa2, wa3, wa4, ...],
  			//	vec![wb1, wb2, wb3, wb4, ...],
  			//	vec![wc1, wc2, wc3, wc4, ...],
  			//	...
			// ]
			//next neuron layer
			// vec![
  			//	 vec![N1, N2, N3, N4,...]
  			// ]

			//we can think of sum[i][j] as the next neuron

			//i = 0, j = 0, k = 0
			//sum[i][j] = N1
			//layer[i][k] = Na
			//weights[k][j] = Wa1
			//so: N1 = Na*Wa1
			//i = 0, j = 0, k = 1
			//sum[i][j] = N1
			//layer[i][k] = Nb because 0th row, next column
			//weights[k][j] = Wb1 because 1st row, 0th column
			//so: N1 = Nb*Wb1 + Na*Wa1
			//this continues, then when j changes, we go to next neuron,
			//	and next column in weight layer
			
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


	//this is for the back propagation to update the weights
	//why is the leaky_relu_derivative important?
	//	helps us understand how much a change in the input to the activation function
	//	(leakyRelu) would affect its output. In backpropagation, we're trying to figure
	//	out how much each neuron contributed to the final error or loss
	pub fn leaky_relu_derivative(x: f64) -> f64 {
		if x >= 0.0 {
			1.0
		} else {
			0.01
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
		//01/28/24 - changed name from bigger to smaller
		let is_epsilon_smaller: bool;
		let p: f64 = rand::thread_rng().gen_range(0.0..=1.0);

		// *epsilon is used instead of just epsilon because in order to change epsilon
		//		i have to say: dereference epsilon to get the f64 value it points to
		//		and then subtract .0001 from it.
		// just epsilon would be like saying, go to this address in memory, and 
		//		subtract 0.0001 from it. NANI?? ARE YOU SURE YOU WANT TO DO THAT
		//02/03/24 - changed to 0.000001
		//03/14/24 - changed to -0.0001 so it will reach 0 within 10000 iterations
		*epsilon -= 0.0001;

		//03/14/24 - so epsilon doesn't decrease past 0
		if *epsilon < 0.0 {
			*epsilon = 0.0;
		}

		if p >= *epsilon {
			//01/28/24 - changed false to true here. 
			//	and reverse below so true = exploit like in exploration fun.
			is_epsilon_smaller = true; //--> so exploit
			is_epsilon_smaller	//return the bool
		}
		else {
			is_epsilon_smaller = false; //--> so explore
			is_epsilon_smaller	//return the bool
		}
	}

	//CODE COMMENTED THIS OUT FOR NOW. THIS IS 100% NECESSARY FOR THE PROPER
	//		 FUNCTIONALITY OF THE NEURAL NETWORK. BUT IT NEEDS PARTS THAT ARENT
	//		 DONE YET AND I WANT TO RUN MY SAVE AND LOAD STATES
	//pub fn reward_ function() -> f64 {
		//let new_balance = 
		//I need to figure out where I would get the balance from. Do I make an entire function just to return a balance
		//or can I return two f64 from 1 function


	//HOW I WILL STRUCTURE THIS FUNCTION*&*&*&*&(*(*(&*&*------------------------------------:
	//		this function will get information from the REST APIs of giver and recipient, 
	//			how_much_i_spent 	will equal how much I spent buying crypto from giver
	//			balance				will equal how much recipient wallet was at before crypto transfer
	//			new_balance			will equal how much recipient wallet was at after+  crypto transfer
	//			change				will equal new_balance minus balance
	//			updated_balance		will equal balance * (1.0 + change);
	//			then return updated_balance.ln()
	//
	//				why 1.0 + change?	so that if change was .05, multiplying it by balance
	//					would mean losing money. I'm trying to find the gain here, so it would be 
	//					balance*1.05.
	//				why ln?				to account for greater loss.
	//						absolute value of  ln(1-x) is greater than ln(1+x). this is good so our
	//						DQN will weigh losses as heavier than "equivalent" gain
	

			//the reason I made it type    Option<f64> is because I don't want to prematurely
			//    assign a value to it, so if somehow it never gets assigned a value,
			//    I can then handle the situation.
			//let how_much_i_spent: Option<f64>;
			//match how_much_i_spent {
			//	Some(_) => (),
			//	None => panic!("how_much_i_spent   is none"),
			//}
			//let how_much_2nd_wallet_changed: Option<f64>;           //same thing as above^^
			//match how_much_2nd_wallet_changed {
			//	Some(_) => (),
			//	None => panic!("how_much_2nd_wallet_changed    is none"),
			//}

			//can't directly subtract Option types. So I need to handle the possiblity that
			//how_much_i_spent    and/or    how_much_2nd_wallet_changed    could carry no value
			//adds another layer of redundancy, which is always good
			//let total_gained = match(how_much_i_spent, how_much_2nd_wallet_changed) {
			//	(Some(spent), Some(gained)) => Some(gained-spent),
			//	_ => None,
			//}.expect("No value found");



	//-----------------MY ACTUAL REWARD FUNCITON------------------------------------------//
		//12/14/23: Im thinking of making this as my reward function but Im not sure what 
		//	the best course of action is.
		//nah, you know what, FUCK ITTTTTT. Im using this for now, and if "oh no, im losing
		//	money in sandbox mode", then Ill change it later

		//self needs to be changed to the porfolio's worth
		//need to REMOVE the parameters, and just input function calls into this function itself 
		//	that get the total portfolio size and 

		//changed 12/27/23:
	pub fn reward(value_prior: f64, value_after: f64 ) -> f64 {
		let multiplier = 1.3;
		let absolute_change = value_after - value_prior;
		let relative_change = absolute_change / value_prior;
		let reward = if absolute_change > 0.0 {
			absolute_change
		} 
		else {
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
		//02/27/24 - added. I'm trying to figure out how I'm getting nan for the rewards.
			if reward.is_nan() {
				log::error!("Reward is NaN. value_after = {}, value_prior = {}", &value_after, &value_prior);
			}
		return reward;
	}


		//for back propagation to update weights.
	//Gives us a measure of how well we're doing. 
	//	The lower the loss the better the network's predictions
	//DONT THINK THIS IS EVEN NEEDED.
	//MAY DEPrecRATE
	pub fn calculate_loss( current_q_value: &f64, target_q_value: &f64) -> f64 {
		(current_q_value - target_q_value).powi(2)
	}
	//This tells us how much the loss's output would change if we made a small change
	//  to its input. If the derivative is positive, it means increasing the weight 
	//	would increase the loss. So to minimize the loss, we should decrease the weight.
	//  If the derivative is negative, increasing the weight would decrease the loss, 
	//	so we should increase the weight. 
	pub fn calculate_loss_derivative(current_q_value: &f64, target_q_value: &f64) -> f64 {
		2.0 * (current_q_value - target_q_value)
	}









	

	/*
	//since gemini's updates are so slow, I am going to manually updated gemini with
	//	rest request right before I do a cycle 
	
	pub async fn update_geminis_bitch_ass(client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str, neural_network: &mut NeuralNetwork) -> Result<f64, Box<dyn Error + Send>> {
		//Gemini documentation:
		//For public API entry points, we limit requests to 120 requests per minute,
		//	 and recommend that you do not exceed 1 request per second.
		//So:
		//	I will do a cycle with gemini and time how long it takes plus 10%
		//Then:
		//	if under a second I will round up and wait that long plus a littleMore
		


		//INSERT DELAY_FOR HERE


		fn sign_gemini(gemini_secret: &str, gemini_payload: &serde_json::Value) -> String {
            let encoded_payload = encode(gemini_payload.to_string());
            let mut mac = Hmac::<Sha384>::new_from_slice(&gemini_secret.as_bytes())
                            .expect("HMAC can take key of any size");
            mac.update(encoded_payload.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            let gemini_signature = hex::encode(code_bytes);
            println!("Gemini signature:\n{}", &gemini_signature);
            gemini_signature
    
        }
        //if no "now" in scope when moving file, 
        //	the code is this:
        ////returns current time.
        //		let now = Utc::now();
        let now = Utc::now();
        let gemini_time_stamp = now.timestamp().to_string();
        let gemini_nonce = gemini_time_stamp;
        let gemini_url = "https://api.gemini.com/v1/pubticker/solusd";
        let gemini_payload = json!({
            "request": "/v1/mytrades",
            "nonce": &gemini_nonce
        });
        let base64_encoded_payload = encode(gemini_payload.to_string());
        let gemini_content_type = "text/plain";
        let gemini_content_length = "0";
        let gemini_cache_control = "no-cache";
        let gemini_signature = sign_gemini(&gemini_secret, &gemini_payload);
        
        let gemini_request = client.get(gemini_url)
                .header("Content-Type", gemini_content_type)
                .header("Content-Length", gemini_content_length)
                .header("X-GEMINI-APIKEY", gemini_api_key)
                .header("X-GEMINI-PAYLOAD", base64_encoded_payload)
                .header("X-GEMINI-SIGNATURE", &gemini_signature)
                .header("Cache-Control", gemini_cache_control)
                .build()
                .expect("couldn't build gemini request");
    
    
        let gemini_response = client.execute(gemini_request).await
                                .expect("Failed to execute Gemini request");
        let gemini_response_text = gemini_response.text().await
                                .expect("Failed to turn response into text");
        let v: serde_json::Value = serde_json::from_str(&gemini_response_text)
                                .expect("Failed to parse JSON");
        let gemini_sell_pricebid: f64 = v["bid"].as_str().unwrap().parse().unwrap();
        //CAN ONLY BUY. NOT SELL
        let gemini_buy_ask: f64 = v["ask"].as_str().unwrap().parse().unwrap();

		let indices = [58, 59];
        let new_values = [gemini_sell_pricebid, gemini_buy_ask];

		neural_network.update_input(&indices, &new_values).await;
	}
	*/

	//01/20/24 added-
	//01/28/24 - added the bool and the new functionality
	//02/16/24 - added index_chosen_for_current_state: &usize as another parameter
	pub fn save_reward(reward: &f64, exploration_or_exploitation: bool, i: &usize, index_chosen_for_current_state: &usize) -> std::io::Result<()> {
		//01/28/24 - added:
		let exploit_or_explore: Option<&str>;
		if exploration_or_exploitation == true {
			exploit_or_explore = Some("exploit");
		}
		else {
			exploit_or_explore = Some("explore");
		}
		let now = Utc::now();
		let timestamp = now.timestamp_millis().to_string();
		let base_path = "D:\\Downloads\\PxOmni\\rust_neural_network_rewards";
		let file_path = Path::new(base_path).join("rewards.txt");
		//01/24/24 - added line directly below this for debugging
			println!("reward saved to:{}", file_path.display());
		//01/24/24 - removed:
			//let mut file = fs::OpenOptions::new().write(true).append(true).open(file_path)?;
		//01/24/24 - added in its place:
			let mut file = fs::OpenOptions::new().write(true).append(true).create(true).open(file_path)?;
		//02/03/24 - added the if condition
			if reward <= &0.0 {
				//02/16/24 - added \tindex_chosen: {} and the var
				writeln!(file, "{},{}\t{:?}\ti:{}\tindex_chosen: {}", reward, timestamp, exploit_or_explore, i, index_chosen_for_current_state)?;
			}
			else {
				writeln!(file, "+{},{}\t{:?}\ti:{}\tindex_chosen: {}", reward, timestamp, exploit_or_explore, i, index_chosen_for_current_state)?;
			}
		//02/27/24 - added. I'm trying to figure out how I'm getting nan for the
		//		reward value so this will help me identify if there are specific 
		//		indexes that are the culprit.
			if reward.is_nan() {
				panic!("In save_reward fn: reward is NaN. index_chosen = {}
				there should be a log::error above with what the value_after
				and value_prior were."
				, &index_chosen_for_current_state);
			}
		//writeln!(file, "{},{}\t{:?}", reward, timestamp, exploit_or_explore)?;
		Ok(())
	}




	pub fn is_folder_empty(folder: &str) -> std::io::Result<bool> {
		let mut entries = fs::read_dir(folder)?;
		Ok(entries.next().is_none())
	}









	

	impl NeuralNetwork {
		
		pub fn feed_forward(&mut self) {
			
			////line of code below added 01/10/24
			////why lock mutex: because it could uses the input layer's values, and so I can't have
			////		them change while I'm accessing them. so lock it, then when I'm done
			////		 and I unlock it, then you can mutate them.
			////Removed 01/10/24 - I will have the mutex on the cycle function itself
			//// 		due to the need for the same input to be fed forward and stored in exp. replay
			//let _guard = self.input_mutex.lock().unwrap();

			//how it's structured
			//neurons:
			// vec![
  			//	vec![Na, Nb, Nc, ...]
  			// ]
			//weights:
			// vec![
  			//	vec![wa1, wa2, wa3, wa4, ...],
  			//	vec![wb1, wb2, wb3, wb4, ...],
  			//	vec![wc1, wc2, wc3, wc4, ...],
  			//	...
			// ]
			//neurons:
			// vec![
  			//	 vec![N1, N2, N3, N4,...]
  			// ]

			//starting at 1 because all the layers rely on the layer before it, 
			//		and the input layer is just the input itself so I dont have
			//		 to compute anything for it
			
			for i in 1..self.layers.len() {

				//i REALLY NEED to understand this part more. I need to know what's being
				//	 multiplied and when and what's being added and when.
				//
				//I actually understand it now
				//so the previous activation is just the output of the neuron in the previous layer
				//and weights is just the weights extending FROM the previous layer.

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
		//added 01/10/24
		//why? because I have a mutex in pub fn cycle that wont allow me to do a feed_forward
		//	unless input layer is cloned
		pub fn feed_forward_with_cloned_input(&mut self, input_layer: &NetworkLayer) {
			// Use the cloned input layer as the first layer in the feed_forward process
			self.layers[0] = input_layer.clone();
		
			for i in 1..self.layers.len() {
				let previous_activations = &self.layers[i-1].data;
				let weights = &self.weights[i-1].data;
				let biases = &self.biases[i-1].data;
		
				self.layers[i].data = matrix_multiply(previous_activations, weights);
				self.layers[i].data = matrix_add(&self.layers[i].data, biases);
		
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
		//01/28/24 - added another return of the bool being exploit_or_explore
		//	so in my reward function I can see if it made money on an exploit
		pub fn exploration_or_exploitation(&self, epsilon: &mut f64) -> (usize, f64, bool) {
			
			// want to see if epsilon greedy returns true or not so that I explore or exploit
			let exploit_or_explore: bool = epsilon_greedy(epsilon);

			//True = exploit
			//establishes values to work with for-loop
			let mut index_of_largest_qvalue: Option<usize> = None;
			let mut largest_qvalue_so_far = f64::MIN;

			let mut indexx: usize = 0;		//this will function as the index in the for loop below

			//01/28/24 - added:
				let last_layer = self.layers.last().expect("last layer didn't exist");
				let last_index = last_layer.columns - 1;

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
					//aka interate over the data in the last layer. aka over the values of the
					//		neurons of the last layer. aka over the Q_VALUES of the last layer
					//all my neurons  are stored in the first inner vector of each network layer,
					//		hence the .data[0]
					for value in &last_layer.data[0] {
						//01/24/24 - got an error thanks to the "let index match" statement below.
						//	the last layer existed because thankfully I added error handling for it.
						//	I can only think of 2 ways this is possible, the feed forward
						//		 didn't work properly or somehow the largest_q_value in
						//		 the output layer was f64::MIN.
						if value > &largest_qvalue_so_far {
							largest_qvalue_so_far = *value;	//just to document that we hit a new max
							index_of_largest_qvalue = Some(indexx);	//to know where the new max was
						}

						indexx += 1;					//to iterate the index value NO MATTER WHAT
					}
				}
				else {
					//01/24/24 - modified to say last layer didnt exist
					//		instead of was empty
					panic!("last_layer.data did not exist. this is in fn exploration_or_exploration when
					 exploit_or_explore == true");
				}

				//-----the original of the above-----//
						//if let Some(last_layer) = self.layers.last() {
						//	for value in &last_layer.data[0] {
						//		if value > &largest_qvalue_so_far {
						//			largest_qvalue_so_far = *value;		//just to document that we hit a new max
						//			index_of_largest_qvalue = Some(indexx);	//to know where the new max was
						//			indexx += 1;						//to iterate the index value
						//
						//		}
						//		else {		//this block executes only if the value isn't bigger
						//					//		than the largest qvalue we have so far.
						//					//Because:	we dont care about storing the index 
						//					//		of a smaller q value,
						//					//		and instead we jsut want to show we visited 
						//					//		another value, then We do this by 
						//					//		just incrementing the index
						//			indexx+=1;		
						//		}
						//	}
						//}
						//else {
						//	panic!("last_layer.data is empty. this is in fn exploration_or_exploration when exploit_or_explore == true");
						//}
				//------end of the original---------//





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
				let index = match index_of_largest_qvalue {
					Some(index) => index,
					//01/24/24 - modified to include indexx
					None => panic!("index_of_largest_qvalue was never initialized. Indexx is:\n{}
					check waht the q values were. they were probably -inf or NaN", indexx),
				};
				//this returns both the index and the largest_q_value_so_far.
				//why no semicolon?
				//		(index, largest_qvalue_so_far) is an expression, aka it returns a value.
				//		 if you add a semicolon, it makes it a statement and doesn't return
				//		 a value
				//01/28/24 - added exploit_or_explore as a return value
				(index, largest_qvalue_so_far, exploit_or_explore)

				
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
				//01/28/24 - changed 0..=indexx
				//	to 0..=last_index
				let index_of_random_qvalue = rand::thread_rng().gen_range(0..=last_index);

				//not even sure if this is needed. I think I can just delete this and in the
				//	 bottom do index_of_random_qvalue instead of index
				//let index = match index_of_random_qvalue {
				//	Some(index) => index,
				//	None => panic!("index_of_random_qvalue was never initialized"),
				//};

				match self.layers.last() {
					Some(last_layer) =>  {
						return (index_of_random_qvalue, last_layer.data[0][index_of_random_qvalue], exploit_or_explore);
					},
					None => panic!("No layers in the network!"),
				}

			}
			
		}












































		//02/06/24 - added: new exploration or exploitation function because I think
		//		this may be a cause in the explosion of the q values as it doesnt
		//		have a chance to stabilize its q values
		pub fn exploration_or_exploitation_v2(&self, i: &usize) -> (usize, f64, bool) {
			
			// want to see if epsilon greedy returns true or not so that I explore or exploit
			//let exploit_or_explore: bool = epsilon_greedy(epsilon);
			let exploit_or_explore: Option<bool>;
			if i <= &1000 {
				exploit_or_explore = Some(false);
			}
			else  {
				//for the first 300...
				//02/17/24 - changed from 1300 to 3000
				if i <= &3000 {
					//every 10 iterations, make it exploit. aka true
					//03/14/24 - changed to 11 so that it wont try to exploit when doing a 
					//		replay buffer. It wont, but so that it DOES EXPLOIT before 3000
					//		iterations.
					if i%11 == 0 {
						exploit_or_explore = Some(true);
					}
					else {
						exploit_or_explore = Some(false);
					}
				}
				//for the next 400...
				//02/17/24 - changed from 1700 to 7_000
				else if i <= &7_000 {
					//every 9 iterations, make it exploit. aka true
					if i%9 == 0 {
						exploit_or_explore = Some(true);
					}
					else {
						exploit_or_explore = Some(false);
					}
				}
				//for the next 500...
				//02/17/24 - changed from 2200 to 15_000
				else if i <= &15_000 {
					//every 8 iterations, make it exploit. aka true
					if i%8 == 0 {
						exploit_or_explore = Some(true);
					}
					else {
						exploit_or_explore = Some(false);
					}
				}
				//for the next 600...
				//02/17/24 - changed from 2800 to 31_000
				else if i <= &31_000 {
					//every 7 iterations, make it exploit. aka true
					if i%7 == 0 {
						exploit_or_explore = Some(true);
					}
					else {
						exploit_or_explore = Some(false);
					}
				}
				//for the next 700...
				//02/17/24 - changed from 3500 to 60_000
				else if i <= &60_000 {
					//every 6 iterations, make it exploit. aka true
					if i%6 == 0 {
						exploit_or_explore = Some(true);
					}
					else {
						exploit_or_explore = Some(false);
					}
				}
				//for the next 800...
				//02/17/24 - changed from 4300 to 80_000
				else if i <= &80_000 {
					//every 5 iterations, make it exploit. aka true
					if i%5 == 0 {
						exploit_or_explore = Some(true);
					}
					else {
						exploit_or_explore = Some(false);
					}
				}
				//for the next 900...
				//02/17/24 - changed from 5200 to 100_000
				else if i <= &100_000 {
					//every 4 iterations, make it exploit. aka true
					if i%4 == 0 {
						exploit_or_explore = Some(true);
					}
					else {
						exploit_or_explore = Some(false);
					}
				}
				//for the next 1000...
				//02/17/24 - changed from: 6200 to 120_000
				else if i <= &120_000 {
					//every 3 iterations, make it exploit. aka true
					if i%3 == 0 {
						exploit_or_explore = Some(true);
					}
					else {
						exploit_or_explore = Some(false);
					}
				}
				//for the next 5000...
				//02/16/24 - changed to next 20000
				//02/17/24 - changed from: 26200 "until 140_000"
				else if i <= &140_000 {
					// if divisible by 2 or 3, make it exploit
					//02/16/24 - changed from 2 or 3, to just 2
					if i%2==0 {
						exploit_or_explore = Some(true);
					}
					else {
						exploit_or_explore = Some(false);
					}
				}
				//for the next whenever...
				else {
					// if divisible by 2 or 3 or 5, make it exploit
					//02/16/24 - changed from 2,3,5  to  2 or 3
					if i%2==0 || i%3 == 0 {
						exploit_or_explore = Some(true);
					}
					else {
						exploit_or_explore = Some(false);
					}
				}
			}



			//True = exploit
			//establishes values to work with for-loop
			let mut index_of_largest_qvalue: Option<usize> = None;
			let mut largest_qvalue_so_far = f64::MIN;

			let mut indexx: usize = 0;		//this will function as the index in the for loop below

			//01/28/24 - added:
				let last_layer = self.layers.last().expect("last layer didn't exist");
				let last_index = last_layer.columns - 1;

			if exploit_or_explore == Some(true) {
				//Below: I will choose the neuron with the top q value.
				//		this would then call another function that executes the specific task



				//let Some(last_layer) = self.layers.last()		means set last_layer 
				//		equal to self.layers.last() aka the last layer aka the output layer.
				//The reason we have the	"if"	is because we want to handle errors
				//		like if somehow there weren't any layers in the first place
				//		Then it would go to the corresponding else block and tell us the error
				//The more error checking the better
				if let Some(last_layer) = self.layers.last() {
					//aka interate over the data in the last layer. aka over the values of the
					//		neurons of the last layer. aka over the Q_VALUES of the last layer
					//all my neurons  are stored in the first inner vector of each network layer,
					//		hence the .data[0]
					for value in &last_layer.data[0] {
						//01/24/24 - got an error thanks to the "let index match" statement below.
						//	the last layer existed because thankfully I added error handling for it.
						//	I can only think of 2 ways this is possible, the feed forward
						//		 didn't work properly or somehow the largest_q_value in
						//		 the output layer was f64::MIN.
						if value > &largest_qvalue_so_far {
							largest_qvalue_so_far = *value;	//just to document that we hit a new max
							index_of_largest_qvalue = Some(indexx);	//to know where the new max was
						}

						indexx += 1;					//to iterate the index value NO MATTER WHAT
					}
				}
				else {
					//01/24/24 - modified to say last layer didnt exist
					//		instead of was empty
					panic!("last_layer.data did not exist. this is in fn exploration_or_exploration_v2 when
					 exploit_or_explore == true");
				}

				//-----the original of the above-----//
						//if let Some(last_layer) = self.layers.last() {
						//	for value in &last_layer.data[0] {
						//		if value > &largest_qvalue_so_far {
						//			largest_qvalue_so_far = *value;		//just to document that we hit a new max
						//			index_of_largest_qvalue = Some(indexx);	//to know where the new max was
						//			indexx += 1;						//to iterate the index value
						//
						//		}
						//		else {		//this block executes only if the value isn't bigger
						//					//		than the largest qvalue we have so far.
						//					//Because:	we dont care about storing the index 
						//					//		of a smaller q value,
						//					//		and instead we jsut want to show we visited 
						//					//		another value, then We do this by 
						//					//		just incrementing the index
						//			indexx+=1;		
						//		}
						//	}
						//}
						//else {
						//	panic!("last_layer.data is empty. this is in fn exploration_or_exploration when exploit_or_explore == true");
						//}
				//------end of the original---------//





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
				let index = match index_of_largest_qvalue {
					Some(index) => index,
					//01/24/24 - modified to include indexx
					None => panic!("index_of_largest_qvalue was never initialized. Indexx is:\n{}", indexx),
				};
				//this returns both the index and the largest_q_value_so_far.
				//why no semicolon?
				//		(index, largest_qvalue_so_far) is an expression, aka it returns a value.
				//		 if you add a semicolon, it makes it a statement and doesn't return
				//		 a value
				//01/28/24 - added exploit_or_explore as a return value

				if let Some(exploit_or_explore) = exploit_or_explore {
					(index, largest_qvalue_so_far, exploit_or_explore)
				} else {
					panic!("exploit_or_explore is somehow none\nexploit_or_explore:{:?}
					index:{}
					largest_qvalue_so_far:{}
					iteration:{}", exploit_or_explore, index, largest_qvalue_so_far, &i );
				}
				//in this point in the code, i now have the index of the largest q value.
				//This value is now returned.
				//In the next function or module, I must then choose the function
				//		that corresponds to said q value.
				//I will do that in another funciton. I might even make an entire 
				//		module just for that function
				
			}
			//so if doing explore
			else {
				//turn exploit_or_explore form Option<bool> to bool
				if let Some(exploit_or_explore) = exploit_or_explore {
					//choose one of the outputs randomly. the specific output would then
					//		call another function to execute said task

					//attaches a random value between 0 and the last neuron to index_of_random_qvalue
					//		because we want to return a random "neuron" because we're doing
					//		explore. explore means do some random shit, so we can then document
					//		if it was good or not
					//01/28/24 - changed 0..=indexx
					//	to 0..=last_index
					let index_of_random_qvalue = rand::thread_rng().gen_range(0..=last_index);


					match self.layers.last() {
						Some(last_layer) =>  {
							return (index_of_random_qvalue, last_layer.data[0][index_of_random_qvalue],
								 exploit_or_explore);
						},
						None => panic!("No layers in the network!"),
					}
				}
				//if exploit_or_explore never had a value attached to it
				else {
					panic!("exploit_or_explore is somehow none\nexploit_or_explore:{:?}
					iteration:{}", exploit_or_explore, &i);
				}

			}
			
		}




















		/*
		pub fn initialization(&mut self, input_size: usize, output_size: usize, number_of_hidden_layers: usize) {
			//intiialization of weights and biases and what not
			//initialization rule I'm following:
        	//	The number of hidden neurons should be 2/3 the size of the input layer, 
			//		plus the size of the output layer.
			//
			//it will make the hidden layers each the same size.
			//  NEED TO ADD ANOTHER PARAMETER FOR IF I WANT PYRAMID, REVERSE PYRAMID, OR NORMAL 
			

			//hidden_size is usize because i cant have a fraction of a neuron, nor a negative size.
			//why as f64 and then as usize?
			//		because if I did 2/3 * (input + output), the 2/3 would be rounded to an int,
			//		 and the result is 0.
			let hidden_size = (2.0 / 3.0 * (input_size + output_size) as f64) as usize;
			//let hidden_size = 2/3 * (input_size + output_size);
			//this creates the random number generator 
    		let mut rng = rand::thread_rng();

			// Input layer
			
			//The .push() is acutally creating a new NetworkLayer with the properties:
			//		rows: 1,
			//		columns: input_size,
			//		data: vec![vec![0.0; input_size]],
			//	and then appending the layer it creates, to the end of the .layers it already has
			 
			self.layers.push(NetworkLayer {
				rows: 1,
				columns: input_size,
				data: vec![vec![0.0; input_size]],
			});

			//---------------------Hidden layers----------------------//

			//--first hidden layer--//
				//for loop removed because I'm only making one layer 
				
				//pushhing NetworkLayer first because each layer needs to be initialized
				//	before establish weights and baises
				
				self.layers.push(NetworkLayer {
					rows: 1,
					columns: hidden_size,
					data: vec![vec![0.0; hidden_size]],
				});

				//this creates the StandardNormal distribution itself 
				let normal_distr = Normal::new(0.0, 1.0).unwrap();


				
				//(0..hidden_size).map(|_| {...})...collect()	this is creating a new Vec
				//		with hidden_size # of elements.
				//		For each element it applies this function: 
				//		normal_distr.sample(&mut rng) * (2.0 / (hidden_size as f64)).sqrt()
				//		.collect()	is returning these results into the new vector
				//		|_|			means we aren't using the values currently there, 
				//					if there are any
				//why no .iter()? because the range itself:	(0..hidden_size) works as the iterator.
				//why 2 layers of (0..hidden_size).map?		the inside layer creates each inside
				//												vec![1, 2, 3, ...]	.
				//						each iteration of the outer (0..hidden_size).map creates
				//							the outer vec![ ] that all the tiny vec![] of each
				//							hidden layer are in
				
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
				//only difference is:
				//self.weights.push(WeightLayer {
				//	rows: hidden_size,
				//	columns: hidden_size,
				//	data: weights,
				//});
				//	instead of rows: input_size above
				

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

				//I THINK THIS SHOULD BE DELETED BECAUSE EVERYTIME I PUSH a weight layer
				//		 I am establishing weights connecting FROM the neurons of the same layer.
				//		 So why would I make weights connecting FROM the output layer. Also, why would my output layer need BIASES?
				//if I delete the things below this, I need to delete the things ABOVE this too because they establish the weights
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
			
			//representation of how shit is initialized in each layer:
			//key word search: how everything is initialized in each layer,
			//		 how each layer is initialized, how weights are initialized, 
			//		 how neurons are initialized, how biases are initalized,
			//neurons:
			// vec![
  			//	vec![Na, Nb, Nc, ...]
  			// ]
			//weights:
			// vec![
  			//	vec![wa1, wa2, wa3, wa4, ...],
  			//	vec![wb1, wb2, wb3, wb4, ...],
  			//	vec![wc1, wc2, wc3, wc4, ...],
  			//	...
			// ]
			//neurons:
			// vec![
  			//	 vec![N1, N2, N3, N4,...]
  			// ]



    	}
		*/




















		pub fn initialization(&mut self, input_size: usize, output_size: usize, number_of_hidden_layers: usize) {
			// Define the size of each hidden layer
			let hidden_size = (2.0 / 3.0 * (input_size + output_size) as f64) as usize;
			println!("hidden size:\n{}", &hidden_size);
		
			// Create the random number generator
			let mut rng = rand::thread_rng();
		
			// Initialize the input layer
			self.layers.push(NetworkLayer {
				rows: 1,
				columns: input_size,
				data: vec![vec![0.0; input_size]],
			});
		
			// Initialize the hidden layers
			for _ in 0..number_of_hidden_layers {
				// Initialize the weights and biases for the current layer
				let normal_distr = Normal::new(0.0, 1.0).unwrap();
				let weights: Vec<Vec<f64>> = (0..self.layers.last().unwrap().columns).map(|_| {
					(0..hidden_size)
					.map(|_| (normal_distr.sample(&mut rng) * (2.0 / (self.layers.last().unwrap().columns as f64)).sqrt())).collect()
				}).collect();
		
				self.weights.push(WeightLayer {
					rows: self.layers.last().unwrap().columns,
					columns: hidden_size,
					data: weights,
				});
		
				self.biases.push(BiasLayer {
					rows: 1,
					columns: hidden_size,
					data: vec![vec![0.01; hidden_size]],
				});
		
				// Initialize the neurons for the current layer
				self.layers.push(NetworkLayer {
					rows: 1,
					columns: hidden_size,
					data: vec![vec![0.0; hidden_size]],
				});
			}
		
			// Initialize the output layer
			let normal_distr = Normal::new(0.0, 1.0).unwrap();
			let weights: Vec<Vec<f64>> = (0..self.layers.last().unwrap().columns).map(|_| {
				(0..output_size)
				.map(|_| (normal_distr.sample(&mut rng) * (2.0 / (self.layers.last().unwrap().columns as f64)).sqrt())).collect()
			}).collect();
		
			self.weights.push(WeightLayer {
				rows: self.layers.last().unwrap().columns,
				columns: output_size,
				data: weights,
			});
		
			self.biases.push(BiasLayer {
				rows: 1,
				columns: output_size,
				data: vec![vec![0.01; output_size]],
			});
		
			self.layers.push(NetworkLayer {
				rows: 1,
				columns: output_size,
				data: vec![vec![0.0; output_size]],
			});

			//01/19/24 - added:
			//this establishes the GradientLayer
			for weight_layer in &self.weights {
				self.gradients.layers.push(GradientLayer {
					rows: weight_layer.rows,
					columns: weight_layer.columns,
					data: vec![vec![0.0; weight_layer.columns]; weight_layer.rows],
				});
			}
		}























		pub async fn update_input(&mut self, indices: &[usize], new_values: &[f64]) {

			//this line added: 01/10/24
			//need mutex because I dont want to update the input layer while I'm reading from it
			//01/11/24 - removed.
			//let _guard = self.input_mutex.lock().await;



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
		//12/23/23 code commented this out and added a better one
		//pub fn print_network(&self) {
		//	for i in 0..self.layers.len() {
		//		println!("Layer {}:", i+1);
		//		for j in 0..self.biases[i].data[0].len() {
		//			println!("Node {}: {:.2}", j+1, self.biases[i].data[0][j]);
		//		}
		//
		//		if i < self.weights.len() {
		//			println!("Weights to next layer:");
		//			for row in &self.weights[i].data {
		//				let weights: Vec<String> = row.iter().map(|&x| format!("{:.2}", x))
		//											.collect();
		//				println!("{}", weights.join("\t"));
		//			}
		//		}
		//	}
		//}













		//added 12/23/23
		//pub fn print_layers(&self) {
		//	for (i, layer) in self.layers.iter().enumerate() {
		//		println!("Layer {}:", i);
		//		for row in &layer.data {
		//			for item in row {
		//				print!("{}, ", item);
		//			}
		//			println!();
		//		}
		//	}
		//}
		//pub fn print_layers(&self) {
		//	for i in 0..self.layers.len() {
		//		println!("Layer {}", i + 1);
		//		println!("Network Layer: {:?}", self.layers[i]);
		//		println!("Weight Layer: {:?}", self.weights[i]);
		//		println!("Bias Layer: {:?}", self.biases[i]);
		//	}
		//}

		//added 12/24/23 the previous print_layers produced an error
		pub fn print_layers(&self) {
			for i in 0..self.layers.len() {
				println!("Layer {}", i + 1);
				println!("Network Layer: {:?}", self.layers[i]);
				if i < self.weights.len() {
					println!("Weight Layer: {:?}", self.weights[i]);
					println!("Bias Layer: {:?}", self.biases[i]);
				}
			}
		}












		//12/23/23 changed function to not use index because it's not being returned and never used
		pub fn calculate_target_q_value(&mut self, reward: f64) -> f64{
			//gamma is basically a numerical representation of how much I value future states
			//	 and their corresponding q_values.
			//		it's value is from 0 to 1. 0 means I dont value the next state at all
			//		1 is excluded because it diverges: 
			//https://ai.stackexchange.com/questions/11708/can-gamma-be-greater-than-1-in-a-dqn 
			//		The higher the gamma the more I value the future rewards
			//0.9 is standard and I wasnt sure what to pick, so I just picked it. 
			//		if I want to change it, I'll change it later.
			//02/03/24 - changed to 0.8. this is when I had 1 sec intervals
			//02/15/24 - changed to 0.7. I have 50 ms and 200 ms intervals. 
			//02/15/24 - changed to 0.6. at 3165 iterations, i saw 10^25
			//02/16/24 - changed to 0.5
			//02/19/24 - changed to 0.4
			//02/29/24 - changed to 0.3
			//03/05/24 - changed to 0.4
			//03/06/24 - changed to 0.5. then 0.6 then 0.7 then 0.8
			//03/09/24 - changed to 0.99. then 0.9
			//03/10/24 - changed to 0.99
			let gamma = 0.99;
			//initialize the largest Q-value so far and its index
			//let mut index_of_largest_qvalue_in_next_state: Option<usize> = None;
			let mut largest_qvalue_so_far_in_next_state = f64::MIN;

			//I want to feed forward so I have a new set of q_values that will serve as my
			//	 "next_q_value"
			self.feed_forward();




			//the "if let Some(..."  is saying if the last layer exists, then perform
			//	 the rest of the calculations
			//self.layers.last() is just the las layer in the neural network, which is the output
			//	 layer. the data in it contains the "next" q_values thanks to the 
			//	 feed_forward() from before.
			//for (index, &value) in last_layer.data[0].iter().enumerate() {
			//		it's a for loop and it is looking in last_layer.data[0] for the q_values.
			//why?
			//because the neural network is structured like this
			//	pub struct NeuralNetwork {
			//	layers: Vec<NetworkLayer>,
			//	weights: Vec<WeightLayer>,
			//	biases: Vec<BiasLayer>,
			//so the layers is really an "array" or a Vec of network layers.
			//network layer is structured like this:
			//pub struct NetworkLayer {
			//	rows: usize,
			//	columns: usize,
			//	data: Vec<Vec<f64>>, 
			//}
			//with columns actually being 0. So last_layer.data[0] is structured like this:
			//	vec![
			//    	vec![0.0, 1.0],   //row 0
			//  ]
			//so iterating through .data[0] because i'm iterating through the first row because
			//	 each actual network layer of the neural network only has 1 row.
			//.iter() means to iterate through it
			//.enumerate() means that it will get the value AND the index.
			//then the inner "if" is just standard for finding the maximum value in a vec or array.
			if let Some(last_layer) = self.layers.last() {
				for  &value in last_layer.data[0].iter() {
					if value > largest_qvalue_so_far_in_next_state {
						largest_qvalue_so_far_in_next_state = value;
						//index_of_largest_qvalue_in_next_state = Some(index);
					}
				}
			}
			else {
				//NEED TO PERFORM A SAVE BEFORE THIS PANIC. HAVENT IMPLEMENTED THAT YET THOUGH
				//NEED to do it before v1 launches
				panic!("Warning: self.layers is empty!");
			}
			// Calculate the target Q-value
			//this is the Bellman Optimality equation.
			let target_q_value = reward + gamma * largest_qvalue_so_far_in_next_state;

			target_q_value

		}













		//MOVED.
		//THESE WERE MOVED TO NOT BE METHODS. they didnt need &self

		////for back propagation to update weights.
		////Gives us a measure of how well we're doing. 
		////	The lower the loss the better the network's predictions
		//pub fn calculate_loss( current_q_value: &f64, target_q_value: &f64) -> f64 {
		//	(current_q_value - target_q_value).powi(2)
		//}
		////This tells us how much the loss's output would change if we made a small change
		////  to its input. If the derivative is positive, it means increasing the weight 
		////	would increase the loss. So to minimize the loss, we should decrease the weight.
		////  If the derivative is negative, increasing the weight would decrease the loss, 
		////	so we should increase the weight. 
		//pub fn calculate_loss_derivative(current_q_value: &f64, target_q_value: &f64) -> f64 {
		//	2.0 * (current_q_value - target_q_value)
		//}



























		//code commenting all of the previous versions out.
		//This is probably the hardest part to understand. But think of a 2d graph and the
		//	 gradient is just the slope of the graph. We're basically seeing which nudges
		//	 to the weights and biases cause the fastest change to the local minimum
		//Aka which changes to which weights matter the most
		//returns a Vec<Vec<f64>> why?
		//		it's a 2D vector of the gradients, which basically tells the weight how much it 
		//		 was off by and whether to increase or decrease the weight
		//so this function computes the gradient of the loss function with respect to the
		//		 weights of the network
					//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//IF FUNCTION HAS _ IN FRONT OF IT IT MEANS IT IS UNUSED
		/*
		pub fn _backpropagate(&mut self, loss_derivative: f64) -> Vec<Vec<Vec<f64>>> {
			//vec![  vec![what_i_want_each_element_to_be;number_of_elements] ; number_of_vectors];
			//gradients   will mimic the WeightLayer but instead the numbers will represent how
			//			   far the weight is from where it thinks it should go.
			//why self.weights[0].data[0].len() as the number_of_elements?
			//		self.weights is a vector of WeightLayer struct. Goto line 27,13.
			//		self.weights[0] accesses the first WeightLayer. Goto line 27.
			//		self.weights[0].data is a 2D vector representing the weights themselves in
			//		 the WeightLayer. Goto line 16. Lines 14 and 15 just represent how to
			//		 index/count them.
			//		self.weights[0].data[0] accesses the first row of weights in the first WeightLayer
			//		self.weights[0].data[0].len() returns number of elements in this first row.
			//		 This represents the number of weights connected out from the first neuron
			//		 in the first layer.
			//why just the first neuron though?
			//		Because the neural network is fully connected. So each neuron in a row has a
			//		 weight to each neuron in the row after it. So, each neuron in 1 layer has
			//		 the same number of weights as each other neuron in that layer/row.
			//why self.weights.len() as the number of vectors?
			//		this represents the number of WeightLayers themselves.
			//let mut gradients = vec![vec![0.0; self.weights[0].data[0].len()]; self.weights.len()];

			let mut gradients = Vec::new();
			//So the issue we encountered before is that if we did this:
			//let mut gradients = vec![vec![0.0; self.weights[0].data[0].len()]; self.weights.len()];
			//Then		each layer of the gradients would be the exact same, so it wouldn't mimic
			//			 the weightsLayer which aren't perfectly uniform in each layer.
			//So in the for loop below we're going through each weight layer, 
			//			AND THEN we are copying the structure of this layer, except for the gradients
			//			 themselves which are set to 0 for now.
			//in hte code comments above let mut gradients ..., it goes into some more detail.
			//.push(layer_gradients) is basically like "append" whatever's in the ( ) to be a part of
			//			 gradients.
			for i in 0..self.weights.len() {
				let layer_gradients = vec![vec![0.0; self.weights[i].data[0].len()]; self.weights[i].data.len()];
				gradients.push(layer_gradients);
			}
		
			//....rev() this iterates it in reverse starting from the weights connecting to
			//		 the output layer going to the first input layer. The last layer is not
			//		 included because there are no weights extending FROM the output layer
			//why in reverse?
			//		Because the error of the output layer is calculated directly from the
			//		 loss function. then this error is propagated backwards to calculate 
			//		 the error of each preceding layer
			for i in (0..self.layers.len()).rev() {
				//self.layers[i].data refers to the outputs of the neurons in each layer,
				//		 aka the activations
				let activations = &self.layers[i].data;
				let weights = &self.weights[i].data;

				//outer j loop iterates over each neuron/"output of the neuron" in the current layer
				for j in 0..activations.len() {

					for k in 0..activations[j].len() {
						let activation_derivative = leaky_relu_derivative(activations[j][k]);
						gradients[i][j][k] = loss_derivative * activation_derivative * weights[j][k];
					}
				}
			}
			gradients
		}
		//once the gradients are established, we just go through the weights and update them as quickly
		//		 as the learning_rate allows us
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//IF FUNCTION HAS _ IN FRONT OF IT IT MEANS IT IS UNUSED
		pub fn _update_weights(&mut self, gradients: &Vec<Vec<Vec<f64>>>) {
			let learning_rate = 0.001;
			for i in 0..self.weights.len() {
				for j in 0..self.weights[i].data.len() {
					for k in 0..self.weights[i].data[j].len() {
						self.weights[i].data[j][k] -= learning_rate * gradients[i][j][k];
					}
				}
			}
		}
		//new because functions above didn't make sense. will code comment these later-------------.------------------------//
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//IF FUNCTION HAS _ IN FRONT OF IT IT MEANS IT IS UNUSED
		pub fn _backpropagatey(&mut self, loss_derivative: f64) -> Vec<Vec<Vec<f64>>> {
			//initializes a NEW empty VECtor to store the gradients
			let mut gradients = Vec::new();
			//for (i, layer) in self.layers.iter().enumerate().rev() {
			//		ITERates over self.layers in REVerse order and returns the i(index) and value (layer)
			//why reverse order?
			//		because that's how you find the gradients. you find the error starting from the output
			for (i, layer) in self.layers.iter().enumerate().rev() {
				//vec![  vec![what_i_want_each_element_to_be;number_of_elements] ; number_of_vectors];
				//layer.data[0]len()		so the data in each layer is a 2D vector and we want to
				//		 access the first row, well because that's how I structured it:
				//		 all the neurons in the layer are in the first row.
				//		So we can just find the length of the first row and find how many gradients
				//		 we need to have.
				//Then we want the big vector to be just however big the whole vector for the layer was
				//		That's the layer.data.len() 	part
				let mut layer_gradients = vec![vec![0.0; layer.data[0].len()]; layer.data.len()];
		
				for (j, neuron) in layer.data.iter().enumerate() {
					for (k, activation) in neuron.iter().enumerate() {
						let activation_derivative = leaky_relu_derivative(*activation);
						let weight = self.weights[i].data[j][k];
						layer_gradients[j][k] = loss_derivative * activation_derivative * weight;
					}
				}
		
				gradients.push(layer_gradients);
			}
			gradients.reverse();
			gradients
		}
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//IF FUNCTION HAS _ IN FRONT OF IT IT MEANS IT IS UNUSED
		pub fn _update_yweights(&mut self, gradients: &Vec<Vec<Vec<f64>>>) {
			let learning_rate = 0.001;
		
			for (i, layer) in self.weights.iter_mut().enumerate()  {
				for (j, neuron) in layer.data.iter_mut().enumerate() {
					for (k, weight) in neuron.iter_mut().enumerate() {
						*weight -= learning_rate * gradients[i][j][k];
					}
				}
			}
		}
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//IF FUNCTION HAS _ IN FRONT OF IT IT MEANS IT IS UNUSED
		pub fn _backpropagationy(&mut self, loss_derivative: &f64, current_q_value: &f64, current_q_value_index: &usize) {
			//the purpose of this function is to find the gradient (aka derivative)
			//		 of the loss funciton with respect to each weight.
			// der. loss (w/ respect to weights) = der. loss (w/ respect to output)  x  der. out (w/ respect to weights).
			//step 1:
			//		find the derivative of the loss function with respect to the output.
			//		 output = q_pred.
			//		 This is just the loss derivative
			//step 2:
			//		a. find the derivative of the output with respect to the weights
			//			i.  aka the derivative of activation function AT each weight connected
			//				 to q_value chosen.
			//				 why? 	because we only want to change the weights that
			//				 contributed to the q_value chosen
			//				 how?
			//				 a. find the weights connected to the current_q_value_index
			//					how?
			//					i. weights initialized like so where the letters are the
			//						 associated neurons that the weights are connected FROM
			//						 and the numbers are the neurons they are connected TO:
			//						 vec![
			//							vec![wa1, wa2, wa3, wa4, ...],
			//							vec![wb1, wb2, wb3, wb4, ...],
			//							vec![wc1, wc2, wc3, wc4, ...],
			//							...
			//						 ]
			//					ii. so if we wanted weights connected to index 3, we would iterate through each layer,
			//						 and then through the rows of the last layer
			//						 and add the 3rd column to our list until we iterated through every row
			//
			//
			//
			//
			//
			//we want a gradients vector because gradients are the slopes of the loss function with
			//		respect to each weight. My plan is to put the gradients into a vec<vec<vec<f64>>>
			//why?
			//		Most outer vec will serve as the encapsulater. The first inner vec will serve
			//		 as which WeightLayer it corresponds to. The most inner vectors will serve as
			//		 the rows whose elements are the gradients that correspond with the weights
			//		  in the exact same position
			let mut gradients = Vec::new();
				//to think about: how do I establish the gradients?
			let last_layer_to_contain_weights = self.layers.len() - 2;
				//FINALLY: gradient calculation for the weight connecting to the current_q_value_index
				//gradient is the product of three terms: the derivative of the loss function, 
				//		the derivative of the activation function at the output of the neuron connected *TO* the weight,
				//		and the output of the neuron that the weight connects *FROM*
				//what does this do?
				//it tells us how the loss function changes as the output of the neurons connected by the weight change.
				//		 And since we cant change the output of the neuron directly, we change the weights
				//leaky_relu_derivative(self.layers.last().unwrap().data[i][current_q_value_index])
				//		this is this finds the derivative of the activation function at: 
				//		 self.layers.last().unwrap().data[i][current_q_value_index]
				//self.layers.last().unwrap()		this is just going to the last layer because it's the output
				//									of the neuron connected *TO* the weight
				//...data[i][current_q_value_index]		so i is always 0 since my neurons are
				//										only in 1 row, so this still works,
				//										 and then it's going into 0th row and then going
				//										 to the index of the q value and getting the q value itself
				//										 and then getting applying the leaky_relu_derivative to it.
			let derivative_of_output_neuron = leaky_relu_derivative(self
													.layers
													.last()
													.unwrap()
													.data[0][*current_q_value_index]);
			//first we are going to go through each weight layer from the 2nd last one
			//why 2nd last?		output layer doesn't have weights coming *FROM* it
			for layer_index in (0..self.weights.len()).rev() {
				let weight_layer = &self.weights[layer_index];
				// Iterate over all weights connected to the current neuron
				//we dont actaully care about the weights themselves, but we just care about which
				//		 layer we're in and this helps us track it. What we actually care about is
				//		 what the weights connect to and from. and this is especially important in
				//		 the last weight layer because we only care about the weights connecting to
				//		 the output neuron
				for j in 0..weight_layer.data[0].len() {
					// Skip the calculation if layer_index is 0 because how can you index something at -1
					if layer_index > 0 {
						// Calculate the derivative of the activation function for the current neuron
						//why the if statement here?
						//		because if we're in the last weight layer, we want the derivative of
						//		 output to only be 1 value for the whole layer
						let derivative_of_neuron = 
							if layer_index == self.weights.len() - 1 {
								derivative_of_output_neuron
							} else {
								//why [0][j] ?
								//		the first [] represents which row you are indexing
								//		the secon [] represents which element you are getting from said row
								//And, all of my neurons are in the first row of their respective layers,
								//		 so no point of varying the first [] because then we wont be
								//		 accessing neurons. we will be accessing a new dimension that
								//		 could unleash the spirit realm
								//so [0][j] represents which neuron we're accessing
								//and layer_index represents which layer we're at presently
								leaky_relu_derivative(self.layers[layer_index].data[0][j])
							};
						// Calculate the gradient for the weight connecting neuron i to neuron j
						//gradient is the product of three terms: the derivative of the loss function, 
						//		the derivative of the activation function at the output of the neuron connected *TO* the weight,
						//		and the output of the neuron that the weight connects *FROM*
						//why [layer_index-1] ?
						//		the answer to that is in the answer to this:
						//		which part of the 3 part gradient calculation corresponds to
						//		 self.layers[layer_index - 1].data[0][j] ?
						//spoiler alert: -1 because it's the from neuron
						let gradient = loss_derivative * derivative_of_neuron * self.layers[layer_index - 1].data[0][j];
						gradients.push(gradient);
					}
				}
			}
			//so this for loop 
		}

		pub fn update_weightsy(&mut self, gradients: &Vec<f64>, learning_rate: f64) {
			// Iterate over all WeightLayers
			for layer_index in 0..self.weights.len() {
				let weight_layer = &mut self.weights[layer_index];
				// Iterate over all weights in the current layer
				for i in 0..weight_layer.data.len() {
					for j in 0..weight_layer.data[i].len() {
						// Update the weight by subtracting the gradient times the learning rate
						weight_layer.data[i][j] -= learning_rate * gradients[layer_index * i + j];
					}
				}
			}
		}
			//issues that I think are in the function below:
			//		1. we aren't going to the last weight layer in 
			//				for layer_index in (0..self.weights.len()).rev() {
			//			because it's 0..self not 0..=self
			//			this is an issue because we wont be able to access the last weight layer 
			//		number 1 rescinded because it's the length, not the index. so it does go to the last layer.
			//		
			//		2. We can't iterate over layer_index 0 because of 
			//				if layer_index > 0 {
			//			why is it there? Because layer_index - 1 in self.layers[layer_index - 1].data[0][j].
			//			this is needed because it corresponds to the from neuron. 
			//		number 2 rescinded because it will still iterate over the neurons in the 0th layer because of the layer_index-1.
			//			We dont need to make the 0th layer be the *TO* neuron because no weights coming in to 0th layer, duh.
			//		3. the *TO* and *FROM* neurons are always going to be in the same column:
			//			TO: leaky_relu_derivative(self.layers[layer_index].data[0][j])
			//		  FROM: self.layers[layer_index - 1].data[0][j]
			//			If you don't see it, the issue is in the .data[0][j]. 
			//			The *FROM* neuron should iterate over every neuron and calculate a value after every iteration before the *TO* neuron is iterated.
			//			What if I change output to i?
			//				wont work because it'll correspond incorrectly because i signifies change in input neuron.
			//			What if I change input to i and output stays j?
			//				wont work because i only iterates after j has finished looping through the columns.
			//				Actually will work as long as it's still hitting all the weights.
			//				 It doesn't matter if it's iterating the output neurons for every input instead of iterating the input neurons for every output neuron
			//				 if it's still hitting all the weights.
			// *FROM* neurons:
			//vec![
			//	vec![Na, Nb, Nc, Nd]
			//]
			//weights:
			//vec![
			//	vec![Wa1, Wa2, Wa3]
			//	vec![Wb1, Wb2, Wb3]
			//	vec![Wc1, Wc2, Wc3]
			//	vec![Wd1, Wd2, Wd3]
			//]
			// *TO* neurons:
			//vec![
			//	vec![N1, N2, N3]
			//]
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//THIS VERSION IS UNUSED
			//IF FUNCTION HAS _ IN FRONT OF IT IT MEANS IT IS UNUSED
		pub fn _backpropagation_v4(&mut self, loss_derivative: &f64, current_q_value: &f64, current_q_value_index: &usize) -> (Vec<f64>, Vec<(usize, usize, usize)>) {
			let mut gradients = Vec::new();
			let mut indices = Vec::new();
			let derivative_of_output_neuron = leaky_relu_derivative(self.layers.last().unwrap().data[0][*current_q_value_index]);
			// Iterate over all WeightLayers in reverse order
			for layer_index in (0..self.weights.len()).rev() {
				let weight_layer = &mut self.weights[layer_index];
				// Iterate over all rows in the current layer, how do I know this? because it's iterating over the LENgth of weight_layer, aka the number of rows
				for i in 0..weight_layer.data.len() {
					//iterate over all the weights in row i, how do I know this? because it's iterating over the LENgth of row i
					for j in 0..weight_layer.data[i].len() {
						// Skip the calculation if layer_index is 0
						if layer_index > 0 {
							// Calculate the derivative of the activation function for the current neuron
							let derivative_of_output_nimrod = 
								if layer_index == self.weights.len() - 1 {
									derivative_of_output_neuron
								} 
								else {
									//this is the output neuron's derivative
									leaky_relu_derivative(self.layers[layer_index].data[0][j])
								};
							// Calculate the gradient for the weight connecting neuron i to neuron j
							let gradient = loss_derivative * derivative_of_output_nimrod * self.layers[layer_index - 1].data[0][j];
							gradients.push(gradient);
							indices.push((layer_index, i, j));
						}
					}
				}
			}
			(gradients, indices)
		}
		//after changes described in the code comments above. the only thing changed is the j in : self.layers[layer_index - 1].data[0][j];
		//	changed to i
		//The concern I have now:
		//		1. Because I said i to be input neuron and j to be output, does that change how it updates the weights?
		//			I think it will correspond correctly because it's still increasing the gradient_index even if it's not iterating i, but only iterating j at the moment
		//		2. what about iterating all the weights, even the ones it didn't update
		//			I think I will need to change the j index to k and iterate k at the end of the j loop and make it k = index of output neuron before the j for loop begins
		//			 and then set k = index of output neuron if I use derivative_of_output_neuron

		//THIS FUNCTION IS UNUSED
		//THIS FUNCTION IS UNUSED
		//THIS FUNCTION IS UNUSED
		//THIS FUNCTION IS UNUSED
		//IF FUNCTION HAS _ IN FRONT OF IT IT MEANS IT IS UNUSED
		pub fn _backpropagation_v5(&mut self, loss_derivative: &f64, current_q_value: &f64, current_q_value_index: &usize) -> (Vec<f64>, Vec<(usize, usize, usize)>) {
						//the purpose of this function is to find the gradient (aka derivative)
			//		 of the loss funciton with respect to each weight.
			// der. loss (w/ respect to weights) = der. loss (w/ respect to output)  x  der. out (w/ respect to weights).
			//step 1:
			//		find the derivative of the loss function with respect to the output.
			//		 output = q_pred.
			//		 This is just the loss derivative
			//step 2:
			//		a. find the derivative of the output with respect to the weights
			//			i.  aka the derivative of activation function AT each weight connected
			//				 to q_value chosen.
			//				 why? 	because we only want to change the weights that
			//				 contributed to the q_value chosen
			//				 how?
			//				 a. find the weights connected to the current_q_value_index
			//					how?
			//					i. weights initialized like so where the letters are the
			//						 associated neurons that the weights are connected FROM
			//						 and the numbers are the neurons they are connected TO:
			//						 vec![
			//							vec![wa1, wa2, wa3, wa4, ...],
			//							vec![wb1, wb2, wb3, wb4, ...],
			//							vec![wc1, wc2, wc3, wc4, ...],
			//							...
			//						 ]
			//					ii. so if we wanted weights connected to index 3, we would iterate through each layer,
			//						 and then through the rows of the last layer
			//						 and add the 3rd column to our list until we iterated through every row
			//we want a gradients vector because gradients are the slopes of the loss function with
			//		respect to each weight. My plan is to put the gradients into a vec<vec<vec<f64>>>
			//why?
			//		Most outer vec will serve as the encapsulater. The first inner vec will serve
			//		 as which WeightLayer it corresponds to. The most inner vectors will serve as
			//		 the rows whose elements are the gradients that correspond with the weights
			//		  in the exact same position
			let mut gradients = Vec::new();
			//why indices? because I need to track what I'm changing for the update_weights function
			let mut indices = Vec::new();
				//gradient is the product of three terms: 1. the derivative of the loss function, 
				//		2. the derivative of the activation function at the output of the neuron connected *TO* the weight,
				//		3. and the output of the neuron that the weight connects *FROM*
				//what does this do?
				//it tells us how the loss function changes as the output of the neurons connected by the weight change.
				//		 And since we cant change the output of the neuron directly, we change the weights
				//leaky_relu_derivative(self.layers.last().unwrap().data[i][current_q_value_index])
				//		this is this finds the derivative of the activation function at: 
				//		 self.layers.last().unwrap().data[i][current_q_value_index]
				//self.layers.last().unwrap()		this is just going to the last layer because it's the output
				//									of the neuron connected *TO* the weight
				//...data[i][current_q_value_index]		so i is always 0 since my neurons are
				//										only in 1 row, so this still works,
				//										 and then it's going into 0th row and then going
				//										 to the index of the q value and getting the q value itself
				//										 and then getting applying the leaky_relu_derivative to it.
			let derivative_of_output_neuron = leaky_relu_derivative(self.layers.last().unwrap().data[0][*current_q_value_index]);
			// Iterate over all WeightLayers in reverse order
			for layer_index in (0..self.weights.len()).rev() {
				let weight_layer = &mut self.weights[layer_index];
				// Iterates over all rows in the current layer, how do I know this? because it's iterating over the LENgth of weight_layer, aka the number of rows
				//is it fine to be 0..weight and not 0..=weight? yes because it's the index so the index starts at 0 and ends 1 before length which is perfect.
				//	Same goes for other for loop too
				for i in 0..weight_layer.data.len() {
					//iterates over all the weights in row i, how do I know this? because it's iterating over the LENgth of row i
					for j in 0..weight_layer.data[i].len() {
						// Skip the calculation if layer_index is 0
						//why? Because it's already calculated all the From neurons from layer_index 0 corresponding to the layer after it.
						//how do I know?
						//		TO neuron derivative (look at layer_index): leaky_relu_derivative(self.layers[layer_index].data[0][j])
						//		From neuron derivative:                                       self.layers[layer_index - 1].data[0][i]
						if layer_index > 0 {
							// Calculate the derivative of the activation function for the current neuron
							let derivative_of_to_neuron = 
								if layer_index == self.weights.len() - 1 {
									derivative_of_output_neuron
								} 
								else {
									//this is the output neuron's derivative
									leaky_relu_derivative(self.layers[layer_index].data[0][j])
								};
							// Calculate the gradient for the weight connecting neuron i to neuron j
							let gradient = loss_derivative * derivative_of_to_neuron * self.layers[layer_index - 1].data[0][i];
							gradients.push(gradient);
							indices.push((layer_index, i, j));
						}
					}
				}
			}
			(gradients, indices)
		}
	*/























		/*
		//Most updated version
		//dont think it will be accessed at same time as feed_forward, so I will not add a mutex
		//also the function, at least I dont think, doesnt directly access the input neurons, just its weights
		pub fn el_backpropagation(&mut self, current_q_value_index: &usize,
			 current_q_value: &f64, target_q_value: &f64) -> (Vec<f64>, Vec<(usize, usize, usize)>) {
			//the purpose of this function is to find the gradient (aka derivative)
			//		 of the loss funciton with respect to each weight.
			// der. loss (w/ respect to weights) = der. loss (w/ respect to output)  x  der. out (w/ respect to weights).
			//step 1:
			//		find the derivative of the loss function with respect to the output.
			//		 output = q_pred.
			//		 This is just the loss derivative
			//step 2:
			//		a. find the derivative of the output with respect to the weights
			//			i.  aka the derivative of activation function AT each weight connected
			//				 to q_value chosen.
			//				 why? 	because we only want to change the weights that
			//				 contributed to the q_value chosen
			//				 how?
			//				 a. find the weights connected to the current_q_value_index
			//					how?
			//					i. weights initialized like so where the letters are the
			//						 associated neurons that the weights are connected FROM
			//						 and the numbers are the neurons they are connected TO:
			//						 vec![
			//							vec![wa1, wa2, wa3, wa4, ...],
			//							vec![wb1, wb2, wb3, wb4, ...],
			//							vec![wc1, wc2, wc3, wc4, ...],
			//							...
			//						 ]
			//					ii. so if we wanted weights connected to index 3, we would iterate through each layer,
			//						 and then through the rows of the last layer
			//						 and add the 3rd column to our list until we iterated through every row
			//
			//
			//
			//
			//
			//we want a gradients vector because gradients are the slopes of the loss function with
			//		respect to each weight. My plan is to put the gradients into a vec<vec<vec<f64>>>
			//why?
			//		Most outer vec will serve as the encapsulater. The first inner vec will serve
			//		 as which WeightLayer it corresponds to. The most inner vectors will serve as
			//		 the rows whose elements are the gradients that correspond with the weights
			//		  in the exact same position
			let mut gradients = Vec::new();
			//why indices? because I need to track what I'm changing for the update_weights function
			let mut indices = Vec::new();
			let loss_derivative = calculate_loss_derivative(&current_q_value, &target_q_value);
			//gradient is the product of three terms: 1. the derivative of the loss function, 
			//		2. the derivative of the activation function at the output of the neuron connected *TO* the weight,
			//		3. and the output of the neuron that the weight connects *FROM*
			//what does this do?
			//it tells us how the loss function changes as the output of the neurons connected by the weight change.
			//		 And since we cant change the output of the neuron directly, we change the weights
			//leaky_relu_derivative(self.layers.last().unwrap().data[i][current_q_value_index])
			//		this is this finds the derivative of the activation function at: 
			//		 self.layers.last().unwrap().data[i][current_q_value_index]
			//self.layers.last().unwrap()		this is just going to the last layer because it's the output
			//									of the neuron connected *TO* the weight
			//...data[i][current_q_value_index]		so i is always 0 since my neurons are
			//										only in 1 row, so this still works,
			//										 and then it's going into 0th row and then going
			//										 to the index of the q value and getting the q value itself
			//										 and then getting applying the leaky_relu_derivative to it.
			let derivative_of_output_neuron = leaky_relu_derivative(self.layers.last().unwrap().data[0][*current_q_value_index]);
			let mut derivative_of_to_neuron: Option<f64>;
			// Iterate over all WeightLayers in reverse order
		//01/13/24 - got index out of bounds error so removed:
			//for layer_index in (0..self.weights.len()).rev() {
		//01/13/24 - added:
			for layer_index in (0..self.weights.len()).rev() {
				let weight_layer = &self.weights[layer_index];
				// Iterates over all rows in the current layer, how do I know this? because it's iterating over the LENgth of weight_layer, aka the number of rows
				//is it fine to be 0..weight and not 0..=weight? yes because it's the index so the index starts at 0 and ends 1 before length which is perfect.
				//	Same goes for other for loop too
				for i in 0..weight_layer.data.len() {
					//iterates over all the weights in row i, how do I know this? because it's iterating over the LENgth of row i
					let mut k: usize = 0;
					for j in 0..weight_layer.data[i].len() {
						// Skip the calculation if layer_index is 0
						//why? Because it's already calculated all the From neurons from layer_index 0 corresponding to the layer after it.
						//how do I know?
						//		TO neuron derivative (look at layer_index): leaky_relu_derivative(self.layers[layer_index].data[0][j])
						//		From neuron derivative:                                       self.layers[layer_index - 1].data[0][i]
						if layer_index > 0 {
							// Calculate the derivative of the activation function for the current neuron
								if layer_index == &self.weights.len() - 1 {
									derivative_of_to_neuron = Some(derivative_of_output_neuron);
									k = *current_q_value_index;
								} 
								else {
									//this is the output neuron's derivative
									derivative_of_to_neuron = Some(leaky_relu_derivative(self.layers[layer_index].data[0][j]));
								};
							// Calculate the gradient for the weight connecting neuron i to neuron j
							if let Some(derivativeoftoneuron) = derivative_of_to_neuron{
								let gradient = loss_derivative * derivativeoftoneuron * self.layers[layer_index - 1].data[0][i];
								gradients.push(gradient);
								indices.push((layer_index, i, k));
								k+=1;
							}
						}
					}
				}
			}
			(gradients, indices)
		}
		*/

		/*
		pub fn el_backpropagation(&mut self, current_q_value_index: &usize,
			current_q_value: &f64, target_q_value: &f64) -> (Vec<f64>, Vec<(usize, usize, usize)>) {

		   let mut gradients = Vec::new();
		   let mut indices = Vec::new();
		   let loss_derivative = calculate_loss_derivative(&current_q_value, &target_q_value);

		   let derivative_of_output_neuron = leaky_relu_derivative(self.layers.last().unwrap().data[0][*current_q_value_index]);
		   let mut derivative_of_to_neuron: Option<f64>;

		   for layer_index in (0..self.weights.len()-1).rev() {
			   if layer_index == 0 {
				   continue;
			   }
			   let weight_layer = &self.weights[layer_index];

			   for i in 0..self.layers[layer_index - 1].data[0].len() {
				   let mut k: usize = 0;
				   for j in 0..weight_layer.data[i].len() {

					   if layer_index > 0 {
						   if layer_index == &self.weights.len() - 1 {
							   derivative_of_to_neuron = Some(derivative_of_output_neuron);
							   k = *current_q_value_index;
						   } 
						   else {
							   derivative_of_to_neuron = Some(leaky_relu_derivative(self.layers[layer_index].data[0][j]));
						   };
						   if let Some(derivativeoftoneuron) = derivative_of_to_neuron{
							   let gradient = loss_derivative * derivativeoftoneuron * self.layers[layer_index - 1].data[0][i];
							   gradients.push(gradient);
							   indices.push((layer_index, i, k));
							   k+=1;
						   }
					   }
				   }
			   }
		   }
		   (gradients, indices)
	   }
	   */

	   //this should address the first layer not being updated
	   /*
	    pub fn el_backpropagation(&mut self, current_q_value_index: &usize,
			current_q_value: &f64, target_q_value: &f64) -> (Vec<f64>, Vec<(usize, usize, usize)>) {

			let mut gradients = Vec::new();
			let mut indices = Vec::new();
			let loss_derivative = calculate_loss_derivative(&current_q_value, &target_q_value);

			let derivative_of_output_neuron = leaky_relu_derivative(self.layers.last().unwrap().data[0][*current_q_value_index]);
			let mut derivative_of_to_neuron: Option<f64>;

			for layer_index in (0..self.weights.len()).rev() {
				let weight_layer = &self.weights[layer_index];

				for i in 0..self.layers[layer_index].data[0].len() {
					let mut k: usize = 0;
					for j in 0..weight_layer.data[i].len() {

						if layer_index == self.weights.len() - 1 {
							derivative_of_to_neuron = Some(derivative_of_output_neuron);
							k = *current_q_value_index;
						} 
						else {
							derivative_of_to_neuron = Some(leaky_relu_derivative(self.layers[layer_index + 1].data[0][j]));
						};
						if let Some(derivativeoftoneuron) = derivative_of_to_neuron{
							let gradient = loss_derivative * derivativeoftoneuron * self.layers[layer_index].data[0][i];
							gradients.push(gradient);
							indices.push((layer_index, i, k));
							k+=1;
						}
					}
				}
			}
			(gradients, indices)
   		}
		*/

		//this should address the lower rows not being updated in conjunction with first layer not being updated
		/*
		pub fn el_backpropagation(&mut self, current_q_value_index: &usize,
			current_q_value: &f64, target_q_value: &f64) -> (Vec<f64>, Vec<(usize, usize, usize)>) {

			let mut gradients = Vec::new();
			let mut indices = Vec::new();
			let loss_derivative = calculate_loss_derivative(&current_q_value, &target_q_value);

			let derivative_of_output_neuron = leaky_relu_derivative(self.layers.last().unwrap().data[0][*current_q_value_index]);
			let mut derivative_of_to_neuron: Option<f64>;

			for layer_index in (0..self.weights.len()).rev() {
				let weight_layer = &self.weights[layer_index];

				for i in 0..weight_layer.data.len() { // Iterate over the neurons in the current layer
					let mut k: usize = 0;
					for j in 0..weight_layer.data[i].len() {

						if layer_index == self.weights.len() - 1 {
							derivative_of_to_neuron = Some(derivative_of_output_neuron);
							k = *current_q_value_index;
						} 
						else {
							derivative_of_to_neuron = Some(leaky_relu_derivative(self.layers[layer_index + 1].data[0][j]));
						};
						if let Some(derivativeoftoneuron) = derivative_of_to_neuron{
							let gradient = loss_derivative * derivativeoftoneuron * self.layers[layer_index].data[0][i];
							gradients.push(gradient);
							indices.push((layer_index, i, k));
							k+=1;
						}
					}
				}
			}
			(gradients, indices)
		}
		*/


	//01/19/24 - added
	pub fn el_backpropagation(&mut self, current_q_value_index: &usize,
		current_q_value: &f64, target_q_value: &f64) {
		
		//02/03/24 - added lambda.
		//common values are between 0 and 0.1.
		//let lambda = 0.01;
		//initialize gradient layer to 0
		//iterate through weight layer in reverse and apply formula to calculate gradient

		//03/13/24 - formulas for Huber Loss function. this could help in convergence
			//03/14/24 - delta = 2.0
			let delta = 1.0;
			let td_error = current_q_value - target_q_value;
			let mut loss_derivative;
			if td_error.abs() <= delta {
				loss_derivative = td_error;
			}
			else {
				loss_derivative = delta * td_error.signum();
			}
			//03/15/24 - from c-dqn paper. adding it. seeing if it works. page 7. equation 12
				let lmsbe = td_error.powi(2);

				if lmsbe > loss_derivative {
					loss_derivative = lmsbe;
				}


		//initializes gradient_layer's data to 0
		for gradient_layer in &mut self.gradients.layers {
			for row in &mut gradient_layer.data {
				for gradient in row.iter_mut() {
					*gradient = 0.0;
				}
			}
		}

		//setting up formulas
		//03/13/24 - removed:
		// let loss_derivative = calculate_loss_derivative(&current_q_value, &target_q_value);
		let derivative_of_output_neuron = leaky_relu_derivative(self.layers.last().unwrap().data[0][*current_q_value_index]);
		//let mut derivative_of_to_neuron: Option<f64>;


		//setting up readability:
		let last_weight_layer = &self.weights.last().unwrap();
		let last_gradient_layer = self.gradients.layers.last_mut().unwrap();
		let second_last_neuron_layer = &self.layers[self.layers.len()-2];



		// First for-loop: Calculate gradients for weights connecting to chosen
		//		output neuron in the last weight layer
		for (j, _weight) in last_weight_layer.data.iter().enumerate() {
			// Calculate gradient for weight[j][chosen_output_neuron_index]
				//calculate derivative of to neuron
					//this is  just derivative of output neuron

				//calculate output of from neuron
					let output_of_from_neuron = &second_last_neuron_layer.data[0][j];
			
			last_gradient_layer.data[j][*current_q_value_index] = loss_derivative*derivative_of_output_neuron*output_of_from_neuron;

			//03/12/24 - added:
			    // Apply gradient cap
				//03/14/24 - changed to 5
				if last_gradient_layer.data[j][*current_q_value_index] > 5.0 {
					last_gradient_layer.data[j][*current_q_value_index] = 5.0;
				}
				else if last_gradient_layer.data[j][*current_q_value_index] < -5.0 {
					last_gradient_layer.data[j][*current_q_value_index] = -5.0;
				}
		}

		// Second for-loop: Calculate gradients for all other weights
		for (i, weight_layer) in self.weights.iter().enumerate() {
			if i != self.weights.len() - 1 { // Skip the last weight layer

				//for readability
				let gradient_layer = &mut self.gradients.layers[i];

				for (j, weight_row) in weight_layer.data.iter().enumerate() {
					for (k, _weight) in weight_row.iter().enumerate() {
						// Calculate gradient for weight[j][i]
						//calculate derivative of to neuron
							//i+1 because it's the NEXT neuron's layer
							//[0] because there's only 1 row
							//[k]? why not [j]? If the sizes of the neuron layers were the same,
							//		then j could work. However, we need to do [k] because the
							//		columns of the weight layer correspond to what neuron the
							//		weight attaches TO.
							//remember:
								//previous neuron layer
								// vec![
								//	vec![Na, Nb, Nc, ...]
								// ]
								//weights:
								// vec![   [k], [k], [k], [k]
								//[j] vec![wa1, wa2, wa3, wa4, ...],
								//[j] vec![wb1, wb2, wb3, wb4, ...],
								//[j] vec![wc1, wc2, wc3, wc4, ...],
								//	...
								// ]
								//next neuron layer
								// vec![
								//	 vec![N1, N2, N3, N4,...]
								// ]

							let derivative_of_to_neuron = leaky_relu_derivative(self.layers[i+1].data[0][k]);
						//calculate output of from neuron
							let output_of_from_neuron = &self.layers[i].data[0][j];

							gradient_layer.data[j][k] = loss_derivative*derivative_of_to_neuron*output_of_from_neuron;
						//01/28/24 - added:. 01/29/24 - changed to 0.2 instead of 1.0
						//02/02/24 - changed from 0.2 to 0.1
						//02/28/24 - changed from 0.1 to 0.001 then to 0.00001 then .000_0001
						//02/29/24 - changed back to 0.1
						//03/10/24 - removed gradient cap.
						//03/14/24 - change to 5.
						//	uncomment if you want gradient cap
							if gradient_layer.data[j][k] > 5.0 {
								gradient_layer.data[j][k] = 5.0;
							}
							else if gradient_layer.data[j][k] < -5.0 {
								gradient_layer.data[j][k] = -5.0;
							} 
					}
				}
			}
		}
	}
	//01/19/24 - added
	pub fn el_update_weights(&mut self, learning_rate: &f64) {
		for (i, weight_layer) in self.weights.iter_mut().enumerate() {
			let gradient_layer = &self.gradients.layers[i];
			for (j, weight_row) in weight_layer.data.iter_mut().enumerate() {
				for (k, weight) in weight_row.iter_mut().enumerate() {
					let gradient = gradient_layer.data[j][k];
					*weight -= learning_rate * gradient;
				}
			}
		}
	}





//03/10/24 - added:
	//03/13/24 - added i 
	pub fn save_all_gradients(&self, i: &usize) {
		let base_path = "D:\\Downloads\\PxOmni\\zgradients";
		//03/14/24 - added. this now allows for the files to not be so large and
		//		take a while to open up:
			let file_number = i / 1000;
			let file_name = format!("all_gradients{}.csv", file_number);
			let file_path = Path::new(base_path).join(file_name);
		//03/14/24 - removed:
			//let file_path = Path::new(base_path).join("all_gradients.csv");
		let mut attempts = 0;
		loop {
			attempts+=1;
			match fs::OpenOptions::new().write(true).append(true).create(true).open(&file_path) {
				Ok(mut file) => {
					for (layer_index, gradient_layer) in self.gradients.layers.iter().enumerate() {
						writeln!(file, "Layer {}", layer_index + 1).unwrap();
						for (row_index, sub_vec) in gradient_layer.data.iter().enumerate() {
							let line = sub_vec.iter()
								.map(|&value| value.to_string())
								.collect::<Vec<String>>()
								.join(",");
							match writeln!(file, "Row {}: {}", row_index + 1, line) {
								Ok(_) => (),
								Err(e) => {
									panic!("save_all_gradients: Failed to write to file after 3 attempts. Error: {}", e);
								}
							}
						}
						writeln!(file, "\n").unwrap(); // Two new lines after each layer
					}
					writeln!(file, "finished iteration {}", &i).unwrap();
					//if it was Ok, break out of loop when done
					break;
				},
				Err(e) => {
					attempts+=1;
					log::error!("save_all_gradients: Failed to open file.
					attempt #: {}\nError: {}", attempts, e);
					if attempts > 3 {
						panic!("save_all_gradients: Failed to open file Error: {}", e);
					}
				}
			}
		} 
	}
//03/12/24 - added:
	//03/13/24 - added i 
	pub fn save_all_weights(&self, i: &usize) {
		let base_path = "D:\\Downloads\\PxOmni\\zweights";
		//03/14/24 - added. this now allows for the files to not be so large and
		//		take a while to open up:
			let file_number = i / 1000;
			let file_name = format!("all_weights{}.csv", file_number);
			let file_path = Path::new(base_path).join(file_name);
		//03/14/24 - removed:	
			//let file_path = Path::new(base_path).join("all_weights.csv");
		let mut attempts = 0;
		
		loop {
			attempts+=1;
			match fs::OpenOptions::new().write(true).append(true).create(true).open(&file_path) {
				Ok(mut file) => {
					for (layer_index, weight_layer) in self.weights.iter().enumerate() {
						writeln!(file, "Layer {}", layer_index + 1).unwrap();
						for (row_index, sub_vec) in weight_layer.data.iter().enumerate() {
							let line = sub_vec.iter()
								.map(|&value| value.to_string())
								.collect::<Vec<String>>()
								.join(",");
							match writeln!(file, "Row {}: {}", row_index + 1, line) {
								Ok(_) => (),
								Err(e) => {
									panic!("save_all_weights: Failed to write to file after 3 attempts. Error: {}", e);
								}
							}
						}
						writeln!(file, "\n").unwrap(); // Two new lines after each layer
					}
					writeln!(file, "finished iteration {}", &i).unwrap();
					//if it was Ok, break out of loop when done
					break;
				},
				Err(e) => {
					attempts+=1;
					log::error!("save_all_weights: Failed to open file.
					attempt #: {}\nError: {}", attempts, e);
					if attempts > 3 {
						panic!("save_all_weights: Failed to open file Error: {}", e);
					}
				}
			}
		} 
	}













	//01/19/24 - removed since created new back propagation function
	/*
	pub fn el_update_weights(&mut self, gradients: &Vec<f64>, indices: &Vec<(usize, usize, usize)>) {
			// Iterate over all gradients and their corresponding indices
			let learning_rate = 0.001;
			for (gradient_index, (layer_index, i, j)) in indices.iter().enumerate() {
				// Print the weight before update
				println!("Weight before update for layer {}, neuron {}: {}", layer_index, i, self.weights[*layer_index].data[*i][*j]);
				// Update the corresponding weight by subtracting the gradient times the learning rate
				self.weights[*layer_index].data[*i][*j] -= learning_rate * gradients[gradient_index];
				// Print the weight after update
				println!("Weight after update for layer {}, neuron {}: {}", layer_index, i, self.weights[*layer_index].data[*i][*j]);
			}
		}

	*/


		//how to save my neural network. Aka, how do I keep all the biases and weights stored 
		//		so that I can easily load the neural network to get new inputs and begin running it
		//there seem to be 2 methods to saving the neural network. Serialization and checkpointing.
		//Serialization:
		//		translating a data structure or object state into a format that can be stored and
		//		 reconstructed later.
		//Checkpointing:
		//		saves a snapshot of the application's sate so that applications can restart from 
		//		that point.
		//Difference:
		//		honestly not sure. They both save the state. I think checkpointing is just used
		//		 for fault tolerance and serialization is a more general term for saving and
		//		 loading a state.







		//save version 1. weird and complicated
		//had to comment them out because they had too many errors
		//pub fn _savev1(&self) -> std::path::PathBuf {
		//	// Create path for saving file
		//	let path = std::path::PathBuf::from("model.h5");
		//	// Create file with write mode
		//	let mut file = std::fs::File::create(&path).unwrap();
		//	// Serialize neural network object into binary format
		//	serde_json::to_writer(&mut file).unwrap();
		//	// Return path
		//	path
		//}
		//load version 1. weird and very complicated. Let's see if we can simplify it
		//pub fn _loadv1(&self) -> Self {
		//	// Load state from file
		//	let path = std::path::PathBuf::from("model.h5");
		//	// Open file with read mode
		//	let mut file = std::fs::File::open(&path).unwrap();
		//	// Deserialize binary format into neural network object
		//	let mut data = Vec::<Vec<f64>>::new();
		//	serde_json::from_reader(&mut file).unwrap().into_iter().map(|layer| {
		//		vec![(layer.rows * layer.columns), layer.data]
		//			.into_iter()
		//			.map(|row| row.iter().map(|x| x.to_f64()).collect::<Vec<_>>())
		//			.collect()
		//	}).collect();
		//	// Deserialize binary format into neural network object
		//	let mut model = NeuralNetwork {
		//		layers: vec![],
		//		weights: vec![],
		//		biases: vec![],
		//	};	
		//	// Deserialize each layer into model.layers vector
		//	for (i, row) in data.iter().enumerate() {
		//		model.layers[i].rows = row.len();
		//		model.layers[i].columns = row[0].len();
		//		model.layers[i].data = row.iter().cloned().collect::<Vec<_>>();
		//	}
		//	// Deserialize each weight into model.weights vector
		//	for (i, row) in data.iter().enumerate() {
		//		model.weights[i].rows = row.len();
		//		model.weights[i].columns = row[0].len();
		//		model.weights[i].data = row.iter().cloned().collect::<Vec<_>>();
		//	}
		//	// Deserialize each bias into model.biases vector
		//	for (i, row) in data.iter().enumerate() {
		//		model.biases[i].rows = row.len();
		//		model.biases[i].columns = 1;
		//		model.biases[i].data = row.iter().cloned().collect::<Vec<_>>();
		//	}
		//	return model;
		//}





		//this seems to save the entire NeuralNetwork, which Im not sure I want. If I understand
		//		 correctly, to save the neural network is to save all the functions too. But I
		//		 just want to save the structs and their corresponding data

		//very simple. new versions
		//why std::io::Restul<()>
		//		when you see Result<()> that means the function will either return
		//		 Ok(()) if no errors and Err with error information if errors.
		//Ok(())
		//		it's like void, there's nothing actually returned
		pub fn save_v2(&self) -> std::io::Result<()> {
			let base_path = "D:\\Downloads\\PxOmni\\rust_save_states";
			//this is used to create the timestamp. it doesnt represent the time today, 
			//		it represents time in ms since Unix epoch
			let now = Utc::now();
			let timestamp = now.timestamp_millis().to_string();
			//this adds the timestamp to the file path so that each file is different
			let file_path = Path::new(base_path).join(timestamp);
			//this literally creates the file. the ? allows the line of code
			//		 to return early if an error was encountered anywhere in
			//		 the line it's in.
			let file = File::create(file_path)?;
			//this creates the writer so that the next line of code can actually write to the file
			//why BufWriter? because it means less writing calls and improves performance.
			//how does it improve performance and less writing calls?
			//		Because no buffer means it writing small pieces of data one at a time.
			//this part doesn't actually write yet. It makes the buffer first, so it can write to the file
			let writer = BufWriter::new(file);
			//this is the actual writer. and it writes the stuff in JSON format to the file. 
			serde_json::to_writer(writer, &self)?;
			Ok(())
		}
		

		
		pub fn load(path: &str) -> std::io::Result<Self> {
			let file = File::open(path)?;
			let reader = BufReader::new(file);
			let network = serde_json::from_reader(reader)?;
			Ok(network)
		}






		//12/27/23 - 
		//		by this point I have to create a training environment to integrate with the neural network.
		//what parts will be faked:
		//		the time for the order
		//		the actual order
		//		the money I  have
		//
		//what parts will not be faked:
		//		the input data
		//		the calculations of money: how much I lost and gained in a transaction, therefore how much shit costs at time of transaction
		//		how the money relates to fees
		//				
		//first smallest step:
		//		create new functions in action_functions.rs that fake shit. so it will get all the data that it needs to get and then output the necessary data.

		//01/09/24 - 
		//		by this point I have created almost all of the sand box functions. I also had to add in XLM. This came with many many changes. 
		//		I then needed to decide if I would have a mutex. I have decided not to since it isn't multithreaded and since no inputs will be changed at the same
		//		time.


		//01/09/24 - 
		//		do I need to make it so there's no mutex until after every node has been updated
		//		 	because then by that point I will be accessing it to do a feed-forward.
		//			And I can't do a feed-forward while my inputs are being changed. Then I'll get
		//		 	the wrong results.
		//		Then do I need to make it so can't do another feed-forward until after I have
		//			established the state, action, reward, next_state.





















		//added 01/19/24:
		pub fn compare_weights(&self, old_weights: &Vec<WeightLayer>, layer_index: usize) {
			let weight_layer = &self.weights[layer_index];
			let old_weight_layer = &old_weights[layer_index];
		
			for (j, weight_row) in weight_layer.data.iter().enumerate() {
				for (k, weight) in weight_row.iter().enumerate() {
					let old_weight = old_weight_layer.data[j][k];
					if weight != &old_weight {
						println!("Weight at layer {}, row {}, column {} changed from {} to {}", layer_index, j, k, old_weight, weight);
					}
				}
			}
		}
		//added 01/19/24:
		pub fn print_weight_layer_rows(&self, layer_index: usize) {
			if layer_index < self.weights.len() {
				println!("Weight layer {} has {} rows", layer_index, self.weights[layer_index].rows);
			} else {
				println!("Invalid layer index");
			}
		}
		//added 01/19/24:
		pub fn print_gradients_for_layer_and_row(&self, layer_index: usize, row_index: usize) {
			if layer_index < self.gradients.layers.len() {
				if row_index < self.gradients.layers[layer_index].rows {
					println!("Gradients for row {} in layer {} are {:?}", row_index, layer_index, self.gradients.layers[layer_index].data[row_index]);
				} else {
					println!("Invalid row index");
				}
			} else {
				println!("Invalid layer index");
			}
		}
		//01/24/24 - added because got error in explor/exploit function saying
		//		index of larget q value was never initialized.
		//		I think this is only possible if feed_forward didnt feed
		//		properly. last layer wasn't empty
		pub fn print_last_network_layer(&self) {
			if let Some(last_layer) = self.layers.last() {
				println!("last network layer information:");
				println!("Rows: {}", last_layer.rows);
				println!("Columns: {}", last_layer.columns);
				println!("Data: {:?}", last_layer.data);
				if let Some(last_column) = last_layer.data.last() {
					println!("Index of last column: {}", last_layer.data.len() - 1);
					println!("Last column data: {:?}", last_column);
				}
				else {
					panic!("last_layer.data.last did not exist.
					 use data print statement above to debug this ");
				}
			} else {
				panic!("No layers in the network");
			}
		}
		
		pub fn print_input_layer(&self) {
			println!("printing first input layer:\n");
			if let Some(first_layer) = self.layers.first() {
				println!("Data: {:?}", first_layer.data);
			}
			else {
				panic!("self.layers.first did not exist. 
				So no layers in network");
			}
		}
		//02/28/24 - added:
		pub fn save_last_network_layer(&self, i: &usize) {
			let mut attempts = 0;
			loop {
				attempts += 1;
				if let Some(last_layer) = self.layers.last() {
					let base_path = "D:\\Downloads\\PxOmni\\rust_neural_network_q_values";
					let file_path = Path::new(base_path).join("last_network_layer.txt");
					match fs::OpenOptions::new().write(true).append(true).create(true).open(file_path) {
						Ok(mut file) => {
							match writeln!(file, "last network layer information:\titeration: {}\tColumns: {}\nData: {:?}"
								, &i, last_layer.columns, last_layer.data) {
		//03/06/24 - removed:

		// 						Ok(_) => break,
		// 						Err(e) => {
		// 							log::error!("save_last_network_layer: Failed to write to file after {} attempts. Error: {}"
		// 							, &i, e);
		// 							if attempts > 3 {
		// 								panic!("save_last_network_layer: Failed to write to file after 3 attempts. Error: {}", e);
		// 							}
		// 							continue;
		// 						}
		// 					}
		// 				},
		// 				Err(e) => {
		// 					log::error!("save_last_network_layer: Failed to open file after {} attempts. Error: {}"
		// 					, &i, e);
		// 					if attempts > 3 {
		// 						panic!("save_last_network_layer: Failed to open file after 3 attempts. Error: {}", e);
		// 					}
		// 					continue;
		// 				}
		// 			}
		// 		} else {
		// 			panic!("save_last_network_layer: No layers in the network");
		// 		}
		// 	}
		// }
		//03/06/24 - added:
								Ok(_) => {
									// Calculate the sum and average of the last_layer data by iterating through both dimensions
									let mut sum = 0.0;
									let mut count = 0;
									for sub_vec in &last_layer.data {
										for &value in sub_vec {
											sum += value;
											count += 1;
										}
									}
									let avg = sum / (count as f64);
									// Print the average to the file
									match writeln!(file, "Average: {}", avg) {
										Ok(_) => break,
										Err(e) => {
											log::error!("save_last_network_layer: Failed to write average to file after {} attempts. Error: {}"
											, &i, e);
											if attempts > 3 {
												panic!("save_last_network_layer: Failed to write average to file after 3 attempts. Error: {}", e);
											}
											continue;
										}
									}
								},
								Err(e) => {
									log::error!("save_last_network_layer: Failed to write to file after {} attempts. Error: {}"
									, &i, e);
									if attempts > 3 {
										panic!("save_last_network_layer: Failed to write to file after 3 attempts. Error: {}", e);
									}
									continue;
								}
							}
						},
						Err(e) => {
							log::error!("save_last_network_layer: Failed to open file after {} attempts. Error: {}"
							, &i, e);
							if attempts > 3 {
								panic!("save_last_network_layer: Failed to open file after 3 attempts. Error: {}", e);
							}
							continue;
						}
					}
				} else {
					panic!("save_last_network_layer: No layers in the network");
				}
			}
		}


























		//added 01/10/24
		//01/20/24 - removed:
			//pub async fn cycle(&mut self, epsilon: &mut f64, value_prior: &mut f64, coinbase_wallet: &mut f64,
			//	kraken_wallet: &mut f64, bitstamp_wallet: &mut f64,gemini_wallet: &mut f64,
			//	coinbase_secret: &str, coinbase_api_key: &str,
			//	kraken_secret: &str, kraken_api_key: &str, gemini_secret: &str, gemini_api_key: &str,
			//	bitstamp_secret: &str, bitstamp_api_key: &str)-> Result<(), Box<dyn Error + Send>> {
		//01/21/24 - added. Only difference is the i: usize part
		/*
		pub async fn cycle(&mut self, i: usize, epsilon: &mut f64, value_prior: &mut f64, coinbase_wallet: &mut f64,
			kraken_wallet: &mut f64, bitstamp_wallet: &mut f64,gemini_wallet: &mut f64,
			coinbase_secret: &str, coinbase_api_key: &str,
			kraken_secret: &str, kraken_api_key: &str, gemini_secret: &str, gemini_api_key: &str,
			bitstamp_secret: &str, bitstamp_api_key: &str)-> Result<(), Box<dyn Error + Send>> {
			
			
			
		//01/21/24 - added:
			//if i%10 == 0 && !self.replay_buffer.is_empty() {
			//	let current_state = self.layers[0].clone();
			//	let transition = self.replay_buffer.sample_random_replay_buffer()?;
			//	//now ready to use sampled transition for training
			//	let state = transition.state.clone();
			//	let action = transition.action;
			//	let reward = transition.reward;
			//	let next_state = transition.next_state.clone();
			//	//establish input layer from replay buffer to then feed it forward
			//	self.layers[0] = state;
			//	self.feed_forward();
			//	//calculate new target q-value
			//	//this is to reset my input layer to what it was before the expReplay
			//	self.layers[0] = current_state;
			//}

			//this will execute once all the inputs have been updated
			//it will do everything from save current state in replay buffer
			//to feed_forward
			//to backpropagation and updating weights

			//need to make sure it doesnt do a feed forward unless there's no queue
			//		 in update_input. there isn't going to be a queue because it will
			//		  be the last in line.


			//MIGHT NEED TO ESTABLISH A MUTEX right now so it doesn't feed forward
			//		 different information than I saved into the state for exp. replay.
			//If I do, then I may just remove the mutex from feed_forward all-together
				//01/11/24 - removed. 
					//let _guard = self.input_mutex.lock().await;
					//let current_state_input_layer_clone = self.layers[0].clone();
					//stuff for exp replay
					//let input_data = self.layers[0].data.clone();
					//need to drop mutex so I can then do the feed_forward
					//drop(_guard);

				//stuff for exp replay
					let mut replay_buffer = ReplayBuffer::new(1);
					let input_data = self.layers[0].data.clone();
					//state stuff
					let input_rows = self.layers[0].rows;
					let input_columns = self.layers[0].columns;

					let state = NetworkLayer {
						rows: input_rows,
						columns: input_columns,
						data: input_data,
					};
				//01/11/24 - removed
					//self.feed_forward_with_cloned_input(&current_state_input_layer_clone);
					//println!("Before feed forward");
				self.feed_forward();
					//println!("After feed forward");

			//for epsilon-greedy
			//println!("Before exploration or exploitation");
			let (index_chosen_for_current_state, q_value_for_current_state) = self.exploration_or_exploitation(epsilon);
			//println!("index chosen: {}\nq value for current state: {}", &index_chosen_for_current_state, &q_value_for_current_state);

			//for exp. replay
			let action = index_chosen_for_current_state;





			////to actually do the functions
			////how does it work:
			//// the  *  in &*coinbase_wallet gets the value from coinbase_wallet,
			//// then the  &  takes a reference to that value. so now I have an
			////		 immutable reference to same value
			//let coinbase_wallet_immutable = &*coinbase_wallet;
			//let kraken_wallet_immutable = &*kraken_wallet;
			//let bitstamp_wallet_immutable = &*bitstamp_wallet;
			//let gemini_wallet_immutable = &*gemini_wallet;
			let client = reqwest::Client::new();
			let value_after: f64 = match index_chosen_for_current_state {
				0 => action_functions::s_i0_do_nothing(value_prior)?,
				1 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i5_sol_5_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				2 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i6_sol_6_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				3 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i7_sol_7_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				4 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
				action_functions::s_i8_sol_8_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				5 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i9_sol_9_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				6 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i10_sol_10_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				7 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i13_sol_3_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				8 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i14_sol_4_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				9 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i15_sol_5_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				10 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i16_sol_6_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				11 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i17_sol_7_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				12 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i18_sol_8_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				13 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i19_sol_9_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				14 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i20_sol_10_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				15 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i23_sol_3_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				16 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i24_sol_4_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				17 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i25_sol_5_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				18 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i26_sol_6_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				19 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i27_sol_7_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				20 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i28_sol_8_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				21 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i29_sol_9_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				22 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i30_sol_10_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				23 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i35_sol_5_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				24 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i36_sol_6_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				25 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i37_sol_7_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				26 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i38_sol_8_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				27 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i39_sol_9_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				28 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i40_sol_10_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				29 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i43_sol_3_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				30 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i44_sol_4_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				31 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i45_sol_5_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				32 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i46_sol_6_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				33 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i47_sol_7_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				34 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i48_sol_8_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				35 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i49_sol_9_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				36 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i50_sol_10_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				37 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i55_sol_5_kraken_coinbase(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				38 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i56_sol_6_kraken_coinbase(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				39 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i57_sol_7_kraken_coinbase(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				40 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i58_sol_8_kraken_coinbase(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				41 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i59_sol_9_kraken_coinbase(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				42 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i60_sol_10_kraken_coinbase(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				43 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i65_sol_5_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				44 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i66_sol_6_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				45 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i67_sol_7_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				46 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i68_sol_8_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				47 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i69_sol_9_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				48 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i70_sol_10_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				49 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i75_xlm_5_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				50 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i76_xlm_6_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				51 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i77_xlm_7_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				52 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i78_xlm_8_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				53 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i79_xlm_9_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				54 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i80_xlm_10_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				55 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i83_xlm_3_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				56 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i84_xlm_4_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				57 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i85_xlm_5_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				58 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i86_xlm_6_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				59 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i87_xlm_7_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				60 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i88_xlm_8_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				61 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i89_xlm_9_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				62 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i90_xlm_10_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				63 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i95_xlm_5_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				64 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i96_xlm_6_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				65 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i97_xlm_7_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				66 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i98_xlm_8_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				67 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i99_xlm_9_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				68 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i100_xlm_10_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				69 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i105_xlm_5_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				70 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i106_xlm_6_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				71 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i107_xlm_7_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				72 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i108_xlm_8_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				73 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i109_xlm_9_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self  ).await?
				},
				74 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i110_xlm_10_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self ).await?
				},
				_ => todo!(),
			};



		//	println!("Before reward");
			let the_reward = reward(*value_prior, value_after);
			//01/20/24 - added:
				//this will save my reward so I can see how my network is improving
				let _unused_variable = save_reward(&the_reward);
				//to reset value_prior so that reward is calculated correctly
				*value_prior = value_after;
		//	println!("After reward");
			//do target q value and then get next state 
			//01/11/24 - removed
				//let _guard = self.input_mutex.lock().await;
				////this gives us the next state's input layer
			//need to clone because then I'll have 2 mutable borrows
		//	println!("Before clone");
			let next_state_input_layer_clone = self.layers[0].clone();
		//	println!("After clone");
				//need to drop mutex so I can then do the feed_forward
				//drop(_guard);
			//let next_state_input_layer = self.layers[0];
			//do I need to add my value_prior as input?
			//I think so because this will help the network decide whether to be risky or not
			//so I need to update the input every time I do an action_function and it
			//		 will do like 1 index and then the update_input thing
			//what are the downsides?
			//	I need to figure out how to update the input immediately after initialization
			//	Easy, just do an update input line right after initialization.
			//	everytime value_prior changes, so everytime I execute an action_function
			//	 I need to update the input. so in every action function, I have to
			//	 update the input
			//DONE DONE DONE

			//now I need to get the target q value, aka the next state's q value
		//	println!("Before calculate target q value");
			let target_q_value = self.calculate_target_q_value(the_reward);
		//	println!("after calculate target q value");
			

			// I now have everything for the experience replay:

			let transition = Transition {
				state,
				action,
				reward : the_reward,
				next_state : next_state_input_layer_clone,
			};






			//does backpropagation
		//	println!("Before back propagation");
			//01/19/24 - removed:
				//let (gradients, indices) = self.el_backpropagation(&index_chosen_for_current_state, &q_value_for_current_state, &target_q_value );
			//01/19/24 - added:
				//let old_weights = self.weights.clone();
				self.el_backpropagation(&index_chosen_for_current_state, &q_value_for_current_state, &target_q_value);
				let learning_rate = 0.0001;
		//	println!("fater back propagation");
			//updates weights. aka... IMPROVEMENT
		//	println!("Before update weights");
			//just for debugging

			//01/19/24 - removed:
				//self.el_update_weights(&gradients, &indices);
			//01/16/24 - added:
				self.el_update_weights(&learning_rate);
				//let layer_index_to_compare_weights = 0;
				//self.compare_weights(&old_weights, layer_index_to_compare_weights);
				//self.print_weight_layer_rows(0);
				//self.print_gradients_for_layer_and_row(0, 54);
			//01/19/24 - removed:
				//let weights_after_update = self.weights.clone();
				//assert!(weights_before_update != weights_after_update, "The weights were not updated!");
			//assert!(weights_before_update == weights_after_update, "The weights were updated!");
		//	println!("after update weights");
			//push transition into the replay buffer
		//	println!("Before pushing transition");
			replay_buffer.push(transition);
		//	println!("after pushing transition");
			//save the buffer to a file
		//	println!("Before saving replay buffer");
			let _dummyvar = replay_buffer.save_replay_buffer_v2();
		//	println!("After saving replay buffer");
		//	println!("Before printing replay buffer");
			replay_buffer.print_replay_buffer();
		//	println!("after saving replay buffer");




			Ok(())
		}
		*/







		//01/21/24 - added:
		//this feed forwards,

		pub  async fn cycle_part_one_of_two(&mut self, i: usize, 
			epsilon: &mut f64, value_prior: &mut f64, coinbase_wallet: &mut f64,
			kraken_wallet: &mut f64, bitstamp_wallet: &mut f64, 
			gemini_wallet: &mut f64, coinbase_secret: &str, 
			coinbase_api_key: &str, kraken_secret: &str, kraken_api_key: &str, 
			gemini_secret: &str, gemini_api_key: &str, bitstamp_secret: &str, 
			bitstamp_api_key: &str) -> (usize, f64, f64) {


			let start = std::time::Instant::now();   //to measure execution time
			//neural_network.feed_forward();
			self.feed_forward();
			let elapsed = start.elapsed();
			println!("Elapsed: {:?}", elapsed);
			//01/24/24 - added because encountered error saying index of 
			//		largest q value was never initialized. 
			//		so use this to debug.
			//02/28/24 - removed:
				//self.print_last_network_layer();
			//02/28/24 - added:
				self.save_last_network_layer(&i);
			//02/06/24 - removed:
			//uncomment if you want to use regular exploration or exploitation
				let (index_chosen_for_current_state, q_value_for_current_state, exploration_or_exploitation) = 
					self.exploration_or_exploitation(epsilon);
			//02/06/24 - added in its place:
			//uncomment if you want to use explor_exploit v2
			// let (index_chosen_for_current_state, q_value_for_current_state, exploration_or_exploitation) = 
			// 	self.exploration_or_exploitation_v2(&i);


			let value_after = self.choose_action_function(value_prior, 
				coinbase_wallet, kraken_wallet, bitstamp_wallet, 
				gemini_wallet, coinbase_secret, coinbase_api_key, kraken_secret, 
				kraken_api_key, gemini_secret, gemini_api_key, bitstamp_secret, 
				bitstamp_api_key, index_chosen_for_current_state).await;
			
			let value_after = match value_after {
				Ok(val) => val,
				Err(e) => {
					panic!("An error occurred when establishing value_after in 
					match:\n{}", e); // or handle the error appropriately
				},
			};

			let the_reward = reward(*value_prior, value_after);
			//01/24/24 - removed so I can handle error:
				//let _unused_variable = save_reward(&the_reward);
				//let _unused_variable = save_reward(&the_reward);
			//01/24/24 - added:
				println!("reward:{}", &the_reward);
				//02/16/24 - added: index_chosen_for_current_state as parameter
				match save_reward(&the_reward, exploration_or_exploitation, &i,
				&index_chosen_for_current_state) {
					Ok(_) => println!("Reward saved successfully."),
					Err(e) => println!("Failed to save reward: {}\n\n\n\n\n", e),
				}
			*value_prior = value_after;

			(index_chosen_for_current_state, q_value_for_current_state, 
				the_reward)
		}


		pub async fn choose_action_function(&mut self, value_prior: &mut f64, 
			coinbase_wallet: &mut f64, kraken_wallet: &mut f64, 
			bitstamp_wallet: &mut f64, gemini_wallet: &mut f64, 
			coinbase_secret: &str, coinbase_api_key: &str, kraken_secret: &str, 
			kraken_api_key: &str, gemini_secret: &str, gemini_api_key: &str, 
			bitstamp_secret: &str, bitstamp_api_key: &str, 
			index_chosen_for_current_state: usize) ->  
			Result<f64, Box<dyn Error + Send>> {


			
			let client = reqwest::Client::new();
			let value_after: f64 = match index_chosen_for_current_state {
				0 => action_functions::s_i0_do_nothing(value_prior)?,
				1 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i5_sol_5_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				2 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i6_sol_6_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				3 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i7_sol_7_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				4 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
				action_functions::s_i8_sol_8_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				5 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i9_sol_9_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				6 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i10_sol_10_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				7 => {//begin removal - end at number 14. changed from sol to xrp
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i113_xrp_3_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				8 => {//changed from sol to xrp
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i114_xrp_4_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				9 => {//changed from sol to xrp
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i115_xrp_5_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				10 => {//changed from sol to xrp
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i116_xrp_6_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				11 => {//changed from sol to xrp
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i117_xrp_7_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				12 => {//changed from sol to xrp
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i118_xrp_8_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				13 => {//changed from sol to xrp
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i119_xrp_9_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				14 => {//end removal//
					let kraken_wallet_immutable = &*kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i120_xrp_10_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				15 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i23_sol_3_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				16 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i24_sol_4_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				17 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i25_sol_5_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				18 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i26_sol_6_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				19 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i27_sol_7_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				20 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i28_sol_8_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				21 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i29_sol_9_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				22 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i30_sol_10_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				23 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i35_sol_5_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				24 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i36_sol_6_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				25 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i37_sol_7_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				26 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i38_sol_8_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				27 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i39_sol_9_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				28 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i40_sol_10_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				29 => {//sol to xrp. 40s to 120s
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i123_xrp_3_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				30 => {//sol to xrp
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i124_xrp_4_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				31 => {//sol to xrp. 
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i125_xrp_5_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				32 => {//sol to xrp
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i126_xrp_6_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				33 => {//sol to xrp
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i127_xrp_7_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				34 => {//sol to xrp
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i128_xrp_8_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				35 => {//sol to xrp
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i129_xrp_9_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				36 => {//sol to xrp
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let kraken_wallet_immutable = &*kraken_wallet;
					action_functions::s_i130_xrp_10_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				37 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i55_sol_5_kraken_coinbase(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				38 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i56_sol_6_kraken_coinbase(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				39 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i57_sol_7_kraken_coinbase(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				40 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i58_sol_8_kraken_coinbase(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				41 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i59_sol_9_kraken_coinbase(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				42 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i60_sol_10_kraken_coinbase(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				43 => {//begin removal. sol to xrp. 65 to 134
					let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i134_xrp_4_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				44 => {//sol to xrp. 66 to 135
					let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i135_xrp_5_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				45 => {//sol to xrp. 67 to 136
					let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i136_xrp_6_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				46 => {//sol to xrp. 68 to 137
					let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i137_xrp_7_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				47 => {//sol to xrp. 69 to 138
					let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i138_xrp_8_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				48 => {//sol to xrp. 70 to 139. at end append 140
					let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i139_xrp_9_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				49 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i75_xlm_5_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				50 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i76_xlm_6_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				51 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i77_xlm_7_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				52 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i78_xlm_8_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				53 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i79_xlm_9_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				54 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i80_xlm_10_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				55 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i83_xlm_3_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				56 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i84_xlm_4_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				57 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i85_xlm_5_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				58 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i86_xlm_6_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				59 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i87_xlm_7_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				60 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i88_xlm_8_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				61 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i89_xlm_9_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				62 => {
					let kraken_wallet_immutable = &* kraken_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i90_xlm_10_coinbase_bitstamp( coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				},
				63 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i95_xlm_5_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				64 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i96_xlm_6_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				65 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i97_xlm_7_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				66 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i98_xlm_8_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				67 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i99_xlm_9_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				68 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i100_xlm_10_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				69 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i105_xlm_5_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				70 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i106_xlm_6_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				71 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i107_xlm_7_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				72 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i108_xlm_8_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				73 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i109_xlm_9_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				74 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let gemini_wallet_immutable = &*gemini_wallet;
					action_functions::s_i110_xlm_10_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				//01/27/24 - below added.
				75 => {let gemini_wallet_immutable = &*gemini_wallet;
					let coinbase_wallet_immutable = &*coinbase_wallet;
					action_functions::s_i140_xrp_10_kraken_bitstamp(&coinbase_wallet_immutable, kraken_wallet, bitstamp_wallet, &gemini_wallet_immutable, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				76 => {//143 to 150
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i143_xrp_3_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				77 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i144_xrp_4_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				78 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i145_xrp_5_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				79 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i146_xrp_6_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				80 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i147_xrp_7_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				81 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i148_xrp_8_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				82 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i149_xrp_9_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				83 => {
					let kraken_wallet_immutable = &*kraken_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i150_xrp_10_gemini_coinbase(coinbase_wallet, &kraken_wallet_immutable, &bitstamp_wallet_immutable, gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				84 => {//begin 153 to 160
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i153_xrp_3_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				85 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i154_xrp_4_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				86 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i155_xrp_5_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				87 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i156_xrp_6_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				88 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i157_xrp_7_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				89 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i158_xrp_8_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				90 => {
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i159_xrp_9_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				91 => {//end 153 to 160
					let coinbase_wallet_immutable = &*coinbase_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i160_xrp_10_gemini_kraken(&coinbase_wallet_immutable, kraken_wallet, &bitstamp_wallet_immutable, gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				},
				92 => {//begin 163 to 170
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i163_xrp_3_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				93 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i164_xrp_4_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				94 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i165_xrp_5_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				95 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i166_xrp_6_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				96 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i167_xrp_7_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				97 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i168_xrp_8_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				98 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i169_xrp_9_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				99 => {//end 163 to 170
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i170_xrp_10_coinbase_kraken(coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				100 => {//begin 174 to 180
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i174_xrp_4_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				101 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i175_xrp_5_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				102 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i176_xrp_6_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				103 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i177_xrp_7_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				104 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i178_xrp_8_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				105 => {
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i179_xrp_9_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				106 => {//end 174 to 180
					let gemini_wallet_immutable = &*gemini_wallet;
					let bitstamp_wallet_immutable = &*bitstamp_wallet;
					action_functions::s_i180_xrp_10_kraken_coinbase( coinbase_wallet, kraken_wallet, &bitstamp_wallet_immutable, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key, self).await?
				},
				// 107 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let gemini_wallet_immutable = &*gemini_wallet;
				// 	action_functions::s_i13_sol_3_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				// },
				// 108 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let gemini_wallet_immutable = &*gemini_wallet;
				// 	action_functions::s_i14_sol_4_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				// },
				// 109 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let gemini_wallet_immutable = &*gemini_wallet;
				// 	action_functions::s_i15_sol_5_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				// },
				// 110 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let gemini_wallet_immutable = &*gemini_wallet;
				// 	action_functions::s_i16_sol_6_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				// },
				// 111 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let gemini_wallet_immutable = &*gemini_wallet;
				// 	action_functions::s_i17_sol_7_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				// },
				// 112 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let gemini_wallet_immutable = &*gemini_wallet;
				// 	action_functions::s_i18_sol_8_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				// },
				// 113 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let gemini_wallet_immutable = &*gemini_wallet;
				// 	action_functions::s_i19_sol_9_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				// },
				// 114 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let gemini_wallet_immutable = &*gemini_wallet;
				// 	action_functions::s_i20_sol_10_coinbase_bitstamp(coinbase_wallet, &kraken_wallet_immutable, bitstamp_wallet, &gemini_wallet_immutable, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key, self).await?
				// },
				// 115 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let coinbase_wallet_immutable = &*coinbase_wallet;
				// 	action_functions::s_i43_sol_3_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				// },
				// 116 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let coinbase_wallet_immutable = &*coinbase_wallet;
				// 	action_functions::s_i44_sol_4_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				// },
				// 117 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let coinbase_wallet_immutable = &*coinbase_wallet;
				// 	action_functions::s_i45_sol_5_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				// },
				// 118 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let coinbase_wallet_immutable = &*coinbase_wallet;
				// 	action_functions::s_i46_sol_6_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				// },
				// 119 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let coinbase_wallet_immutable = &*coinbase_wallet;
				// 	action_functions::s_i47_sol_7_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				// },
				// 120 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let coinbase_wallet_immutable = &*coinbase_wallet;
				// 	action_functions::s_i48_sol_8_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				// },
				// 121 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let coinbase_wallet_immutable = &*coinbase_wallet;
				// 	action_functions::s_i49_sol_9_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				// },
				// 122 => {
				// 	let kraken_wallet_immutable = &*kraken_wallet;
				// 	let coinbase_wallet_immutable = &*coinbase_wallet;
				// 	action_functions::s_i50_sol_10_gemini_bitstamp(&coinbase_wallet_immutable, &kraken_wallet_immutable, bitstamp_wallet, gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key, self).await?
				// },
				//01/24/24 - removed:
					// _ => todo!(),
				//01/24/24 - added:
						//val will match any value not covered by the other arms and bind it to the variable
					val => {
						panic!("got action function to be above 211101010101010101251010101257275jKJGIERJIOFWENIGGERFDSJKREI. it is: {}", val);
					},
			};




			Ok(value_after)

		}

		//03/13/24 - added i
		pub fn cycle_part_two_of_two(&mut self, 
			index_chosen_for_current_state: usize, 
			q_value_for_current_state: f64, the_reward: f64, i: &usize) {

			//this just finishes everything up
			let target_q_value = self.calculate_target_q_value(the_reward);
			self.el_backpropagation(&index_chosen_for_current_state,
				&q_value_for_current_state, &target_q_value);
			//02/02/24 - changed from 0.0001 to 0.00001
			//02/29/24 - from 0.00001 to 0.000_001
			//03/10/24 - changed to 1.0
			//03/12/24 - changed to 0.1. 03/13/24 - to 0.01 then 0.001 then 0.0001
			let learning_rate = 0.0001;
			self.el_update_weights(&learning_rate);
			
			//03/10/24 - added:
			self.save_all_gradients(&i);
			//03/12/24 - added:
			self.save_all_weights(&i);
		}

	}












	







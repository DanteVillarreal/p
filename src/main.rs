



// use p::action_functions::s_i0_do_nothing;
// use rand_distr::num_traits::AsPrimitive;
// use rand_distr::{StandardNormal, Normal, Distribution};
// use reqwest::Client;
// use serde::de::value;                                //to actually make the request itself
use std::env;                                       //so I can use .env files and I dont have to put key details on github
// use rand::Rng;
// use hmac::{Hmac, Mac,};	                            //so I can do the signature stuff
// use sha2::{Sha256, Sha384, Sha512, Digest};	        //so I can do signature stuff
use p::network::{NeuralNetwork, NetworkLayer, ReplayBuffer, GradientNetwork, Transition, delete_most_recent_replay_buffers};
//use p::network::NetworkLayer;
//use p::network::ReplayBuffer;
//use p::network::GradientNetwork;                    //to use gradients
// use p::action_functions;
use p::{execute_action_functions, standardization_functions};
use p::network;
use tokio;                                          //so I can do async
use dotenv::dotenv;
//use mod network;
//use mod actions;
//use std::time::Instant;                             //this is to record time for execution
use std::process::{Command, Stdio, ChildStdout};                 //for piping websocket client
use std::io::{BufRead, BufReader};//this is to help us read from stdin
//use serde_json::Value;          //good for parsing intput in JSON format
//use tokio::time::delay_for;                         //for "sleep", but in async functions. 02/09/24 - removed for tokio update
//use std::time::Duration;                            //for use in conjunction with delay_for
//use std::sync::Mutex;                             //cant use this because not async
use tokio::task;                                    //to do child spawns
//use std::error::Error;                              //to do box error 
use tokio::sync::Mutex;                             // Use async Mutex from Tokio
use std::sync::Arc;  								// Use Arc to share Mutex among multiple tasks
//use tokio::sync::MutexGuard;
//use tokio::time::delay_until;                       //for async "waits". 02/09/24 - removed for tokio update
use simplelog;                                      //to have panics in a file
use log_panics;                                     //to have panics in a file
use std::fs;                                  //for file handling
use log;                                            //for logging errors to panic file
use std::sync::atomic::{AtomicBool, Ordering};      //02/14/24 - added. acts as flag for the 2 tasks im joining in the loop




///-----FOR PARSING-----////
//use std::env;
//use std::process::{Command, Stdio};
//use serde_json::Value;          //good for parsing intput in JSON format
                                //  aka what the messages are in
//use std::io::{BufRead, BufReader};//this is to help us read from stdin

//---step 3 below----///////
/*
    fn generate_random_number () {
            //generates random number between 0 and 100 including both 0 and 100
            rand::thread_rng().gen_range(0..=100);

            //generate random numbr between 0 and 100 excluding 100
            rand::thread_rng().gen_range(0..100);
            

            //generate random number with mean 0 and std dev of 1
            let standard_distributed_number: f64 = rand::thread_rng().sample(StandardNormal);
            //or//
            let desired_mean = 0.0;
            let desired_std_dev = 1.0;
            let distribution_type = Normal::new(desired_mean, desired_std_dev).unwrap();
            let distributed_number = distribution_type.sample(&mut rand::thread_rng());
            println!("{} is from a standard normal distribution. {} is too",
                    standard_distributed_number, distributed_number);
            //or//
            //--dont like unwrap();
            let desired_mean = 0.0;
            let desired_std_dev = 1.0;
            let distribution_type_result = Normal::new(desired_mean, desired_std_dev);
            let distribution_type = match distribution_type_result {
                Ok(distribution_type_result) => distribution_type_result,
                Err(error) => panic!("Could not make a random number in var distribution_type_result. 
                                            Error code: {:?}", error),
            };
            let distributed_number = distribution_type.sample(&mut rand::thread_rng());
            println!("{} is from a standard normal distribution. {} is too",
                    standard_distributed_number, distributed_number);
    }
*/
///--step 3 above----////////  



















//-----ALL-FOR-PARSING-UNDER-THIS//

//01/16/24- removed:
    //async fn handle_sol_coinbase(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
//01/16/24 - added line directly beneath this:
	//async fn handle_sol_coinbase(message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, updated: &mut [bool; 60]) {
//01/24/24 - changed to this:
//02/09/24 - finally commented out all the functions
// async fn handle_sol_coinbase(message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, updated: &mut [bool; 60], divisor: &f64) {
//     //if the message contains the word "heartbeat", ignore the entire message basically
//     if message.contains("heartbeat") {
//         println!("Coinbase sol eartbeat message. ignoring...it's contents:\n{}", message);
//         return;
//     }
//     if message.contains("subscriptions") {
//         println!("Coinbase sol SubsCriptions message. ignoring...it's contents:\n{}", message);
//         return;
//     }
//     if message.trim().is_empty() {
//         println!("Coinbase sol: blank message received\nmessage: {}", message);
//         return;
//     }
//     let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

//     //variable declaration so I can have a larger scope
//     let mut coinbase_price = 0.0;
//     let mut coinbase_volume_24h = 0.0;
//     let mut coinbase_low_24h = 0.0;
//     let mut coinbase_high_24h = 0.0;
//     let mut coinbase_low_52w = 0.0;
//     let mut coinbase_high_52w = 0.0;
//     let mut coinbase_price_percent_chg_24h = 0.0;

//     match data {
//         Ok(value) => {
//             let ticker = &value["events"][0]["tickers"][0];
//             coinbase_price = ticker["price"].as_str().unwrap().parse::<f64>().unwrap();
//             coinbase_volume_24h = ticker["volume_24_h"].as_str().unwrap().parse::<f64>().unwrap();
//             coinbase_low_24h = ticker["low_24_h"].as_str().unwrap().parse::<f64>().unwrap();
//             coinbase_high_24h = ticker["high_24_h"].as_str().unwrap().parse::<f64>().unwrap();
//             coinbase_low_52w = ticker["low_52_w"].as_str().unwrap().parse::<f64>().unwrap();
//             coinbase_high_52w = ticker["high_52_w"].as_str().unwrap().parse::<f64>().unwrap();
//             coinbase_price_percent_chg_24h = ticker["price_percent_chg_24_h"].as_str().unwrap().parse::<f64>().unwrap();
//         }
//         Err(e) => println!("Failed to parse SOL COINBASE message. \nError {}\n{}", e, message),
//     }

//         let indices = [0, 1, 2, 3, 4, 5, 6];
//         let new_values = [coinbase_price, coinbase_volume_24h, coinbase_low_24h, 
//                     coinbase_high_24h, 
//                     coinbase_low_52w, coinbase_high_52w, coinbase_price_percent_chg_24h,];

// 		//01/24/24 - added transformed_values. then removed it and added scaled_values
// 			//let transformed_values: Vec<f64> = new_values.iter().map(|x: &f64| x.ln()).collect();
// 			let scaled_values: Vec<f64> = new_values.iter().map(|&x| x / divisor).collect();
//         //01/16/24 - added lock:
//         let mut neural_network = shared_neural_network.lock().await;
// 		//01/24/24 - removed normal update_input. then added transformed. then removed it and added scaled:
//         	//neural_network.update_input(&indices, &new_values).await;
// 			//neural_network.update_input(&indices, &transformed_values).await;
// 			neural_network.update_input(&indices, &scaled_values).await;
//         //to mark the inputs as changed
//         for index in indices {
//             updated[index] = true;
//         }
//         //if updated.iter().all(|&x| x) {
//         //    neural_network.print_layers();
//         //} 
//         //else {
//         //    let not_updated: Vec<String> = updated.iter()
//         //    .enumerate()
//         //    .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
//         //    .collect();
//         //    println!("Neurons: {} have not been updated", not_updated.join(", "));
//         //}

//     }



 











//     ////NEED TO SEE IF i HAVE TO NORMALIZE THIS DATA FIRST
//     //let indices = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
//     //let new_values = [&coinbase_price, &coinbase_open_24h, &coinbase_volume_24h, &coinbase_low_24h, 
//     //            &coinbase_high_24h, &coinbase_volume_30d, &coinbase_best_bid, &coinbase_best_bid_size, 
//     //            &coinbase_best_ask, &coinbase_best_ask_size, &coinbase_side, &coinbase_last_size];
//     //neural_network.update_input(&indices, &new_values);

// //01/16/24 - removed:    
//     //async fn handle_xlm_coinbase(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
// //01/16/24 - added line below this:
// 	//async fn handle_xlm_coinbase(message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, updated: &mut [bool; 60]) {
// //01/24/24 - modified fn header to this:
// async fn handle_xlm_coinbase(message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, updated: &mut [bool; 60], divisor: &f64) {
//         //if the message contains the word "heartbeat", ignore the entire message basically
//         if message.contains("heartbeat") {
//             println!("Coinbase xlm: heartbeat message. ignoring...it's contents:\n{}", message);
//             return;
//         }
//         if message.contains("subscriptions") {
//             println!("Coinbase xlm: SubsCriptions message. ignoring...it's contents:\n{}", message);
//             return;
//         }
//         if message.trim().is_empty() {
//             println!("Coinbase xlm: blank message received\nmessage: {}", message);
//             return;
//         }
//         let data: Result<Value, serde_json::Error> = serde_json::from_str(message);
    
//         //variable declaration so I can have a larger scope
//         let mut coinbase_price = 0.0;
//         let mut coinbase_volume_24h = 0.0;
//         let mut coinbase_low_24h = 0.0;
//         let mut coinbase_high_24h = 0.0;
//         let mut coinbase_low_52w = 0.0;
//         let mut coinbase_high_52w = 0.0;
//         let mut coinbase_price_percent_chg_24h = 0.0;
    
//         match data {
//             Ok(value) => {
//                 let ticker = &value["events"][0]["tickers"][0];
//                 coinbase_price = ticker["price"].as_str().unwrap().parse::<f64>().unwrap();
//                 coinbase_volume_24h = ticker["volume_24_h"].as_str().unwrap().parse::<f64>().unwrap();
//                 coinbase_low_24h = ticker["low_24_h"].as_str().unwrap().parse::<f64>().unwrap();
//                 coinbase_high_24h = ticker["high_24_h"].as_str().unwrap().parse::<f64>().unwrap();
//                 coinbase_low_52w = ticker["low_52_w"].as_str().unwrap().parse::<f64>().unwrap();
//                 coinbase_high_52w = ticker["high_52_w"].as_str().unwrap().parse::<f64>().unwrap();
//                 coinbase_price_percent_chg_24h = ticker["price_percent_chg_24_h"].as_str().unwrap().parse::<f64>().unwrap();
//             }
//             Err(e) => println!("Failed to parse SOL COINBASE message. \nError {}\n{}", e, message),
//         }
 
//             let indices = [7, 8, 9, 10, 11, 12, 13];
//             let new_values = [coinbase_price, coinbase_volume_24h, coinbase_low_24h, 
//             coinbase_high_24h, 
//             coinbase_low_52w, coinbase_high_52w, coinbase_price_percent_chg_24h,];

// 			//01/24/24 - added log transform to shrink inputs. Then removed it and added scaled_values
// 				//let transformed_values: Vec<f64> = new_values.iter().map(|x: &f64| x.ln()).collect();
// 				let scaled_values: Vec<f64> = new_values.iter().map(|&x| x / divisor).collect();
//             //01/16/24 - added lock:
//             let mut neural_network = shared_neural_network.lock().await;
// 			//01/24/24 - removed and added transformed. Then removed and added scaled:
//             	//neural_network.update_input(&indices, &new_values).await;
// 				//neural_network.update_input(&indices, &transformed_values).await;
// 				neural_network.update_input(&indices, &scaled_values).await;
//             //to mark the inputs as changed
//             for index in indices {
//                 updated[index] = true;
//             }
//             //if updated.iter().all(|&x| x) {
//             //    neural_network.print_layers();
//             //} 
//             //else {
//             //    let not_updated: Vec<String> = updated.iter()
//             //    .enumerate()
//             //   .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
//             //    .collect();
//             //    println!("Neurons: {} have not been updated", not_updated.join(", "));
//             //}
//         }





// //01/16/24 - removed:
//     //async fn handle_sol_kraken(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
// //01/16/24 - added in its place:. 01/24/24 - added divisor
// async fn handle_sol_kraken(message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, updated: &mut [bool; 60], divisor: &f64) {
//     if message.contains("heartbeat") {
//         println!("Kraken sol heartbeat message. ignoring...it's contents:\n{}", message);
//         return;
//     }
//     if message.contains("systemStatus") {
//         println!("Kraken sol: initial system message. ignoring message... it's contents:\n{}", message);
//         return;
//     }
//     if message.contains("subscriptionStatus") {
//         println!("Kraken sol: initial  SUB message. ignoring...it's contents:\n{}", message);
//         return;
//     }
//     if message.trim().is_empty() {
//         println!("Kraken sol: blank message received\nmessage: {}", message);
//         return;
//     }
//     let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

//     let mut a_0 = 0.0;
//     let mut a_1 = 0.0;
//     let mut a_2 = 0.0;

//     let mut b_0 = 0.0;
//     let mut b_1 = 0.0;
//     let mut b_2 = 0.0;

//     let mut c_0 = 0.0;
//     let mut c_1 = 0.0;

//     let mut v_0 = 0.0;
//     let mut v_1 = 0.0;

//     let mut p_0 = 0.0;
//     let mut p_1 = 0.0;

//     let mut t_0 = 0.0;
//     let mut t_1 = 0.0;

//     let mut l_0 = 0.0;
//     let mut l_1 = 0.0;

//     let mut h_0 = 0.0;
//     let mut h_1 = 0.0;

//     let mut o_0 = 0.0;
//     let mut o_1 = 0.0;

//     match data {
//         Ok(value) => {
//             let ticker = &value[1];
//             a_0 = ticker["a"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for a[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse a[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             a_1 = ticker["a"][1].as_i64().unwrap_or_else(|| {
//                 println!("Failed to get string for a[1]. Full message: {}", message);
//                 panic!();
//             }) as f64;
//             a_2 = ticker["a"][2].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for a[2]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse a[2] as f64. Full message: {}", message);
//                 panic!();
//             });
//             b_0 = ticker["b"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for b[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse b[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             b_1 = ticker["b"][1].as_i64().unwrap_or_else(|| {
//                 println!("Failed to get string for a[1]. Full message: {}", message);
//                 panic!();
//             }) as f64;
//             b_2 = ticker["b"][2].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for b[2]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse b[2] as f64. Full message: {}", message);
//                 panic!();
//             });
//             c_0 = ticker["c"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for c[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse c[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             c_1 = ticker["c"][1].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for c[1]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse c[1] as f64. Full message: {}", message);
//                 panic!();
//             });
//             v_0 = ticker["v"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for v[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse v[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             v_1 = ticker["v"][1].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for v[1]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse v[1] as f64. Full message: {}", message);
//                 panic!();
//             });
//             p_0 = ticker["p"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for p[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse p[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             p_1 = ticker["p"][1].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for p[1]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse p[1] as f64. Full message: {}", message);
//                 panic!();
//             });
//             t_0 = ticker["t"][0].as_i64().unwrap_or_else(|| {
//                 println!("Failed to get string for a[1]. Full message: {}", message);
//                 panic!();
//             }) as f64;
//             t_1 = ticker["t"][1].as_i64().unwrap_or_else(|| {
//                 println!("Failed to get string for a[1]. Full message: {}", message);
//                 panic!();
//             }) as f64;
//             l_0 = ticker["l"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for l[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse l[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             l_1 = ticker["l"][1].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for l[1]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse l[1] as f64. Full message: {}", message);
//                 panic!();
//             });
//             h_0 = ticker["h"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for h[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse h[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             h_1 = ticker["h"][1].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for h[1]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse h[1] as f64. Full message: {}", message);
//                 panic!();
//             });
//             o_0 = ticker["o"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for o[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse o[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             o_1 = ticker["o"][1].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for o[1]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse o[1] as f64. Full message: {}", message);
//                 panic!();
//             });
//         }
//         Err(e) => println!("Failed to parse message: {}", e),
//     }
//     //println!("a_0: {}, a_1: {}, a_2: {}, b_0: {}, b_1: {}, b_2: {}, c_0: {}, c_1: {}, v_0: {}, v_1: {}, p_0: {}, p_1: {}, t_0: {}, t_1: {}, l_0: {}, l_1: {}, h_0: {}, h_1: {}, o_0: {}, o_1: {}", 
//     //&a_0, &a_1, &a_2, &b_0, &b_1, &b_2, &c_0, &c_1, &v_0, &v_1, &p_0, &p_1, &t_0, &t_1, &l_0, &l_1, &h_0, &h_1, &o_0, &o_1);
//     let indices: [usize; 20] = [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 
//     30, 31, 32, 33];
//     let new_values = [a_0, a_1, a_2, b_0, b_1, b_2, c_0, c_1, 
//         v_0, v_1, p_0, p_1, t_0, t_1, l_0, l_1, h_0, h_1, o_0, o_1];
//     //let new_values = [&a_price, &a_whole_lot_volume, &a_lot_volume, &b_price, 
//     //    &b_whole_lot_volume, &b_lot_volume, &c_price, &c_lot_volume, 
//     //    &v_today, &v_last24hours, &p_today, &p_last24hours, &t_today, 
//     //    &t_last24hours, &l_today, &l_last24hours, &h_today, &h_last24hours, 
//     //    &o_today, &o_last24hours];

// 	//01/24/24 - added log transform to shrink inputs. Then removed it and added scaled_inputs
// 		//let transformed_values: Vec<f64> = new_values.iter().map(|x: &f64| x.ln()).collect();
// 		let scaled_values: Vec<f64> = new_values.iter().map(|&x| x / divisor).collect();
//     //01/16/24 - added lock:
//     let mut neural_network = shared_neural_network.lock().await;
// 	//01/24/24 - removed and added transformed. Then removed and added scaled:
//     	//neural_network.update_input(&indices, &new_values).await;
// 		//neural_network.update_input(&indices, &transformed_values).await;
// 		neural_network.update_input(&indices, &scaled_values).await;
	

//     //to mark the inputs as changed
//     for index in indices {
//         updated[index] = true;
//     }
//     //if updated.iter().all(|&x| x) {
//     //    neural_network.print_layers();
//     //} 
//     //else {
//     //    let not_updated: Vec<String> = updated.iter()
//     //    .enumerate()
//     //    .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
//     //    .collect();
//     //    println!("Neurons: {} have not been updated", not_updated.join(", "));
//     //}
// }













// //01/16/24 - removed:
//     //async fn handle_xlm_kraken(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
// //01/16/24 - added in its place:. 01/24/24 - added divisor
// async fn handle_xlm_kraken(message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, updated: &mut [bool; 60], divisor: &f64) {
//     if message.contains("heartbeat") {
//         println!("Kraken xlm heartbeat message. ignoring...it's contents:\n{}", message);
//         return;
//     }
//     if message.contains("systemStatus") {
//         println!("Kraken xlm: initial system message. ignoring...it's contents:\n{}", message);
//         return;
//     }
//     if message.contains("subscriptionStatus") {
//         println!("Kraken xlm: initial  SUB message. ignoring...it's contents:\n{}", message);
//         return;
//     }
//     if message.trim().is_empty() {
//         println!("Kraken xlm: blank message received\nmessage: {}", message);
//         return;
//     }
//     let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

//     let mut a_0 = 0.0;
//     let mut a_1 = 0.0;
//     let mut a_2 = 0.0;

//     let mut b_0 = 0.0;
//     let mut b_1 = 0.0;
//     let mut b_2 = 0.0;

//     let mut c_0 = 0.0;
//     let mut c_1 = 0.0;

//     let mut v_0 = 0.0;
//     let mut v_1 = 0.0;

//     let mut p_0 = 0.0;
//     let mut p_1 = 0.0;

//     let mut t_0 = 0.0;
//     let mut t_1 = 0.0;

//     let mut l_0 = 0.0;
//     let mut l_1 = 0.0;

//     let mut h_0 = 0.0;
//     let mut h_1 = 0.0;

//     let mut o_0 = 0.0;
//     let mut o_1 = 0.0;

//     match data {
//         Ok(value) => {
//             let ticker = &value[1];
//             a_0 = ticker["a"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for a[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse a[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             a_1 = ticker["a"][1].as_i64().unwrap_or_else(|| {
//                 println!("Failed to get string for a[1]. Full message: {}", message);
//                 panic!();
//             }) as f64;
//             a_2 = ticker["a"][2].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for a[2]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse a[2] as f64. Full message: {}", message);
//                 panic!();
//             });
//             b_0 = ticker["b"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for b[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse b[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             b_1 = ticker["b"][1].as_i64().unwrap_or_else(|| {
//                 println!("Failed to get string for a[1]. Full message: {}", message);
//                 panic!();
//             }) as f64;
//             b_2 = ticker["b"][2].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for b[2]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse b[2] as f64. Full message: {}", message);
//                 panic!();
//             });
//             c_0 = ticker["c"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for c[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse c[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             c_1 = ticker["c"][1].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for c[1]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse c[1] as f64. Full message: {}", message);
//                 panic!();
//             });
//             v_0 = ticker["v"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for v[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse v[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             v_1 = ticker["v"][1].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for v[1]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse v[1] as f64. Full message: {}", message);
//                 panic!();
//             });
//             p_0 = ticker["p"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for p[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse p[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             p_1 = ticker["p"][1].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for p[1]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse p[1] as f64. Full message: {}", message);
//                 panic!();
//             });
//             t_0 = ticker["t"][0].as_i64().unwrap_or_else(|| {
//                 println!("Failed to get string for a[1]. Full message: {}", message);
//                 panic!();
//             }) as f64;
//             t_1 = ticker["t"][1].as_i64().unwrap_or_else(|| {
//                 println!("Failed to get string for a[1]. Full message: {}", message);
//                 panic!();
//             }) as f64;
//             l_0 = ticker["l"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for l[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse l[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             l_1 = ticker["l"][1].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for l[1]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse l[1] as f64. Full message: {}", message);
//                 panic!();
//             });
//             h_0 = ticker["h"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for h[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse h[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             h_1 = ticker["h"][1].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for h[1]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse h[1] as f64. Full message: {}", message);
//                 panic!();
//             });
//             o_0 = ticker["o"][0].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for o[0]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse o[0] as f64. Full message: {}", message);
//                 panic!();
//             });
//             o_1 = ticker["o"][1].as_str().unwrap_or_else(|| {
//                 println!("Failed to get string for o[1]. Full message: {}", message);
//                 panic!();
//             }).parse::<f64>().unwrap_or_else(|_| {
//                 println!("Failed to parse o[1] as f64. Full message: {}", message);
//                 panic!();
//             });
//         }
//         Err(e) => println!("Failed to parse message: {}", e),
//     }
//     //println!("a_0: {}, a_1: {}, a_2: {}, b_0: {}, b_1: {}, b_2: {}, c_0: {}, c_1: {}, v_0: {}, v_1: {}, p_0: {}, p_1: {}, t_0: {}, t_1: {}, l_0: {}, l_1: {}, h_0: {}, h_1: {}, o_0: {}, o_1: {}", 
//     //&a_0, &a_1, &a_2, &b_0, &b_1, &b_2, &c_0, &c_1, &v_0, &v_1, &p_0, &p_1, &t_0, &t_1, &l_0, &l_1, &h_0, &h_1, &o_0, &o_1);
//     let indices: [usize; 20] = [34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 
//     50, 51, 52, 53];
//     let new_values = [a_0, a_1, a_2, b_0, b_1, b_2, c_0, c_1, 
//         v_0, v_1, p_0, p_1, t_0, t_1, l_0, l_1, h_0, h_1, o_0, o_1];

// 	//01/24/24 - added to log transform values to shrink inputs. removed then added scaled
// 		//let transformed_values: Vec<f64> = new_values.iter().map(|x: &f64| x.ln()).collect();
// 		let scaled_values: Vec<f64> = new_values.iter().map(|&x| x / divisor).collect();
//     //01/16/24 - added lock:
//     let mut neural_network = shared_neural_network.lock().await;
// 	//01/24/24 - removed and added transformed. then removed and added scaled:
//     	//neural_network.update_input(&indices, &new_values).await;
// 		//neural_network.update_input(&indices, &transformed_values).await;
// 		neural_network.update_input(&indices, &scaled_values).await;
//     //to mark the inputs as changed
//     for index in indices {
//         updated[index] = true;
//     }
//     //if updated.iter().all(|&x| x) {
//     //    neural_network.print_layers();
//     //} 
//     //else {
//     //    let not_updated: Vec<String> = updated.iter()
//     //    .enumerate()
//     //    .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
//     //    .collect();
//     //    println!("Neurons: {} have not been updated", not_updated.join(", "));
//     //}
// }
















// //01/16/24 - removed:
//     //async fn handle_sol_bitstamp(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
// //01/16/24 - added in its place:. 01/24/24 - added divisor
// async fn handle_sol_bitstamp(message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, updated: &mut [bool; 60], divisor: &f64) {
//     // Handle Bitstamp message
//     if message.contains("subscription") {
//         println!("Bitstamp sol subscription succeeded. unimportant message\nmessage: {}", message);
//         return;
//     }
//     if message.contains("heartbeat") {
//         println!("Bitstamp sol heartbeat message\nmessage: {}", message);
//         return;
//     }
//     if message.trim().is_empty() {
//         println!("Bitstamp sol: blank message received\nmessage: {}", message);
//         return;
//     }

//     let v: Result<Value, serde_json::Error> = serde_json::from_str(message);

//     let mut amount = 0.0;
//     let mut price = 0.0;

//     match v {
//         Ok(value) => {
//             if let Value::Object(map) = &value {
//                 // Check if the object has a key "data" whose value is an object
//                 if let Some(Value::Object(data)) = map.get("data") {
//                     // Extract the values
//                     amount = data.get("amount").and_then(Value::as_f64).unwrap();
//                     price = data.get("price").and_then(Value::as_f64).unwrap();

//                     println!("Sol Bitstamp:\namount: {}\nprice: {}\n\n\n", &amount, &price);
        
//                 }
//             }
//         },
//         Err(e) => {
//             println!("Failed to parse JSON Bitstamp message\nError: {}\nMessage: {}", e, message);
//         },
//     }


//     let indices: [usize; 2] = [54, 55];
//     let new_values = [amount, price];
// 	//01/24/24 - added tranformed_values to shrink inputs. removed then added scaled_values
// 		//let transformed_values: Vec<f64> = new_values.iter().map(|x: &f64| x.ln()).collect();
// 		let scaled_values: Vec<f64> = new_values.iter().map(|&x| x / divisor).collect();
//     //01/16/24 - added lock:
//     let mut neural_network = shared_neural_network.lock().await;
// 	//01/24/24 - removed and added transformed. then removed and added scaled:
//     	//neural_network.update_input(&indices, &new_values).await;
// 		//neural_network.update_input(&indices, &transformed_values).await;
// 		neural_network.update_input(&indices, &scaled_values).await;
//     //to mark the inputs as changed
//     for index in indices {
//         updated[index] = true;
//     }
//     //if updated.iter().all(|&x| x) {
//     //    neural_network.print_layers();
//     //} 
//     //else {
//     //    let not_updated: Vec<String> = updated.iter()
//     //    .enumerate()
//     //    .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
//     //    .collect();
//     //    println!("Neurons: {} have not been updated", not_updated.join(", "));
//     //}

// }








// //01/16/24 - removed:
//     //async fn handle_xlm_bitstamp(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
// //01/16/24 - added line directly below:. 01/24/24 - added divisor:
// async fn handle_xlm_bitstamp(message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, updated: &mut [bool; 60], divisor: &f64) {
//     // Handle Bitstamp message
//     if message.contains("subscription") {
//         println!("Bitstamp xlm subscription succeeded. unimportant message\nmessage: {}", message);
//         return;
//     }
//     if message.contains("heartbeat") {
//         println!("Bitstamp xlm heartbeat message\nmessage: {}", message);
//         return;
//     }
//     if message.trim().is_empty() {
//         println!("Bitstamp xlm: blank message received\nmessage: {}", message);
//         return;
//     }

//     let v: Result<Value, serde_json::Error> = serde_json::from_str(message);

//     let mut amount = 0.0;
//     let mut price = 0.0;

//     match v {
//         Ok(value) => {
//             if let Value::Object(map) = &value {
//                 // Check if the object has a key "data" whose value is an object
//                 if let Some(Value::Object(data)) = map.get("data") {
//                     // Extract the values
//                     amount = data.get("amount").and_then(Value::as_f64).unwrap();
//                     price = data.get("price").and_then(Value::as_f64).unwrap();

//                     println!("XLM Bitstamp:\namount: {}\nprice: {}\n\n\n", &amount, &price);
        
//                 }
//             }
//         },
//         Err(e) => {
//             println!("Failed to parse JSON Bitstamp message\nError: {}\nMessage: {}", e, message);
//         },
//     }


//     let indices: [usize; 2] = [56, 57];
//     let new_values = [amount, price];
// 	//01/24/24 - added to accomplish log transorm. then removed and added scaled
// 		//let transformed_values: Vec<f64> = new_values.iter().map(|x: &f64| x.ln()).collect();
// 		let scaled_values: Vec<f64> = new_values.iter().map(|&x| x / divisor).collect();
//     //01/16/24 - added lock:
//     let mut neural_network = shared_neural_network.lock().await;
// 	//01/24/24 - removed update input and added transformed. then removed and added scaled
//     	//neural_network.update_input(&indices, &new_values).await;
// 		//neural_network.update_input(&indices, &transformed_values).await;
// 		neural_network.update_input(&indices, &scaled_values).await;
//     //to mark the inputs as changed
//     for index in indices {
//         updated[index] = true;
//     }
//     //for debugging
//     //if updated.iter().all(|&x| x) {
//     //    neural_network.print_layers();
//     //} 
//     //else {
//     //    let not_updated: Vec<String> = updated.iter()
//     //    .enumerate()
//     //    .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
//     //    .collect();
//     //   println!("Neurons: {} have not been updated", not_updated.join(", "));
//     //}

// }









// //01/16/24 - removed
//     //async fn handle_sol_gemini(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
// //01/16/24 - added in its place:. 01/24/24 - added divisor
// async fn handle_sol_gemini(message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, updated: &mut [bool; 60], divisor: &f64) {
//     //02/07/24 - added loop to handle random error:
//     let mut attempts = 0;
//     loop {
//         if message.contains("heartbeat") {
//             println!("Gemini heartbeat message. ignoring...");
//             return;
//         }
//         if message.trim().is_empty() {
//             println!("Gemini: blank message received\nmessage: {}", message);
//             return;
//         }
//         let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

//         let mut amount: Option<f64> = None;
//         let mut price: Option<f64> = None;

//         match data {
//             Ok(value) => {
//                 if value.get("socket_sequence").and_then(Value::as_i64) == Some(0) {
//                     println!("Gemini: socket sequence is 0, ignoring...");
//                     return;
//                 }
//                 if let Value::Object(map) = &value {
//                     if let Some(Value::Array(events)) = map.get("events") {
//                         if let Some(Value::Object(event)) = events.get(0) {
//                             amount = event.get("amount").and_then(|v| v.as_str()).and_then(|s| s.parse::<f64>().ok());
//                             price = event.get("price").and_then(|v| v.as_str()).and_then(|s| s.parse::<f64>().ok());
//                             //02/07/24 - added:
//                             if amount.is_none() || price.is_none() {
//                                 attempts += 1;
//                                 if attempts >= 3 {
//                                     panic!("Failed to parse amount: {:?} and/or price: {:?} after 3 attempts\nGemini message:\n{}", amount, price, &message);
//                                 }
//                                 continue;
//                             }
//                         }
//                     }
//                 }
//             },
//             Err(e) => {
//                 println!("Failed to parse JSON Gemini message\nError: {}\nMessage: {}", e, message);
//                 //02/07/24 - added:
//                 attempts += 1;
//                 if attempts >= 3 {
//                     panic!("Failed to parse JSON Gemini message after 3 attempts\nError: {}\nMessage: {}", e, message);
//                 }
//                 continue;
//             },
//         }
//         //02/07/24 - removed if let and resulting else branch:
//             //if let (Some(amount), Some(price)) = (amount, price) {
//         //02/07/24 - added in its place:
//         match (amount, price) {
//             (Some(amount), Some(price)) => {
//             let indices = [58, 59];
//             let new_values = [amount, price];
//             //01/16/24 - added lock:
//             //01/24/24 - added transformed_values. then removed and added scaled_values
//                 //let transformed_values: Vec<f64> = new_values.iter().map(|x: &f64| x.ln()).collect();
//                 let scaled_values: Vec<f64> = new_values.iter().map(|&x| x / divisor).collect();
//             let mut neural_network = shared_neural_network.lock().await;
//             //01/24/24 - removed and added transformed. then removed and added scaled
//                 //neural_network.update_input(&indices, &new_values).await;
//                 //neural_network.update_input(&indices, &transformed_values).await;
//                 neural_network.update_input(&indices, &scaled_values).await;
//             //02/07/24 - added break and end of match
//                 break;
//             },
//             _ => {
//                 attempts += 1;
//                 if attempts >= 3 {
//                     panic!("Failed to parse amount: {:?} and/or price: {:?} after 3 attempts\nGemini message:\n{}", amount, price, &message);
//                 }
//                 continue;
//             },
//         }
//             //to mark the inputs as changed
//             //02/07/24 - removed:
//                 //for index in indices {
//                 //    updated[index] = true;
//                 //}
//         //} else {
//             //02/07/24 - changed from:
//                 //println!("Failed to parse amount and/or price");
//                 //println!("Gemini message:\n{}", message);
//                 //panic!();
//             //to:
//                 //panic!("Failed to parse amount: {:?} and/or price: {:?}
//                 //Gemini message:\n{}", amount, price, &message);

//         //}
//         //counting the neurons for the the amount in each wallet, I will have 40 input neurons.
//     }

// }

//-----ALL-FOR-PARSING-ABOVE-THIS//




//added 01/11/24 - because I need async to be able to run this and the cycle fn at same time
//async fn read_lines(reader: BufReader<ChildStdout>, neural_network: &mut NeuralNetwork, 
//    updated: &mut [bool; 60]) {
//01/16/24 - removed
    //async fn read_lines(reader: BufReader<ChildStdout>, 
    //    neural_network: &mut MutexGuard<'_, NeuralNetwork>, updated: &mut [bool; 60]) {
//01/16/24 - added in its place
	//async fn read_lines(reader: BufReader<ChildStdout>, 
	//	shared_neural_network: Arc<Mutex<NeuralNetwork>>, updated: &mut [bool; 60]) {    
//01/24/24 - changed to:
//03/08/24 - removed, divisor: &f64
async fn read_lines(reader: BufReader<ChildStdout>, 
    shared_neural_network: Arc<Mutex<NeuralNetwork>>) {    

    for line_being_read in reader.lines() {
        //01/16/24 - added line right below this
        //01/17/24 - removed the line right below this
        //let mut neural_network = shared_neural_network.lock().await;
        //error handling in case it doesn't read Input clrrectly
        match line_being_read{
            
            //this gives me 1 line of output which is good because each message is
            //   printed in 1 line but it looks like multiple because it overflows
            Ok(line_being_read) => {

                
                //line.splintn(2, ':'): iterates over the line and splits it at  ":".
                //.collect()            collects the 2 substrings and puts it into a
                //                  Vec<&str>   which is a vector of substrings. 
                //                  Aka an array of substrings
                //parts now contains the 2 substrings split at the  :
                let parts: Vec<&str> = line_being_read.splitn(2, ':').collect();


                if parts[0] == "Message sent successfully" {
                    continue;
                }
                //this checks if parts has exactly 2 elements
                //If it doesn't this means that the input string didnt contain a colon
                //  so it is unexpected behavior. So panic (at least for now)
                //WILL NEED to implement a save state before the panic
                
                if parts.len() != 2 {
                    //02/07/24 - changed from panic! to log::error. 
                    //      Then added "continue" so it skips to next 
                    //      iteration of loop
                    log::error!("got a weird line of input. The input was\n
                            {:?}", parts);
                    continue
                }
                //gets the first element of the parts and trims leading or
                //  trailing whitespace
                let prefix = parts[0].trim();
                //this is the actual guts of the message
                let message = parts[1].trim();

                //this is operating as an if statement of if: "coinbase received"
                //  go to handle_coinbase(message) function
                //else if "kraken received"...
                //and if it's none of them, print that it's unknown and panic
                //01/16/24 - removed
                    //match prefix {
                    //    "SOL Coinbase Received" => handle_sol_coinbase(message, neural_network, updated).await,
                    //    "XLM Coinbase Received" => handle_xlm_coinbase(message, neural_network, updated).await,
                    //    "SOL Kraken Received" => handle_sol_kraken(message, neural_network, updated).await,
                    //    "XLM Kraken Received" => handle_xlm_kraken(message, neural_network, updated).await,
                    //    "SOL Bitstamp received" => handle_sol_bitstamp(message, neural_network, updated).await,
                    //    "XLM Bitstamp received" => handle_xlm_bitstamp(message, neural_network, updated).await,
                    //    "Gemini received" => handle_sol_gemini(message, neural_network, updated).await,
                    //    _ => panic!("Unknown prefix: {}", prefix),
                    //}
                //01/16/24 - added in its place. then removed on 01/24/24:
					//match prefix {
					//	"SOL Coinbase Received" => handle_sol_coinbase(message, shared_neural_network.clone(), updated, divisor).await,
					//	"XLM Coinbase Received" => handle_xlm_coinbase(message, shared_neural_network.clone(), updated, divisor).await,
					//	"SOL Kraken Received" => handle_sol_kraken(message, shared_neural_network.clone(), updated, divisor).await,
					//	"XLM Kraken Received" => handle_xlm_kraken(message, shared_neural_network.clone(), updated, divisor).await,
					//	"SOL Bitstamp received" => handle_sol_bitstamp(message, shared_neural_network.clone(), updated, divisor).await,
					//	"XLM Bitstamp received" => handle_xlm_bitstamp(message, shared_neural_network.clone(), updated, divisor).await,
					//	"Gemini received" => handle_sol_gemini(message, shared_neural_network.clone(), updated, divisor).await,
					//	_ => panic!("Unknown prefix: {}", prefix),
					//}
					//01/24/24 - was thinking of adding it like this, but changed it to do the below:
						//match prefix {
						//	"SOL Coinbase Received" => handle_sol_coinbase(message, shared_neural_network.clone(), divisor).await,
						//	"XLM Coinbase Received" => handle_xlm_coinbase(message, shared_neural_network.clone(), divisor).await,
						//	"XRP Coinbase Received"
						//	"Coinbase consolidated heartbeat"
						//	"Coinbase subscriptions"
						//	"Coinbase unknown message received"
						//	"Coinbase had non-text message"
						//	"Error reading Coinbase"
						//	"SOL Kraken Received" => handle_sol_kraken(message, shared_neural_network.clone(), divisor).await,
						//	"XLM Kraken Received" => handle_xlm_kraken(message, shared_neural_network.clone(), divisor).await,
						//	"XRP Kraken Received"
						//	"Kraken consolidated heartbeat"
						//	"Kraken system status received"
						//	"Kraken unknown message received"
						//	"Kraken had non-text message"
						//	"Error reading Kraken"
						//	"Bitstamp empty message"
						//	"Bitstamp subscription received"
						//	//"SOL Bitstamp received" => handle_sol_bitstamp(message, shared_neural_network.clone(), updated, divisor).await,
						//	"XLM Bitstamp received" => handle_xlm_bitstamp(message, shared_neural_network.clone(), divisor).await,
						//	"XRP Bitstamp received"
						//	"Bitstamp consolidated heartbeat"
						//	"Bitstamp Unknown message received"
						//	"Bitstamp had non-text message"
						//	"Error reading Bitstamp"
						//	"Gemini received solana" => handle_sol_gemini(message, shared_neural_network.clone(), divisor).await,
						//	"Error reading Gemini solana"
						//	"Gemini received xrp"
						//	"Error reading Gemini xrp"
						//	"didn't come from any of big four"
						//	"Error reading message not from big four"
						//	"Failed to connect"
						//	_ => panic!("Unknown prefix: {}", prefix),
						//}
						match prefix {
							prefix if prefix.contains("Coinbase") => 
                            //03/08/24 - removed ,divisor from hande_all_coinbase
                            execute_action_functions::handle_all_coinbase(prefix, 
                                message, shared_neural_network
                                    .clone()).await,
							prefix if prefix.contains("Kraken") => 
                            //03/08/24 - removed ,divisor from handle_all_kraken
                            execute_action_functions::handle_all_kraken(prefix, 
                                message, shared_neural_network
                                    .clone()).await,
							prefix if prefix.contains("Bitstamp") => 
                            //03/08/24 - removed ,divisor from handle_all_bitstmap
                            execute_action_functions::handle_all_bitstamp(prefix, 
                                message, shared_neural_network
                                    .clone()).await,
							prefix if prefix.contains("Gemini") => 
                            //03/08/24 - removed ,divisor from handle_all_gemini
                            execute_action_functions::handle_all_gemini(prefix, 
                                message, shared_neural_network
                                    .clone()).await,
							_ => execute_action_functions::handle_all_others(prefix, 
                                message),
						}

            },
            Err(e) => {
                //02/07/24 - changed to log::error from eprintln!
                //      do not need continue as it is last thing in loop
                log::error!("Error reading line from stdin: {}", e);
            },
        }
    }

}




















//12/23/23 code commented everything, added the new lines of code labelled below then added the return to fn main()
#[tokio::main]
//02/14/24 - removed: ->Result<(), Box<dyn Error>>
async fn main()   {

    env::set_var("RUST_BACKTRACE", "1");

    //01/28/24 - added: saves panics to a file instead of stdout or stderr
    //  so when panics occur and the websocket client continues running,
    //  I will actually be able to see what the original panic was.
    let log_file = fs::OpenOptions::new()
    .append(true)
    .create(true)
    .open(r"D:\Downloads\PxOmni\rust_log_panics\p.log")
    .unwrap();

    simplelog::CombinedLogger::init(
        vec![
            simplelog::WriteLogger::new(simplelog::LevelFilter::Info, simplelog::Config::default(), log_file),
        ]
    ).unwrap();

    log_panics::init();




    //this is just example code to evaluate if save and load of network works and it does  
    
    //01/20/24 - added:
    let replay_buffer = ReplayBuffer {
        capacity: 1, // Set this to your desired capacity
        buffer: Vec::new(),
    };

    //01/19/24 - added:
    let gradient_network = GradientNetwork {
        layers: Vec::new(),
    };
    let mut neural_network = NeuralNetwork {
        layers: Vec::new(),
        weights: Vec::new(),
        biases: Vec::new(),
        //01/19/24 - added:
        gradients: gradient_network,
        //01/20/24 - added:
        replay_buffer,
    };
    //01/24/24 - was: (65, 75, 2) now it's below. input size from execute_action_functions.

    //uncomment this if you want to initialize the network from new
        //neural_network.initialization(88, 107, 1); // Initialize with [input size], [output size], [# hidden layers]

    //uncomment this if you want to load from a saved state
        let path = "D:\\Downloads\\PxOmni\\rust_save_states\\1710831598636"; // Replace with your file path
        neural_network = NeuralNetwork::load(path).expect("couldn't load network");
        neural_network.print_layers();
    //the first number in the initialization and the number below MUST be the same size
    //01/24/24 - removed
        //let mut updated = [false; 60];
    let mut value_prior = 2000.0;
    let mut coinbase_wallet = 500.0;
    let mut bitstamp_wallet = 500.0;
    let mut kraken_wallet = 500.0;
    let mut gemini_wallet = 500.0;


	//01/24/24 - added:
	//---------------VERY IMPORTANT-------------//

	//---------------VERY IMPORTANT-------------//

	//---------------VERY IMPORTANT-------------//

	//---------------VERY IMPORTANT-------------//
	//IF YOU EVER CHANGE THIS NUMBER, MAKE SURE TO PUT ALL REPLAY BUFFERS IN NEW
	//	 FOLDER WITH ORIGINAL DIVISOR VALUE or else neural network will get wrong inputs
		//let divisor = 1_000_000.0;

    //this will allow me to do async mutex
    let shared_neural_network = Arc::new(Mutex::new(neural_network));
    //let mut neural_network = shared_neural_network.lock().await;
    //let indices = [60, 61, 62, 63, 64];
    //let new_values = [value_prior, coinbase_wallet, bitstamp_wallet, kraken_wallet, gemini_wallet];
    //neural_network.update_input(&indices, &new_values);




    //CHANGE THIS NUMBER IF RESTARTING FROM A SAVED STATE SO YOU DO THE CORRECT AMOUNT OF EXPLORATION
    let mut epsilon = 0.9055;
    //---------beginning of code so I can execute functions----------//




    //02/13/24 - for testing purposes. will remove once done:
    // let start_time = tokio::time::Instant::now(); // Capture the time before the line executes

    // let mut neural_network = shared_neural_network.lock().await; // The line of code

    // let elapsed_time = start_time.elapsed(); // Calculate the elapsed time

    // println!("Time taken: {:?}", elapsed_time); // Print the elapsed time







    
    dotenv().expect("Failed to load .env file");
    let coinbase_secret = env::var("COINBASE_SECRET_KEY").expect("SECRET_KEY must be set. check if even have .env file and if that is in it");
	let coinbase_api_key = env::var("COINBASE_API_KEY").expect("API_KEY must be set. check if even have .env file and if that is in it");
    let kraken_secret = env::var("KRAKEN_PRIVATE_KEY").expect("KRAKEN_PRIVATE_KEY must be set. check if even have .env file and if that is in it");
	let kraken_api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY must be set. check if even have .env file and if that is in it");
    let bitstamp_api_key = env::var("BITSTAMP_API_KEY").expect("could not find BITSTAMP_API_KEY spelled exactly like this. check if even have .env file");
	let bitstamp_secret = env::var("BITSTAMP_SECRET_KEY").expect("could not find BITSTAMP_SECRET_KEY spelt exactly like this in .env file. check if even have .env file");
    let gemini_api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set with exact name in .env file. check if even have .env file");
    let gemini_secret = env::var("GEMINI_SECRET_KEY").expect("GEMINI_SECRET_KEY must be set with exact name in .env file. check if even have .env file");
    //let client = reqwest::Client::new();
   
    //01/21/24 - removed:
        //println!("reached let cycle task");
        //let cycle_task = task::spawn( {
        //    let shared_neural_network = Arc::clone(&shared_neural_network);
        //    async move{
        //    
        //    //because gemini is so slow and I dont know how to update the inputs without breaking everything
        //    //  I will do a 5 minute wait so Gemini can update and then I will begin the cycles.
        //    //I will print the neural_network before each cycle to make sure the input layer and weights
        //    //  have been updated.
        //
        //    //delay_for(Duration::from_secs(10)).await;
        //                //01/17/24 - added:
        //                println!("reached let when");
        //                let when = tokio::time::Instant::now() + Duration::from_secs(10);
        //                delay_until(when).await;
        //    println!("reached for _ ");
        //    for i in 0..100_000 {
        //        //01/16/24 - added:
        //            //println!("Before delay, hopefully you get lines from websocket client being read");
        //            //delay_for(Duration::from_secs(5)).await;
        //            //println!("after delay, hopefully you this shows up in console. but just in case:\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        //        //01/17/24 - added curly braces for scope of lock
        //            //01/19/24 - added:
        //        {
        //            //println!("reached let mut neural network");
        //            let mut neural_network = shared_neural_network.lock().await;
        //            //01/20/24 - added:
        //                //why? neural network will probably lose bunches of money at first
        //                //   and I dont want the neural network to learn using balance
        //                //   under 1800 dollars.
        //            if coinbase_wallet <= 450.0 {
        //                coinbase_wallet = 500.0;
        //                //then print to new file that we reset balance at coinbase
        //                //  at certain time so that I can track which neural network
        //                //  iteration is performing good
        //                value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
        //            }
        //            if kraken_wallet <= 450.0 {
        //                kraken_wallet = 500.0;
        //                //then print to new file that we reset balance at coinbase
        //                //  at certain time so that I can track which neural network
        //                //  iteration is performing good
        //                value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
        //            }
        //            if gemini_wallet <= 450.0 {
        //                gemini_wallet = 500.0;
        //                //then print to new file that we reset balance at coinbase
        //                //  at certain time so that I can track which neural network
        //                //  iteration is performing good
        //                value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
        //            }
        //            if bitstamp_wallet <= 450.0 {
        //                bitstamp_wallet = 500.0;
        //                //then print to new file that we reset balance at coinbase
        //                //  at certain time so that I can track which neural network
        //                //  iteration is performing good
        //                value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
        //            }
        //            //01/18/24 - added to debug
        //                //println!("before weight update");
        //                //neural_network.print_layers();
        //            //01/19/24 - added to see why last inputs are not updating
        //                let indices = [60, 61, 62, 63, 64];
        //                let new_values = [value_prior, coinbase_wallet, bitstamp_wallet, kraken_wallet, gemini_wallet];
        //                neural_network.update_input(&indices, &new_values).await;
        //                //01/20/24 - added: 
        //            neural_network.cycle(i, &mut epsilon, &mut value_prior,
        //                &mut coinbase_wallet, &mut kraken_wallet, &mut bitstamp_wallet,
        //                &mut gemini_wallet, &coinbase_secret, &coinbase_api_key,
        //                &kraken_secret, &kraken_api_key, &gemini_secret,
        //                    &gemini_api_key, &bitstamp_secret, &bitstamp_api_key).await?;            
        //            //01/18/24 - added to debug:
        //                //println!("after lock. this should show up");
        //                //println!("After weight update");
        //                //neural_network.print_layers();
        //            //01/17/24 - removed:
        //                //neural_network.print_layers();
        //            //01/16/24 - added - this BREAKS THE CODE. not the drop, nor the print, but the delay_for does
        //                //drop(neural_network);
        //                //println!("Before delay, hopefully you get lines from websocket client being read");
        //                //delay_for(Duration::from_secs(5)).await;
        //                //println!("after delay, hopefully you this shows up in console. but just in case:\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        //        //01/17/24 - added curly brace. this is so the neural network is dropped after the lock
        //        let _unused_variable = neural_network.save_v2();
        //        }
        //        //01/17/24 - added:
        //            println!("5 sec delay");
        //            let when = tokio::time::Instant::now() + Duration::from_secs(5);
        //            delay_until(when).await;
        //    }
        //    Ok::<(), Box<dyn Error + Send>>(())
        //}
        //});









    //02/14/24 - added:
    //      this will allow me to close and restart my client whenever it panics
        // let is_websocket_running = Arc::new(Mutex::new(true));
        // let is_websocket_running_clone = Arc::clone(&is_websocket_running);

        




























    //01/21/24 - added:
        //what is different between the two?
        //  manual dropping of neural network
        //  separation of cycle function into 2 parts
        //      1. before state change 
        //      2. after state change
        //  moved experience replay out of cycle functions
        //  added functionality to accomodate for changes
    let folder = "D:\\Downloads\\PxOmni\\rust_replay_buffers";
    //let is_empty: Option<bool>;

//02/14/24 - MOVED CYCLE_TASK AND PUT IT IN THE LOOP BELOW

//02/14/24 - moved this to the loop below
    // let cycle_task = task::spawn( {
    //     let shared_neural_network = 
    //         Arc::clone(&shared_neural_network);
    //     async move{
    //         println!("original delay to warm up neural network...
    //         This will take 15 minutes...");
    //         let when = tokio::time::Instant::now() + tokio::time::Duration::from_secs(15*60);
    //         //02/09/24 - tokio update. removed delay_until for sleep_until
    //         tokio::time::sleep_until(when).await;






    //         //CHANGE BACK TO 0..100_000





    //         for i in 0..100_000 {
    //             //02/14/24 - added then removed:
    //             //      it is so my neural network does not
    //             //       iterate if my websocket client is not running
    //                 // while !*is_websocket_running.lock().await {
    //                 //     tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    //                 // }
    //             //01/22/24 - removed:
    //                 //is_empty = network::is_folder_empty(folder);
    //                 //if let Some(false) = is_empty {
    //             //01/22/24 - added:
	// 			//01/24/24 - last modification:
    //             let is_empty_result = network::is_folder_empty(folder);
    //             if let Ok(is_empty) = is_empty_result {
	// 				//if it's not empty & it's the 10th, then sample from transition
    //                 //01/29/24 - added if i > 200 just in case.
    //                 //02/03/24 - changed to 400
	// 				if i > 400  && is_empty == false && i%10 == 0 {
	// 					//=====NEURAL NETWORK LOCKED=====//
	// 					//===============================//
	// 						let mut neural_network = 
	// 							shared_neural_network.lock().await;
	// 						println!("I am in process of doing an experience replay");
	// 						let current_state = neural_network.layers[0].clone();
	// 						let transition_result = neural_network.replay_buffer
	// 							.sample_random_replay_buffer();
	// 						if let Ok(transition) = transition_result {
	// 							//now ready to use sampled transition for training
	// 							let state = transition.state.clone();
	// 							let index_chosen_for_current_state = transition.action;
	// 							let reward = transition.reward;
	// 							let next_state = transition.next_state.clone();

	// 							//why self.layers[0] = 0?
	// 							//  so that it uses the exp replay's input as the input
	// 							//set as input
	// 							neural_network.layers[0] = state;
	// 							//feed to make output layer new version of q-values.
	// 							//Why?
	// 							//	so basically we're going through the transition
	// 							//	again so that we can calculate more accurate
	// 							//	 q-values with our newer weights.
	// 							neural_network.feed_forward();
	// 							//for debugging
	// 							neural_network.print_last_network_layer();
	// 							//get new q_value 
	// 							let q_value_for_current_state = neural_network.layers
	// 								.last().unwrap().data[0][index_chosen_for_current_state];
	// 							//set next state as input to get target q value.
	// 							//aka the next state's max q-value with some
	// 							//	 other numbers added in there.
	// 							neural_network.layers[0] = next_state;
	// 							let target_q_value = neural_network
	// 								.calculate_target_q_value(reward);
	// 							//calculate gradients so we can update weights
	// 							neural_network
	// 								.el_backpropagation(&index_chosen_for_current_state, 
	// 									&q_value_for_current_state, &target_q_value);
	// 							//make our neural network learn from replay buffer
    //                             //02/02/24 - changed from 0.0001 to 0.00001
	// 							let learning_rate = 0.0001;
	// 							neural_network.el_update_weights(&learning_rate);






	// 							//this is to reset my input layer to what it was before the
	// 							// expReplay
	// 							neural_network.layers[0] = current_state;
	// 							//01/24/24 - for debugging:
	// 								println!("iteratrion number is: {}", i);
	// 								println!("just did an exp replay");
	// 						}
	// 						else {
    //                             //02/07/24 - replaced:
	// 							    //panic!("error when making transition");
    //                                 panic!("Error when making transition at iteration number {}
    //                                 could not sample from replay buffer. 
    //                                 The current state of the neural network is: {:?}", i, neural_network);
	// 						}
								
	// 				}
	// 				//code wont reach here if first condition is met. so will
	// 				//	 change top so it's if is_empty == false && i%10 == 0.
	// 				//code will now reach here if one of the top conditions
	// 				//	 is not met. so folder can be non-empty and not 10th
	// 				//	 iteration of loop and it will go to this "else"
    //                 else {
                    
    //                     //=====NEURAL NETWORK LOCKED=====//
    //                     //===============================//
    //                         let mut neural_network =
    //                             shared_neural_network.lock().await;
                            
        
    //                         //for experience replay
    //                         let input_data = neural_network.layers[0].data.clone();
    //                         //state stuff
    //                         let input_rows = neural_network.layers[0].rows;
    //                         let input_columns = neural_network.layers[0].columns;
        
    //                         let state = NetworkLayer {
    //                             rows: input_rows,
    //                             columns: input_columns,
    //                             data: input_data,
    //                         };
        
    //                             //why? neural network will probably lose bunches of money at first
    //                             //   and I dont want the neural network to learn using balance
    //                             //   under 1800 dollars.
    //                         if coinbase_wallet <= 450.0 {
    //                             coinbase_wallet = 500.0;
    //                             //then print to new file that we reset balance at coinbase
    //                             //  at certain time so that I can track which neural network
    //                             //  iteration is performing good
    //                             value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
    //                         }
    //                         if kraken_wallet <= 450.0 {
    //                             kraken_wallet = 500.0;
    //                             //then print to new file that we reset balance at coinbase
    //                             //  at certain time so that I can track which neural network
    //                             //  iteration is performing good
    //                             value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
    //                         }
    //                         if gemini_wallet <= 450.0 {
    //                             gemini_wallet = 500.0;
    //                             //then print to new file that we reset balance at coinbase
    //                             //  at certain time so that I can track which neural network
    //                             //  iteration is performing good
    //                             value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
    //                         }
    //                         if bitstamp_wallet <= 450.0 {
    //                             bitstamp_wallet = 500.0;
    //                             //then print to new file that we reset balance at coinbase
    //                             //  at certain time so that I can track which neural network
    //                             //  iteration is performing good
    //                             value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
    //                         }
        
        
                            
    //                         //this value of the wallets for the inputs
    //                         let indices = [60, 61, 62, 63, 64];
    //                         let new_values = [value_prior, coinbase_wallet, 
    //                             bitstamp_wallet, kraken_wallet, gemini_wallet];

	// 						let scaled_values: Vec<f64> = new_values.iter().map(|&x| x / divisor).collect();
	// 						neural_network.update_input(&indices, &scaled_values).await;
	// 						neural_network.print_input_layer();
    //                             //01/20/24 - added: 
    //                         let (index_chosen_for_current_state, q_value_for_current_state, 
    //                             the_reward);
	// 						println!("iteration number is this: {}", i);
    //                         //this is to get us the values that part two is going ot use
    //                         let result = neural_network.cycle_part_one_of_two(i, &mut epsilon, 
    //                             &mut value_prior, &mut coinbase_wallet, &mut kraken_wallet,
    //                             &mut bitstamp_wallet, &mut gemini_wallet, &coinbase_secret,
    //                             &coinbase_api_key, &kraken_secret, &kraken_api_key, &gemini_secret,
    //                             &gemini_api_key, &bitstamp_secret, &bitstamp_api_key, &divisor).await;
        
    //                         //01/23/24 - removed:
    //                             //match result {
    //                                 //Ok(values) => {
    //                                     //index_chosen_for_current_state = values.0;
    //                                     //q_value_for_current_state = values.1;
    //                                     //the_reward = values.2;
    //                                 //},
    //                                 //Err(e) => eprintln!("An error occurred: {}", e),
    //                             //}
    //                         //01/23/24 - added:
    //                         index_chosen_for_current_state = result.0;
    //                         q_value_for_current_state = result.1;
    //                         the_reward = result.2;
	// 						//01/24/24 - added line directly below:
	// 						println!("chosen index:{}", &index_chosen_for_current_state);
        
    //                     //----------------NEURAL-NETWORK-DROPPED-----------------//
    //                         drop(neural_network);
    //                         //02/13/24 - changed from 1 sec to 50 millisec. this should allow for more
    //                         //      immediate state to be seen. I just want enough time for all the inputs
    //                         //      in the queue to be realized. Hopefully this is enough time
    //                         println!("target q value 50 millisec delay for input queue to be finished");
    //                         //02/09/24 - changed Duration from std::time::duration to tokio::time::Duration
    //                         let when = tokio::time::Instant::now() + tokio::time::Duration::from_millis(50);
    //                         //02/09/24 - tokio updated. removed delay_until. replaced with sleep_until
    //                         tokio::time::sleep_until(when).await;
        
        
        
        
    //                     //=====NEURAL NETWORK LOCKED=====//
    //                     //===============================//
    //                         let mut neural_network = 
    //                             shared_neural_network.lock().await;
    //                         let next_state_input_layer_clone = neural_network.layers[0].clone();
    //                         let transition = Transition {
    //                             state,
    //                             //01/23/24 - removed:
    //                                 //action,
    //                             //01/23/24 - added:
    //                             action: index_chosen_for_current_state,
    //                             reward : the_reward,
    //                             next_state : next_state_input_layer_clone,
    //                         };
    //                         neural_network.cycle_part_two_of_two(index_chosen_for_current_state,
    //                             q_value_for_current_state, the_reward);
    //                         let _unused_variable = neural_network.save_v2();
        
        
    //                     //----------------NEURAL-NETWORK-DROPPED-----------------//
    //                         drop(neural_network);
        
    //                         //save replay
    //                         //I have to make a new replay buffer variable because if I dont then
    //                         //   Im doing an illegal move.
    //                         //I dont think this will be an issue with the 2 replay buffers
    //                         //	because I'm not getting the transition from the buffer itself.
	// 						//Instead, I'm getting/writing the transition from/to the file
	// 						//	 that contains all the transitions.
    //                         let mut replay_buffer = ReplayBuffer::new(1);
    //                         replay_buffer.push(transition);
    //                         let _dummyvar = replay_buffer.save_replay_buffer_v2();
    //                         //02/13/24 - added:
    //                             println!("200 millisec delay for new inputs");
    //                             let when = tokio::time::Instant::now() + tokio::time::Duration::from_millis(200);
    //                             tokio::time::sleep_until(when).await;
    //                     }
    //             }
    //             else {
    //                 //upgraded panic to log error from just saying there was no empty result to:
    //                 log::error!("there was no empty_result.
    //                 iteration number: {}
    //                 folder path: {}", &i, &folder);

    //             }
    //             //01/22/24 - removed:
    //             /*
    //                 else {
                        
    //                 //=====NEURAL NETWORK LOCKED=====//
    //                 //===============================//
    //                     let mut neural_network =
    //                         shared_neural_network.lock().await;
                        

    //                     //for experience replay
    //                     let input_data = neural_network.layers[0].data.clone();
    //                     //state stuff
    //                     let input_rows = neural_network.layers[0].rows;
    //                     let input_columns = neural_network.layers[0].columns;

    //                     let state = NetworkLayer {
    //                         rows: input_rows,
    //                         columns: input_columns,
    //                         data: input_data,
    //                     };

    //                         //why? neural network will probably lose bunches of money at first
    //                         //   and I dont want the neural network to learn using balance
    //                         //   under 1800 dollars.
    //                     if coinbase_wallet <= 450.0 {
    //                         coinbase_wallet = 500.0;
    //                         //then print to new file that we reset balance at coinbase
    //                         //  at certain time so that I can track which neural network
    //                         //  iteration is performing good
    //                         value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
    //                     }
    //                     if kraken_wallet <= 450.0 {
    //                         kraken_wallet = 500.0;
    //                         //then print to new file that we reset balance at coinbase
    //                         //  at certain time so that I can track which neural network
    //                         //  iteration is performing good
    //                         value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
    //                     }
    //                     if gemini_wallet <= 450.0 {
    //                         gemini_wallet = 500.0;
    //                         //then print to new file that we reset balance at coinbase
    //                         //  at certain time so that I can track which neural network
    //                         //  iteration is performing good
    //                         value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
    //                     }
    //                     if bitstamp_wallet <= 450.0 {
    //                         bitstamp_wallet = 500.0;
    //                         //then print to new file that we reset balance at coinbase
    //                         //  at certain time so that I can track which neural network
    //                         //  iteration is performing good
    //                         value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
    //                     }


                        
    //                     //this value of the wallets for the inputs
    //                     let indices = [60, 61, 62, 63, 64];
    //                     let new_values = [value_prior, coinbase_wallet, 
    //                         bitstamp_wallet, kraken_wallet, gemini_wallet];
    //                     neural_network.update_input(&indices, &new_values).await;

    //                         //01/20/24 - added: 
    //                     let (index_chosen_for_current_state, q_value_for_current_state, 
    //                         the_reward);

    //                     //this is to get us the values that part two is going ot use
    //                     result = neural_network.cycle_part_one_of_two(i, &mut epsilon, 
    //                         &mut value_prior, &mut coinbase_wallet, &mut kraken_wallet,
    //                         &mut bitstamp_wallet, &mut gemini_wallet, &coinbase_secret,
    //                         &coinbase_api_key, &kraken_secret, &kraken_api_key, &gemini_secret,
    //                         &gemini_api_key, &bitstamp_secret, &bitstamp_api_key).await;

    //                     match result {
    //                         Ok(values) => {
    //                             index_chosen_for_current_state = values.0;
    //                             q_value_for_current_state = values.1;
    //                             the_reward = values.2;
    //                             println!("The function returned: {}, {}, {}", i, f1, f2);
    //                         },
    //                         Err(e) => eprintln!("An error occurred: {}", e),
    //                     }



    //                 //----------------NEURAL-NETWORK-DROPPED-----------------//
    //                     drop(neural_network);
    //                     println!("1 sec delay for new inputs");
    //                     let when = tokio::time::Instant::now() + Duration::from_secs(1);
    //                     delay_until(when).await;




    //                 //=====NEURAL NETWORK LOCKED=====//
    //                 //===============================//
    //                     let mut neural_network = 
    //                         shared_neural_network.lock().await;
    //                     let next_state_input_layer_clone = neural_network.layers[0].clone();
    //                     let transition = Transition {
    //                         state,
    //                         action,
    //                         reward : the_reward,
    //                         next_state : next_state_input_layer_clone,
    //                     };
    //                     neural_network.cycle_part_two_of_two(index_chosen_for_current_state,
    //                         q_value_for_current_state, the_reward);
    //                     let _unused_variable = neural_network.save_v2();


    //                 //----------------NEURAL-NETWORK-DROPPED-----------------//
    //                     drop(neural_network);

    //                     //save replay
    //                     replay_buffer.push(transition);
    //                     let _dummyvar = replay_buffer.save_replay_buffer_v2();
    //                 }
    //             */
    //         }
    //         Ok::<(), Box<dyn Error + Send>>(())
    //     }
    // });





































    //02/14/24 - added and removed:
    //      this will allow me to close and restart my client whenever it panics
    //let cycle_task = Arc::new(Mutex::new(cycle_task));

    // let websocket_client_task = {
    //     let is_websocket_running = is_websocket_running_clone;
    //     //let cycle_task = Arc::clone(&cycle_task);
    //     task::spawn(async move {
    //         loop {
    //             let mut websocket_client = Command::new(
    //                 r"C:\Users\djv60\projects\testw\target\debug\testw.exe")
    //                 .stdout(Stdio::piped())
    //                 .spawn()
    //                 .expect("Failed to start WebSocket client");
    
    //             let websocket_client_stdout = websocket_client.stdout.take().expect("Failed to get stdout");
    //             let reader = BufReader::new(websocket_client_stdout);
    
    //             let read_lines_task = task::spawn({
    //                 let shared_neural_network = Arc::clone(&shared_neural_network);
    //                 async move {
    //                     read_lines(reader, shared_neural_network, &divisor).await;
    //                 }
    //             });
    
    //             tokio::select! {
    //                 cycle_result = cycle_task => {
    //                     match cycle_result {
    //                         Ok(result) => {
    //                             // Handle successful completion of cycle_task.
    //                         }
    //                         Err(err) => {
    //                             // Handle error from cycle_task.
    //                             *is_websocket_running.lock().await = false;
    //                             let _ = websocket_client.kill();
    //                             log::error!("client panicked. Probably coinbase, let's be honest,
    //                                 killed it and now waiting 15 minutes 
    //                                 JUST IN CASE       I missed a gemini price change
    //                                 for it to start back up.");
    //                             tokio::time::sleep(tokio::time::Duration::from_secs(15*60)).await;
    //                         }
    //                     }
    //                 }
    //                 read_lines_result = read_lines_task => {
    //                     match read_lines_result {
    //                         Ok(result) => {
    //                             // Handle successful completion of read_lines_task.
    //                         }
    //                         Err(err) => {
    //                             // Handle error from read_lines_task.
    //                             *is_websocket_running.lock().await = false;
    //                             let _ = websocket_client.kill();
    //                             log::error!("client panicked. Probably coinbase, let's be honest,
    //                                 killed it and now waiting 15 minutes 
    //                                 JUST IN CASE       I missed a gemini price change
    //                                 for it to start back up.");
    //                             tokio::time::sleep(tokio::time::Duration::from_secs(15*60)).await;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     })
    // };







//02/14/24 - added all of it:
    //this is to go back to the iteration I was previously at. 
    //02/16/24 - changed from 0 to 6033 because I ended at 6043. then at 6775.
    //02/17/24 - changed to 22350
    //02/19/24 - chagned to 0. 02/20/24 - 22000
	//02/23/24 - removed:
    	//let mut iteration_counter_for_for_loop_total: usize = 22_000;
    //02/23/24 - added in its place:
    //02/27/24 - changed to 26698.
    //02/28/24 - restarting at 0
    //03/19/24 - changed to 8946 then 9826
    //03/20/24 - changed to 18900
		let iteration_counter_for_for_loop_total = 
			Arc::new(Mutex::new(18900));
    
    loop {
        // Create a new AtomicBool wrapped in an Arc for each iteration
        //      This allows us to safely share and modify this value 
        //      across tasks.
        let should_stop = 
            Arc::new(AtomicBool::new(false));
        // Clone the Arc for each task. 
        //  This gives each task its own reference to the shared AtomicBool.
            let stop1 = should_stop.clone();
            let stop1_clone = Arc::clone(&stop1);

		//02/23/24 - removed:
        	//let mut iteration_counter_for_for_loop_this_iteration: usize = 0;
			//let mut replay_buffer_counter_for_this_iteration: usize = 0;
		//02/23/24 - added in its place:
			let iteration_counter_for_for_loop_this_iteration =
				Arc::new(Mutex::new(0));
			let replay_buffer_counter_for_this_iteration =
				Arc::new(Mutex::new(0));


        // Clone the Arc for cycle_task. This does not create a deep copy of 
        //  the NeuralNetwork.
        // Instead, it creates a new Arc that points to the same NeuralNetwork 
        //  instance.
        // Any modifications made to the NeuralNetwork through this Arc will
        // be visible through the original Arc.
        let shared_neural_network_clone1 = Arc::clone(&shared_neural_network);
		let iteration_counter_for_for_loop_total_clone = Arc::clone(&iteration_counter_for_for_loop_total);
		let replay_buffer_counter_for_this_iteration_clone = Arc::clone(&replay_buffer_counter_for_this_iteration);
		let iteration_counter_for_for_loop_this_iteration_clone = Arc::clone(&iteration_counter_for_for_loop_this_iteration);

        let coinbase_secret_clone = coinbase_secret.clone(); // Clone coinbase_secret
        let kraken_secret_clone = kraken_secret.clone();
        let gemini_secret_clone = gemini_secret.clone();
        let bitstamp_secret_clone = bitstamp_secret.clone();

        let coinbase_api_key_clone = coinbase_api_key.clone();
        let bitstamp_api_key_clone = bitstamp_api_key.clone();
        let gemini_api_key_clone = gemini_api_key.clone();
        let kraken_api_key_clone = kraken_api_key.clone();

        let cycle_task = task::spawn({
            let stop1 = stop1_clone;
            let shared_neural_network = 
                shared_neural_network_clone1;

            let coinbase_secret = coinbase_secret_clone; // Use the clone here
            let kraken_secret = kraken_secret_clone;
            let gemini_secret = gemini_secret_clone;
            let bitstamp_secret = bitstamp_secret_clone;
            let coinbase_api_key = coinbase_api_key_clone;
            let kraken_api_key = kraken_api_key_clone;
            let gemini_api_key = gemini_api_key_clone;
            let bitstamp_api_key = bitstamp_api_key_clone;

			let replay_buffer_counter_for_this_iteration = replay_buffer_counter_for_this_iteration_clone;
			let iteration_counter_for_for_loop_this_iteration = iteration_counter_for_for_loop_this_iteration_clone;
			let iteration_counter_for_for_loop_total = iteration_counter_for_for_loop_total_clone;

            async move{
                println!(" delay to warm up neural network...
            This will take 15 minutes...");
            let when = tokio::time::Instant::now() +
                 tokio::time::Duration::from_secs(3 * 60);
            tokio::time::sleep_until(when).await;

                //start loop at whatever number the counter is at
                //02/17/24 - changed from ..100_000 to 200_000
				//02/23/24 - modified from:
					// for i in 
					// iteration_counter_for_for_loop_total..200_000 {
				//02/23/24 - to this to accomodate for changing it for mutex

				let start = *iteration_counter_for_for_loop_total.lock().await;
				for i in start..200_000 {
                    // Check the should_stop flag at the beginning of 
                    //  each iteration
                    if stop1.load(Ordering::Relaxed) {
                        // If the flag is set to true, break the loop 
                        //  and stop the task
                        break; 
                    }
                //beginning of original code.
                    let is_empty_result = network::is_folder_empty(folder);
                    if let Ok(is_empty) = is_empty_result {

                        if i > 400  && is_empty == false && i%10 == 0 {
                            //=====NEURAL NETWORK LOCKED=====//
                            //===============================//
                                let mut neural_network = 
                                    shared_neural_network.lock().await;
                                println!("I am in process of doing an experience replay");
                                let current_state = neural_network.layers[0].clone();
                                let transition_result = neural_network.replay_buffer
                                    .sample_random_replay_buffer();
                                if let Ok(transition) = transition_result {
                                    //now ready to use sampled transition for training
                                    let state = transition.state.clone();
                                    let index_chosen_for_current_state = transition.action;
                                    let reward = transition.reward;
                                    let next_state = transition.next_state.clone();

                                    //why self.layers[0] = 0?
                                    //  so that it uses the exp replay's input as the input
                                    //set as input
                                    neural_network.layers[0] = state;
                                    //feed to make output layer new version of q-values.
                                    //Why?
                                    //	so basically we're going through the transition
                                    //	again so that we can calculate more accurate
                                    //	 q-values with our newer weights.
                                    neural_network.feed_forward();
                                    //for debugging
                                    neural_network.print_last_network_layer();
                                    //get new q_value 
                                    let q_value_for_current_state = neural_network.layers
                                        .last().unwrap().data[0][index_chosen_for_current_state];
                                    //set next state as input to get target q value.
                                    //aka the next state's max q-value with some
                                    //	 other numbers added in there.
                                    neural_network.layers[0] = next_state;
                                    let target_q_value = neural_network
                                        .calculate_target_q_value(reward);
                                    //calculate gradients so we can update weights
                                    neural_network
                                        .el_backpropagation(&index_chosen_for_current_state, 
                                            &q_value_for_current_state, &target_q_value);
                                    //make our neural network learn from replay buffer
                                    //02/02/24 - changed from 0.0001 to 0.00001
                                    //03/09/24 - changed to 0.1
                                    //03/10/24 - changed to 1.0
                                    //03/12/24 - changed to 0.1. 03/13/24  - to 0.01 then 0.001 then 0.0001
                                    let learning_rate = 0.0001;
                                    neural_network.el_update_weights(&learning_rate);
                                    
                                    //03/10/24 - added. 03/13/24 - added i
                                    neural_network.save_all_gradients(&i);
                                    //03/12/24 - added: 03/13/24 - added i
                                    neural_network.save_all_weights(&i);




                                    //this is to reset my input layer to what it was before the
                                    // expReplay
                                    neural_network.layers[0] = current_state;
                                    //01/24/24 - for debugging:
                                        println!("iteratrion number is: {}", i);
                                        println!("just did an exp replay");
                                }
                                else {
                                    //02/07/24 - replaced:
                                        //panic!("error when making transition");
                                        panic!("Error when making transition at iteration number {}
                                        could not sample from replay buffer. 
                                        The current state of the neural network is: {:?}", i, neural_network);
                                }
                                //03/14/24 - added to save after every iteration:
                                let _unused_variable = neural_network.save_v2();
                                    
                        }

                        //so if replay_buffer is empty or i<= 400 or i cannot be cleanly divided by 10 then...
                        else {
                        
                            //=====NEURAL NETWORK LOCKED=====//
                            //===============================//
                                let mut neural_network =
                                    shared_neural_network.lock().await;
                                
            
                                //for experience replay
                                let input_data = neural_network.layers[0].data.clone();
                                //state stuff
                                let input_rows = neural_network.layers[0].rows;
                                let input_columns = neural_network.layers[0].columns;
            
                                let state = NetworkLayer {
                                    rows: input_rows,
                                    columns: input_columns,
                                    data: input_data,
                                };
            


                                    //why? neural network will probably lose bunches of money at first
                                    //   and I dont want the neural network to learn using balance
                                    //   under 1800 dollars.
                                if coinbase_wallet <= 450.0 {
                                    coinbase_wallet = 500.0;
                                    //later print to new file that we reset balance at coinbase
                                    //  at certain time so that I can track which neural network
                                    //  iteration is performing good
                                    value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
                                }
                                if kraken_wallet <= 450.0 {
                                    kraken_wallet = 500.0;
                                    //later print to new file that we reset balance at coinbase
                                    //  at certain time so that I can track which neural network
                                    //  iteration is performing good
                                    value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
                                }
                                if gemini_wallet <= 450.0 {
                                    gemini_wallet = 500.0;
                                    //later print to new file that we reset balance at coinbase
                                    //  at certain time so that I can track which neural network
                                    //  iteration is performing good
                                    value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
                                }
                                if bitstamp_wallet <= 450.0 {
                                    bitstamp_wallet = 500.0;
                                    //later print to new file that we reset balance at coinbase
                                    //  at certain time so that I can track which neural network
                                    //  iteration is performing good
                                    value_prior = coinbase_wallet + bitstamp_wallet + gemini_wallet + kraken_wallet;
                                }
            
            
                                
                                //this value of the wallets for the inputs
                                //03/09/24 - added:
                                let value_prior_unscaled = value_prior;
                                let coinbase_wallet_unscaled = coinbase_wallet;
                                let gemini_wallet_unscaled = gemini_wallet;
                                let bitstamp_wallet_unscaled = bitstamp_wallet;
                                let kraken_wallet_unscaled = kraken_wallet;
                                //03/08/24 - updated indices
                                let indices = [56, 57, 58, 59, 60];
                                //scale everything
                                value_prior = standardization_functions::normal_value_prior_standardization(&value_prior);
                                coinbase_wallet = standardization_functions::normal_wallet_standardization(&coinbase_wallet);
                                bitstamp_wallet = standardization_functions::normal_wallet_standardization(&bitstamp_wallet); 
                                kraken_wallet = standardization_functions::normal_wallet_standardization(&kraken_wallet);
                                gemini_wallet = standardization_functions::normal_wallet_standardization(&gemini_wallet);
                                //put scaled values in an array to then one to one match them to the index and update that index
                                let scaled_values = [value_prior, coinbase_wallet, 
                                    bitstamp_wallet, kraken_wallet, gemini_wallet];

                                //updates input
                                //let scaled_values: Vec<f64> = new_values.iter().map(|&x| x / divisor).collect();
                                neural_network.update_input(&indices, &scaled_values).await;
                                neural_network.print_input_layer();

                                let (index_chosen_for_current_state, q_value_for_current_state, 
                                    the_reward);
                                println!("iteration number is this: {}", i);
                                //03/09/24 - added to UNscale value_after and wallets for the cycle funciton
                                    value_prior = value_prior_unscaled;
                                    coinbase_wallet = coinbase_wallet_unscaled;
                                    gemini_wallet = gemini_wallet_unscaled;
                                    kraken_wallet = kraken_wallet_unscaled;
                                    bitstamp_wallet = bitstamp_wallet_unscaled;
                                //this is to get us the values that part two is going ot use
                                let result = neural_network.cycle_part_one_of_two(i, &mut epsilon, 
                                    &mut value_prior, &mut coinbase_wallet, &mut kraken_wallet,
                                    &mut bitstamp_wallet, &mut gemini_wallet, &coinbase_secret,
                                    &coinbase_api_key, &kraken_secret, &kraken_api_key, &gemini_secret,
                                    &gemini_api_key, &bitstamp_secret, &bitstamp_api_key).await;
            
                                index_chosen_for_current_state = result.0;
                                q_value_for_current_state = result.1;
                                the_reward = result.2;

                                println!("chosen index:{}", &index_chosen_for_current_state);
            
                            //----------------NEURAL-NETWORK-DROPPED-----------------//
                                drop(neural_network);
                                println!("target q value 50 millisec delay for input queue to be finished");
                                let when = tokio::time::Instant::now() + tokio::time::Duration::from_millis(50);
                                tokio::time::sleep_until(when).await;
            
            
            
            
                            //=====NEURAL NETWORK LOCKED=====//
                            //===============================//
                                let mut neural_network = 
                                    shared_neural_network.lock().await;
                                let next_state_input_layer_clone = neural_network.layers[0].clone();
                                let transition = Transition {
                                    state,
                                    action: index_chosen_for_current_state,
                                    reward : the_reward,
                                    next_state : next_state_input_layer_clone,
                                };
                                neural_network.cycle_part_two_of_two(index_chosen_for_current_state,
                                    q_value_for_current_state, the_reward, &i);
                                let _unused_variable = neural_network.save_v2();
            
            
                            //----------------NEURAL-NETWORK-DROPPED-----------------//
                                drop(neural_network);
            
                                //save replay
                                //I have to make a new replay buffer variable because if I dont then
                                //   Im doing an illegal move.
                                //I dont think this will be an issue with the 2 replay buffers
                                //	because I'm not getting the transition from the buffer itself.
                                //Instead, I'm getting/writing the transition from/to the file
                                //	 that contains all the transitions.
                                let mut replay_buffer = ReplayBuffer::new(1);
                                replay_buffer.push(transition);
                                //02/14/24 - changed to actually do something wtih error
                                match replay_buffer.save_replay_buffer_v2() {
                                    Ok(_) => {
										//02/23/24 - added:
											let iteration_counter_for_for_loop_this_iteration = 
												iteration_counter_for_for_loop_this_iteration.lock().await;
											let iteration_counter_for_for_loop_total = 
												iteration_counter_for_for_loop_total.lock().await;
                                        log::info!("successfully saved replay_buffer at this loop's 
                                        iteration: {}
                                        total iteration: {}",
                                         *iteration_counter_for_for_loop_this_iteration, 
                                         *iteration_counter_for_for_loop_total );

										 let mut replay_buffer_counter_for_this_iteration = 
										 	replay_buffer_counter_for_this_iteration.lock().await;

										 *replay_buffer_counter_for_this_iteration+=1;
                                    },
                                    Err(e) => {
                                        // Handle the error case here.
										let iteration_counter_for_for_loop_this_iteration = 
											iteration_counter_for_for_loop_this_iteration.lock().await;
										let iteration_counter_for_for_loop_total = 
											iteration_counter_for_for_loop_total.lock().await;

                                        log::error!("An error occurred while saving replay buffer at this loop's
                                        iteration: {}
                                        total iteration: {}
                                        error: {}", 
										*iteration_counter_for_for_loop_this_iteration, 
                                        *iteration_counter_for_for_loop_total, e);
                                        return;
                                    },
                                };



                                println!("200 millisec delay for new inputs");
                                let when = tokio::time::Instant::now() + tokio::time::Duration::from_millis(200);
                                tokio::time::sleep_until(when).await;
                            }
                    }
                    else {
                        log::error!("there was no empty_result.
                        iteration number: {}
                        folder path: {}", &i, &folder);

                    }

                //end of original code. also removed the Ok() box error thing
                    //this is to get how many times it was incremented this
                    // go-around
					//02/23/24 - from:
                    	//iteration_counter_for_for_loop_this_iteration+=1;
					//02/23/24 - changed to:
					let mut iteration_counter_for_for_loop_this_iteration = 
						iteration_counter_for_for_loop_this_iteration.lock().await;
					*iteration_counter_for_for_loop_this_iteration+=1;
                }
            
            }
        });
        
        let mut websocket_client = 
        Command::new(r"C:\Users\djv60\projects\testw\target\debug\testw.exe")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start WebSocket client");

    
        let websocket_client_stdout = websocket_client.stdout.take().expect("Failed to get stdout");
        let reader = BufReader::new(websocket_client_stdout);
    
        let shared_neural_network_clone2 = 
            Arc::clone(&shared_neural_network);
        let read_lines_task = task::spawn({
            let shared_neural_network = shared_neural_network_clone2;
            async move{
                //03/08/24 - removed: , &divisor
                read_lines(reader, shared_neural_network).await;
            }
        });
    


        //02/23/24 - added:
        //	super complicated but basically tokio::select! creates its own
        //      context so it captures variables by value, not by reference.
		//	so because replay_buffer_counter_for_this_iteration was initialized
		//		as 0 initially, tokio::select! has it now as 0.
		//That being said:
		//	the way to bypass this is to *WRAP* the data so tokio::select! takes
		//		a snapshot of the wrapper so that we can then go forth with
		//		"interior mutability". 
		//How do we do this?
		//	You use Cell to wrap it. I will need to do this to the following vars:
		//		replay_buffer_counter
		//		both iteration_counter's



        tokio::select! {
            _ = read_lines_task => {
                // This branch is executed if read_lines_task finishes first (either 
                //      successfully or with a panic)
                // Handle panic in read_lines_task here
                    // Stop cycle_task from doing more loops
                        stop1.store(true, Ordering::Relaxed); 
                    //if replay_buffer_counter_for_this_iteration >= 9, delete most recent 9
                    //else  delete those
                    //if iteration_counter_for_for_loop_this_iteration > 8, delete most
                    //      recent 8. then add this number to 
                    //      iteration_counter_for_for_loop_total
                    //kill websocket_client
					//02/23/24 - generic update everywhere that adds .get() to almost any call to
					//		replay_buffer_counter
					//02/23/24 - added:
					let replay_buffer_counter_for_this_iteration =
						replay_buffer_counter_for_this_iteration.lock().await;
					let iteration_counter_for_for_loop_this_iteration = 
						iteration_counter_for_for_loop_this_iteration.lock().await;
					let mut iteration_counter_for_for_loop_total = 
						iteration_counter_for_for_loop_total.lock().await;



                    if *replay_buffer_counter_for_this_iteration >= 9 {
                        //02/23/24 - added:
                        log::info!("deleting last 9 replays");
                        let _ = delete_most_recent_replay_buffers(9);
                    }
                    else {
                        log::info!("there weren't 9 replays so only deleting:
                        {}", *replay_buffer_counter_for_this_iteration);
                        let _ = delete_most_recent_replay_buffers(*replay_buffer_counter_for_this_iteration);
                    }
                    if *iteration_counter_for_for_loop_this_iteration > 8 {
						//02/23/24 - removed and replaced with
                        	//iteration_counter_for_for_loop_total += 
							//iteration_counter_for_for_loop_this_iteration - 8;
							*iteration_counter_for_for_loop_total +=
								*iteration_counter_for_for_loop_this_iteration - 8;

                    }

                        let kill_result = websocket_client.kill();
                        match kill_result {
                            Ok(_) => {
                                log::info!("Successfully killed the websocket client
                                    at iteration: {}
                                    this loop's iteration: {}.", 
									*iteration_counter_for_for_loop_total, 
                                    *iteration_counter_for_for_loop_this_iteration);
                            },
                            Err(e) => {
                                panic!("Failed to kill the websocket client: {:?}
                                total iteration: {}
                                this loop's iteration: {}", e,
								*iteration_counter_for_for_loop_total, 
                                *iteration_counter_for_for_loop_this_iteration);
                            }
                        }
                    //wait 5 minutes
                    let when = tokio::time::Instant::now() + tokio::time::Duration::from_secs(5*60);
                    tokio::time::sleep_until(when).await;
                    //restart loop
                    //continue;
            }
            res = cycle_task => {
                // This branch is executed if cycle_task finishes first (either successfully or with a panic)
				//02/23/24 - from:
                	//iteration_counter_for_for_loop_total += iteration_counter_for_for_loop_this_iteration;
				//02/23/24 - to:
					let mut iteration_counter_for_for_loop_total = iteration_counter_for_for_loop_total.lock().await;
					let iteration_counter_for_for_loop_this_iteration = iteration_counter_for_for_loop_this_iteration.lock().await;

					*iteration_counter_for_for_loop_total += *iteration_counter_for_for_loop_this_iteration;
                match res {
                    Ok(_) => {
                        // If cycle_task finishes, panic the whole program
                        panic!("cycle_task finished. total iteration count was {}.
                        check if this is equal to the max that Im iterating for.
                        if it is that means it finished successfully", 
                        *iteration_counter_for_for_loop_total );
                    }
                    Err(e) => {
                        // If cycle_task panics, panic the whole program
                        panic!("cycle_task panicked: {:?}
                        total iteration count was {}", e, 
						*iteration_counter_for_for_loop_total);
                    }
                }

            }
        }
        
    }






























    
    //// Print the network
    //neural_network.print_layers();

    

    //// Save the network
    //neural_network.save_v2()?;

    //// Load the network
    //let path = "D:\\Downloads\\PxOmni\\rust_save_states\\1703492925570"; // Replace with your file path
    //let loaded_network = NeuralNetwork::load(path)?;

    //// Print the loaded network
    //loaded_network.print_layers();

    //let right_now = Instant::now();   //to measure execution time
    //neural_network.feed_forward();
    //let elapsed = right_now.elapsed();
    //println!("Elapsed: {:?}", elapsed);




    //why do I preface the "" with r?
    //                  because this tells Rust that it is a string literal and the
    //                  \ does not have to be escaped, like a \t or \n.
    //Command::new      starts the path inside the ( ) as a new process.
    //.stdout(Stdio::piped())
    //                  sends the output of the executable ran in the parentheses
    //                   of the Command::new to a pipe.
    //what is a pipe:   basically a direct link between 2 programs where the output of
    //                   1 program becomes the input of the other program.
    //why do I have to send it to a pipe?
    //                  because the output of the WebSocket client would be printed
    //                   directly to the console and not read by the parser.
    //.spawn()          starts the WebSocket client as a new process.
    //                  It allows the process aka the client to run independenly of the
    //                   parser program that started it.
    //.expect()         This is just super basic error handling.
    // let websocket_client = Command::new(
    //         r"C:\Users\djv60\projects\testw\target\debug\testw.exe")
    //         .stdout(Stdio::piped())
    //         .spawn()
    //         .expect("Failed to start WebSocket client");

    

    // //BufReader::new(....)
    // //                  This is creating a new "buffered reader"
    // //what is a buffered reader?
    // //                  a type of reader that adds buffering.
    // //why?              buffering is a process of storing data temporarily in memory
    // //                  ,aka the buffer, while it's being moved from 1 place to another
    // //why?              It significantly improves performance when reading large amounts
    // //                   of data because w/o buffering the parser program would have to 
    // //                   ask the OS to read each byte of data individually. so super slow. 
    // //                   so with buffering I can read large blocks of data at once
    // //websocket_client.stdout
    // //                  this is getting the standard output (stdout) of hte websocket client
    // //                   process that was spawned.
    // //.expect()
    // //                  It is type Option<...> so that means if it doesnt find anything in 
    // //                  the stdout it will execute the .expect() and panic

    // let reader = BufReader::new(websocket_client.stdout
    //         .expect("Failed to get stdout"));
    // //let read_lines_task = task::spawn(async {
    // //    read_lines(reader, &mut neural_network, &mut updated).await;
    // //});
    // let read_lines_task = task::spawn( {
    //     let shared_neural_network = Arc::clone(&shared_neural_network);
    //     //01/24/24 - removed:
    //         //let mut updated = updated.clone();
    //     async move{
    //         //01/16/24 - removed
    //             //read_lines(reader, &mut shared_neural_network.lock().await, &mut updated).await;
    //         //01/16/24 - added in its place
    //             //read_lines(reader, shared_neural_network, &mut updated).await;
	// 		//01/16/24 - changed to:
	// 			//read_lines(reader, shared_neural_network, &mut updated).await;
    //         //01/24/24 - changed to:
    //         read_lines(reader, shared_neural_network, &divisor).await;

    //     }
    // });
    // let _ = tokio::try_join!(cycle_task, read_lines_task)?;


    //let stdin = io::stdin();
    
    //this will run indefinitely and will not stop if there is a break in the
    //  input. it will pause and wait for more data to become available.

    ////01/11/24 - code commented out and put in its own function
    //for line_being_read in reader.lines() {
    //    
    //    //error handling in case it doesn't read Input clrrectly
    //    match line_being_read{
    //        
    //        //this gives me 1 line of output which is good because each message is
    //        //   printed in 1 line but it looks like multiple because it overflows
    //        Ok(line_being_read) => {
    //    
    //            
    //            //line.splintn(2, ':'): iterates over the line and splits it at  ":".
    //            //.collect()            collects the 2 substrings and puts it into a
    //            //                  Vec<&str>   which is a vector of substrings. 
    //            //                  Aka an array of substrings
    //            //parts now contains the 2 substrings split at the  :
    //            let parts: Vec<&str> = line_being_read.splitn(2, ':').collect();
    //
    //
    //            if parts[0] == "Message sent successfully" {
     //               continue;
    //            }
    //            //this checks if parts has exactly 2 elements
    //            //If it doesn't this means that the input string didnt contain a colon
   //             //  so it is unexpected behavior. So panic (at least for now)
    //            //WILL NEED to implement a save state before the panic
    //            
    //            if parts.len() != 2 {
    //                panic!("got a weird line of input. The input was\n
    //                        {:?}", parts);
    //            }
    //            //gets the first element of the parts and trims leading or
    //            //  trailing whitespace
    //            let prefix = parts[0].trim();
     //           //this is the actual guts of the message
    //            let message = parts[1].trim();
    //
    //            //this is operating as an if statement of if: "coinbase received"
    //            //  go to handle_coinbase(message) function
    //            //else if "kraken received"...
    //            //and if it's none of them, print that it's unknown and panic
    //            match prefix {
     //               "SOL Coinbase Received" => handle_sol_coinbase(message, &mut neural_network, &mut updated),
    //                "XLM Coinbase Received" => handle_xlm_coinbase(message, &mut neural_network, &mut updated),
    //                "SOL Kraken Received" => handle_sol_kraken(message, &mut neural_network, &mut updated),
    //                "XLM Kraken Received" => handle_xlm_kraken(message, &mut neural_network, &mut updated),
    //                "SOL Bitstamp received" => handle_sol_bitstamp(message, &mut neural_network, &mut updated),
    //                "XLM Bitstamp received" => handle_xlm_bitstamp(message, &mut neural_network, &mut updated),
    //                "Gemini received" => handle_sol_gemini(message, &mut neural_network, &mut updated),
    //                _ => panic!("Unknown prefix: {}", prefix),
    //            }
    //
     //       },
    //        Err(e) => {
    //            eprintln!("Error reading line from stdin: {}", e);
    //            panic!();
    //            //it will panic because it may be crucial to read every
    //            //  line. so exit program if it doesn't. but now that I think
    //            //  about it I should probably save the state of the DQN
    //            //  if I am implementing this program into the DQN later
    //            //why?
    //            //  so that it doesn't have to relearn everything.
    //            //HOWEVER:
    //            //  I dont have a function to save the state of the DQN
    //            //  but I should add it here though
    //        },
    //    }
    //}
    
//-----ALL-FOR-PARSING-ABOVE-THIS//














    //------------------------for----experience----replay---below----------------------//




    //---------beginning of code so I can execute functions----------//
    
    //dotenv().expect("Failed to load .env file");
    //let coinbase_secret = env::var("COINBASE_SECRET_KEY").expect("SECRET_KEY must be set. check if even have .env file and if that is in it");
	//let coinbase_api_key = env::var("COINBASE_API_KEY").expect("API_KEY must be set. check if even have .env file and if that is in it");
    //let kraken_secret = env::var("KRAKEN_PRIVATE_KEY").expect("KRAKEN_PRIVATE_KEY must be set. check if even have .env file and if that is in it");
	//let kraken_api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY must be set. check if even have .env file and if that is in it");
    //let bitstamp_api_key = env::var("BITSTAMP_API_KEY").expect("could not find BITSTAMP_API_KEY spelled exactly like this. check if even have .env file");
	//let bitstamp_secret = env::var("BITSTAMP_SECRET_KEY").expect("could not find BITSTAMP_SECRET_KEY spelt exactly like this in .env file. check if even have .env file");
    //let gemini_api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set with exact name in .env file. check if even have .env file");
    //let gemini_secret = env::var("GEMINI_SECRET_KEY").expect("GEMINI_SECRET_KEY must be set with exact name in .env file. check if even have .env file");
    //let client = reqwest::Client::new();

    //test variables
    //let mut value_prior = 2000.0;
    //let mut coinbase_wallet = 500.0;
    //let mut bitstamp_wallet = 500.0;
    //let mut kraken_wallet = 500.0;
    //let mut gemini_wallet = 500.0;

    //let value_after = action_functions::s_i1_sol_1_coinbase_kraken( &mut coinbase_wallet, &mut kraken_wallet, &bitstamp_wallet,
    //    &gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key  ).await;

    //let client = reqwest::Client::new();
    //let value_after = action_functions::s_i11_sol_1_coinbase_bitstamp(&value_prior, &mut coinbase_wallet, &kraken_wallet, &mut bitstamp_wallet,
    //        &gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key).await;
    //let client = reqwest::Client::new();
    //let value_after = action_functions::s_i21_sol_1_gemini_coinbase(&value_prior, &mut coinbase_wallet, &kraken_wallet, &mut bitstamp_wallet,
    //            &mut gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &gemini_secret, &gemini_api_key).await;
    //let client = reqwest::Client::new();
    //let value_after = action_functions::s_i31_sol_1_gemini_kraken(&value_prior, &coinbase_wallet, &mut kraken_wallet, &bitstamp_wallet,
    //    &mut gemini_wallet, &kraken_secret, &kraken_api_key, client, &gemini_secret, &gemini_api_key).await;
    //let client = reqwest::Client::new();
    //let value_after = action_functions::s_i41_sol_1_gemini_bitstamp(&value_prior, &coinbase_wallet, &kraken_wallet, &mut bitstamp_wallet,
    //    &mut gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &gemini_secret, &gemini_api_key).await;
    //let client = reqwest::Client::new();
    //let value_after = action_functions::s_i51_sol_1_kraken_coinbase(&value_prior, &mut coinbase_wallet, &mut kraken_wallet, &bitstamp_wallet,
    //    &gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key  ).await;
    //let client = reqwest::Client::new();
    //let value_after = action_functions::s_i61_sol_1_kraken_bitstamp(&value_prior, &coinbase_wallet, &mut kraken_wallet, &mut bitstamp_wallet,
    //    &gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key  ).await;

    //let client = reqwest::Client::new();
    //let value_after = action_functions::s_i75_xlm_5_coinbase_kraken( &mut coinbase_wallet, &mut kraken_wallet, &bitstamp_wallet,
    //        &gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key  ).await;
    //let client = reqwest::Client::new();
    //let value_after = action_functions::s_i83_xlm_3_coinbase_bitstamp( &mut coinbase_wallet, &kraken_wallet, &mut bitstamp_wallet,
    //    &gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key).await;
    //let client = reqwest::Client::new();
    //let value_after = action_functions::s_i95_xlm_5_kraken_coinbase( &mut coinbase_wallet, &mut kraken_wallet, &bitstamp_wallet,
    //    &gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key  ).await;
    
    //let client = reqwest::Client::new();
    //let value_after = action_functions::s_i105_xlm_5_kraken_bitstamp(&coinbase_wallet, &mut kraken_wallet, &mut bitstamp_wallet,
    //    &gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key  ).await;

    //02/14/24 - removed:
    //Ok(())
}


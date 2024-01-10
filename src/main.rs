

use p::action_functions::s_i0_do_nothing;
use rand_distr::num_traits::AsPrimitive;
use rand_distr::{StandardNormal, Normal, Distribution};
use reqwest::Client;                                //to actually make the request itself
use std::env;                                       //so I can use .env files and I dont have to put key details on github
use rand::Rng;
use hmac::{Hmac, Mac,};	                            //so I can do the signature stuff
use sha2::{Sha256, Sha384, Sha512, Digest};	        //so I can do signature stuff
use p::network::NeuralNetwork;
use p::action_functions;
use p::execute_action_functions;
use tokio;                                          //so I can do async
use dotenv::dotenv;
//use mod network;
//use mod actions;
use std::time::Instant;                             //this is to record time for execution
use std::process::{Command, Stdio};                 //for piping websocket client
use std::io::{BufRead, BufReader};//this is to help us read from stdin
use serde_json::Value;          //good for parsing intput in JSON format
use tokio::time::delay_for;                         //for "sleep", but in async functions



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

fn handle_sol_coinbase(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
    //if the message contains the word "heartbeat", ignore the entire message basically
    if message.contains("heartbeat") {
        println!("Coinbase sol eartbeat message. ignoring...it's contents:\n{}", message);
        return;
    }
    if message.contains("subscriptions") {
        println!("Coinbase sol SubsCriptions message. ignoring...it's contents:\n{}", message);
        return;
    }
    if message.trim().is_empty() {
        println!("Coinbase sol: blank message received\nmessage: {}", message);
        return;
    }
    let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

    //variable declaration so I can have a larger scope
    let mut coinbase_price = 0.0;
    let mut coinbase_volume_24h = 0.0;
    let mut coinbase_low_24h = 0.0;
    let mut coinbase_high_24h = 0.0;
    let mut coinbase_low_52w = 0.0;
    let mut coinbase_high_52w = 0.0;
    let mut coinbase_price_percent_chg_24h = 0.0;

    match data {
        Ok(value) => {
            let ticker = &value["events"][0]["tickers"][0];
            coinbase_price = ticker["price"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_volume_24h = ticker["volume_24_h"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_low_24h = ticker["low_24_h"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_high_24h = ticker["high_24_h"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_low_52w = ticker["low_52_w"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_high_52w = ticker["high_52_w"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_price_percent_chg_24h = ticker["price_percent_chg_24_h"].as_str().unwrap().parse::<f64>().unwrap();
        }
        Err(e) => println!("Failed to parse SOL COINBASE message. \nError {}\n{}", e, message),
    }

        let indices = [0, 1, 2, 3, 4, 5, 6];
        let new_values = [coinbase_price, coinbase_volume_24h, coinbase_low_24h, 
                    coinbase_high_24h, 
                    coinbase_low_52w, coinbase_high_52w, coinbase_price_percent_chg_24h,];

        neural_network.update_input(&indices, &new_values);
        //to mark the inputs as changed
        for index in indices {
            updated[index] = true;
        }
        if updated.iter().all(|&x| x) {
            neural_network.print_layers();
        } 
        else {
            let not_updated: Vec<String> = updated.iter()
            .enumerate()
            .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
            .collect();
            println!("Neurons: {} have not been updated", not_updated.join(", "));
        }

    }



 











    ////NEED TO SEE IF i HAVE TO NORMALIZE THIS DATA FIRST
    //let indices = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    //let new_values = [&coinbase_price, &coinbase_open_24h, &coinbase_volume_24h, &coinbase_low_24h, 
    //            &coinbase_high_24h, &coinbase_volume_30d, &coinbase_best_bid, &coinbase_best_bid_size, 
    //            &coinbase_best_ask, &coinbase_best_ask_size, &coinbase_side, &coinbase_last_size];
    //neural_network.update_input(&indices, &new_values);

    fn handle_xlm_coinbase(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
        //if the message contains the word "heartbeat", ignore the entire message basically
        if message.contains("heartbeat") {
            println!("Coinbase xlm: heartbeat message. ignoring...it's contents:\n{}", message);
            return;
        }
        if message.contains("subscriptions") {
            println!("Coinbase xlm: SubsCriptions message. ignoring...it's contents:\n{}", message);
            return;
        }
        if message.trim().is_empty() {
            println!("Coinbase xlm: blank message received\nmessage: {}", message);
            return;
        }
        let data: Result<Value, serde_json::Error> = serde_json::from_str(message);
    
        //variable declaration so I can have a larger scope
        let mut coinbase_price = 0.0;
        let mut coinbase_volume_24h = 0.0;
        let mut coinbase_low_24h = 0.0;
        let mut coinbase_high_24h = 0.0;
        let mut coinbase_low_52w = 0.0;
        let mut coinbase_high_52w = 0.0;
        let mut coinbase_price_percent_chg_24h = 0.0;
    
        match data {
            Ok(value) => {
                let ticker = &value["events"][0]["tickers"][0];
                coinbase_price = ticker["price"].as_str().unwrap().parse::<f64>().unwrap();
                coinbase_volume_24h = ticker["volume_24_h"].as_str().unwrap().parse::<f64>().unwrap();
                coinbase_low_24h = ticker["low_24_h"].as_str().unwrap().parse::<f64>().unwrap();
                coinbase_high_24h = ticker["high_24_h"].as_str().unwrap().parse::<f64>().unwrap();
                coinbase_low_52w = ticker["low_52_w"].as_str().unwrap().parse::<f64>().unwrap();
                coinbase_high_52w = ticker["high_52_w"].as_str().unwrap().parse::<f64>().unwrap();
                coinbase_price_percent_chg_24h = ticker["price_percent_chg_24_h"].as_str().unwrap().parse::<f64>().unwrap();
            }
            Err(e) => println!("Failed to parse SOL COINBASE message. \nError {}\n{}", e, message),
        }
 
            let indices = [7, 8, 9, 10, 11, 12, 13];
            let new_values = [coinbase_price, coinbase_volume_24h, coinbase_low_24h, 
            coinbase_high_24h, 
            coinbase_low_52w, coinbase_high_52w, coinbase_price_percent_chg_24h,];


            neural_network.update_input(&indices, &new_values);
            //to mark the inputs as changed
            for index in indices {
                updated[index] = true;
            }
            if updated.iter().all(|&x| x) {
                neural_network.print_layers();
            } 
            else {
                let not_updated: Vec<String> = updated.iter()
                .enumerate()
                .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
                .collect();
                println!("Neurons: {} have not been updated", not_updated.join(", "));
            }
        }






fn handle_sol_kraken(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
    if message.contains("heartbeat") {
        println!("Kraken sol heartbeat message. ignoring...it's contents:\n{}", message);
        return;
    }
    if message.contains("systemStatus") {
        println!("Kraken sol: initial system message. ignoring message... it's contents:\n{}", message);
        return;
    }
    if message.contains("subscriptionStatus") {
        println!("Kraken sol: initial  SUB message. ignoring...it's contents:\n{}", message);
        return;
    }
    if message.trim().is_empty() {
        println!("Kraken sol: blank message received\nmessage: {}", message);
        return;
    }
    let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

    let mut a_0 = 0.0;
    let mut a_1 = 0.0;
    let mut a_2 = 0.0;

    let mut b_0 = 0.0;
    let mut b_1 = 0.0;
    let mut b_2 = 0.0;

    let mut c_0 = 0.0;
    let mut c_1 = 0.0;

    let mut v_0 = 0.0;
    let mut v_1 = 0.0;

    let mut p_0 = 0.0;
    let mut p_1 = 0.0;

    let mut t_0 = 0.0;
    let mut t_1 = 0.0;

    let mut l_0 = 0.0;
    let mut l_1 = 0.0;

    let mut h_0 = 0.0;
    let mut h_1 = 0.0;

    let mut o_0 = 0.0;
    let mut o_1 = 0.0;

    match data {
        Ok(value) => {
            let ticker = &value[1];
            a_0 = ticker["a"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for a[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse a[0] as f64. Full message: {}", message);
                panic!();
            });
            a_1 = ticker["a"][1].as_i64().unwrap_or_else(|| {
                println!("Failed to get string for a[1]. Full message: {}", message);
                panic!();
            }) as f64;
            a_2 = ticker["a"][2].as_str().unwrap_or_else(|| {
                println!("Failed to get string for a[2]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse a[2] as f64. Full message: {}", message);
                panic!();
            });
            b_0 = ticker["b"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for b[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse b[0] as f64. Full message: {}", message);
                panic!();
            });
            b_1 = ticker["b"][1].as_i64().unwrap_or_else(|| {
                println!("Failed to get string for a[1]. Full message: {}", message);
                panic!();
            }) as f64;
            b_2 = ticker["b"][2].as_str().unwrap_or_else(|| {
                println!("Failed to get string for b[2]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse b[2] as f64. Full message: {}", message);
                panic!();
            });
            c_0 = ticker["c"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for c[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse c[0] as f64. Full message: {}", message);
                panic!();
            });
            c_1 = ticker["c"][1].as_str().unwrap_or_else(|| {
                println!("Failed to get string for c[1]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse c[1] as f64. Full message: {}", message);
                panic!();
            });
            v_0 = ticker["v"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for v[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse v[0] as f64. Full message: {}", message);
                panic!();
            });
            v_1 = ticker["v"][1].as_str().unwrap_or_else(|| {
                println!("Failed to get string for v[1]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse v[1] as f64. Full message: {}", message);
                panic!();
            });
            p_0 = ticker["p"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for p[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse p[0] as f64. Full message: {}", message);
                panic!();
            });
            p_1 = ticker["p"][1].as_str().unwrap_or_else(|| {
                println!("Failed to get string for p[1]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse p[1] as f64. Full message: {}", message);
                panic!();
            });
            t_0 = ticker["t"][0].as_i64().unwrap_or_else(|| {
                println!("Failed to get string for a[1]. Full message: {}", message);
                panic!();
            }) as f64;
            t_1 = ticker["t"][1].as_i64().unwrap_or_else(|| {
                println!("Failed to get string for a[1]. Full message: {}", message);
                panic!();
            }) as f64;
            l_0 = ticker["l"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for l[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse l[0] as f64. Full message: {}", message);
                panic!();
            });
            l_1 = ticker["l"][1].as_str().unwrap_or_else(|| {
                println!("Failed to get string for l[1]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse l[1] as f64. Full message: {}", message);
                panic!();
            });
            h_0 = ticker["h"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for h[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse h[0] as f64. Full message: {}", message);
                panic!();
            });
            h_1 = ticker["h"][1].as_str().unwrap_or_else(|| {
                println!("Failed to get string for h[1]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse h[1] as f64. Full message: {}", message);
                panic!();
            });
            o_0 = ticker["o"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for o[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse o[0] as f64. Full message: {}", message);
                panic!();
            });
            o_1 = ticker["o"][1].as_str().unwrap_or_else(|| {
                println!("Failed to get string for o[1]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse o[1] as f64. Full message: {}", message);
                panic!();
            });
        }
        Err(e) => println!("Failed to parse message: {}", e),
    }
    //println!("a_0: {}, a_1: {}, a_2: {}, b_0: {}, b_1: {}, b_2: {}, c_0: {}, c_1: {}, v_0: {}, v_1: {}, p_0: {}, p_1: {}, t_0: {}, t_1: {}, l_0: {}, l_1: {}, h_0: {}, h_1: {}, o_0: {}, o_1: {}", 
    //&a_0, &a_1, &a_2, &b_0, &b_1, &b_2, &c_0, &c_1, &v_0, &v_1, &p_0, &p_1, &t_0, &t_1, &l_0, &l_1, &h_0, &h_1, &o_0, &o_1);
    let indices: [usize; 20] = [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 
    30, 31, 32, 33];
    let new_values = [a_0, a_1, a_2, b_0, b_1, b_2, c_0, c_1, 
        v_0, v_1, p_0, p_1, t_0, t_1, l_0, l_1, h_0, h_1, o_0, o_1];
    //let new_values = [&a_price, &a_whole_lot_volume, &a_lot_volume, &b_price, 
    //    &b_whole_lot_volume, &b_lot_volume, &c_price, &c_lot_volume, 
    //    &v_today, &v_last24hours, &p_today, &p_last24hours, &t_today, 
    //    &t_last24hours, &l_today, &l_last24hours, &h_today, &h_last24hours, 
    //    &o_today, &o_last24hours];

    neural_network.update_input(&indices, &new_values);
    //to mark the inputs as changed
    for index in indices {
        updated[index] = true;
    }
    if updated.iter().all(|&x| x) {
        neural_network.print_layers();
    } 
    else {
        let not_updated: Vec<String> = updated.iter()
        .enumerate()
        .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
        .collect();
        println!("Neurons: {} have not been updated", not_updated.join(", "));
    }
}














fn handle_xlm_kraken(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
    if message.contains("heartbeat") {
        println!("Kraken xlm heartbeat message. ignoring...it's contents:\n{}", message);
        return;
    }
    if message.contains("systemStatus") {
        println!("Kraken xlm: initial system message. ignoring...it's contents:\n{}", message);
        return;
    }
    if message.contains("subscriptionStatus") {
        println!("Kraken xlm: initial  SUB message. ignoring...it's contents:\n{}", message);
        return;
    }
    if message.trim().is_empty() {
        println!("Kraken xlm: blank message received\nmessage: {}", message);
        return;
    }
    let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

    let mut a_0 = 0.0;
    let mut a_1 = 0.0;
    let mut a_2 = 0.0;

    let mut b_0 = 0.0;
    let mut b_1 = 0.0;
    let mut b_2 = 0.0;

    let mut c_0 = 0.0;
    let mut c_1 = 0.0;

    let mut v_0 = 0.0;
    let mut v_1 = 0.0;

    let mut p_0 = 0.0;
    let mut p_1 = 0.0;

    let mut t_0 = 0.0;
    let mut t_1 = 0.0;

    let mut l_0 = 0.0;
    let mut l_1 = 0.0;

    let mut h_0 = 0.0;
    let mut h_1 = 0.0;

    let mut o_0 = 0.0;
    let mut o_1 = 0.0;

    match data {
        Ok(value) => {
            let ticker = &value[1];
            a_0 = ticker["a"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for a[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse a[0] as f64. Full message: {}", message);
                panic!();
            });
            a_1 = ticker["a"][1].as_i64().unwrap_or_else(|| {
                println!("Failed to get string for a[1]. Full message: {}", message);
                panic!();
            }) as f64;
            a_2 = ticker["a"][2].as_str().unwrap_or_else(|| {
                println!("Failed to get string for a[2]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse a[2] as f64. Full message: {}", message);
                panic!();
            });
            b_0 = ticker["b"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for b[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse b[0] as f64. Full message: {}", message);
                panic!();
            });
            b_1 = ticker["b"][1].as_i64().unwrap_or_else(|| {
                println!("Failed to get string for a[1]. Full message: {}", message);
                panic!();
            }) as f64;
            b_2 = ticker["b"][2].as_str().unwrap_or_else(|| {
                println!("Failed to get string for b[2]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse b[2] as f64. Full message: {}", message);
                panic!();
            });
            c_0 = ticker["c"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for c[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse c[0] as f64. Full message: {}", message);
                panic!();
            });
            c_1 = ticker["c"][1].as_str().unwrap_or_else(|| {
                println!("Failed to get string for c[1]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse c[1] as f64. Full message: {}", message);
                panic!();
            });
            v_0 = ticker["v"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for v[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse v[0] as f64. Full message: {}", message);
                panic!();
            });
            v_1 = ticker["v"][1].as_str().unwrap_or_else(|| {
                println!("Failed to get string for v[1]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse v[1] as f64. Full message: {}", message);
                panic!();
            });
            p_0 = ticker["p"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for p[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse p[0] as f64. Full message: {}", message);
                panic!();
            });
            p_1 = ticker["p"][1].as_str().unwrap_or_else(|| {
                println!("Failed to get string for p[1]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse p[1] as f64. Full message: {}", message);
                panic!();
            });
            t_0 = ticker["t"][0].as_i64().unwrap_or_else(|| {
                println!("Failed to get string for a[1]. Full message: {}", message);
                panic!();
            }) as f64;
            t_1 = ticker["t"][1].as_i64().unwrap_or_else(|| {
                println!("Failed to get string for a[1]. Full message: {}", message);
                panic!();
            }) as f64;
            l_0 = ticker["l"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for l[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse l[0] as f64. Full message: {}", message);
                panic!();
            });
            l_1 = ticker["l"][1].as_str().unwrap_or_else(|| {
                println!("Failed to get string for l[1]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse l[1] as f64. Full message: {}", message);
                panic!();
            });
            h_0 = ticker["h"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for h[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse h[0] as f64. Full message: {}", message);
                panic!();
            });
            h_1 = ticker["h"][1].as_str().unwrap_or_else(|| {
                println!("Failed to get string for h[1]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse h[1] as f64. Full message: {}", message);
                panic!();
            });
            o_0 = ticker["o"][0].as_str().unwrap_or_else(|| {
                println!("Failed to get string for o[0]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse o[0] as f64. Full message: {}", message);
                panic!();
            });
            o_1 = ticker["o"][1].as_str().unwrap_or_else(|| {
                println!("Failed to get string for o[1]. Full message: {}", message);
                panic!();
            }).parse::<f64>().unwrap_or_else(|_| {
                println!("Failed to parse o[1] as f64. Full message: {}", message);
                panic!();
            });
        }
        Err(e) => println!("Failed to parse message: {}", e),
    }
    //println!("a_0: {}, a_1: {}, a_2: {}, b_0: {}, b_1: {}, b_2: {}, c_0: {}, c_1: {}, v_0: {}, v_1: {}, p_0: {}, p_1: {}, t_0: {}, t_1: {}, l_0: {}, l_1: {}, h_0: {}, h_1: {}, o_0: {}, o_1: {}", 
    //&a_0, &a_1, &a_2, &b_0, &b_1, &b_2, &c_0, &c_1, &v_0, &v_1, &p_0, &p_1, &t_0, &t_1, &l_0, &l_1, &h_0, &h_1, &o_0, &o_1);
    let indices: [usize; 20] = [34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 
    50, 51, 52, 53];
    let new_values = [a_0, a_1, a_2, b_0, b_1, b_2, c_0, c_1, 
        v_0, v_1, p_0, p_1, t_0, t_1, l_0, l_1, h_0, h_1, o_0, o_1];


    neural_network.update_input(&indices, &new_values);
    //to mark the inputs as changed
    for index in indices {
        updated[index] = true;
    }
    if updated.iter().all(|&x| x) {
        neural_network.print_layers();
    } 
    else {
        let not_updated: Vec<String> = updated.iter()
        .enumerate()
        .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
        .collect();
        println!("Neurons: {} have not been updated", not_updated.join(", "));
    }
}

















fn handle_sol_bitstamp(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
    // Handle Bitstamp message
    if message.contains("subscription") {
        println!("Bitstamp sol subscription succeeded. unimportant message\nmessage: {}", message);
        return;
    }
    if message.contains("heartbeat") {
        println!("Bitstamp sol heartbeat message\nmessage: {}", message);
        return;
    }
    if message.trim().is_empty() {
        println!("Bitstamp sol: blank message received\nmessage: {}", message);
        return;
    }

    let v: Result<Value, serde_json::Error> = serde_json::from_str(message);

    let mut amount = 0.0;
    let mut price = 0.0;

    match v {
        Ok(value) => {
            if let Value::Object(map) = &value {
                // Check if the object has a key "data" whose value is an object
                if let Some(Value::Object(data)) = map.get("data") {
                    // Extract the values
                    amount = data.get("amount").and_then(Value::as_f64).unwrap();
                    price = data.get("price").and_then(Value::as_f64).unwrap();

                    println!("Sol Bitstamp:\namount: {}\nprice: {}\n\n\n", &amount, &price);
        
                }
            }
        },
        Err(e) => {
            println!("Failed to parse JSON Bitstamp message\nError: {}\nMessage: {}", e, message);
        },
    }


    let indices: [usize; 2] = [54, 55];
    let new_values = [amount, price];

    neural_network.update_input(&indices, &new_values);
    //to mark the inputs as changed
    for index in indices {
        updated[index] = true;
    }
    if updated.iter().all(|&x| x) {
        neural_network.print_layers();
    } 
    else {
        let not_updated: Vec<String> = updated.iter()
        .enumerate()
        .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
        .collect();
        println!("Neurons: {} have not been updated", not_updated.join(", "));
    }

}









fn handle_xlm_bitstamp(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
    // Handle Bitstamp message
    if message.contains("subscription") {
        println!("Bitstamp xlm subscription succeeded. unimportant message\nmessage: {}", message);
        return;
    }
    if message.contains("heartbeat") {
        println!("Bitstamp xlm heartbeat message\nmessage: {}", message);
        return;
    }
    if message.trim().is_empty() {
        println!("Bitstamp xlm: blank message received\nmessage: {}", message);
        return;
    }

    let v: Result<Value, serde_json::Error> = serde_json::from_str(message);

    let mut amount = 0.0;
    let mut price = 0.0;

    match v {
        Ok(value) => {
            if let Value::Object(map) = &value {
                // Check if the object has a key "data" whose value is an object
                if let Some(Value::Object(data)) = map.get("data") {
                    // Extract the values
                    amount = data.get("amount").and_then(Value::as_f64).unwrap();
                    price = data.get("price").and_then(Value::as_f64).unwrap();

                    println!("XLM Bitstamp:\namount: {}\nprice: {}\n\n\n", &amount, &price);
        
                }
            }
        },
        Err(e) => {
            println!("Failed to parse JSON Bitstamp message\nError: {}\nMessage: {}", e, message);
        },
    }


    let indices: [usize; 2] = [56, 57];
    let new_values = [amount, price];

    neural_network.update_input(&indices, &new_values);
    //to mark the inputs as changed
    for index in indices {
        updated[index] = true;
    }
    //for debugging
    if updated.iter().all(|&x| x) {
        neural_network.print_layers();
    } 
    else {
        let not_updated: Vec<String> = updated.iter()
        .enumerate()
        .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
        .collect();
        println!("Neurons: {} have not been updated", not_updated.join(", "));
    }

}










fn handle_sol_gemini(message: &str, neural_network: &mut NeuralNetwork, updated: &mut [bool; 60]) {
    if message.contains("heartbeat") {
        println!("Gemini heartbeat message. ignoring...");
        return;
    }
    if message.trim().is_empty() {
        println!("Gemini: blank message received\nmessage: {}", message);
        return;
    }
    let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

    let mut amount: Option<f64> = None;
    let mut price: Option<f64> = None;

    match data {
        Ok(value) => {
            if value.get("socket_sequence").and_then(Value::as_i64) == Some(0) {
                println!("Gemini: socket sequence is 0, ignoring...");
                return;
            }
            if let Value::Object(map) = &value {
                if let Some(Value::Array(events)) = map.get("events") {
                    if let Some(Value::Object(event)) = events.get(0) {
                        amount = event.get("amount").and_then(|v| v.as_str()).and_then(|s| s.parse::<f64>().ok());
                        price = event.get("price").and_then(|v| v.as_str()).and_then(|s| s.parse::<f64>().ok());
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to parse JSON Gemini message\nError: {}\nMessage: {}", e, message);
        },
    }

    if let (Some(amount), Some(price)) = (amount, price) {
        let indices = [58, 59];
        let new_values = [amount, price];

        neural_network.update_input(&indices, &new_values);
        //to mark the inputs as changed
        for index in indices {
            updated[index] = true;
        }
        if updated.iter().all(|&x| x) {
            neural_network.print_layers();
        } 
        else {
            let not_updated: Vec<String> = updated.iter()
            .enumerate()
            .filter_map(|(index, &updated)| if !updated { Some(index.to_string()) } else { None })
            .collect();
            println!("Neurons: {} have not been updated", not_updated.join(", "));
        }
    } else {
        println!("Failed to parse amount and/or price");
        println!("Gemini message:\n{}", message);
        panic!();
    }
    //counting the neurons for the the amount in each wallet, I will have 40 input neurons.

}

//-----ALL-FOR-PARSING-ABOVE-THIS//


























//12/23/23 code commented everything, added the new lines of code labelled below then added the return to fn main()
#[tokio::main]
async fn main()  {
    
//-----ALL-FOR-PARSING-UNDER-THIS//
    
    env::set_var("RUST_BACKTRACE", "1");

    //this is just example code to evaluate if save and load of network works and it does  
    let mut neural_network = NeuralNetwork {
        layers: Vec::new(),
        weights: Vec::new(),
        biases: Vec::new(),
    };
    neural_network.initialization(60, 80, 2); // Initialize with [input size], [output size], [# hidden layers]
    //the first number in the initialization and the number below MUST be the same size
    let mut updated = [false; 60];




    //NOT FOR PARSING BUT I NEED THIS BEFORE THE FOR LOOP STARTS
    let mut epsilon = 1.0;
    //---------beginning of code so I can execute functions----------//
    
    dotenv().expect("Failed to load .env file");
    let coinbase_secret = env::var("COINBASE_SECRET_KEY").expect("SECRET_KEY must be set. check if even have .env file and if that is in it");
	let coinbase_api_key = env::var("COINBASE_API_KEY").expect("API_KEY must be set. check if even have .env file and if that is in it");
    let kraken_secret = env::var("KRAKEN_PRIVATE_KEY").expect("KRAKEN_PRIVATE_KEY must be set. check if even have .env file and if that is in it");
	let kraken_api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY must be set. check if even have .env file and if that is in it");
    let bitstamp_api_key = env::var("BITSTAMP_API_KEY").expect("could not find BITSTAMP_API_KEY spelled exactly like this. check if even have .env file");
	let bitstamp_secret = env::var("BITSTAMP_SECRET_KEY").expect("could not find BITSTAMP_SECRET_KEY spelt exactly like this in .env file. check if even have .env file");
    let gemini_api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set with exact name in .env file. check if even have .env file");
    let gemini_secret = env::var("GEMINI_SECRET_KEY").expect("GEMINI_SECRET_KEY must be set with exact name in .env file. check if even have .env file");
    let client = reqwest::Client::new();
    let mut value_prior = 2000.0;
    let mut coinbase_wallet = 500.0;
    let mut bitstamp_wallet = 500.0;
    let mut kraken_wallet = 500.0;
    let mut gemini_wallet = 500.0;
    //--end of code to execute funcitons







    
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
    let websocket_client = Command::new(
            r"C:\Users\djv60\projects\testw\target\debug\testw.exe")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start WebSocket client");

    

    //BufReader::new(....)
    //                  This is creating a new "buffered reader"
    //what is a buffered reader?
    //                  a type of reader that adds buffering.
    //why?              buffering is a process of storing data temporarily in memory
    //                  ,aka the buffer, while it's being moved from 1 place to another
    //why?              It significantly improves performance when reading large amounts
    //                   of data because w/o buffering the parser program would have to 
    //                   ask the OS to read each byte of data individually. so super slow. 
    //                   so with buffering I can read large blocks of data at once
    //websocket_client.stdout
    //                  this is getting the standard output (stdout) of hte websocket client
    //                   process that was spawned.
    //.expect()
    //                  It is type Option<...> so that means if it doesnt find anything in 
    //                  the stdout it will execute the .expect() and panic

    let reader = BufReader::new(websocket_client.stdout
            .expect("Failed to get stdout"));


    //let stdin = io::stdin();
    
    //this will run indefinitely and will not stop if there is a break in the
    //  input. it will pause and wait for more data to become available.
    for line_being_read in reader.lines() {

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
                    panic!("got a weird line of input. The input was\n
                            {:?}", parts);
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
                match prefix {
                    "SOL Coinbase Received" => handle_sol_coinbase(message, &mut neural_network, &mut updated),
                    "XLM Coinbase Received" => handle_xlm_coinbase(message, &mut neural_network, &mut updated),
                    "SOL Kraken Received" => handle_sol_kraken(message, &mut neural_network, &mut updated),
                    "XLM Kraken Received" => handle_xlm_kraken(message, &mut neural_network, &mut updated),
                    "SOL Bitstamp received" => handle_sol_bitstamp(message, &mut neural_network, &mut updated),
                    "XLM Bitstamp received" => handle_xlm_bitstamp(message, &mut neural_network, &mut updated),
                    "Gemini received" => handle_sol_gemini(message, &mut neural_network, &mut updated),
                    _ => panic!("Unknown prefix: {}", prefix),
                }

            },
            Err(e) => {
                eprintln!("Error reading line from stdin: {}", e);
                panic!();
                //it will panic because it may be crucial to read every
                //  line. so exit program if it doesn't. but now that I think
                //  about it I should probably save the state of the DQN
                //  if I am implementing this program into the DQN later
                //why?
                //  so that it doesn't have to relearn everything.
                //HOWEVER:
                //  I dont have a function to save the state of the DQN
                //  but I should add it here though
            },
        }
    }
    
//-----ALL-FOR-PARSING-ABOVE-THIS//














    //------------------------for----experience----replay---below----------------------//


    let mut replay_buffer = ReplayBuffer::new(10);

    // Create a real transition
    let state = NetworkLayer { /* fill with your real data */ };
    let action = /* your real action */;
    let reward = /* your real reward */;
    let next_state = NetworkLayer { /* fill with your real data */ };

    let transition = Transition {
        state,
        action,
        reward,
        next_state,
    };

    // Push the transition into the buffer
    replay_buffer.push(transition);

    // Save the buffer to a file
    replay_buffer.save_to_file("replay_buffer.json").unwrap();

    // Load the buffer from the file
    let loaded_buffer = ReplayBuffer::load_from_file("replay_buffer.json").unwrap();

    // Check if the loaded buffer is the same as the original one
    assert_eq!(replay_buffer.buffer.len(), loaded_buffer.buffer.len());
    assert_eq!(replay_buffer.capacity, loaded_buffer.capacity);
    // Add more checks if necessary



 //------------------------for----experience----replay---above----------------------//    

















    //---------beginning of code so I can execute functions----------//
    
    dotenv().expect("Failed to load .env file");
    let coinbase_secret = env::var("COINBASE_SECRET_KEY").expect("SECRET_KEY must be set. check if even have .env file and if that is in it");
	let coinbase_api_key = env::var("COINBASE_API_KEY").expect("API_KEY must be set. check if even have .env file and if that is in it");
    let kraken_secret = env::var("KRAKEN_PRIVATE_KEY").expect("KRAKEN_PRIVATE_KEY must be set. check if even have .env file and if that is in it");
	let kraken_api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY must be set. check if even have .env file and if that is in it");
    let bitstamp_api_key = env::var("BITSTAMP_API_KEY").expect("could not find BITSTAMP_API_KEY spelled exactly like this. check if even have .env file");
	let bitstamp_secret = env::var("BITSTAMP_SECRET_KEY").expect("could not find BITSTAMP_SECRET_KEY spelt exactly like this in .env file. check if even have .env file");
    let gemini_api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set with exact name in .env file. check if even have .env file");
    let gemini_secret = env::var("GEMINI_SECRET_KEY").expect("GEMINI_SECRET_KEY must be set with exact name in .env file. check if even have .env file");
    let client = reqwest::Client::new();

    //test variables
    let mut value_prior = 2000.0;
    let mut coinbase_wallet = 500.0;
    let mut bitstamp_wallet = 500.0;
    let mut kraken_wallet = 500.0;
    let mut gemini_wallet = 500.0;

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

    let client = reqwest::Client::new();
    let value_after = action_functions::s_i75_xlm_5_coinbase_kraken( &mut coinbase_wallet, &mut kraken_wallet, &bitstamp_wallet,
            &gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key  ).await;
    let client = reqwest::Client::new();
    let value_after = action_functions::s_i83_xlm_3_coinbase_bitstamp( &mut coinbase_wallet, &kraken_wallet, &mut bitstamp_wallet,
        &gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &bitstamp_secret, &bitstamp_api_key).await;
    let client = reqwest::Client::new();
    let value_after = action_functions::s_i95_xlm_5_kraken_coinbase( &mut coinbase_wallet, &mut kraken_wallet, &bitstamp_wallet,
        &gemini_wallet, &coinbase_secret, &coinbase_api_key, client, &kraken_secret, &kraken_api_key  ).await;
    
    let client = reqwest::Client::new();
    let value_after = action_functions::s_i105_xlm_5_kraken_bitstamp(&coinbase_wallet, &mut kraken_wallet, &mut bitstamp_wallet,
        &gemini_wallet, &bitstamp_secret, &bitstamp_api_key, client, &kraken_secret, &kraken_api_key  ).await;


    
}

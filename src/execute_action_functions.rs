use crate::network::NeuralNetwork;                      // to use neural network struct
use tokio::sync::Mutex;                             // Use async Mutex from Tokio
use std::sync::Arc;  								// Use Arc to share Mutex among multiple tasks
use serde_json::Value;                              // for parsing input form websocket client
use crate::standardization_functions;


//helper function
//03/08/24 - changed from: -> Option<f64>   to
//                         -> Result<f64, String>
pub fn parse_f64(value: Option<&str>, message: &str, prefix: &str) -> Result<f64, String>{
    match value {
        Some(str_value) => {
            match str_value.parse::<f64>() {
                //03/08/24 - changed => Some(parsed_value) to
                //                   => Ok(parsed_value)
                Ok(parsed_value) => Ok(parsed_value),
                Err(e) => {
                    //03/08/24 - changed from panic to log::error and added the other stuff
                        // panic!("Failed to parse as f64: {:?}
                        // message: {}
                        // prefix: {}", e, message, prefix);
                        let error_message = format!("Failed to parse as f64: {:?}\nmessage: {}\nprefix: {}", e, message, prefix);
                        log::error!("{}", &error_message);
                        Err(error_message)
                }
            }
        },
        None => {
            //03/08/24 - removed
                // panic!("Value is not a string. 
                // message: {}
                // prefix: {}", message, prefix);
            //03/08/24 - replaced with:
                let error_message = format!("Value is not a string.\nmessage: {}\nprefix: {}", message, prefix);
                log::error!("{}", &error_message);
                Err(error_message)
        }
    }
}




//03/08/24 - removed:
// pub async fn handle_all_coinbase(prefix: &str, message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, divisor: &f64) {
//     if prefix.contains("Coinbase Received") {
//         //do same calculations and then differentiate between coins
//         //   at the end
//         let data: Result<Value, serde_json::Error> = serde_json::from_str(message);
//         //variable declaration so I can have a larger scope
//         let coinbase_price: Option<f64>;
//         let coinbase_volume_24h: Option<f64>;
//         let coinbase_low_24h: Option<f64>;
//         let coinbase_high_24h: Option<f64>;
//         let coinbase_low_52w: Option<f64>;
//         let coinbase_high_52w: Option<f64>;
//         let coinbase_price_percent_chg_24h: Option<f64>;
//         match data {
//             Ok(value) => {
//                 let ticker = &value["events"][0]["tickers"][0];
//                 coinbase_price = parse_f64(ticker["price"]
//                     .as_str(), message, prefix);
//                 coinbase_volume_24h = parse_f64(ticker["volume_24_h"]
//                     .as_str(), message, prefix);
//                 coinbase_low_24h = parse_f64(ticker["low_24_h"]
//                     .as_str(), message, prefix);
//                 coinbase_high_24h = parse_f64(ticker["high_24_h"]
//                     .as_str(), message, prefix);
//                 coinbase_low_52w = parse_f64(ticker["low_52_w"]
//                     .as_str(), message, prefix);
//                 coinbase_high_52w = parse_f64(ticker["high_52_w"]
//                     .as_str(), message, prefix);
//                 coinbase_price_percent_chg_24h = parse_f64(ticker["price_percent_chg_24_h"]
//                     .as_str(), message, prefix);
//             },
//             Err(e) => panic!("Failed to get data from COINBASE message.
//                 Error {}\n{}", e, message),
//         }
//         let new_values = [coinbase_price, coinbase_volume_24h,
//         coinbase_low_24h, coinbase_high_24h, coinbase_low_52w,
//         coinbase_high_52w, coinbase_price_percent_chg_24h,];
//         let mut scaled_values: Vec<f64> = Vec::new();
//         for value in &new_values {
//             if let Some(val) = value {
//                 scaled_values.push(val / divisor);
//             } 
//             else {
//                 println!("One of the values was None");
//                 panic!("coinbase_price: {:?}, coinbase_volume_24_h: {:?},
//                 coinbase_low_24h: {:?}, coinbase_high_24h: {:?}, 
//                 coinbase_low_52w: {:?}, coinbase_high_52w: {:?}, 
//                 coinbase_price_percent_chg_24h: {:?}, message:\n{}", &coinbase_price,
//                 &coinbase_volume_24h, &coinbase_low_24h, &coinbase_high_24h,
//                 &coinbase_low_52w, &coinbase_high_52w,
//                 &coinbase_price_percent_chg_24h, message);
//             }
//         }
//         if prefix.contains("SOL") {
//             //do the indices and update input and lock
//             let indices: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
//             println!("updating input 0 to 6. price:{:?}", &coinbase_price);
//             let mut neural_network = 
//                 shared_neural_network.lock().await;
//             neural_network.update_input(&indices, &scaled_values).await;
//         }
//         else if prefix.contains("XLM") {
//             //do the indices and update input and lock
//             let indices: [usize; 7] = [7, 8, 9, 10, 11, 12, 13];
//             println!("updating input 7 to 13. price:{:?}", &coinbase_price);
//             let mut neural_network = 
//                 shared_neural_network.lock().await;
//             neural_network.update_input(&indices, &scaled_values).await;
//         }
//         else if prefix.contains("XRP") {
//             //first indices larger than last time
//             println!("updating input 65 to 71. price:{:?}", &coinbase_price);
//             let indices: [usize; 7] = [65, 66, 67, 68, 69, 70, 71];
//             let mut neural_network = 
//                 shared_neural_network.lock().await;
//             neural_network.update_input(&indices, &scaled_values).await;
//         }
//         else {
//             panic!("This shouid never occur. Somehow prefix cointained
//             Coinbase Received but didn't contain the phrases SOL, XLM, or XRP.
//             prefix is: {}", prefix);
//         }
//     }
//     else if prefix.contains("Coinbase consolidated heartbeat") || 
//         prefix.contains("Coinbase subscriptions") {
//         println!("Coinbase: standard server messages. Ignoring...");
//     }
//     else {
//         println!("Coinbase: got a weird message:{}", message);
//     }
// }
// //03/08/24 - removed entire function and replaced with the one below it:
//     pub async fn handle_all_kraken(prefix: &str, message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, divisor: &f64) {
//         if prefix.contains("Kraken Received") {
//             let data: Result<Value, serde_json::Error> = serde_json::from_str(message);
//             let mut a_0: Option<f64> = None;
//             let mut a_1: Option<f64> = None;
//             let mut a_2: Option<f64> = None;
//             let mut b_0: Option<f64> = None;
//             let mut b_1: Option<f64> = None;
//             let mut b_2: Option<f64> = None;
//             let mut c_0: Option<f64> = None;
//             let mut c_1: Option<f64> = None;
//             let mut v_0: Option<f64> = None;
//             let mut v_1: Option<f64> = None;
//             let mut p_0: Option<f64> = None;
//             let mut p_1: Option<f64> = None;
//             let mut t_0: Option<f64> = None;
//             let mut t_1: Option<f64> = None;
//             let mut l_0: Option<f64> = None;
//             let mut l_1: Option<f64> = None;
//             let mut h_0: Option<f64> = None;
//             let mut h_1: Option<f64> = None;
//             let mut o_0: Option<f64> = None;
//             let mut o_1: Option<f64> = None;
//             match data {
//                 Ok(value) => {
//                     let ticker = &value[1];
//                     a_0 = Some(ticker["a"][0].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for a[0]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse a[0] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     a_1 = Some(ticker["a"][1].as_i64().unwrap_or_else(|| {
//                         println!("Failed to get string for a[1]. Full message: {}", message);
//                         panic!();
//                     }) as f64);
//                     a_2 = Some(ticker["a"][2].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for a[2]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse a[2] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     b_0 = Some(ticker["b"][0].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for b[0]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse b[0] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     b_1 = Some(ticker["b"][1].as_i64().unwrap_or_else(|| {
//                         println!("Failed to get string for a[1]. Full message: {}", message);
//                         panic!();
//                     }) as f64);
//                     b_2 = Some(ticker["b"][2].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for b[2]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse b[2] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     c_0 = Some(ticker["c"][0].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for c[0]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse c[0] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     c_1 = Some(ticker["c"][1].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for c[1]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse c[1] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     v_0 = Some(ticker["v"][0].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for v[0]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse v[0] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     v_1 = Some(ticker["v"][1].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for v[1]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse v[1] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     p_0 = Some(ticker["p"][0].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for p[0]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse p[0] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     p_1 = Some(ticker["p"][1].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for p[1]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse p[1] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     t_0 = Some(ticker["t"][0].as_i64().unwrap_or_else(|| {
//                         println!("Failed to get string for a[1]. Full message: {}", message);
//                         panic!();
//                     }) as f64);
//                     t_1 = Some(ticker["t"][1].as_i64().unwrap_or_else(|| {
//                         println!("Failed to get string for a[1]. Full message: {}", message);
//                         panic!();
//                     }) as f64);
//                     l_0 = Some(ticker["l"][0].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for l[0]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse l[0] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     l_1 = Some(ticker["l"][1].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for l[1]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse l[1] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     h_0 = Some(ticker["h"][0].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for h[0]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse h[0] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     h_1 = Some(ticker["h"][1].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for h[1]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse h[1] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     o_0 = Some(ticker["o"][0].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for o[0]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse o[0] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                     o_1 = Some(ticker["o"][1].as_str().unwrap_or_else(|| {
//                         println!("Failed to get string for o[1]. Full message: {}", message);
//                         panic!();
//                     }).parse::<f64>().unwrap_or_else(|_| {
//                         println!("Failed to parse o[1] as f64. Full message: {}", message);
//                         panic!();
//                     }));
//                 }
//                 Err(e) => println!("Failed to parse message: {}", e),
//             }
//             let new_values = [a_0, a_1, a_2, b_0, b_1, b_2, c_0, c_1, 
//             v_0, v_1, p_0, p_1, t_0, t_1, l_0, l_1, h_0, h_1, o_0, o_1];
//             let mut scaled_values: Vec<f64> = Vec::new();
//             for value in &new_values {
//                 if let Some(val) = value {
//                     scaled_values.push(val / divisor);
//                 } 
//                 else {
//                     println!("One of the values was None");
//                     panic!("a_0: {:?}, a_1: {:?}, a_2: {:?}, b_0: {:?}, b_1: {:?}, 
//                     b_2: {:?}, c_0: {:?}, c_1: {:?}, v_0: {:?}, v_1: {:?}, p_0: {:?},
//                     p_1: {:?}, t_0: {:?}, t_1: {:?}, l_0: {:?}, l_1: {:?}, h_0: {:?},
//                     h_1: {:?}, o_0: {:?}, o_1: {:?}\nmessage: {}", 
//                     &a_0, &a_1, &a_2, &b_0, &b_1, &b_2, &c_0, &c_1, &v_0, &v_1, &p_0,
//                     &p_1, &t_0, &t_1, &l_0, &l_1, &h_0, &h_1, &o_0, &o_1, message);
//                 }
//             }
//             if prefix.contains("SOL") {
//                 let indices: [usize; 20] = [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
//                     25, 26, 27, 28, 29, 30, 31, 32, 33];
//                 println!("updating input 14 to 33. best ask:{:?} best bid: {:?}", &a_0, &b_0);
//                 let mut neural_network = 
//                     shared_neural_network.lock().await;
//                 neural_network.update_input(&indices, &scaled_values).await;
//             }
//             else if prefix.contains("XLM") {
//                 let indices: [usize; 20] = [34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
//                     46, 47, 48, 49, 50, 51, 52, 53];
//                 println!("updating input 34 to 53. best ask:{:?} best bid: {:?}", &a_0, &b_0);
//                 let mut neural_network = 
//                     shared_neural_network.lock().await;
//                 neural_network.update_input(&indices, &scaled_values).await;
//             }
//             else if prefix.contains("XRP") {
//                 //second set of indices larger than last time.
//                 //  65 to 71 is in coinbase
//                 let indices: [usize; 20] = [72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
//                     84, 85, 86, 87, 88, 89, 90, 91];
//                 println!("updating input 72 to 91. best ask:{:?} best bid: {:?}", &a_0, &b_0);
//                 let mut neural_network = 
//                     shared_neural_network.lock().await;
//                 neural_network.update_input(&indices, &scaled_values).await;
//             }
//             else {
//                 panic!("This shouid never occur. Somehow prefix cointained
//                 Kraken Received but didn't contain the phrases SOL, XLM, or XRP.
//                 prefix is: {}", prefix);
//             }
//         }
//         else if prefix.contains("consolidated heartbeat") ||
//             prefix.contains("system status received") || prefix.contains("subscription status received"){
//             println!("Kraken: standard server messages. Ignoring...");
//         }
//         else {
//             println!("Kraken: got a weird message: {}\nprefix: {}", message, prefix);
//         }
//     }

//03/08/24 - added in its place:
pub async fn handle_all_coinbase(prefix: &str, message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>) {
    // Add loop and attempts mechanics
    let mut attempts = 0;
    loop {
        if prefix.contains("Coinbase Received") {
            // Parse the message
            let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

            // Initialize all variables
            let coinbase_price: Option<f64>;
            let coinbase_volume_24h: Option<f64>;
            let coinbase_low_24h: Option<f64>;
            let coinbase_high_24h: Option<f64>;
            let coinbase_low_52w: Option<f64>;
            let coinbase_high_52w: Option<f64>;
            let coinbase_price_percent_chg_24h: Option<f64>;
            match data {
                Ok(value) => {
                    let ticker = &value["events"][0]["tickers"][0];
                    coinbase_price = match parse_f64(ticker["price"].as_str(), message, prefix) {
                        Ok(val) => Some(val),
                        Err(e) => {
                            attempts += 1;
                            log::error!("{}", e);
                            if attempts >= 3 {
                                panic!("{}", e);
                            }
                            continue;
                        }
                    };
                    
                    coinbase_volume_24h = match parse_f64(ticker["volume_24_h"].as_str(), message, prefix) {
                        Ok(val) => Some(val),
                        Err(e) => {
                            attempts += 1;
                            log::error!("{}", e);
                            if attempts >= 3 {
                                panic!("{}", e);
                            }
                            continue;
                        }
                    };
                    
                    coinbase_low_24h = match parse_f64(ticker["low_24_h"].as_str(), message, prefix) {
                        Ok(val) => Some(val),
                        Err(e) => {
                            attempts += 1;
                            log::error!("{}", e);
                            if attempts >= 3 {
                                panic!("{}", e);
                            }
                            continue;
                        }
                    };
                    
                    coinbase_high_24h = match parse_f64(ticker["high_24_h"].as_str(), message, prefix) {
                        Ok(val) => Some(val),
                        Err(e) => {
                            attempts += 1;
                            log::error!("{}", e);
                            if attempts >= 3 {
                                panic!("{}", e);
                            }
                            continue;
                        }
                    };
                    
                    coinbase_low_52w = match parse_f64(ticker["low_52_w"].as_str(), message, prefix) {
                        Ok(val) => Some(val),
                        Err(e) => {
                            attempts += 1;
                            log::error!("{}", e);
                            if attempts >= 3 {
                                panic!("{}", e);
                            }
                            continue;
                        }
                    };
                    
                    coinbase_high_52w = match parse_f64(ticker["high_52_w"].as_str(), message, prefix) {
                        Ok(val) => Some(val),
                        Err(e) => {
                            attempts += 1;
                            log::error!("{}", e);
                            if attempts >= 3 {
                                panic!("{}", e);
                            }
                            continue;
                        }
                    };
                    
                    coinbase_price_percent_chg_24h = match parse_f64(ticker["price_percent_chg_24_h"].as_str(), message, prefix) {
                        Ok(val) => Some(val),
                        Err(e) => {
                            attempts += 1;
                            log::error!("{}", e);
                            if attempts >= 3 {
                                panic!("{}", e);
                            }
                            continue;
                        }
                    };
                },
                Err(e) => {
                    attempts += 1;
                    log::error!("Failed to parse message: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                    if attempts >= 3 {
                        panic!("Failed to parse JSON Coinbase message after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                    }
                    continue;
                },
            }

            // DOUBLE REDUNDANCY
            if coinbase_price.is_none() || coinbase_volume_24h.is_none() || coinbase_low_24h.is_none() || coinbase_high_24h.is_none() || coinbase_low_52w.is_none() || coinbase_high_52w.is_none() || coinbase_price_percent_chg_24h.is_none() {
                attempts += 1;
                log::error!("Failed to parse values after {} attempts\nCoinbase message:\n{}", attempts, message);
                if attempts >= 3 {
                    panic!("Failed to parse values after 3 attempts\nCoinbase message:\n{}", message);
                }
                continue;
            }

            // TRIPLE REDUNDANCY
            // if coinbase_price.is_some() && coinbase_volume_24h.is_some() && coinbase_low_24h.is_some() && coinbase_high_24h.is_some() && coinbase_low_52w.is_some() && coinbase_high_52w.is_some() && coinbase_price_percent_chg_24h.is_some() {
            //     break;
            // }
            if let (Some(mut coinbase_price), Some(mut coinbase_volume_24h), Some(mut coinbase_low_24h), 
            Some(mut coinbase_high_24h), Some(mut coinbase_low_52w), Some(mut coinbase_high_52w), 
            Some(mut coinbase_price_percent_chg_24h)) = 
            (coinbase_price, coinbase_volume_24h, coinbase_low_24h, coinbase_high_24h, coinbase_low_52w, coinbase_high_52w, coinbase_price_percent_chg_24h) {
                if prefix.contains("SOL") {
                    //do the indices and update input and lock
                    coinbase_price = standardization_functions::sol_lognorm_standardization_high_price_24h(&coinbase_price);
                    coinbase_volume_24h = standardization_functions::sol_lognorm_standardization_total_volume_24h(&coinbase_volume_24h);
                    coinbase_low_24h = standardization_functions::sol_lognorm_standardization_low_price_24h(&coinbase_low_24h);
                    coinbase_high_24h = standardization_functions::sol_lognorm_standardization_high_price_24h(&coinbase_high_24h);
                    coinbase_low_52w = standardization_functions::sol_normal_standardization_low_52w(&coinbase_low_52w);
                    coinbase_high_52w = standardization_functions::sol_normal_standardization_high_52w(&coinbase_high_52w);
                    coinbase_price_percent_chg_24h = standardization_functions::sol_normal_standardization_price_percent_change_24h(&coinbase_price_percent_chg_24h);

                    let scaled_values = [coinbase_price, coinbase_volume_24h, 
                    coinbase_low_24h, coinbase_high_24h, coinbase_low_52w, coinbase_high_52w, 
                    coinbase_price_percent_chg_24h];




                    let indices: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
                    println!("updating input 0 to 6. scaled sol price:{:?}", &coinbase_price);
                    let mut neural_network = 
                        shared_neural_network.lock().await;
                    neural_network.update_input(&indices, &scaled_values).await;
                    break;
                }
                else if prefix.contains("XLM") {
                    //do the indices and update input and lock
                    coinbase_price = standardization_functions::xlm_lognorm_standardization_high_price_24h(&coinbase_price);
                    coinbase_volume_24h = standardization_functions::xlm_lognorm_standardization_total_volume_24h(&coinbase_volume_24h);
                    coinbase_low_24h = standardization_functions::xlm_lognorm_standardization_low_price_24h(&coinbase_low_24h);
                    coinbase_high_24h = standardization_functions::xlm_lognorm_standardization_high_price_24h(&coinbase_high_24h);
                    coinbase_low_52w = standardization_functions::xlm_normal_standardization_low_52w(&coinbase_low_52w);
                    coinbase_high_52w = standardization_functions::xlm_normal_standardization_high_52w(&coinbase_high_52w);
                    coinbase_price_percent_chg_24h = standardization_functions::xlm_normal_standardization_price_percent_change_24h(&coinbase_price_percent_chg_24h);

                    let scaled_values = [coinbase_price, coinbase_volume_24h, 
                    coinbase_low_24h, coinbase_high_24h, coinbase_low_52w, coinbase_high_52w, 
                    coinbase_price_percent_chg_24h];




                    let indices: [usize; 7] = [7, 8, 9, 10, 11, 12, 13];
                    println!("updating input 7 to 13. scaled xlm price:{:?}", &coinbase_price);
                    let mut neural_network = 
                        shared_neural_network.lock().await;
                    neural_network.update_input(&indices, &scaled_values).await;
                    break;
                }
                else if prefix.contains("XRP") {
                    //first indices larger than last time
                    coinbase_price = standardization_functions::xrp_lognorm_standardization_high_price_24h(&coinbase_price);
                    coinbase_volume_24h = standardization_functions::xrp_lognorm_standardization_total_volume_24h(&coinbase_volume_24h);
                    coinbase_low_24h = standardization_functions::xrp_lognorm_standardization_low_price_24h(&coinbase_low_24h);
                    coinbase_high_24h = standardization_functions::xrp_lognorm_standardization_high_price_24h(&coinbase_high_24h);
                    coinbase_low_52w = standardization_functions::xrp_normal_standardization_low_52w(&coinbase_low_52w);
                    coinbase_high_52w = standardization_functions::xrp_normal_standardization_high_52w(&coinbase_high_52w);
                    coinbase_price_percent_chg_24h = standardization_functions::xrp_normal_standardization_price_percent_change_24h(&coinbase_price_percent_chg_24h);

                    let scaled_values = [coinbase_price, coinbase_volume_24h, 
                    coinbase_low_24h, coinbase_high_24h, coinbase_low_52w, coinbase_high_52w, 
                    coinbase_price_percent_chg_24h];





                    println!("updating input 61 to 67. scaled xrp price:{:?}", &coinbase_price);
                    let indices: [usize; 7] = [61, 62, 63, 64, 65, 66, 67];
                    let mut neural_network = 
                        shared_neural_network.lock().await;
                    neural_network.update_input(&indices, &scaled_values).await;
                    break;
                }
                else {
                    panic!("This shouid never occur. Somehow prefix cointained
                    Coinbase Received but didn't contain the phrases SOL, XLM, or XRP.
                    prefix is: {}
                    message is:\n{}", prefix, message);
                }
            }



        } else if prefix.contains("Coinbase consolidated heartbeat") || 
                  prefix.contains("Coinbase subscriptions") {
            log::info!("Coinbase: standard server messages. Ignoring...");
            break;
        } else {
            log::error!("Coinbase: got a weird message: {}", message);
            break;
        }
    }

    // The rest of your code goes here...
        
}


pub async fn handle_all_kraken(prefix: &str, message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>) {
    // Add loop and attempts mechanics
    let mut attempts = 0;
    loop {
        if prefix.contains("Kraken Received") {
            // Parse the message
            let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

            // Initialize all variables
            let a_0: Option<f64>;

            let a_2: Option<f64>;
            let b_0: Option<f64>;

            let b_2: Option<f64>;
            let c_0: Option<f64>;
            let c_1: Option<f64>;
            let v_0: Option<f64>;
            let v_1: Option<f64>;
            let p_0: Option<f64>;
            let p_1: Option<f64>;
            let t_0: Option<f64>;
            let t_1: Option<f64>;
            let l_0: Option<f64>;
            let l_1: Option<f64>;
            let h_0: Option<f64>;
            let h_1: Option<f64>;
            let o_0: Option<f64>;
            let o_1: Option<f64>;

            match data {
                Ok(value) => {
                    let ticker = &value[1];

                    // Parsing logic for a_0
                    match ticker["a"][0].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => a_0 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse a_0 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse a_0 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert a_0 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert a_0 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // // Parsing logic for a_1
                    // match ticker["a"][1].as_str() {
                    //     Some(s) => match s.parse::<f64>() {
                    //         Ok(val) => a_1 = Some(val),
                    //         Err(e) => {
                    //             attempts += 1;
                    //             log::error!("WSP kraken: Failed to parse a_1 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                    //             if attempts >= 3 {
                    //                 panic!("Failed to parse a_1 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                    //             }
                    //             continue;
                    //         }
                    //     },
                    //     None => {
                    //         attempts += 1;
                    //         log::error!("WSP kraken: Failed to convert a_1 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                    //         if attempts >= 3 {
                    //             panic!("Failed to convert a_1 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                    //         }
                    //         continue;
                    //     }
                    // }

                    // Parsing logic for a_2
                    match ticker["a"][2].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => a_2 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse a_2 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse a_2 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert a_2 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert a_2 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // Parsing logic for b_0
                    match ticker["b"][0].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => b_0 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse b_0 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse b_0 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert b_0 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert b_0 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // // Parsing logic for b_1
                    // match ticker["b"][1].as_str() {
                    //     Some(s) => match s.parse::<f64>() {
                    //         Ok(val) => b_1 = Some(val),
                    //         Err(e) => {
                    //             attempts += 1;
                    //             log::error!("WSP kraken: Failed to parse b_1 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                    //             if attempts >= 3 {
                    //                 panic!("Failed to parse b_1 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                    //             }
                    //             continue;
                    //         }
                    //     },
                    //     None => {
                    //         attempts += 1;
                    //         log::error!("WSP kraken: Failed to convert b_1 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                    //         if attempts >= 3 {
                    //             panic!("Failed to convert b_1 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                    //         }
                    //         continue;
                    //     }
                    // }

                    // Parsing logic for b_2
                    match ticker["b"][2].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => b_2 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse b_2 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse b_2 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert b_2 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert b_2 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // Parsing logic for c_0
                    match ticker["c"][0].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => c_0 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse c_0 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse c_0 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert c_0 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert c_0 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }
                        // Parsing logic for c_1
                    match ticker["c"][1].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => c_1 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse c_1 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse c_1 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert c_1 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert c_1 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }
                    // Parsing logic for v_0
                    match ticker["v"][0].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => v_0 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse v_0 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse v_0 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert v_0 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert v_0 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // Parsing logic for v_1
                    match ticker["v"][1].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => v_1 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse v_1 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse v_1 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert v_1 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert v_1 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // Parsing logic for p_0
                    match ticker["p"][0].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => p_0 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse p_0 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse p_0 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert p_0 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert p_0 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // Parsing logic for p_1
                    match ticker["p"][1].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => p_1 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse p_1 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse p_1 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert p_1 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert p_1 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // Parsing logic for t_0
                    match ticker["t"][0].as_i64() {
                        Some(val) => t_0 = Some(val as f64),
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert t_0 to i64\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert t_0 to i64 after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }
                    // Parsing logic for t_1
                    match ticker["t"][1].as_i64() {
                        Some(val) => t_1 = Some(val as f64),
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert t_1 to i64\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert t_1 to i64 after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // Parsing logic for l_0
                    match ticker["l"][0].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => l_0 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse l_0 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse l_0 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert l_0 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert l_0 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // Parsing logic for l_1
                    match ticker["l"][1].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => l_1 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse l_1 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse l_1 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert l_1 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert l_1 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // Parsing logic for h_0
                    match ticker["h"][0].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => h_0 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse h_0 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse h_0 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert h_0 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert h_0 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // Parsing logic for h_1
                    match ticker["h"][1].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => h_1 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse h_1 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse h_1 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert h_1 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert h_1 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // Parsing logic for o_0
                    match ticker["o"][0].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => o_0 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse o_0 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse o_0 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("WSP kraken: Failed to convert o_0 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert o_0 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }

                    // Parsing logic for o_1
                    match ticker["o"][1].as_str() {
                        Some(s) => match s.parse::<f64>() {
                            Ok(val) => o_1 = Some(val),
                            Err(e) => {
                                attempts += 1;
                                log::error!("WSP kraken: Failed to parse o_1 as f64: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                                if attempts >= 3 {
                                    panic!("Failed to parse o_1 as f64 after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                                }
                                continue;
                            }
                        },
                        None => {
                            attempts += 1;
                            log::error!("wSP Kraken: Failed to convert o_1 to string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("Failed to convert o_1 to string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    }
                    // Check each variable individually AGAIN
                    if a_0.is_none() || a_2.is_none() || b_0.is_none() || b_2.is_none() || c_0.is_none() ||
                    c_1.is_none() || v_0.is_none() || v_1.is_none() || p_0.is_none() || p_1.is_none() || t_0.is_none() || t_1.is_none() || 
                    l_0.is_none() || l_1.is_none() || h_0.is_none() || h_1.is_none() || o_0.is_none() || o_1.is_none() {
                        attempts += 1;
                        log::error!("WSP kraken: Failed to parse values after {} attempts\nKraken message:\n{}", attempts, message);
                        if attempts >= 3 {
                            panic!("Failed to parse values after 3 attempts\nKraken message:\n{}", message);
                        }
                        continue;
                    }

                    // TRIPLE REDUNDANCY!
                    if let (Some(mut a_0), Some(mut a_2), Some(mut b_0), Some(mut b_2),
                    Some(mut c_0), Some(mut c_1), Some(mut v_0), Some(mut v_1), Some(mut p_0), Some(mut p_1), 
                    Some(mut t_0), Some(mut t_1), Some(mut l_0), Some(mut l_1), Some(mut h_0), Some(mut h_1), 
                    Some(mut o_0), Some(mut o_1)) = 
                    (a_0, a_2, b_0, b_2, c_0, c_1, v_0, v_1, p_0, p_1, t_0, t_1, l_0, l_1, h_0, h_1, o_0, o_1) {
                        //println!("before scaling: a_0 = {a_0}, ")
                        if prefix.contains("SOL") {
                            a_0 = standardization_functions::sol_lognorm_standardization_high_price_24h(&a_0);
                            //now that I think about it, a1 and b1 are a waste of inputs. they are just a2 and b2 rounded up to the nearest int.
                            //a_1 WILL BE REMOVED ALTER
                            a_2 = standardization_functions::sol_lognorm_standardization_lot_volume_per_trade(&a_2);
                            b_0 = standardization_functions::sol_lognorm_standardization_high_price_24h(&b_0);
                            //b_1 WILL BE REMOVED LATER
                            b_2 = standardization_functions::sol_lognorm_standardization_lot_volume_per_trade(&b_2);
                            c_0 = standardization_functions::sol_lognorm_standardization_close_price_24h(&c_0);
                            c_1 = standardization_functions::sol_lognorm_standardization_lot_volume_per_trade(&c_1);
                            v_0 = standardization_functions::sol_lognorm_standardization_total_volume_24h(&v_0);
                            v_1 = standardization_functions::sol_lognorm_standardization_total_volume_24h(&v_1);
                            p_0 = standardization_functions::sol_lognorm_standardization_vwap_24h(&p_0);
                            p_1 = standardization_functions::sol_lognorm_standardization_vwap_24h(&p_1);
                            t_0 = standardization_functions::sol_lognorm_standardization_total_trades_24h(&t_0);
                            t_1 = standardization_functions::sol_lognorm_standardization_total_trades_24h(&t_1);
                            l_0 = standardization_functions::sol_lognorm_standardization_low_price_24h(&l_0);
                            l_1 = standardization_functions::sol_lognorm_standardization_low_price_24h(&l_1);
                            h_0 = standardization_functions::sol_lognorm_standardization_high_price_24h(&h_0);
                            h_1 = standardization_functions::sol_lognorm_standardization_high_price_24h(&h_1);
                            o_0 = standardization_functions::sol_lognorm_standardization_open_price_24h(&o_0);
                            o_1 = standardization_functions::sol_lognorm_standardization_open_price_24h(&o_1);
                            //rest of variables
                            let scaled_values = [a_0, a_2, b_0, b_2, c_0, c_1, 
                            v_0, v_1, p_0, p_1, t_0, t_1, l_0, l_1, h_0, h_1, o_0, o_1];





                            let indices: [usize; 18] = [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                                25, 26, 27, 28, 29, 30, 31];
                            println!("updating input 14 to 31. scaled kraken xrp best ask:{:?} best bid: {:?}", &a_0, &b_0);
                            let mut neural_network = 
                                shared_neural_network.lock().await;
                            neural_network.update_input(&indices, &scaled_values).await;
                            break;
                        }
                        else if prefix.contains("XLM") {
                            a_0 = standardization_functions::xlm_lognorm_standardization_high_price_24h(&a_0);
                            //now that I think about it, a1 and b1 are a waste of inputs. they are just a2 and b2 rounded up to the nearest int.
                            //a_1 WILL BE REMOVED ALTER
                            a_2 = standardization_functions::xlm_lognorm_standardization_lot_volume_per_trade(&a_2);
                            b_0 = standardization_functions::xlm_lognorm_standardization_high_price_24h(&b_0);
                            //b_1 WILL BE REMOVED LATER
                            b_2 = standardization_functions::xlm_lognorm_standardization_lot_volume_per_trade(&b_2);
                            c_0 = standardization_functions::xlm_lognorm_standardization_close_price_24h(&c_0);
                            c_1 = standardization_functions::xlm_lognorm_standardization_lot_volume_per_trade(&c_1);
                            v_0 = standardization_functions::xlm_lognorm_standardization_total_volume_24h(&v_0);
                            v_1 = standardization_functions::xlm_lognorm_standardization_total_volume_24h(&v_1);
                            p_0 = standardization_functions::xlm_lognorm_standardization_vwap_24h(&p_0);
                            p_1 = standardization_functions::xlm_lognorm_standardization_vwap_24h(&p_1);
                            t_0 = standardization_functions::xlm_lognorm_standardization_total_trades_24h(&t_0);
                            t_1 = standardization_functions::xlm_lognorm_standardization_total_trades_24h(&t_1);
                            l_0 = standardization_functions::xlm_lognorm_standardization_low_price_24h(&l_0);
                            l_1 = standardization_functions::xlm_lognorm_standardization_low_price_24h(&l_1);
                            h_0 = standardization_functions::xlm_lognorm_standardization_high_price_24h(&h_0);
                            h_1 = standardization_functions::xlm_lognorm_standardization_high_price_24h(&h_1);
                            o_0 = standardization_functions::xlm_lognorm_standardization_open_price_24h(&o_0);
                            o_1 = standardization_functions::xlm_lognorm_standardization_open_price_24h(&o_1);
                            let scaled_values = [a_0, a_2, b_0, b_2, c_0, c_1, 
                            v_0, v_1, p_0, p_1, t_0, t_1, l_0, l_1, h_0, h_1, o_0, o_1];

                            let indices: [usize; 18] = [32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
                                46, 47, 48, 49];
                            println!("updating input 34 to 53. scaled kraken xrp best ask:{:?} best bid: {:?}", &a_0, &b_0);
                            let mut neural_network = 
                                shared_neural_network.lock().await;
                            neural_network.update_input(&indices, &scaled_values).await;
                            break;
                        }
                        else if prefix.contains("XRP") {
                            a_0 = standardization_functions::xrp_lognorm_standardization_high_price_24h(&a_0);
                            //now that I think about it, a1 and b1 are a waste of inputs. they are just a2 and b2 rounded up to the nearest int.
                            //a_1 WILL BE REMOVED ALTER
                            a_2 = standardization_functions::xrp_lognorm_standardization_lot_volume_per_trade(&a_2);
                            b_0 = standardization_functions::xrp_lognorm_standardization_high_price_24h(&b_0);
                            //b_1 WILL BE REMOVED LATER
                            b_2 = standardization_functions::xrp_lognorm_standardization_lot_volume_per_trade(&b_2);
                            c_0 = standardization_functions::xrp_lognorm_standardization_close_price_24h(&c_0);
                            c_1 = standardization_functions::xrp_lognorm_standardization_lot_volume_per_trade(&c_1);
                            v_0 = standardization_functions::xrp_lognorm_standardization_total_volume_24h(&v_0);
                            v_1 = standardization_functions::xrp_lognorm_standardization_total_volume_24h(&v_1);
                            p_0 = standardization_functions::xrp_lognorm_standardization_vwap_24h(&p_0);
                            p_1 = standardization_functions::xrp_lognorm_standardization_vwap_24h(&p_1);
                            t_0 = standardization_functions::xrp_lognorm_standardization_total_trades_24h(&t_0);
                            t_1 = standardization_functions::xrp_lognorm_standardization_total_trades_24h(&t_1);
                            l_0 = standardization_functions::xrp_lognorm_standardization_low_price_24h(&l_0);
                            l_1 = standardization_functions::xrp_lognorm_standardization_low_price_24h(&l_1);
                            h_0 = standardization_functions::xrp_lognorm_standardization_high_price_24h(&h_0);
                            h_1 = standardization_functions::xrp_lognorm_standardization_high_price_24h(&h_1);
                            o_0 = standardization_functions::xrp_lognorm_standardization_open_price_24h(&o_0);
                            o_1 = standardization_functions::xrp_lognorm_standardization_open_price_24h(&o_1);
                            let scaled_values = [a_0, a_2, b_0, b_2, c_0, c_1, 
                            v_0, v_1, p_0, p_1, t_0, t_1, l_0, l_1, h_0, h_1, o_0, o_1];
                            //second set of indices larger than last time.

                            let indices: [usize; 18] = [68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
                                84, 85];
                            println!("updating input 68 to 85. scaled kraken xrp best ask:{:?} best bid: {:?}", &a_0, &b_0);
                            let mut neural_network = 
                                shared_neural_network.lock().await;
                            neural_network.update_input(&indices, &scaled_values).await;
                            break;
                        }
                        else {
                            panic!("This shouid never occur. Somehow prefix cointained
                            Kraken Received but didn't contain the phrases SOL, XLM, or XRP.
                            prefix is: {}", prefix);
                        }
                    }
                },
                Err(e) => {
                    attempts += 1;
                    log::error!("WSP KRAKEN: Failed to parse message: {}\nMessage: {}\nPrefix: {}\nAttempt: {}", e, message, prefix, attempts);
                    if attempts >= 3 {
                        panic!("Failed to parse JSON Kraken message after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                    }
                    continue;
                },
            }
        } else if prefix.contains("consolidated heartbeat") ||
                  prefix.contains("system status received") || 
                  prefix.contains("subscription status received"){
            log::info!("WSP Kraken: standard server messages. Ignoring...");
            break;
        } else {
            log::error!("WSP Kraken: got a weird message: {}\nprefix: {}", message, prefix);
            break;
        }
    }
}
//03/08/24 - removed: ,divisor: &f64
pub async fn handle_all_bitstamp(prefix: &str, message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>) {
    //03/08/24 - added:
    let mut attempts = 0;
    loop {
        if prefix.contains("Bitstamp received") {
            let v: Result<Value, serde_json::Error> = serde_json::from_str(message);
            let amount: Option<f64>;
            let price: Option<f64>;
            match v {
                Ok(value) => {
                    if let Value::Object(map) = &value {
                        // Check if the object has a key "data" whose value is an object
                        if let Some(Value::Object(data)) = map
                        .get("data") {
                            // Extract the values
                            //03/08/24 - changed from:
                                // amount = data.get("amount").and_then(Value::as_f64);
                                // price = data.get("price").and_then(Value::as_f64);
                            //03/08/24 - to:
                                amount = match data.get("amount") {
                                    Some(val) => match val.as_f64() {
                                        Some(f) => Some(f),
                                        None => {
                                            attempts += 1;
                                            log::error!("WSP bitstamp: Failed to parse 'amount' as f64\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                                            if attempts >= 3 {
                                                panic!("Failed to parse 'amount' as f64 after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                                            }
                                            continue;
                                        }
                                    },
                                    None => {
                                        attempts += 1;
                                        log::error!("WSP bitstamp: 'amount' is not a string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                                        if attempts >= 3 {
                                            panic!("'amount' is not a string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                                        }
                                        continue;
                                    }
                                };
                                price = match data.get("price") {
                                    Some(val) => match val.as_f64() {
                                        Some(f) => Some(f),
                                        None => {
                                            attempts += 1;
                                            log::error!("WSP bitstamp: Failed to parse 'price' as f64\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                                            if attempts >= 3 {
                                                panic!("Failed to parse 'price' as f64 after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                                            }
                                            continue;
                                        }
                                    },
                                    None => {
                                        attempts += 1;
                                        log::error!("WSP bitstamp: 'price' is not a string\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                                        if attempts >= 3 {
                                            panic!("'price' is not a string after 3 attempts. Message: {}, Prefix: {}", message, prefix);
                                        }
                                        continue;
                                    }
                                };
                        } 
                        else {
                            attempts += 1;
                            log::error!("WSP bitstamp: map does not contain a 'data' key\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("bitstamp : The map does not contain a 'data' key
                                message: {}", message);
                            }
                            continue;
                        }
                    } 
                    else {
                        //03/08/24 - removed:
                            //panic!("Value is not an object.\nmessage:{}", message);
                        //03/08/24 - added:
                            attempts += 1;
                            log::error!("WSP bitstamp: map does not contain a 'data' key\nMessage: {}\nPrefix: {}\nAttempt: {}", message, prefix, attempts);
                            if attempts >= 3 {
                                panic!("WSP bitstamp: The map does not contain a 'data' key
                                message: {}", message);
                            }
                            continue;
                    }
                },
                Err(e) => {
                    //03/08/24 - removed:
                        // panic!("WSP bitstamp: Failed to parse JSON Bitstamp message\nError: {}\nMessage: {}"
                        // , e, message);
                    //03/08/24 - added in its place:
                        attempts += 1;
                        log::error!("WSP bitstamp: Failed to parse JSON Bitstamp message\nMessage: {}\nPrefix: {}\nAttempt: {}
                        error: {}", message, prefix, attempts, e);
                        if attempts >= 3 {
                            panic!("WSP bitstamp: The map does not contain a 'data' key
                            message: {}", message);
                        }
                        continue;
                
                },
            }
            //03/08/24 - removed:
                // let new_values = [amount, price];
                // let mut scaled_values: Vec<f64> = Vec::new();
                // for value in &new_values {
                //     if let Some(val) = value {
                //         scaled_values.push(val / divisor);
                //     } 
                //     else {
                //         println!("One of the values was None");
                //         panic!("amount: {:?}, price: {:?}", &amount, &price);
                //     }
                // }
            if let (Some(mut amount), Some(mut price)) = (amount, price) {
                if prefix.contains("XRP") {
                    //going to use previous SOL indices as XRP

                    amount = standardization_functions::xrp_lognorm_standardization_lot_volume_per_trade(&amount);
                    price = standardization_functions::xrp_lognorm_standardization_high_price_24h(&price);

                    let scaled_values = [amount, price];


                    let indices: [usize; 2] = [50, 51];
                    println!("updating input 50, 51. SCALED bitstamp xlm  price:{:?}", &price);
                    let mut neural_network = 
                        shared_neural_network.lock().await;
                    neural_network.update_input(&indices, &scaled_values).await;
                    break;
                }
                else if prefix.contains("XLM") {
                    amount = standardization_functions::xlm_lognorm_standardization_lot_volume_per_trade(&amount);
                    price = standardization_functions::xlm_lognorm_standardization_high_price_24h(&price);

                    let scaled_values = [amount, price];
                    let indices: [usize; 2] = [52, 53];
                    println!("updating input 52, 53. SCALED bitstamp xlm price:{:?}", &price);
                    let mut neural_network = 
                        shared_neural_network.lock().await;
                    neural_network.update_input(&indices, &scaled_values).await;
                    break;
                }
                else {
                    panic!("This shouid never occur. Somehow prefix cointained phrase
                    Bitstamp received but didn't contain the phrases XLM or XRP.
                    prefix is: {}\nmessage: {}", prefix, message);
                }
            }

        }
        else if prefix.contains("consolidated heartbeat") || 
        prefix.contains("subscription received") {
            //03/08/24 - changed from println! to log::info!
            log::info!("WSP Bitstamp: standard server messages. Ignoring...");
            break;
        }
        else {
            //03/08/24 - changed from println! to log::error!
            log::error!("WSP Bitstamp: got a weird message: {}\nprefix:{}", message, prefix);
            break;
        }
    }
}
//03/08/24 - removed: , divisor: &f64
pub async fn handle_all_gemini(prefix: &str, message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>) {
    //02/07/24 - loop and "attempts" mechanics added
    //      loop is so if there is a random error due to corrupted value, it will be processed again
    let mut attempts = 0;
    loop {
        if prefix.contains("Gemini received") {
            if message.contains("heartbeat") {
                println!("gemini heartbeat. ignoring...");
                return;
            }
            else if message.is_empty() {
                println!("gemini empty message. ignoring...");
                return;
            }
            else {
                let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

                let amount: Option<f64>;
                let price: Option<f64>;

                match data {
                    Ok(value) => {
                        if value.get("socket_sequence")
                        .and_then(Value::as_i64) == Some(0) {
                            println!("Gemini: socket sequence is 0, ignoring...");
                            return;
                        }
                        if let Value::Object(map) = &value {
                            if let Some(Value::Array(events)) = map
                            .get("events") {
                                if let Some(Value::Object(event)) =
                                events.get(0) {
                                    //03/06/24 - removed:
                                        // amount = event.get("amount")
                                        //     .and_then(|v| v.as_str())
                                        //     .and_then(|s| s.parse::<f64>().ok());
                                        // price = event.get("price")
                                        //     .and_then(|v| v.as_str())
                                        //     .and_then(|s| s.parse::<f64>().ok());
                                    //03/06/24 - added in its place:
                                        match event.get("amount") {
                                            Some(v) => match v.as_str() {
                                                Some(s) => match s.parse::<f64>() {
                                                    Ok(price_val) => amount = Some(price_val),
                                                    Err(e) => {
                                                        attempts += 1;
                                                        log::error!("WSP Gemini amount: Failed to parse
                                                        string to f64: 
                                                        attempts: {}
                                                        error:{}\nprefix: {}\nmessage: {}",
                                                            attempts, e, &prefix, &message);
                                                        if attempts >= 3 {
                                                            panic!("Gemini: Failed to parse price
                                                            after 3 attempts\nGemini message:\n{}", 
                                                                &message);
                                                        }
                                                        continue;
                                                    }
                                                },
                                                None => {
                                                    attempts += 1;
                                                    log::error!("WSP Gemini amount: Failed to convert
                                                    Value to string 
                                                    attempt#:{}\nprefix: {}\nmessage: {}",
                                                        attempts, &prefix, &message);
                                                    if attempts >= 3 {
                                                        panic!("Failed to convert Value to string.
                                                        after 3 attempts\nGemini message:\n{}",
                                                            &message);
                                                    }
                                                    continue;
                                                }
                                            },
                                            None => {
                                                attempts += 1;
                                                log::error!("WSP Gemini amount:Failed to parse amount:
                                                attempt#:{}
                                                prefix: {}
                                                message: {}", attempts, &prefix, &message);
                                                if attempts >= 3 {
                                                    panic!("Failed to parse amount
                                                    after 3 attempts\nGemini message:\n{}",
                                                        &message);
                                                }
                                                continue;
                                            }
                                        }
                                        
                                        match event.get("price") {
                                            Some(v) => match v.as_str() {
                                                Some(s) => match s.parse::<f64>() {
                                                    Ok(price_val) => price = Some(price_val),
                                                    Err(e) => {
                                                        attempts += 1;
                                                        log::error!("WSP Gemini price: Failed to parse
                                                        string to f64: 
                                                        attempt:{}
                                                        error:{}\nprefix: {}\nmessage: {}", attempts,
                                                        e, &prefix, &message);
                                                        if attempts >= 3 {
                                                            panic!("Failed to parse price to f64
                                                            after 3 attempts\nGemini message:\n{}",
                                                            &message);
                                                        }
                                                        continue;
                                                    }
                                                },
                                                None => {
                                                    attempts += 1;
                                                    log::error!("WSP Gemini price: Failed to convert 
                                                    Value to string:
                                                    attempt#:{}
                                                    prefix: {}
                                                    message: {}", 
                                                        attempts, &prefix, &message);
                                                    if attempts >= 3 {
                                                        panic!("Failed to convert
                                                        Value to string after 3 attempts
                                                        prefix: {}Gemini message:\n{}", 
                                                            &prefix, &message);
                                                    }
                                                    continue;
                                                }
                                            },
                                            None => {
                                                attempts += 1;
                                                log::error!("WSP Gemini price: Failed to get price:
                                                attempt#: {}
                                                prefix: {}
                                                message: {}", attempts, &prefix, &message);
                                                if attempts >= 3 {
                                                    panic!("Failed to parse price after 3 attempts
                                                    Gemini message:\n{}", &message);
                                                }
                                                continue;
                                            }
                                        }
                                    //02/07/24 - added if block. DOUBLE REDUNDANCY
                                    if amount.is_none() || price.is_none() {
                                        attempts += 1;
                                        log::error!("WSP gemini: amount and or price is none. amount: {:?}. price: {:?}
                                        attempt#:{}
                                        prefix:{}
                                        message:\n{}", amount, price, attempts, prefix, message);
                                        if attempts >= 3 {
                                            panic!("Failed to parse amount: {:?} and/or price: {:?} after 3 attempts\nGemini message:\n{}", amount, price, &message);
                                        }
                                        continue;
                                    }
                                } 
                                else {
                                    attempts += 1;
                                    log::error!("WSP Gemini: event is not an object. message: 
                                    {}
                                    , prefix: {}
                                    attempt #:{}", message, prefix, &attempts);
                                    if attempts >= 3 {
                                        panic!("Gemini: event is not an object after 3 attempts. message: {}, prefix: {}", message, prefix);
                                    }
                                    continue;
                                }
                            } 
                            else {
                                attempts += 1;
                                log::error!("WSP Gemini: events is not an array.
                                message: {}
                                , prefix: {}
                                attempt#:{}", message, prefix, &attempts);
                                if attempts >= 3 {
                                    panic!("Gemini: events is not an array after 3 attempts. message: {}, prefix: {}", message, prefix);
                                }
                                continue;
                            }
                        } 
                        else {
                            attempts += 1;
                            log::error!("Gemini: Value is not an object after 3 attempts.
                            message: {}
                            , prefix: {}
                            attempt#:{}", message, prefix, &attempts);
                            if attempts >= 3 {
                                panic!("Gemini: Value is not an object after 3 attempts. message: {}, prefix: {}", message, prefix);
                            }
                            continue;
                        }
                    },
                    Err(e) => {
                        attempts += 1;
                        log::error!("Failed to parse JSON Gemini message after 3 attempts.
                        Error: {}
                        , Message: {}
                        , Prefix: {}
                        attempt#:{}", e, message, prefix, &attempts);
                        if attempts >= 3 {
                            panic!("Failed to parse JSON Gemini message after 3 attempts. Error: {}, Message: {}, Prefix: {}", e, message, prefix);
                        }
                        continue;
                    },
                }
                //03/08/24 - removed:
                    // let new_values = [amount, price];
                    // let mut scaled_values: Vec<f64> = Vec::new();
                    // for value in &new_values {
                    //     if let Some(val) = value {
                    //         scaled_values.push(val / divisor);
                    //     } 
                    //     else {
                    //         println!("One of the values was None");
                    //         panic!("amount: {:?}, price: {:?}", &amount, &price);
                    //     }
                    // }
                //03/08/24 - added:
                if let (Some(mut amount), Some(mut price)) = (amount, price) {
                    if prefix.contains("sol") {

                        amount = standardization_functions::sol_lognorm_standardization_lot_volume_per_trade(&amount);
                        price = standardization_functions::sol_lognorm_standardization_high_price_24h(&price);

                        let scaled_values = [amount, price];

                        let indices = [54, 55];
                        println!("updating input 54, 55. price:{:?}", &price);
                        let mut neural_network = 
                            shared_neural_network.lock().await;
                        neural_network.update_input(&indices, &scaled_values)
                        .await;
                        break;
                    }
                    else if prefix.contains("xrp") {

                        amount = standardization_functions::xrp_lognorm_standardization_lot_volume_per_trade(&amount);
                        price = standardization_functions::xrp_lognorm_standardization_high_price_24h(&price);

                        let scaled_values = [amount, price];

                        let indices = [86, 87];
                        println!("updating input 86, 87. price:{:?}", &price);
                        let mut neural_network = 
                            shared_neural_network.lock().await;
                        neural_network.update_input(&indices, &scaled_values)
                        .await;
                        break;
                    }
                    else {
                        panic!("This shouid never occur. Somehow prefix cointained phrase
                        Gemini received but didn't contain the phrases XLM or XRP.
                        prefix is: {}\nmessage: {}", prefix, message);
                    }
                }
            }
        }
        else {
            log::error!("Gemini: got a weird message: {}\nprefix:{}", message, prefix);
            break;
        }
    }
}
pub fn handle_all_others(prefix: &str, message: &str) {
    //03/08/24 - changed from println to log::error
    log::error!("weird ass message that doesn't contain the words:
    Coinbase, Kraken, Bitstmap, nor Gemini.
    prefix: {}\nmessage: {}", &prefix, &message);
}

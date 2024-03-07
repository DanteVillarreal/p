use crate::network::NeuralNetwork;                      // to use neural network struct
use tokio::sync::Mutex;                             // Use async Mutex from Tokio
use std::sync::Arc;  								// Use Arc to share Mutex among multiple tasks
use serde_json::Value;                              // for parsing input form websocket client


//helper function
pub fn parse_f64(value: Option<&str>, message: &str, prefix: &str) -> Option<f64> {
    match value {
        Some(str_value) => {
            match str_value.parse::<f64>() {
                Ok(parsed_value) => Some(parsed_value),
                Err(e) => {
                    panic!("Failed to parse as f64: {:?}
                    message: {}
                    prefix: {}", e, message, prefix);
                }
            }
        },
        None => {
            panic!("Value is not a string. 
            message: {}
            prefix: {}", message, prefix);
        }
    }
}

pub async fn handle_all_coinbase(prefix: &str, message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, divisor: &f64) {

    if prefix.contains("Coinbase Received") {
        //do same calculations and then differentiate between coins
        //   at the end
        let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

        //variable declaration so I can have a larger scope
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
                coinbase_price = parse_f64(ticker["price"]
                    .as_str(), message, prefix);
                coinbase_volume_24h = parse_f64(ticker["volume_24_h"]
                    .as_str(), message, prefix);
                coinbase_low_24h = parse_f64(ticker["low_24_h"]
                    .as_str(), message, prefix);
                coinbase_high_24h = parse_f64(ticker["high_24_h"]
                    .as_str(), message, prefix);
                coinbase_low_52w = parse_f64(ticker["low_52_w"]
                    .as_str(), message, prefix);
                coinbase_high_52w = parse_f64(ticker["high_52_w"]
                    .as_str(), message, prefix);
                coinbase_price_percent_chg_24h = parse_f64(ticker["price_percent_chg_24_h"]
                    .as_str(), message, prefix);
            },
            Err(e) => panic!("Failed to get data from COINBASE message.
                Error {}\n{}", e, message),
        }

        let new_values = [coinbase_price, coinbase_volume_24h,
        coinbase_low_24h, coinbase_high_24h, coinbase_low_52w,
        coinbase_high_52w, coinbase_price_percent_chg_24h,];
        let mut scaled_values: Vec<f64> = Vec::new();
        for value in &new_values {
            if let Some(val) = value {
                scaled_values.push(val / divisor);
            } 
            else {
                println!("One of the values was None");
                panic!("coinbase_price: {:?}, coinbase_volume_24_h: {:?},
                coinbase_low_24h: {:?}, coinbase_high_24h: {:?}, 
                coinbase_low_52w: {:?}, coinbase_high_52w: {:?}, 
                coinbase_price_percent_chg_24h: {:?}, message:\n{}", &coinbase_price,
                &coinbase_volume_24h, &coinbase_low_24h, &coinbase_high_24h,
                &coinbase_low_52w, &coinbase_high_52w,
                &coinbase_price_percent_chg_24h, message);
            }
        }
        if prefix.contains("SOL") {
            //do the indices and update input and lock
            let indices: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
            println!("updating input 0 to 6. price:{:?}", &coinbase_price);
            let mut neural_network = 
                shared_neural_network.lock().await;
            neural_network.update_input(&indices, &scaled_values).await;
        }
        else if prefix.contains("XLM") {
            //do the indices and update input and lock
            let indices: [usize; 7] = [7, 8, 9, 10, 11, 12, 13];
            println!("updating input 7 to 13. price:{:?}", &coinbase_price);
            let mut neural_network = 
                shared_neural_network.lock().await;
            neural_network.update_input(&indices, &scaled_values).await;

        }
        else if prefix.contains("XRP") {
            //first indices larger than last time
            println!("updating input 65 to 71. price:{:?}", &coinbase_price);
            let indices: [usize; 7] = [65, 66, 67, 68, 69, 70, 71];
            let mut neural_network = 
                shared_neural_network.lock().await;
            neural_network.update_input(&indices, &scaled_values).await;
        }
        else {
            panic!("This shouid never occur. Somehow prefix cointained
            Coinbase Received but didn't contain the phrases SOL, XLM, or XRP.
            prefix is: {}", prefix);
        }
        
    }
    else if prefix.contains("Coinbase consolidated heartbeat") || 
        prefix.contains("Coinbase subscriptions") {

        println!("Coinbase: standard server messages. Ignoring...");
    }
    else {
        println!("Coinbase: got a weird message:{}", message);
    }

}
pub async fn handle_all_kraken(prefix: &str, message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, divisor: &f64) {


    if prefix.contains("Kraken Received") {
        let data: Result<Value, serde_json::Error> = serde_json::from_str(message);
    
        let mut a_0: Option<f64> = None;
        let mut a_1: Option<f64> = None;
        let mut a_2: Option<f64> = None;

        let mut b_0: Option<f64> = None;
        let mut b_1: Option<f64> = None;
        let mut b_2: Option<f64> = None;

        let mut c_0: Option<f64> = None;
        let mut c_1: Option<f64> = None;

        let mut v_0: Option<f64> = None;
        let mut v_1: Option<f64> = None;

        let mut p_0: Option<f64> = None;
        let mut p_1: Option<f64> = None;

        let mut t_0: Option<f64> = None;
        let mut t_1: Option<f64> = None;

        let mut l_0: Option<f64> = None;
        let mut l_1: Option<f64> = None;

        let mut h_0: Option<f64> = None;
        let mut h_1: Option<f64> = None;

        let mut o_0: Option<f64> = None;
        let mut o_1: Option<f64> = None;
    
        match data {
            Ok(value) => {
                let ticker = &value[1];
                a_0 = Some(ticker["a"][0].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for a[0]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse a[0] as f64. Full message: {}", message);
                    panic!();
                }));
                a_1 = Some(ticker["a"][1].as_i64().unwrap_or_else(|| {
                    println!("Failed to get string for a[1]. Full message: {}", message);
                    panic!();
                }) as f64);
                a_2 = Some(ticker["a"][2].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for a[2]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse a[2] as f64. Full message: {}", message);
                    panic!();
                }));
                b_0 = Some(ticker["b"][0].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for b[0]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse b[0] as f64. Full message: {}", message);
                    panic!();
                }));
                b_1 = Some(ticker["b"][1].as_i64().unwrap_or_else(|| {
                    println!("Failed to get string for a[1]. Full message: {}", message);
                    panic!();
                }) as f64);
                b_2 = Some(ticker["b"][2].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for b[2]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse b[2] as f64. Full message: {}", message);
                    panic!();
                }));
                c_0 = Some(ticker["c"][0].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for c[0]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse c[0] as f64. Full message: {}", message);
                    panic!();
                }));
                c_1 = Some(ticker["c"][1].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for c[1]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse c[1] as f64. Full message: {}", message);
                    panic!();
                }));
                v_0 = Some(ticker["v"][0].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for v[0]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse v[0] as f64. Full message: {}", message);
                    panic!();
                }));
                v_1 = Some(ticker["v"][1].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for v[1]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse v[1] as f64. Full message: {}", message);
                    panic!();
                }));
                p_0 = Some(ticker["p"][0].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for p[0]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse p[0] as f64. Full message: {}", message);
                    panic!();
                }));
                p_1 = Some(ticker["p"][1].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for p[1]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse p[1] as f64. Full message: {}", message);
                    panic!();
                }));
                t_0 = Some(ticker["t"][0].as_i64().unwrap_or_else(|| {
                    println!("Failed to get string for a[1]. Full message: {}", message);
                    panic!();
                }) as f64);
                t_1 = Some(ticker["t"][1].as_i64().unwrap_or_else(|| {
                    println!("Failed to get string for a[1]. Full message: {}", message);
                    panic!();
                }) as f64);
                l_0 = Some(ticker["l"][0].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for l[0]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse l[0] as f64. Full message: {}", message);
                    panic!();
                }));
                l_1 = Some(ticker["l"][1].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for l[1]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse l[1] as f64. Full message: {}", message);
                    panic!();
                }));
                h_0 = Some(ticker["h"][0].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for h[0]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse h[0] as f64. Full message: {}", message);
                    panic!();
                }));
                h_1 = Some(ticker["h"][1].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for h[1]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse h[1] as f64. Full message: {}", message);
                    panic!();
                }));
                o_0 = Some(ticker["o"][0].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for o[0]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse o[0] as f64. Full message: {}", message);
                    panic!();
                }));
                o_1 = Some(ticker["o"][1].as_str().unwrap_or_else(|| {
                    println!("Failed to get string for o[1]. Full message: {}", message);
                    panic!();
                }).parse::<f64>().unwrap_or_else(|_| {
                    println!("Failed to parse o[1] as f64. Full message: {}", message);
                    panic!();
                }));
            }
            Err(e) => println!("Failed to parse message: {}", e),
        }
        let new_values = [a_0, a_1, a_2, b_0, b_1, b_2, c_0, c_1, 
        v_0, v_1, p_0, p_1, t_0, t_1, l_0, l_1, h_0, h_1, o_0, o_1];
        let mut scaled_values: Vec<f64> = Vec::new();
        for value in &new_values {
            if let Some(val) = value {
                scaled_values.push(val / divisor);
            } 
            else {
                println!("One of the values was None");
                panic!("a_0: {:?}, a_1: {:?}, a_2: {:?}, b_0: {:?}, b_1: {:?}, 
                b_2: {:?}, c_0: {:?}, c_1: {:?}, v_0: {:?}, v_1: {:?}, p_0: {:?},
                 p_1: {:?}, t_0: {:?}, t_1: {:?}, l_0: {:?}, l_1: {:?}, h_0: {:?},
                 h_1: {:?}, o_0: {:?}, o_1: {:?}\nmessage: {}", 
                &a_0, &a_1, &a_2, &b_0, &b_1, &b_2, &c_0, &c_1, &v_0, &v_1, &p_0,
                &p_1, &t_0, &t_1, &l_0, &l_1, &h_0, &h_1, &o_0, &o_1, message);

            }
        }
        if prefix.contains("SOL") {
            let indices: [usize; 20] = [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                25, 26, 27, 28, 29, 30, 31, 32, 33];
            println!("updating input 14 to 33. best ask:{:?} best bid: {:?}", &a_0, &b_0);
            let mut neural_network = 
                shared_neural_network.lock().await;
            neural_network.update_input(&indices, &scaled_values).await;
        }
        else if prefix.contains("XLM") {
            let indices: [usize; 20] = [34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
                46, 47, 48, 49, 50, 51, 52, 53];
            println!("updating input 34 to 53. best ask:{:?} best bid: {:?}", &a_0, &b_0);
            let mut neural_network = 
                shared_neural_network.lock().await;
            neural_network.update_input(&indices, &scaled_values).await;
        }
        else if prefix.contains("XRP") {
            //second set of indices larger than last time.
            //  65 to 71 is in coinbase
            let indices: [usize; 20] = [72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
                84, 85, 86, 87, 88, 89, 90, 91];
            println!("updating input 72 to 91. best ask:{:?} best bid: {:?}", &a_0, &b_0);
            let mut neural_network = 
                shared_neural_network.lock().await;
            neural_network.update_input(&indices, &scaled_values).await;
        }
        else {
            panic!("This shouid never occur. Somehow prefix cointained
            Kraken Received but didn't contain the phrases SOL, XLM, or XRP.
            prefix is: {}", prefix);
        }
    }
    else if prefix.contains("consolidated heartbeat") ||
        prefix.contains("system status received") || prefix.contains("subscription status received"){
        println!("Kraken: standard server messages. Ignoring...");
    }
    else {
        println!("Kraken: got a weird message: {}\nprefix: {}", message, prefix);
    }
}

pub async fn handle_all_bitstamp(prefix: &str, message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, divisor: &f64) {

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
                        amount = data.get("amount").and_then(Value::as_f64);
                        price = data.get("price").and_then(Value::as_f64);
                    } 
                    else {
                        panic!("The map does not contain a 'data' key
                        message: {}", message);
                    }
                } 
                else {
                    panic!("Value is not an object.\nmessage:{}", message);
                }
            },
            Err(e) => {
                panic!("Failed to parse JSON Bitstamp message\nError: {}\nMessage: {}"
                , e, message);
            
            },
        }
        let new_values = [amount, price];
        let mut scaled_values: Vec<f64> = Vec::new();
        for value in &new_values {
            if let Some(val) = value {
                scaled_values.push(val / divisor);
            } 
            else {
                println!("One of the values was None");
                panic!("amount: {:?}, price: {:?}", &amount, &price);
            }
        }
        if prefix.contains("XRP") {
            //going to use SOL indices as XRP
            let indices: [usize; 2] = [54, 55];
            println!("updating input 54, 55. price:{:?}", &price);
            let mut neural_network = 
                shared_neural_network.lock().await;
            neural_network.update_input(&indices, &scaled_values).await;
        }
        else if prefix.contains("XLM") {
            let indices: [usize; 2] = [56, 57];
            println!("updating input 56, 57. price:{:?}", &price);
            let mut neural_network = 
                shared_neural_network.lock().await;
            neural_network.update_input(&indices, &scaled_values).await;
        }
        else {
            panic!("This shouid never occur. Somehow prefix cointained phrase
            Bitstamp received but didn't contain the phrases XLM or XRP.
            prefix is: {}\nmessage: {}", prefix, message);
        }

    }
    else if prefix.contains("consolidated heartbeat") || 
    prefix.contains("subscription received") {
        println!("Bitstamp: standard server messages. Ignoring...");
    }
    else {
        println!("Bitstamp: got a weird message: {}\nprefix:{}", message, prefix);
    }
}
pub async fn handle_all_gemini(prefix: &str, message: &str, shared_neural_network: Arc<Mutex<NeuralNetwork>>, divisor: &f64) {
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
                                                        log::error!("Gemini amount: Failed to parse
                                                        string to f64: {}\nprefix: {}\nmessage: {}",
                                                            e, &prefix, &message);
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
                                                    log::error!("Gemini amount: Failed to convert
                                                    Value to string \nprefix: {}\nmessage: {}",
                                                        &prefix, &message);
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
                                                log::error!("Gemini amount:Failed to parse amount:
                                                prefix: {}
                                                message: {}", &prefix, &message);
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
                                                        log::error!("Gemini price: Failed to parse
                                                        string to f64: {}\nprefix: {}\nmessage: {}",
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
                                                    log::error!("Gemini price: Failed to convert 
                                                    Value to string:
                                                    prefix: {}
                                                    message: {}", 
                                                        &prefix, &message);
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
                                                log::error!("Gemini price: Failed to get price:
                                                prefix: {}
                                                message: {}", &prefix, &message);
                                                if attempts >= 3 {
                                                    panic!("Failed to parse price after 3 attempts
                                                    Gemini message:\n{}", &message);
                                                }
                                                continue;
                                            }
                                        }
                                    //02/07/24 - added if block
                                    if amount.is_none() || price.is_none() {
                                        attempts += 1;
                                        if attempts >= 3 {
                                            panic!("Failed to parse amount: {:?} and/or price: {:?} after 3 attempts\nGemini message:\n{}", amount, price, &message);
                                        }
                                        continue;
                                    }
                                } 
                                else {
                                    attempts += 1;
                                    log::error!("Gemini: event is not an object after 3 attempts. message: 
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
                                log::error!("Gemini: events is not an array after 3 attempts. 
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

                let new_values = [amount, price];
                let mut scaled_values: Vec<f64> = Vec::new();
                for value in &new_values {
                    if let Some(val) = value {
                        scaled_values.push(val / divisor);
                    } 
                    else {
                        println!("One of the values was None");
                        panic!("amount: {:?}, price: {:?}", &amount, &price);
                    }
                }
                if prefix.contains("sol") {
                    let indices = [58, 59];
                    println!("updating input 58, 59. price:{:?}", &price);
                    let mut neural_network = 
                        shared_neural_network.lock().await;
                    neural_network.update_input(&indices, &scaled_values)
                    .await;
                    break;
                }
                else if prefix.contains("xrp") {
                    let indices = [92, 93];
                    println!("updating input 92, 93. price:{:?}", &price);
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
        else {
            println!("Gemini: got a weird message: {}\nprefix:{}", message, prefix);
            break;
        }
    }
}
pub fn handle_all_others(prefix: &str, message: &str) {
    println!("weird ass message that doesn't contain the words:
    Coinbase, Kraken, Bitstmap, nor Gemini.
    prefix: {}\nmessage: {}", &prefix, &message);
}

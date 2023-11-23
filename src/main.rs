use rand_distr::{StandardNormal, Normal, Distribution};

use rand::Rng;

use crate::network::Network::NeuralNetwork;
//use mod network;
//use mod actions;



///-----FOR PARSING-----////
use std::env;
use std::process::{Command, Stdio};
use serde_json::Value;          //good for parsing intput in JSON format
                                //  aka what the messages are in
use std::io::{BufRead, BufReader};//this is to help us read from stdin

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
fn handle_coinbase(message: &str) {
    //if the message contains the word "heartbeat", ignore the entire message basically
    if message.contains("heartbeat") {
        println!("Coinbase heartbeat message. ignoring...");
        return;
    }
    if message.trim().is_empty() {
        println!("Coinbase: blank message received\nmessage: {}", message);
        return;
    }
    let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

    //variable declaration so I can have a larger scope
    let mut coinbase_price = 0.0;
    let mut coinbase_open_24h = 0.0;
    let mut coinbase_volume_24h = 0.0;
    let mut coinbase_low_24h = 0.0;
    let mut coinbase_high_24h = 0.0;
    let mut coinbase_volume_30d = 0.0;
    let mut coinbase_best_bid = 0.0;
    let mut coinbase_best_bid_size = 0.0;
    let mut coinbase_best_ask = 0.0;
    let mut coinbase_best_ask_size = 0.0;
    let mut coinbase_side = "pppp";
    let mut coinbase_last_size = 0.0;


    match data {
        Ok(value) => {


            coinbase_price = value["price"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_open_24h = value["open_24h"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_volume_24h = value["volume_24h"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_low_24h = value["low_24h"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_high_24h = value["high_24h"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_volume_30d = value["volume_30d"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_best_bid = value["best_bid"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_best_bid_size = value["best_bid_size"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_best_ask = value["best_ask"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_best_ask_size = value["best_ask_size"].as_str().unwrap().parse::<f64>().unwrap();
            coinbase_side = value["side"].as_str().unwrap();
            coinbase_last_size = value["last_size"].as_str().unwrap().parse::<f64>().unwrap();


            //just for debugging
            println!("coinbase_price: {}\ncoinbase_open_24h: {}\ncoinbase_volume_24h: {}
                \ncoinbase_low_24h: {}\ncoinbase_high_24h: {}\ncoinbase_volume_30d: {}
                \ncoinbase_best_bid: {}\ncoinbase_best_bid_size: {}\ncoinbase_best_ask: {}
                \ncoinbase_best_ask_size: {}\ncoinbase_side: {}\ncoinbase_last_size: {}", 
                &coinbase_price, &coinbase_open_24h, &coinbase_volume_24h, &coinbase_low_24h, 
                &coinbase_high_24h, &coinbase_volume_30d, &coinbase_best_bid, &coinbase_best_bid_size, 
                &coinbase_best_ask, &coinbase_best_ask_size, &coinbase_side, &coinbase_last_size);



            /*
            if let Some(price) = value.get("price") {
                coinbase_price = price;
            }

            // Check if the payload is an object
            if let Value::Object(map) = &value {
                // Check if the object has a key "events" whose value is an array
                if let Some(Value::Array(events)) = map.get("events") {
                    // Check if the first element of the array is an object
                    if let Some(Value::Object(event)) = events.get(0) {
                        // Check if the object has a key "tickers" whose value is an array
                        if let Some(Value::Array(tickers)) = event.get("tickers") {
                            // Check if the first element of the array is an object
                            if let Some(Value::Object(ticker)) = tickers.get(0) {
                                // Extract the values
                                let price = ticker.get("price").and_then(Value::as_str).unwrap();
                                let volume_24_h = ticker.get("volume_24_h").and_then(Value::as_str)
                                                        .unwrap();
                                let low_24_h = ticker.get("low_24_h").and_then(Value::as_str)
                                                        .unwrap();
                                let high_24_h = ticker.get("high_24_h").and_then(Value::as_str)
                                                        .unwrap();
                                let low_52_w = ticker.get("low_52_w").and_then(Value::as_str)
                                                        .unwrap();
                                let high_52_w = ticker.get("high_52_w").and_then(Value::as_str)
                                                        .unwrap();
                                let price_percent_chg_24_h = ticker.get("price_percent_chg_24_h")
                                                        .and_then(Value::as_str).unwrap();

                                //this is just for debugging purposes
                                println!("Coinbase: ticker: {:?}\nprice: {}\nvolume_24_h: {}\nlow_24_h: {}\n
                                        high_24_h: {}\nlow_52_w: {}\nhigh_52_w: {}\nprice_percent_chg_24_h: {}\n",
                                        &ticker, &price, &volume_24_h, &low_24_h, &high_24_h, &low_52_w, &high_52_w,
                                        &price_percent_chg_24_h);

                                //this is to actually update the neural network with these new inputs
                                let indices = [/* indices of the inputs to update */];
                                let new_values = [/* new values for those inputs */];
                                neural_network.update_input(&indices, &new_values);
                            }
                        }
                    }
                }
            }
            */
        },
        Err(e) => {
            println!("Failed to parse JSON Coinbase message\nError: {}\nMessage: {}", e, message);

        },
    }

    //NEED TO SEE IF i HAVE TO NORMALIZE THIS DATA FIRST
    let indices = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let new_values = [&coinbase_price, &coinbase_open_24h, &coinbase_volume_24h, &coinbase_low_24h, 
                &coinbase_high_24h, &coinbase_volume_30d, &coinbase_best_bid, &coinbase_best_bid_size, 
                &coinbase_best_ask, &coinbase_best_ask_size, &coinbase_side, &coinbase_last_size];
    neural_network.update_input(&indices, &new_values);



}

fn handle_kraken(message: &str) {
    if message.contains("heartbeat") {
        println!("Kraken: heartbeat message. ignoring...");
        return;
    }
    if message.trim().is_empty() {
        println!("Kraken: blank message received\nmessage: {}", message);
        return;
    }
    //put the entire message into a different var type that is easier to handle
    let data: Result<Value, serde_json::Error> = serde_json::from_str(message);

    //variable initialization
    //AFTER PROTOTYPE: CHANGE THESE TO OPTION<F64> for better error handling and error detection
    let mut a_price = 0.0;
    let mut a_whole_lot_volume: i64 = 0;
    let mut a_lot_volume = 0.0;

    let b_price = 0.0;
    let b_whole_lot_volume: i64 = 0;
    let b_lot_vlume = 0.0;

    let c_price = 0.0;
    let c_lot_volume = 0.0;

    let v_today = 0.0;
    let v_last24hours = 0.0;

    let p_today = 0.0;
    let p_last24hours = 0.0;

    let t_today: i64 = 0;
    let t_last24hours: i64 = 0;

    let l_today = 0.0;
    let l_last24hours = 0.0;

    let h_today = 0.0;
    let h_last24hours = 0.0;

    let o_today = 0.0;
    let o_last24hours = 0.0;

    //handle errors if any are present during parsing
    match data {
        
        
        Ok(value) => {
            //check if the entire message is an array (remember this is JSON format), it should be.
            if let Value::Array(array) = &value {
                
                //check if 2nd element of array is an object (this is JSON format), it should be
                if let Value::Object(map) = &array[1] {
                    //a values
                    if let Some(Value::Array(a_values)) = map.get("a") {
                        // Extract the values
                        //why as_str()      becasue the data is like this:
                        //              {"a":["0.394800",872,"872.19427794"]
                        //              this means that the 0.394800 is first a string. 
                        //              so I need to first use it as such and then change it
                        //               to an f64
                        //              this is why we have the .parse::<f64>().unwrap(); part
                        a_price = a_values[0].as_str().unwrap().parse::<f64>().unwrap();
                        //why as_i64()      because this pre-formatted from the server as an int 64
                        a_whole_lot_volume = a_values[1].as_i64().unwrap();
                        a_lot_volume = a_values[2].as_str().unwrap().parse::<f64>().unwrap();

                        println!("Kraken: a_values {:?}\na_price: {}\na_whole_lot_volume: {}
                                \na_lot_volume: {}", 
                                &a_values, &a_price, &a_whole_lot_volume, &a_lot_volume);
                    }
                    else {
                        //just in case
                        println!("****\t'a' VALUE did not exist\n 
                                    be careful with this input\n
                                    maybe you have to change how you are parsing the input
                                    from now on..");
                    }

                    //b values
                    if let Some(Value::Array(b_values)) = map.get("b") {
                        // Extract the values
                        b_price = b_values[0].as_str().unwrap().parse::<f64>().unwrap();
                        b_whole_lot_volume = b_values[1].as_i64().unwrap();
                        b_lot_volume = b_values[2].as_str().unwrap().parse::<f64>().unwrap();

                        println!("Kraken: b_values:{:?}\nb_price: {}\nb_whole_lot_volume: {}
                                \nb_lot_volume: {}", 
                        &b_values, &b_price, &b_whole_lot_volume, &b_lot_volume);
                    }
                    else {
                        println!("****\t'b' VALUE did not exist\n 
                                    be careful with this input\n
                                    maybe you have to change how you are parsing the input 
                                    from now on..");
                    }

                    //c values
                    if let Some(Value::Array(c_values)) = map.get("c") {
                        c_price = c_values[0].as_str().unwrap().parse::<f64>().unwrap();
                        c_lot_volume = c_values[1].as_str().unwrap().parse::<f64>().unwrap();

                        println!("Kraken: c_values: {:?}\nc_price: {}\nc_lot_volume: {}", &c_values, 
                        &c_price, &c_lot_volume);
                    }
                    else {
                        println!("****\t'c' VALUE did not exist\n 
                                    be careful with this input\n
                                    maybe you have to change how you are parsing the input 
                                    from now on..");
                    }

                    //v values
                    if let Some(Value::Array(v_values)) = map.get("v") {
                        v_today = v_values[0].as_str().unwrap().parse::<f64>().unwrap();
                        v_last24hours = v_values[1].as_str().unwrap().parse::<f64>().unwrap();

                        println!("Kraken:v_values: {:?}\nv_today: {}\nv_last24hours: {}", &v_values, 
                        &v_today, &v_last24hours);
                    }
                    else {
                        println!("****\t'v' VALUE did not exist\n 
                                    be careful with this input\n
                                    maybe you have to change how you are parsing the input 
                                    from now on..");
                    }

                    //p values
                    if let Some(Value::Array(p_values)) = map.get("p") {
                        p_today = p_values[0].as_str().unwrap().parse::<f64>().unwrap();
                        p_last24hours = p_values[1].as_str().unwrap().parse::<f64>().unwrap();

                        println!("Kraken:p_values: {:?}\np_today: {}\np_last24hours: {}", &p_values, 
                        &p_today, &p_last24hours);
                    }
                    else {
                        println!("****\t'v' VALUE did not exist\n 
                                    be careful with this input\n
                                    maybe you have to change how you are parsing the input 
                                    from now on..");
                    }
                    
                    //t values
                    if let Some(Value::Array(t_values)) = map.get("t") {
                        t_today = t_values[0].as_i64().unwrap();
                        t_last24hours = t_values[1].as_i64().unwrap();

                        println!("Kraken: t_values: {:?}\nt_today: {}\nt_last24hours: {}", &t_values, 
                        &t_today, &t_last24hours);
                    }
                    else {
                        println!("****\t't' VALUE did not exist\n 
                                    be careful with this input\n
                                    maybe you have to change how you are parsing the input 
                                    from now on..");
                    }

                    //l values
                    if let Some(Value::Array(l_values)) = map.get("l") {
                        l_today = l_values[0].as_str().unwrap().parse::<f64>().unwrap();
                        l_last24hours = l_values[1].as_str().unwrap().parse::<f64>().unwrap();

                        println!("Kraken: l_values: {:?}\nl_today: {}\nl_last24hours: {}", &l_values, 
                        &l_today, &l_last24hours);
                    }
                    else {
                        println!("****\t'l' VALUE did not exist\n 
                                    be careful with this input\n
                                    maybe you have to change how you are parsing the input 
                                    from now on..");
                    }

                    //h values
                    if let Some(Value::Array(h_values)) = map.get("h") {
                        h_today = h_values[0].as_str().unwrap().parse::<f64>().unwrap();
                        h_last24hours = h_values[1].as_str().unwrap().parse::<f64>().unwrap();
                        println!("Kraken: h_values: {:?}\nh_today: {}\nh_last24hours: {}", &h_values, 
                        &h_today, &h_last24hours);
                    }
                    else {
                        println!("****\t'h' VALUE did not exist\n 
                                    be careful with this input\n
                                    maybe you have to change how you are parsing the input 
                                    from now on..");
                    }

                    //o values
                    if let Some(Value::Array(o_values)) = map.get("o") {
                        o_today = o_values[0].as_str().unwrap().parse::<f64>().unwrap();
                        o_last24hours = o_values[1].as_str().unwrap().parse::<f64>().unwrap();

                        println!("Kraken: o_values: {:?}\no_today: {}\no_last24hours: {}", &o_values, 
                        &o_today, &o_last24hours);
                    }
                    else {
                        println!("****\t'o' VALUE did not exist\n 
                                    be careful with this input\n
                                    maybe you have to change how you are parsing the input 
                                    from now on..");
                    }

                }
            }
        },
        Err(e) => {
            println!("Failed to parse JSON Kraken message\nError: {}\nMessage: {}", e, message);
        },
    }

    let indices: [usize; 20] = [12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 
                                28, 29, 30, 31];
    let new_values = [&a_price, &a_whole_lot_volume, &a_lot_volume, &b_price, 
                                &b_whole_lot_volume, &b_lot_vlume, &c_price, &c_lot_volume, 
                                &v_today, &v_last24hours, &p_today, &p_last24hours, &t_today, 
                                &t_last24hours, &l_today, &l_last24hours, &h_today, &h_last24hours, 
                                &o_today, &o_last24hours];
    neural_network.update_input(&indices, &new_values);



}

fn handle_bitstamp(message: &str) {
    // Handle Bitstamp message
    if message.contains("subscription") {
        println!("subscription succeeded. unimportant message\nmessage: {}", message);
        return;
    }

    if message.trim().is_empty() {
        println!("Bitstamp: blank message received\nmessage: {}", message);
        return;
    }

    let v: Result<Value, serde_json::Error> = serde_json::from_str(message);

    let mut amount = 0.0;
    let mut price:i64 = 0;

    match v {
        Ok(value) => {
            if let Value::Object(map) = &value {
                // Check if the object has a key "data" whose value is an object
                if let Some(Value::Object(data)) = map.get("data") {
                    // Extract the values
                    amount = data.get("amount").and_then(Value::as_f64).unwrap();
                    price = data.get("price").and_then(Value::as_i64).unwrap();

                    println!("Bitstamp:\namount: {}\nprice: {}\n\n\n", &amount, &price);
        
                }
            }
        },
        Err(e) => {
            println!("Failed to parse JSON Bitstamp message\nError: {}\nMessage: {}", e, message);
        },
    }


    let indices: [usize; 2] = [32, 33];
    let new_values = [&amount, &price];
    neural_network.update_input(&indices, &new_values);

}

fn handle_gemini(message: &str) {
    if message.contains("heartbeat") {
        println!("Gemini heartbeat message. ignoring...");
        return;
    }
    if message.trim().is_empty() {
        println!("Gemini: blank message received\nmessage: {}", message);
        return;
    }
    let data: Result<Value, serde_json::Error> = serde_json::from_str(message);


    match data {
        Ok(value) => {
    // Check if the payload is an object
            if let Value::Object(map) = &value {
                // Check if the object has a key "events" whose value is an array
                if let Some(Value::Array(events)) = map.get("events") {
                    // Check if the first element of the array is an object
                    if let Some(Value::Object(event)) = events.get(0) {
                        // Extract the values
                        let amount = event.get("amount").and_then(Value::as_str).unwrap();
                        let maker_side = event.get("makerSide").and_then(Value::as_str).unwrap();
                        let price = event.get("price").and_then(Value::as_str).unwrap();
                        
                        println!("gemini:\namount: {}\nmaker_side: {}\nprice: {}\n\n\n", &amount, 
                                &maker_side, &price);
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to parse JSON Gemini message\nError: {}\nMessage: {}", e, message);
        },
    }
}
//-----ALL-FOR-PARSING-ABOVE-THIS//




















fn main() {
//-----ALL-FOR-PARSING-UNDER-THIS//
    env::set_var("RUST_BACKTRACE", "1");
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
                //  go to handle_coinbase(message)
                //else if "kraken received"...
                //and if it's none of them, print that it's unknown and panic
                match prefix {
                    "Coinbase Received" => handle_coinbase(message),
                    "Kraken Received" => handle_kraken(message),
                    "Bitstamp received" => handle_bitstamp(message),
                    "Gemini received" => handle_gemini(message),
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
}

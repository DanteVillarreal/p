use chrono::Utc;            //to get time
use hmac::{Hmac, Mac,};	                            //so I can do the signature stuff
use sha2::{Sha256, Sha384, Sha512, Digest};	        //so I can do signature stuff
use hex;
//use reqwest::Client;                                //to actually make the request itself
//use serde_json::Result;                             //for parsing
//use serde::{Deserialize, Serialize};                //for the deserialization/serialization that I need
use serde_json::Value;
use std::error::Error;
use url::form_urlencoded;
use base64;
use base64::encode;
use serde_json::json;                               //use for gemini formatting
use uuid::Uuid;										//this is for bitstamp. part of the input for the signature needs to have a weird nonce
/*
    pub fn nothing() {
        //runs the action according to the index
        //i need to make this wait until the next state begins and then give it a reward?


    }
    //all my functions from this point on are going to be in this format:
    //pub fn [coin]_[percentage, floored]_[exchange to buy]_[exchange to sell] () {}
    pub fn eos_two_coinbase_kraken () -> f64 {
        //this will go to the [coinbase] api, get how much [2%] of my total usd is in it, buy [eos] from there
        //transfer it to [kraken] wallet api, and immediately sell all of it
        //





//------------------I MIGHT PUT THIS IN ANOTHER FUNCTION-----------------------/////
        //----I WILL HAVE TO CHANGE RETURN TYPE OF ABOVE FUNCTION OF COURSE----//
        //then it will calculate how much I spent on the coins.
        //Calculate how much my wallet in the 2nd exchange went up by
        //then subtract 2nd number - 1st number
        //to return a f64 that I will then use in my reward function

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
        };

        //at this point in the code, I have now returned the f64 I waned
        //now I will use this f64 for the reward function

//-----------ABOVE----------------------------------------------------------///////

    }

    pub fn eos_two_coinbase_pionex () {

    }

    pub fn eos_two_kraken_coinbase () {

    }
    //rest of eos_two
    pub fn eos_three
    //rest of eos_three

    pub fn eos_four
    //rest of eos_four

    pub fn eos_five
    //rest of eos_five

    pub fn eos_six
    //rest of eos_six

    pub fn eos_seven
    //rest of eos_seven







*/
    //function format:
    //s = sandbox
    //i# = index followed by the number
    //then coin name
    //then percentage
    //then [exchange buy]
    //then [exchange sell]

    pub fn s_i0_do_nothing(value_prior: &f64) -> f64{
        *value_prior
    }

    pub async fn s_i1_sol_1_coinbase_kraken(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
        //look at m, then look at functions to figure out current price of sol at coinbase,
        //      then do .01 * coinbase_wallet - trading_fee = how much sol in usd Im sending. 
        //      then do coinbase_wallet = coinbase_wallet - (.01 * coinbase_wallet + trading_fee)
        //      then do price_of_sol / how much I'm buying   or oposite  to get how much sol im sending
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then figure out price at kraken
        //      then sell it there with the trading fee.
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then set kraken_wallet = new value of wallet.
        //      then add values of both wallets to bitstamp_wallet and gemini_wallet
        //      then compare to value_prior
        //      tehn do reward function?
        //I'll have the keys in main so it doesn't have to load everything everytime, it can just store it in RAM
        //------all stuff below this is to actually complete the request to get how much money it costs
        let now = Utc::now();
        let time_stamp = now.timestamp().to_string();
        let method = "GET";
        let request_path = "/api/v3/brokerage/best_bid_ask";
        let body = "";
        let message = format!("{}{}{}{}", &time_stamp, 
        &method, &request_path, &body);
        type HmacSha256 = Hmac<Sha256>;
        fn sign(message: &str, coinbase_secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                    .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
        }
        let coinbase_signature = sign(&message, &coinbase_secret);

        let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
        .header("CB-ACCESS-KEY", coinbase_api_key)
        .header("CB-ACCESS-SIGN", &coinbase_signature)
        .header("CB-ACCESS-TIMESTAMP", &time_stamp)
        .build()?;
        //manages the error I described above
        //let request = match request {
        //Ok(req) => req,
        //Err(e) => {
        //eprintln!("Failed to build request: \n{}", e);
        //return Err(e);
        //}
        //};

        let response = client.execute(request).await?;
        //let response = match response {
        //    Ok(resp) => resp,
        //    Err(e) => {
        //        eprintln!("Failed to execute request: \n{}", e);
        //        return Err(e);
        //    }
        //};


        let response_text = response.text().await?;

        //added 12/29/23
        //this is the parsing
        let v: Value = serde_json::from_str(&response_text)?;
        let mut coinbase_sell_price = 0.0;
        let mut coinbase_buy_price = 0.0;

        // Access the pricebooks array
        if let Some(pricebooks) = v["pricebooks"].as_array() {
            // Iterate over each pricebook
            for pricebook in pricebooks {
                // Access the product_id, bids, and asks
                let product_id = pricebook["product_id"].as_str().unwrap_or("");
                let bids = &pricebook["bids"][0];
                let asks = &pricebook["asks"][0];
        
                // Access the price and size of the bids and asks
                coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                let bid_size = bids["size"].as_str().unwrap_or("size not found");
                coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
        
                println!("Product ID: {}", product_id);
                //println!("Best bid: {} (size: {})", bid_price, bid_size);
                //println!("Best ask: {} (size: {})", ask_price, ask_size);
            }
        }

        //manages any errors from line above
        //let response_text = match response_text {
        //    Ok(t) => t,
        //    Err(e) => {
        //        eprintln!("Failed to read response text: \n{}", e);
        //        return;
        //    }
        //};

        //prints the actual response
        //println!("list accounts response\n{:?}", &response_text);























        //---KRAKEN--//

        //basically Kraken requires a value that is always increasing to be in each request.
        //I didnt use now.timestamp().to_string()  because just in case I have 2 
        //	requests in a second I dont want to be penalized.
        //if no "now" in scope when moving file, 
        //	the code is this:
        ////returns current time. MAY NEED TO USE LOCAL TIME
        //		let now = Utc::now();
        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            println!("Nonce:\n{}", nonce_str);
            println!("Encoded payload:\n{}", post_data);
            println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            println!("Ask price: {}", kraken_buy_price_ask);
            println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations
            let coinbase_taker_fee = 0.008;

            let total_spent = 0.01*(*coinbase_wallet);
            let fee_for_purchase = total_spent*coinbase_taker_fee;
            let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            //new state of coinbase wallet below
            *coinbase_wallet -= total_spent;
            let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations
            let kraken_taker_fee = 0.0026;
            
            let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
            let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
            let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
            *kraken_wallet += money_from_sell_after_fees;

            let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;


        //this will count as value after



            return Ok(value_after)

     }

     pub async fn s_i2_sol_2_coinbase_kraken(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
    
            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //---KRAKEN--//
    
            //basically Kraken requires a value that is always increasing to be in each request.
            //I didnt use now.timestamp().to_string()  because just in case I have 2 
            //	requests in a second I dont want to be penalized.
            //if no "now" in scope when moving file, 
            //	the code is this:
            ////returns current time. MAY NEED TO USE LOCAL TIME
            //		let now = Utc::now();
            let nonce = now.timestamp_millis().to_string();
            let data = vec![
                ("nonce", &nonce),
                // Add more parameters as needed
            ];
            //let post_data: String = form_urlencoded::Serializer::new(String::new())
            //    .extend_pairs(data)
            //    .finish();
            
            let url_path = "/0/public/Ticker?pair=SOLUSD";
            //let message = format!("{}{}{}", url_path, nonce, post_data);
    
            fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
            -> String {
                // Create the post data
                let post_data: String = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(data)
                    .finish();
                //FOR DEBUGGING
                //println!("Private key:\n{}", secret);
                println!("Nonce:\n{}", nonce_str);
                println!("Encoded payload:\n{}", post_data);
                println!("URI Path:\n{}", url_path);
            
                // Create the encoded string (nonce + post data) and hash it
                let encoded = format!("{}{}", nonce_str, post_data);
                let mut hasher = sha2::Sha256::new();
                hasher.update(encoded);
                let encoded_hash = hasher.finalize();
            
                // Create the message (url_path + encoded_hash as bytes)
                let mut message = url_path.as_bytes().to_vec();
                message.extend_from_slice(&encoded_hash);
            
                // Create a HMAC-SHA512 object with the base64-decoded secret
                let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
                let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                    .expect("HMAC can take key of any size");
            
                // Compute the HMAC of the message
                mac.update(&message);
                let result = mac.finalize();
            
                // Return the base64-encoded HMAC
                let signature = base64::encode(result.into_bytes());
                println!("Kraken signature:\n{}", signature);
            
                signature
            }
    
            let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);
    
            //kraken asked for 3 headers: key, sign, and content type with its corresponding info
            //.body is nonce because in the Kraken code provided in cURL: 
            //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
            //--data-urlencode "nonce=<YOUR-NONCE>"
            //		this means that nonce is added to the body of the request
            let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                    .header("API-Key", kraken_api_key)
                    .header("API-Sign", &kraken_signature)
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                    .body(format!("nonce={}", nonce))
                    .build()
                    .expect("Failed to build kraken request");
    
    
            let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");
    
            let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");
    
            let v: Value = serde_json::from_str(&kraken_response_text)?;
            let mut kraken_buy_price_ask = 0.0;
            let mut kraken_sell_price_bid = 0.0;
            if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
                // Access the ask and bid prices
                kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
                kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            
                println!("Ask price: {}", kraken_buy_price_ask);
                println!("Bid price: {}", kraken_sell_price_bid );
            }
            else {
                println!("didnt parse kraken correctly.");
            }
    
            //println!("response:\n{:?}", kraken_response_text);
            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.02*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
    
    
            //this will count as value after
    
    
    
                return Ok(value_after)
        
    }

    pub async fn s_i3_sol_3_coinbase_kraken(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //---KRAKEN--//
    
            //basically Kraken requires a value that is always increasing to be in each request.
            //I didnt use now.timestamp().to_string()  because just in case I have 2 
            //	requests in a second I dont want to be penalized.
            //if no "now" in scope when moving file, 
            //	the code is this:
            ////returns current time. MAY NEED TO USE LOCAL TIME
            //		let now = Utc::now();
            let nonce = now.timestamp_millis().to_string();
            let data = vec![
                ("nonce", &nonce),
                // Add more parameters as needed
            ];
            //let post_data: String = form_urlencoded::Serializer::new(String::new())
            //    .extend_pairs(data)
            //    .finish();
            
            let url_path = "/0/public/Ticker?pair=SOLUSD";
            //let message = format!("{}{}{}", url_path, nonce, post_data);
    
            fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
            -> String {
                // Create the post data
                let post_data: String = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(data)
                    .finish();
                //FOR DEBUGGING
                //println!("Private key:\n{}", secret);
                println!("Nonce:\n{}", nonce_str);
                println!("Encoded payload:\n{}", post_data);
                println!("URI Path:\n{}", url_path);
            
                // Create the encoded string (nonce + post data) and hash it
                let encoded = format!("{}{}", nonce_str, post_data);
                let mut hasher = sha2::Sha256::new();
                hasher.update(encoded);
                let encoded_hash = hasher.finalize();
            
                // Create the message (url_path + encoded_hash as bytes)
                let mut message = url_path.as_bytes().to_vec();
                message.extend_from_slice(&encoded_hash);
            
                // Create a HMAC-SHA512 object with the base64-decoded secret
                let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
                let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                    .expect("HMAC can take key of any size");
            
                // Compute the HMAC of the message
                mac.update(&message);
                let result = mac.finalize();
            
                // Return the base64-encoded HMAC
                let signature = base64::encode(result.into_bytes());
                println!("Kraken signature:\n{}", signature);
            
                signature
            }
    
            let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);
    
            //kraken asked for 3 headers: key, sign, and content type with its corresponding info
            //.body is nonce because in the Kraken code provided in cURL: 
            //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
            //--data-urlencode "nonce=<YOUR-NONCE>"
            //		this means that nonce is added to the body of the request
            let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                    .header("API-Key", kraken_api_key)
                    .header("API-Sign", &kraken_signature)
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                    .body(format!("nonce={}", nonce))
                    .build()
                    .expect("Failed to build kraken request");
    
    
            let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");
    
            let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");
    
            let v: Value = serde_json::from_str(&kraken_response_text)?;
            let mut kraken_buy_price_ask = 0.0;
            let mut kraken_sell_price_bid = 0.0;
            if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
                // Access the ask and bid prices
                kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
                kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            
                println!("Ask price: {}", kraken_buy_price_ask);
                println!("Bid price: {}", kraken_sell_price_bid );
            }
            else {
                println!("didnt parse kraken correctly.");
            }
    
            //println!("response:\n{:?}", kraken_response_text);
            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.03*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
    
    
            //this will count as value after
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i4_sol_4_coinbase_kraken(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //---KRAKEN--//
    
            //basically Kraken requires a value that is always increasing to be in each request.
            //I didnt use now.timestamp().to_string()  because just in case I have 2 
            //	requests in a second I dont want to be penalized.
            //if no "now" in scope when moving file, 
            //	the code is this:
            ////returns current time. MAY NEED TO USE LOCAL TIME
            //		let now = Utc::now();
            let nonce = now.timestamp_millis().to_string();
            let data = vec![
                ("nonce", &nonce),
                // Add more parameters as needed
            ];
            //let post_data: String = form_urlencoded::Serializer::new(String::new())
            //    .extend_pairs(data)
            //    .finish();
            
            let url_path = "/0/public/Ticker?pair=SOLUSD";
            //let message = format!("{}{}{}", url_path, nonce, post_data);
    
            fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
            -> String {
                // Create the post data
                let post_data: String = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(data)
                    .finish();
                //FOR DEBUGGING
                //println!("Private key:\n{}", secret);
                println!("Nonce:\n{}", nonce_str);
                println!("Encoded payload:\n{}", post_data);
                println!("URI Path:\n{}", url_path);
            
                // Create the encoded string (nonce + post data) and hash it
                let encoded = format!("{}{}", nonce_str, post_data);
                let mut hasher = sha2::Sha256::new();
                hasher.update(encoded);
                let encoded_hash = hasher.finalize();
            
                // Create the message (url_path + encoded_hash as bytes)
                let mut message = url_path.as_bytes().to_vec();
                message.extend_from_slice(&encoded_hash);
            
                // Create a HMAC-SHA512 object with the base64-decoded secret
                let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
                let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                    .expect("HMAC can take key of any size");
            
                // Compute the HMAC of the message
                mac.update(&message);
                let result = mac.finalize();
            
                // Return the base64-encoded HMAC
                let signature = base64::encode(result.into_bytes());
                println!("Kraken signature:\n{}", signature);
            
                signature
            }
    
            let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);
    
            //kraken asked for 3 headers: key, sign, and content type with its corresponding info
            //.body is nonce because in the Kraken code provided in cURL: 
            //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
            //--data-urlencode "nonce=<YOUR-NONCE>"
            //		this means that nonce is added to the body of the request
            let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                    .header("API-Key", kraken_api_key)
                    .header("API-Sign", &kraken_signature)
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                    .body(format!("nonce={}", nonce))
                    .build()
                    .expect("Failed to build kraken request");
    
    
            let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");
    
            let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");
    
            let v: Value = serde_json::from_str(&kraken_response_text)?;
            let mut kraken_buy_price_ask = 0.0;
            let mut kraken_sell_price_bid = 0.0;
            if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
                // Access the ask and bid prices
                kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
                kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            
                println!("Ask price: {}", kraken_buy_price_ask);
                println!("Bid price: {}", kraken_sell_price_bid );
            }
            else {
                println!("didnt parse kraken correctly.");
            }
    
            //println!("response:\n{:?}", kraken_response_text);
            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.04*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
    
    
            //this will count as value after
    
    
    
            return Ok(value_after)

    }

    pub async fn s_i5_sol_5_coinbase_kraken(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //---KRAKEN--//
    
            //basically Kraken requires a value that is always increasing to be in each request.
            //I didnt use now.timestamp().to_string()  because just in case I have 2 
            //	requests in a second I dont want to be penalized.
            //if no "now" in scope when moving file, 
            //	the code is this:
            ////returns current time. MAY NEED TO USE LOCAL TIME
            //		let now = Utc::now();
            let nonce = now.timestamp_millis().to_string();
            let data = vec![
                ("nonce", &nonce),
                // Add more parameters as needed
            ];
            //let post_data: String = form_urlencoded::Serializer::new(String::new())
            //    .extend_pairs(data)
            //    .finish();
            
            let url_path = "/0/public/Ticker?pair=SOLUSD";
            //let message = format!("{}{}{}", url_path, nonce, post_data);
    
            fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
            -> String {
                // Create the post data
                let post_data: String = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(data)
                    .finish();
                //FOR DEBUGGING
                //println!("Private key:\n{}", secret);
                println!("Nonce:\n{}", nonce_str);
                println!("Encoded payload:\n{}", post_data);
                println!("URI Path:\n{}", url_path);
            
                // Create the encoded string (nonce + post data) and hash it
                let encoded = format!("{}{}", nonce_str, post_data);
                let mut hasher = sha2::Sha256::new();
                hasher.update(encoded);
                let encoded_hash = hasher.finalize();
            
                // Create the message (url_path + encoded_hash as bytes)
                let mut message = url_path.as_bytes().to_vec();
                message.extend_from_slice(&encoded_hash);
            
                // Create a HMAC-SHA512 object with the base64-decoded secret
                let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
                let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                    .expect("HMAC can take key of any size");
            
                // Compute the HMAC of the message
                mac.update(&message);
                let result = mac.finalize();
            
                // Return the base64-encoded HMAC
                let signature = base64::encode(result.into_bytes());
                println!("Kraken signature:\n{}", signature);
            
                signature
            }
    
            let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);
    
            //kraken asked for 3 headers: key, sign, and content type with its corresponding info
            //.body is nonce because in the Kraken code provided in cURL: 
            //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
            //--data-urlencode "nonce=<YOUR-NONCE>"
            //		this means that nonce is added to the body of the request
            let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                    .header("API-Key", kraken_api_key)
                    .header("API-Sign", &kraken_signature)
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                    .body(format!("nonce={}", nonce))
                    .build()
                    .expect("Failed to build kraken request");
    
    
            let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");
    
            let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");
    
            let v: Value = serde_json::from_str(&kraken_response_text)?;
            let mut kraken_buy_price_ask = 0.0;
            let mut kraken_sell_price_bid = 0.0;
            if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
                // Access the ask and bid prices
                kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
                kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            
                println!("Ask price: {}", kraken_buy_price_ask);
                println!("Bid price: {}", kraken_sell_price_bid );
            }
            else {
                println!("didnt parse kraken correctly.");
            }
    
            //println!("response:\n{:?}", kraken_response_text);
            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.05*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
    
    
            //this will count as value after
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i6_sol_6_coinbase_kraken(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
    
            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //---KRAKEN--//
    
            //basically Kraken requires a value that is always increasing to be in each request.
            //I didnt use now.timestamp().to_string()  because just in case I have 2 
            //	requests in a second I dont want to be penalized.
            //if no "now" in scope when moving file, 
            //	the code is this:
            ////returns current time. MAY NEED TO USE LOCAL TIME
            //		let now = Utc::now();
            let nonce = now.timestamp_millis().to_string();
            let data = vec![
                ("nonce", &nonce),
                // Add more parameters as needed
            ];
            //let post_data: String = form_urlencoded::Serializer::new(String::new())
            //    .extend_pairs(data)
            //    .finish();
            
            let url_path = "/0/public/Ticker?pair=SOLUSD";
            //let message = format!("{}{}{}", url_path, nonce, post_data);
    
            fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
            -> String {
                // Create the post data
                let post_data: String = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(data)
                    .finish();
                //FOR DEBUGGING
                //println!("Private key:\n{}", secret);
                println!("Nonce:\n{}", nonce_str);
                println!("Encoded payload:\n{}", post_data);
                println!("URI Path:\n{}", url_path);
            
                // Create the encoded string (nonce + post data) and hash it
                let encoded = format!("{}{}", nonce_str, post_data);
                let mut hasher = sha2::Sha256::new();
                hasher.update(encoded);
                let encoded_hash = hasher.finalize();
            
                // Create the message (url_path + encoded_hash as bytes)
                let mut message = url_path.as_bytes().to_vec();
                message.extend_from_slice(&encoded_hash);
            
                // Create a HMAC-SHA512 object with the base64-decoded secret
                let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
                let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                    .expect("HMAC can take key of any size");
            
                // Compute the HMAC of the message
                mac.update(&message);
                let result = mac.finalize();
            
                // Return the base64-encoded HMAC
                let signature = base64::encode(result.into_bytes());
                println!("Kraken signature:\n{}", signature);
            
                signature
            }
    
            let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);
    
            //kraken asked for 3 headers: key, sign, and content type with its corresponding info
            //.body is nonce because in the Kraken code provided in cURL: 
            //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
            //--data-urlencode "nonce=<YOUR-NONCE>"
            //		this means that nonce is added to the body of the request
            let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                    .header("API-Key", kraken_api_key)
                    .header("API-Sign", &kraken_signature)
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                    .body(format!("nonce={}", nonce))
                    .build()
                    .expect("Failed to build kraken request");
    
    
            let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");
    
            let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");
    
            let v: Value = serde_json::from_str(&kraken_response_text)?;
            let mut kraken_buy_price_ask = 0.0;
            let mut kraken_sell_price_bid = 0.0;
            if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
                // Access the ask and bid prices
                kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
                kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            
                println!("Ask price: {}", kraken_buy_price_ask);
                println!("Bid price: {}", kraken_sell_price_bid );
            }
            else {
                println!("didnt parse kraken correctly.");
            }
    
            //println!("response:\n{:?}", kraken_response_text);
            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.06*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
    
    
            //this will count as value after
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i7_sol_7_coinbase_kraken(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //---KRAKEN--//
    
            //basically Kraken requires a value that is always increasing to be in each request.
            //I didnt use now.timestamp().to_string()  because just in case I have 2 
            //	requests in a second I dont want to be penalized.
            //if no "now" in scope when moving file, 
            //	the code is this:
            ////returns current time. MAY NEED TO USE LOCAL TIME
            //		let now = Utc::now();
            let nonce = now.timestamp_millis().to_string();
            let data = vec![
                ("nonce", &nonce),
                // Add more parameters as needed
            ];
            //let post_data: String = form_urlencoded::Serializer::new(String::new())
            //    .extend_pairs(data)
            //    .finish();
            
            let url_path = "/0/public/Ticker?pair=SOLUSD";
            //let message = format!("{}{}{}", url_path, nonce, post_data);
    
            fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
            -> String {
                // Create the post data
                let post_data: String = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(data)
                    .finish();
                //FOR DEBUGGING
                //println!("Private key:\n{}", secret);
                println!("Nonce:\n{}", nonce_str);
                println!("Encoded payload:\n{}", post_data);
                println!("URI Path:\n{}", url_path);
            
                // Create the encoded string (nonce + post data) and hash it
                let encoded = format!("{}{}", nonce_str, post_data);
                let mut hasher = sha2::Sha256::new();
                hasher.update(encoded);
                let encoded_hash = hasher.finalize();
            
                // Create the message (url_path + encoded_hash as bytes)
                let mut message = url_path.as_bytes().to_vec();
                message.extend_from_slice(&encoded_hash);
            
                // Create a HMAC-SHA512 object with the base64-decoded secret
                let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
                let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                    .expect("HMAC can take key of any size");
            
                // Compute the HMAC of the message
                mac.update(&message);
                let result = mac.finalize();
            
                // Return the base64-encoded HMAC
                let signature = base64::encode(result.into_bytes());
                println!("Kraken signature:\n{}", signature);
            
                signature
            }
    
            let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);
    
            //kraken asked for 3 headers: key, sign, and content type with its corresponding info
            //.body is nonce because in the Kraken code provided in cURL: 
            //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
            //--data-urlencode "nonce=<YOUR-NONCE>"
            //		this means that nonce is added to the body of the request
            let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                    .header("API-Key", kraken_api_key)
                    .header("API-Sign", &kraken_signature)
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                    .body(format!("nonce={}", nonce))
                    .build()
                    .expect("Failed to build kraken request");
    
    
            let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");
    
            let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");
    
            let v: Value = serde_json::from_str(&kraken_response_text)?;
            let mut kraken_buy_price_ask = 0.0;
            let mut kraken_sell_price_bid = 0.0;
            if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
                // Access the ask and bid prices
                kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
                kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            
                println!("Ask price: {}", kraken_buy_price_ask);
                println!("Bid price: {}", kraken_sell_price_bid );
            }
            else {
                println!("didnt parse kraken correctly.");
            }
    
            //println!("response:\n{:?}", kraken_response_text);
            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.07*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
    
    
            //this will count as value after
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i8_sol_8_coinbase_kraken(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
    
            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //---KRAKEN--//
    
            //basically Kraken requires a value that is always increasing to be in each request.
            //I didnt use now.timestamp().to_string()  because just in case I have 2 
            //	requests in a second I dont want to be penalized.
            //if no "now" in scope when moving file, 
            //	the code is this:
            ////returns current time. MAY NEED TO USE LOCAL TIME
            //		let now = Utc::now();
            let nonce = now.timestamp_millis().to_string();
            let data = vec![
                ("nonce", &nonce),
                // Add more parameters as needed
            ];
            //let post_data: String = form_urlencoded::Serializer::new(String::new())
            //    .extend_pairs(data)
            //    .finish();
            
            let url_path = "/0/public/Ticker?pair=SOLUSD";
            //let message = format!("{}{}{}", url_path, nonce, post_data);
    
            fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
            -> String {
                // Create the post data
                let post_data: String = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(data)
                    .finish();
                //FOR DEBUGGING
                //println!("Private key:\n{}", secret);
                println!("Nonce:\n{}", nonce_str);
                println!("Encoded payload:\n{}", post_data);
                println!("URI Path:\n{}", url_path);
            
                // Create the encoded string (nonce + post data) and hash it
                let encoded = format!("{}{}", nonce_str, post_data);
                let mut hasher = sha2::Sha256::new();
                hasher.update(encoded);
                let encoded_hash = hasher.finalize();
            
                // Create the message (url_path + encoded_hash as bytes)
                let mut message = url_path.as_bytes().to_vec();
                message.extend_from_slice(&encoded_hash);
            
                // Create a HMAC-SHA512 object with the base64-decoded secret
                let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
                let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                    .expect("HMAC can take key of any size");
            
                // Compute the HMAC of the message
                mac.update(&message);
                let result = mac.finalize();
            
                // Return the base64-encoded HMAC
                let signature = base64::encode(result.into_bytes());
                println!("Kraken signature:\n{}", signature);
            
                signature
            }
    
            let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);
    
            //kraken asked for 3 headers: key, sign, and content type with its corresponding info
            //.body is nonce because in the Kraken code provided in cURL: 
            //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
            //--data-urlencode "nonce=<YOUR-NONCE>"
            //		this means that nonce is added to the body of the request
            let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                    .header("API-Key", kraken_api_key)
                    .header("API-Sign", &kraken_signature)
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                    .body(format!("nonce={}", nonce))
                    .build()
                    .expect("Failed to build kraken request");
    
    
            let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");
    
            let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");
    
            let v: Value = serde_json::from_str(&kraken_response_text)?;
            let mut kraken_buy_price_ask = 0.0;
            let mut kraken_sell_price_bid = 0.0;
            if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
                // Access the ask and bid prices
                kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
                kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            
                println!("Ask price: {}", kraken_buy_price_ask);
                println!("Bid price: {}", kraken_sell_price_bid );
            }
            else {
                println!("didnt parse kraken correctly.");
            }
    
            //println!("response:\n{:?}", kraken_response_text);
            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.08*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
    
    
            //this will count as value after
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i9_sol_9_coinbase_kraken(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
    
            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //---KRAKEN--//
    
            //basically Kraken requires a value that is always increasing to be in each request.
            //I didnt use now.timestamp().to_string()  because just in case I have 2 
            //	requests in a second I dont want to be penalized.
            //if no "now" in scope when moving file, 
            //	the code is this:
            ////returns current time. MAY NEED TO USE LOCAL TIME
            //		let now = Utc::now();
            let nonce = now.timestamp_millis().to_string();
            let data = vec![
                ("nonce", &nonce),
                // Add more parameters as needed
            ];
            //let post_data: String = form_urlencoded::Serializer::new(String::new())
            //    .extend_pairs(data)
            //    .finish();
            
            let url_path = "/0/public/Ticker?pair=SOLUSD";
            //let message = format!("{}{}{}", url_path, nonce, post_data);
    
            fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
            -> String {
                // Create the post data
                let post_data: String = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(data)
                    .finish();
                //FOR DEBUGGING
                //println!("Private key:\n{}", secret);
                println!("Nonce:\n{}", nonce_str);
                println!("Encoded payload:\n{}", post_data);
                println!("URI Path:\n{}", url_path);
            
                // Create the encoded string (nonce + post data) and hash it
                let encoded = format!("{}{}", nonce_str, post_data);
                let mut hasher = sha2::Sha256::new();
                hasher.update(encoded);
                let encoded_hash = hasher.finalize();
            
                // Create the message (url_path + encoded_hash as bytes)
                let mut message = url_path.as_bytes().to_vec();
                message.extend_from_slice(&encoded_hash);
            
                // Create a HMAC-SHA512 object with the base64-decoded secret
                let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
                let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                    .expect("HMAC can take key of any size");
            
                // Compute the HMAC of the message
                mac.update(&message);
                let result = mac.finalize();
            
                // Return the base64-encoded HMAC
                let signature = base64::encode(result.into_bytes());
                println!("Kraken signature:\n{}", signature);
            
                signature
            }
    
            let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);
    
            //kraken asked for 3 headers: key, sign, and content type with its corresponding info
            //.body is nonce because in the Kraken code provided in cURL: 
            //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
            //--data-urlencode "nonce=<YOUR-NONCE>"
            //		this means that nonce is added to the body of the request
            let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                    .header("API-Key", kraken_api_key)
                    .header("API-Sign", &kraken_signature)
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                    .body(format!("nonce={}", nonce))
                    .build()
                    .expect("Failed to build kraken request");
    
    
            let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");
    
            let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");
    
            let v: Value = serde_json::from_str(&kraken_response_text)?;
            let mut kraken_buy_price_ask = 0.0;
            let mut kraken_sell_price_bid = 0.0;
            if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
                // Access the ask and bid prices
                kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
                kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            
                println!("Ask price: {}", kraken_buy_price_ask);
                println!("Bid price: {}", kraken_sell_price_bid );
            }
            else {
                println!("didnt parse kraken correctly.");
            }
    
            //println!("response:\n{:?}", kraken_response_text);
            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.09*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
    
    
            //this will count as value after
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i10_sol_10_coinbase_kraken(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
    
            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //---KRAKEN--//
    
            //basically Kraken requires a value that is always increasing to be in each request.
            //I didnt use now.timestamp().to_string()  because just in case I have 2 
            //	requests in a second I dont want to be penalized.
            //if no "now" in scope when moving file, 
            //	the code is this:
            ////returns current time. MAY NEED TO USE LOCAL TIME
            //		let now = Utc::now();
            let nonce = now.timestamp_millis().to_string();
            let data = vec![
                ("nonce", &nonce),
                // Add more parameters as needed
            ];
            //let post_data: String = form_urlencoded::Serializer::new(String::new())
            //    .extend_pairs(data)
            //    .finish();
            
            let url_path = "/0/public/Ticker?pair=SOLUSD";
            //let message = format!("{}{}{}", url_path, nonce, post_data);
    
            fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
            -> String {
                // Create the post data
                let post_data: String = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(data)
                    .finish();
                //FOR DEBUGGING
                //println!("Private key:\n{}", secret);
                println!("Nonce:\n{}", nonce_str);
                println!("Encoded payload:\n{}", post_data);
                println!("URI Path:\n{}", url_path);
            
                // Create the encoded string (nonce + post data) and hash it
                let encoded = format!("{}{}", nonce_str, post_data);
                let mut hasher = sha2::Sha256::new();
                hasher.update(encoded);
                let encoded_hash = hasher.finalize();
            
                // Create the message (url_path + encoded_hash as bytes)
                let mut message = url_path.as_bytes().to_vec();
                message.extend_from_slice(&encoded_hash);
            
                // Create a HMAC-SHA512 object with the base64-decoded secret
                let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
                let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                    .expect("HMAC can take key of any size");
            
                // Compute the HMAC of the message
                mac.update(&message);
                let result = mac.finalize();
            
                // Return the base64-encoded HMAC
                let signature = base64::encode(result.into_bytes());
                println!("Kraken signature:\n{}", signature);
            
                signature
            }
    
            let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);
    
            //kraken asked for 3 headers: key, sign, and content type with its corresponding info
            //.body is nonce because in the Kraken code provided in cURL: 
            //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
            //--data-urlencode "nonce=<YOUR-NONCE>"
            //		this means that nonce is added to the body of the request
            let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                    .header("API-Key", kraken_api_key)
                    .header("API-Sign", &kraken_signature)
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                    .body(format!("nonce={}", nonce))
                    .build()
                    .expect("Failed to build kraken request");
    
    
            let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");
    
            let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");
    
            let v: Value = serde_json::from_str(&kraken_response_text)?;
            let mut kraken_buy_price_ask = 0.0;
            let mut kraken_sell_price_bid = 0.0;
            if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
                // Access the ask and bid prices
                kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
                kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            
                println!("Ask price: {}", kraken_buy_price_ask);
                println!("Bid price: {}", kraken_sell_price_bid );
            }
            else {
                println!("didnt parse kraken correctly.");
            }
    
            //println!("response:\n{:?}", kraken_response_text);
            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.10*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
    
    
            //this will count as value after
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i11_sol_1_coinbase_bitstamp(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, bitstamp_secret: &str, bitstamp_api_key: &str )-> Result<(f64), Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //-------------------------Bitstamp---------------------------//
                //Bitstamp for some reason needs nonce to be 36 characters long. 
	//	nonce is basically a unique id that needs to be different every time you make a request. 
	//	usually time-since-UNIX epoch will do but for some reason bitstmap requires both a timestamp
	//	and a nonce. Because of the nonce needing to be 36 chars, it's easier to use a uuid crate 
	//	and just make it a random 36 char string from it.

	
	//the exact same as the Coinbase signature. we'll see if it works
	//apparently it doesnt so I will code comment it out for now
	//fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
	//	let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
	//											.expect("HMAC can take key of any size");
	//	mac.update(bitstamp_message.as_bytes());
	//	let result = mac.finalize();
	//	let code_bytes = result.into_bytes();
	//	hex::encode(code_bytes)
	//}

	fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------Account-balances------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	println!("Bitstamp:\n{:?}", bitstamp_response_text);
            
    


            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.01*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i12_sol_2_coinbase_bitstamp(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, bitstamp_secret: &str, bitstamp_api_key: &str )-> Result<(f64), Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //-------------------------Bitstamp---------------------------//
                //Bitstamp for some reason needs nonce to be 36 characters long. 
	//	nonce is basically a unique id that needs to be different every time you make a request. 
	//	usually time-since-UNIX epoch will do but for some reason bitstmap requires both a timestamp
	//	and a nonce. Because of the nonce needing to be 36 chars, it's easier to use a uuid crate 
	//	and just make it a random 36 char string from it.

	
	//the exact same as the Coinbase signature. we'll see if it works
	//apparently it doesnt so I will code comment it out for now
	//fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
	//	let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
	//											.expect("HMAC can take key of any size");
	//	mac.update(bitstamp_message.as_bytes());
	//	let result = mac.finalize();
	//	let code_bytes = result.into_bytes();
	//	hex::encode(code_bytes)
	//}

	fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------Account-balances------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	println!("Bitstamp:\n{:?}", bitstamp_response_text);
            
    


            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.02*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i13_sol_3_coinbase_bitstamp(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, bitstamp_secret: &str, bitstamp_api_key: &str )-> Result<(f64), Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //-------------------------Bitstamp---------------------------//
                //Bitstamp for some reason needs nonce to be 36 characters long. 
	//	nonce is basically a unique id that needs to be different every time you make a request. 
	//	usually time-since-UNIX epoch will do but for some reason bitstmap requires both a timestamp
	//	and a nonce. Because of the nonce needing to be 36 chars, it's easier to use a uuid crate 
	//	and just make it a random 36 char string from it.

	
	//the exact same as the Coinbase signature. we'll see if it works
	//apparently it doesnt so I will code comment it out for now
	//fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
	//	let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
	//											.expect("HMAC can take key of any size");
	//	mac.update(bitstamp_message.as_bytes());
	//	let result = mac.finalize();
	//	let code_bytes = result.into_bytes();
	//	hex::encode(code_bytes)
	//}

	fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------Account-balances------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	println!("Bitstamp:\n{:?}", bitstamp_response_text);
            
    


            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.03*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i14_sol_4_coinbase_bitstamp(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, bitstamp_secret: &str, bitstamp_api_key: &str )-> Result<(f64), Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //-------------------------Bitstamp---------------------------//
                //Bitstamp for some reason needs nonce to be 36 characters long. 
	//	nonce is basically a unique id that needs to be different every time you make a request. 
	//	usually time-since-UNIX epoch will do but for some reason bitstmap requires both a timestamp
	//	and a nonce. Because of the nonce needing to be 36 chars, it's easier to use a uuid crate 
	//	and just make it a random 36 char string from it.

	
	//the exact same as the Coinbase signature. we'll see if it works
	//apparently it doesnt so I will code comment it out for now
	//fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
	//	let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
	//											.expect("HMAC can take key of any size");
	//	mac.update(bitstamp_message.as_bytes());
	//	let result = mac.finalize();
	//	let code_bytes = result.into_bytes();
	//	hex::encode(code_bytes)
	//}

	fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------Account-balances------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	println!("Bitstamp:\n{:?}", bitstamp_response_text);
            
    


            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.04*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i15_sol_5_coinbase_bitstamp(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, bitstamp_secret: &str, bitstamp_api_key: &str )-> Result<(f64), Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //-------------------------Bitstamp---------------------------//
                //Bitstamp for some reason needs nonce to be 36 characters long. 
	//	nonce is basically a unique id that needs to be different every time you make a request. 
	//	usually time-since-UNIX epoch will do but for some reason bitstmap requires both a timestamp
	//	and a nonce. Because of the nonce needing to be 36 chars, it's easier to use a uuid crate 
	//	and just make it a random 36 char string from it.

	
	//the exact same as the Coinbase signature. we'll see if it works
	//apparently it doesnt so I will code comment it out for now
	//fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
	//	let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
	//											.expect("HMAC can take key of any size");
	//	mac.update(bitstamp_message.as_bytes());
	//	let result = mac.finalize();
	//	let code_bytes = result.into_bytes();
	//	hex::encode(code_bytes)
	//}

	fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------Account-balances------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	println!("Bitstamp:\n{:?}", bitstamp_response_text);
            
    


            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.05*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i16_sol_6_coinbase_bitstamp(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, bitstamp_secret: &str, bitstamp_api_key: &str )-> Result<(f64), Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //-------------------------Bitstamp---------------------------//
                //Bitstamp for some reason needs nonce to be 36 characters long. 
	//	nonce is basically a unique id that needs to be different every time you make a request. 
	//	usually time-since-UNIX epoch will do but for some reason bitstmap requires both a timestamp
	//	and a nonce. Because of the nonce needing to be 36 chars, it's easier to use a uuid crate 
	//	and just make it a random 36 char string from it.

	
	//the exact same as the Coinbase signature. we'll see if it works
	//apparently it doesnt so I will code comment it out for now
	//fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
	//	let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
	//											.expect("HMAC can take key of any size");
	//	mac.update(bitstamp_message.as_bytes());
	//	let result = mac.finalize();
	//	let code_bytes = result.into_bytes();
	//	hex::encode(code_bytes)
	//}

	fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------Account-balances------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	println!("Bitstamp:\n{:?}", bitstamp_response_text);
            
    


            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.06*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i17_sol_7_coinbase_bitstamp(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, bitstamp_secret: &str, bitstamp_api_key: &str )-> Result<(f64), Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //-------------------------Bitstamp---------------------------//
                //Bitstamp for some reason needs nonce to be 36 characters long. 
	//	nonce is basically a unique id that needs to be different every time you make a request. 
	//	usually time-since-UNIX epoch will do but for some reason bitstmap requires both a timestamp
	//	and a nonce. Because of the nonce needing to be 36 chars, it's easier to use a uuid crate 
	//	and just make it a random 36 char string from it.

	
	//the exact same as the Coinbase signature. we'll see if it works
	//apparently it doesnt so I will code comment it out for now
	//fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
	//	let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
	//											.expect("HMAC can take key of any size");
	//	mac.update(bitstamp_message.as_bytes());
	//	let result = mac.finalize();
	//	let code_bytes = result.into_bytes();
	//	hex::encode(code_bytes)
	//}

	fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------Account-balances------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	println!("Bitstamp:\n{:?}", bitstamp_response_text);
            
    


            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.07*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i18_sol_8_coinbase_bitstamp(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, bitstamp_secret: &str, bitstamp_api_key: &str )-> Result<(f64), Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //-------------------------Bitstamp---------------------------//
                //Bitstamp for some reason needs nonce to be 36 characters long. 
	//	nonce is basically a unique id that needs to be different every time you make a request. 
	//	usually time-since-UNIX epoch will do but for some reason bitstmap requires both a timestamp
	//	and a nonce. Because of the nonce needing to be 36 chars, it's easier to use a uuid crate 
	//	and just make it a random 36 char string from it.

	
	//the exact same as the Coinbase signature. we'll see if it works
	//apparently it doesnt so I will code comment it out for now
	//fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
	//	let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
	//											.expect("HMAC can take key of any size");
	//	mac.update(bitstamp_message.as_bytes());
	//	let result = mac.finalize();
	//	let code_bytes = result.into_bytes();
	//	hex::encode(code_bytes)
	//}

	fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------Account-balances------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	println!("Bitstamp:\n{:?}", bitstamp_response_text);
            
    


            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.08*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i19_sol_9_coinbase_bitstamp(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, bitstamp_secret: &str, bitstamp_api_key: &str )-> Result<(f64), Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //-------------------------Bitstamp---------------------------//
                //Bitstamp for some reason needs nonce to be 36 characters long. 
	//	nonce is basically a unique id that needs to be different every time you make a request. 
	//	usually time-since-UNIX epoch will do but for some reason bitstmap requires both a timestamp
	//	and a nonce. Because of the nonce needing to be 36 chars, it's easier to use a uuid crate 
	//	and just make it a random 36 char string from it.

	
	//the exact same as the Coinbase signature. we'll see if it works
	//apparently it doesnt so I will code comment it out for now
	//fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
	//	let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
	//											.expect("HMAC can take key of any size");
	//	mac.update(bitstamp_message.as_bytes());
	//	let result = mac.finalize();
	//	let code_bytes = result.into_bytes();
	//	hex::encode(code_bytes)
	//}

	fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------Account-balances------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	println!("Bitstamp:\n{:?}", bitstamp_response_text);
            
    


            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.09*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i20_sol_10_coinbase_bitstamp(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, bitstamp_secret: &str, bitstamp_api_key: &str )-> Result<(f64), Box<dyn Error>> {

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //-------------------------Bitstamp---------------------------//
                //Bitstamp for some reason needs nonce to be 36 characters long. 
	//	nonce is basically a unique id that needs to be different every time you make a request. 
	//	usually time-since-UNIX epoch will do but for some reason bitstmap requires both a timestamp
	//	and a nonce. Because of the nonce needing to be 36 chars, it's easier to use a uuid crate 
	//	and just make it a random 36 char string from it.

	
	//the exact same as the Coinbase signature. we'll see if it works
	//apparently it doesnt so I will code comment it out for now
	//fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
	//	let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
	//											.expect("HMAC can take key of any size");
	//	mac.update(bitstamp_message.as_bytes());
	//	let result = mac.finalize();
	//	let code_bytes = result.into_bytes();
	//	hex::encode(code_bytes)
	//}

	fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------Account-balances------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	println!("Bitstamp:\n{:?}", bitstamp_response_text);
            
    


            //coinbase calculations
                let coinbase_taker_fee = 0.008;
    
                let total_spent = 0.10*(*coinbase_wallet);
                let fee_for_purchase = total_spent*coinbase_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of coinbase wallet below
                *coinbase_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i21_sol_1_gemini_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Coinbase---------------------------------------------//

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.01;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                let coinbase_taker_fee = 0.008;
                let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + *gemini_wallet + bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }    

    pub async fn s_i22_sol_2_gemini_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Coinbase---------------------------------------------//

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.02;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                let coinbase_taker_fee = 0.008;
                let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + *gemini_wallet + bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i23_sol_3_gemini_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Coinbase---------------------------------------------//

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.03;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                let coinbase_taker_fee = 0.008;
                let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + *gemini_wallet + bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i24_sol_4_gemini_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Coinbase---------------------------------------------//

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.04;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                let coinbase_taker_fee = 0.008;
                let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + *gemini_wallet + bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i25_sol_5_gemini_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Coinbase---------------------------------------------//

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.05;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                let coinbase_taker_fee = 0.008;
                let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + *gemini_wallet + bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i26_sol_6_gemini_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Coinbase---------------------------------------------//

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.06;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                let coinbase_taker_fee = 0.008;
                let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + *gemini_wallet + bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i27_sol_7_gemini_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Coinbase---------------------------------------------//

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.07;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                let coinbase_taker_fee = 0.008;
                let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + *gemini_wallet + bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i28_sol_8_gemini_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Coinbase---------------------------------------------//

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.08;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                let coinbase_taker_fee = 0.008;
                let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + *gemini_wallet + bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i29_sol_9_gemini_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Coinbase---------------------------------------------//

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.09;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                let coinbase_taker_fee = 0.008;
                let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + *gemini_wallet + bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i30_sol_10_gemini_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Coinbase---------------------------------------------//

            let now = Utc::now();
            let time_stamp = now.timestamp().to_string();
            let method = "GET";
            let request_path = "/api/v3/brokerage/best_bid_ask";
            let body = "";
            let message = format!("{}{}{}{}", &time_stamp, 
            &method, &request_path, &body);
            type HmacSha256 = Hmac<Sha256>;
            fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                        .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
            }
            let coinbase_signature = sign(&message, &coinbase_secret);
    
            let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
            .header("CB-ACCESS-KEY", coinbase_api_key)
            .header("CB-ACCESS-SIGN", &coinbase_signature)
            .header("CB-ACCESS-TIMESTAMP", &time_stamp)
            .build()?;
            //manages the error I described above
            //let request = match request {
            //Ok(req) => req,
            //Err(e) => {
            //eprintln!("Failed to build request: \n{}", e);
            //return Err(e);
            //}
            //};
    
            let response = client.execute(request).await?;
            //let response = match response {
            //    Ok(resp) => resp,
            //    Err(e) => {
            //        eprintln!("Failed to execute request: \n{}", e);
            //        return Err(e);
            //    }
            //};
    
    
            let response_text = response.text().await?;
    
            //added 12/29/23
            //this is the parsing
            let v: Value = serde_json::from_str(&response_text)?;
            let mut coinbase_sell_price = 0.0;
            let mut coinbase_buy_price = 0.0;
    
            // Access the pricebooks array
            if let Some(pricebooks) = v["pricebooks"].as_array() {
                // Iterate over each pricebook
                for pricebook in pricebooks {
                    // Access the product_id, bids, and asks
                    let product_id = pricebook["product_id"].as_str().unwrap_or("");
                    let bids = &pricebook["bids"][0];
                    let asks = &pricebook["asks"][0];
            
                    // Access the price and size of the bids and asks
                    coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                    let bid_size = bids["size"].as_str().unwrap_or("size not found");
                    coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                    let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
            
                    println!("Product ID: {}", product_id);
                    //println!("Best bid: {} (size: {})", bid_price, bid_size);
                    //println!("Best ask: {} (size: {})", ask_price, ask_size);
                }
            }
    
            //manages any errors from line above
            //let response_text = match response_text {
            //    Ok(t) => t,
            //    Err(e) => {
            //        eprintln!("Failed to read response text: \n{}", e);
            //        return;
            //    }
            //};
    
            //prints the actual response
            //println!("list accounts response\n{:?}", &response_text);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.10;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                let coinbase_taker_fee = 0.008;
                let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + *coinbase_wallet + *gemini_wallet + bitstamp_wallet;
    
    
    
                return Ok(value_after)

    }

    pub async fn s_i31_sol_1_gemini_kraken(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, kraken_secret: &str, kraken_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Kraken---------------------------------------------//

        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            println!("Nonce:\n{}", nonce_str);
            println!("Encoded payload:\n{}", post_data);
            println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            println!("Ask price: {}", kraken_buy_price_ask);
            println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            ////new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations
            //let kraken_taker_fee = 0.0026;
            
            //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
            //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
            //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
            //*kraken_wallet += money_from_sell_after_fees;

            //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;

    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.01;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = *kraken_wallet + coinbase_wallet + *gemini_wallet + bitstamp_wallet;
                //println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i32_sol_2_gemini_kraken(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, kraken_secret: &str, kraken_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Kraken---------------------------------------------//

        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            println!("Nonce:\n{}", nonce_str);
            println!("Encoded payload:\n{}", post_data);
            println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            println!("Ask price: {}", kraken_buy_price_ask);
            println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            ////new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations
            //let kraken_taker_fee = 0.0026;
            
            //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
            //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
            //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
            //*kraken_wallet += money_from_sell_after_fees;

            //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;

    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.02;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = *kraken_wallet + coinbase_wallet + *gemini_wallet + bitstamp_wallet;
                //println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i33_sol_3_gemini_kraken(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, kraken_secret: &str, kraken_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Kraken---------------------------------------------//

        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            println!("Nonce:\n{}", nonce_str);
            println!("Encoded payload:\n{}", post_data);
            println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            println!("Ask price: {}", kraken_buy_price_ask);
            println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            ////new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations
            //let kraken_taker_fee = 0.0026;
            
            //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
            //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
            //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
            //*kraken_wallet += money_from_sell_after_fees;

            //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;

    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.03;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = *kraken_wallet + coinbase_wallet + *gemini_wallet + bitstamp_wallet;
                //println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i34_sol_4_gemini_kraken(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, kraken_secret: &str, kraken_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Kraken---------------------------------------------//

        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            println!("Nonce:\n{}", nonce_str);
            println!("Encoded payload:\n{}", post_data);
            println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            println!("Ask price: {}", kraken_buy_price_ask);
            println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            ////new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations
            //let kraken_taker_fee = 0.0026;
            
            //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
            //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
            //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
            //*kraken_wallet += money_from_sell_after_fees;

            //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;

    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.04;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = *kraken_wallet + coinbase_wallet + *gemini_wallet + bitstamp_wallet;
                //println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i35_sol_5_gemini_kraken(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, kraken_secret: &str, kraken_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Kraken---------------------------------------------//

        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            println!("Nonce:\n{}", nonce_str);
            println!("Encoded payload:\n{}", post_data);
            println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            println!("Ask price: {}", kraken_buy_price_ask);
            println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            ////new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations
            //let kraken_taker_fee = 0.0026;
            
            //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
            //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
            //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
            //*kraken_wallet += money_from_sell_after_fees;

            //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;

    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.05;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = *kraken_wallet + coinbase_wallet + *gemini_wallet + bitstamp_wallet;
                //println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i36_sol_6_gemini_kraken(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, kraken_secret: &str, kraken_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Kraken---------------------------------------------//

        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            println!("Nonce:\n{}", nonce_str);
            println!("Encoded payload:\n{}", post_data);
            println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            println!("Ask price: {}", kraken_buy_price_ask);
            println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            ////new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations
            //let kraken_taker_fee = 0.0026;
            
            //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
            //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
            //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
            //*kraken_wallet += money_from_sell_after_fees;

            //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;

    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.06;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = *kraken_wallet + coinbase_wallet + *gemini_wallet + bitstamp_wallet;
                //println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i37_sol_7_gemini_kraken(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, kraken_secret: &str, kraken_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Kraken---------------------------------------------//

        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            println!("Nonce:\n{}", nonce_str);
            println!("Encoded payload:\n{}", post_data);
            println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            println!("Ask price: {}", kraken_buy_price_ask);
            println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            ////new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations
            //let kraken_taker_fee = 0.0026;
            
            //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
            //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
            //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
            //*kraken_wallet += money_from_sell_after_fees;

            //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;

    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.07;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = *kraken_wallet + coinbase_wallet + *gemini_wallet + bitstamp_wallet;
                //println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i38_sol_8_gemini_kraken(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, kraken_secret: &str, kraken_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Kraken---------------------------------------------//

        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            println!("Nonce:\n{}", nonce_str);
            println!("Encoded payload:\n{}", post_data);
            println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            println!("Ask price: {}", kraken_buy_price_ask);
            println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            ////new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations
            //let kraken_taker_fee = 0.0026;
            
            //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
            //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
            //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
            //*kraken_wallet += money_from_sell_after_fees;

            //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;

    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.08;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = *kraken_wallet + coinbase_wallet + *gemini_wallet + bitstamp_wallet;
                //println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i39_sol_9_gemini_kraken(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, kraken_secret: &str, kraken_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Kraken---------------------------------------------//

        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            println!("Nonce:\n{}", nonce_str);
            println!("Encoded payload:\n{}", post_data);
            println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            println!("Ask price: {}", kraken_buy_price_ask);
            println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            ////new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations
            //let kraken_taker_fee = 0.0026;
            
            //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
            //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
            //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
            //*kraken_wallet += money_from_sell_after_fees;

            //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;

    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.09;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = *kraken_wallet + coinbase_wallet + *gemini_wallet + bitstamp_wallet;
                //println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i40_sol_10_gemini_kraken(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &mut f64, kraken_secret: &str, kraken_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);









        //---------------------------Kraken---------------------------------------------//

        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            println!("Nonce:\n{}", nonce_str);
            println!("Encoded payload:\n{}", post_data);
            println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            println!("Ask price: {}", kraken_buy_price_ask);
            println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            ////new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations
            //let kraken_taker_fee = 0.0026;
            
            //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
            //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
            //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
            //*kraken_wallet += money_from_sell_after_fees;

            //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;

    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.10;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                let kraken_taker_fee = 0.0026;
                
                let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                *kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = *kraken_wallet + coinbase_wallet + *gemini_wallet + bitstamp_wallet;
                //println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i41_sol_1_gemini_bitstamp(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &mut f64, bitstamp_secret: &str, bitstamp_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);














    //------BITSTAMP------//
    type HmacSha256 = Hmac<Sha256>;
    fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------FOR ACTUAL REQUEST MESSAGE------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    //println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	//println!("Bitstamp:\n{:?}", bitstamp_response_text);





    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.01;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    


            //bitstamp calculations for sell
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + coinbase_wallet + *gemini_wallet + *bitstamp_wallet;
                println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i42_sol_2_gemini_bitstamp(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &mut f64, bitstamp_secret: &str, bitstamp_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);














    //------BITSTAMP------//
    type HmacSha256 = Hmac<Sha256>;
    fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------FOR ACTUAL REQUEST MESSAGE------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    //println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	//println!("Bitstamp:\n{:?}", bitstamp_response_text);





    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.02;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    


            //bitstamp calculations for sell
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + coinbase_wallet + *gemini_wallet + *bitstamp_wallet;
                println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i43_sol_3_gemini_bitstamp(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &mut f64, bitstamp_secret: &str, bitstamp_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);














    //------BITSTAMP------//
    type HmacSha256 = Hmac<Sha256>;
    fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------FOR ACTUAL REQUEST MESSAGE------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    //println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	//println!("Bitstamp:\n{:?}", bitstamp_response_text);





    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.03;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    


            //bitstamp calculations for sell
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + coinbase_wallet + *gemini_wallet + *bitstamp_wallet;
                println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i44_sol_4_gemini_bitstamp(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &mut f64, bitstamp_secret: &str, bitstamp_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);














    //------BITSTAMP------//
    type HmacSha256 = Hmac<Sha256>;
    fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------FOR ACTUAL REQUEST MESSAGE------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    //println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	//println!("Bitstamp:\n{:?}", bitstamp_response_text);





    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.04;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    


            //bitstamp calculations for sell
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + coinbase_wallet + *gemini_wallet + *bitstamp_wallet;
                println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i45_sol_5_gemini_bitstamp(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &mut f64, bitstamp_secret: &str, bitstamp_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);














    //------BITSTAMP------//
    type HmacSha256 = Hmac<Sha256>;
    fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------FOR ACTUAL REQUEST MESSAGE------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    //println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	//println!("Bitstamp:\n{:?}", bitstamp_response_text);





    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.05;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    


            //bitstamp calculations for sell
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + coinbase_wallet + *gemini_wallet + *bitstamp_wallet;
                println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i46_sol_6_gemini_bitstamp(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &mut f64, bitstamp_secret: &str, bitstamp_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);














    //------BITSTAMP------//
    type HmacSha256 = Hmac<Sha256>;
    fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------FOR ACTUAL REQUEST MESSAGE------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    //println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	//println!("Bitstamp:\n{:?}", bitstamp_response_text);





    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.06;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    


            //bitstamp calculations for sell
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + coinbase_wallet + *gemini_wallet + *bitstamp_wallet;
                println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i47_sol_7_gemini_bitstamp(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &mut f64, bitstamp_secret: &str, bitstamp_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);














    //------BITSTAMP------//
    type HmacSha256 = Hmac<Sha256>;
    fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------FOR ACTUAL REQUEST MESSAGE------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    //println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	//println!("Bitstamp:\n{:?}", bitstamp_response_text);





    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.07;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    


            //bitstamp calculations for sell
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + coinbase_wallet + *gemini_wallet + *bitstamp_wallet;
                println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i48_sol_8_gemini_bitstamp(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &mut f64, bitstamp_secret: &str, bitstamp_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);














    //------BITSTAMP------//
    type HmacSha256 = Hmac<Sha256>;
    fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------FOR ACTUAL REQUEST MESSAGE------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    //println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	//println!("Bitstamp:\n{:?}", bitstamp_response_text);





    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.08;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    


            //bitstamp calculations for sell
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + coinbase_wallet + *gemini_wallet + *bitstamp_wallet;
                println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i49_sol_9_gemini_bitstamp(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &mut f64, bitstamp_secret: &str, bitstamp_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);














    //------BITSTAMP------//
    type HmacSha256 = Hmac<Sha256>;
    fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------FOR ACTUAL REQUEST MESSAGE------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    //println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	//println!("Bitstamp:\n{:?}", bitstamp_response_text);





    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.09;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    


            //bitstamp calculations for sell
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + coinbase_wallet + *gemini_wallet + *bitstamp_wallet;
                println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i50_sol_10_gemini_bitstamp(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &f64, bitstamp_wallet: &mut f64,
        gemini_wallet: &mut f64, bitstamp_secret: &str, bitstamp_api_key: &str, client: reqwest::Client, gemini_secret: &str, gemini_api_key: &str )-> Result<(f64), Box<dyn Error>> {

        //------------------------------Gemini-----------------------------------------//
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
        println!("Bid: {}, Ask: {}", gemini_sell_pricebid, gemini_buy_ask);














    //------BITSTAMP------//
    type HmacSha256 = Hmac<Sha256>;
    fn bitstamp_sign(bitstamp_message: &str, bitstamp_secret: &str) -> String {
		let mut mac = HmacSha256::new_from_slice(&bitstamp_secret.as_bytes())
			.expect("HMAC can take key of any size");
		mac.update(bitstamp_message.as_bytes());
		let result = mac.finalize();
		let code_bytes = result.into_bytes();
		hex::encode(code_bytes)
	}
	

	let content_type = "application/x-www-form-urlencoded";
	let payload_string = "offset=1";
	//if we needed content_type, it is here
	//let content_type = "application/json";
	//this is the bitstamp message IF we needed content_type
	//let bitstamp_message = format!("BITSTAMP {}POSThttps://www.bitstamp.net/api/v2/account_balances/{}{}{}v2{}", 
	//	bitstamp_api_key, content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);








//--------------------FOR ACTUAL REQUEST MESSAGE------------------------------------------//



	let the_uuid = Uuid::new_v4();
	let bitstamp_nonce = the_uuid.to_string();
	let bitstamp_timestamp = now.timestamp_millis().to_string();
	//let content_type = "application/x-www-form-urlencoded";
	let bitstamp_message = format!("BITSTAMP {}GETwww.bitstamp.net/api/v2/ticker/sol-usd/{}{}{}{}v2{}", 
			bitstamp_api_key, "", content_type, bitstamp_nonce, bitstamp_timestamp, payload_string);

	let bitstamp_signature = bitstamp_sign(&bitstamp_message, &bitstamp_secret);

	let bitstamp_request = client.get("https://www.bitstamp.net/api/v2/ticker/solusd/")
		.header("X-Auth", format!("BITSTAMP {}", bitstamp_api_key))
		.header("X-Auth-Signature", bitstamp_signature)
		.header("X-Auth-Nonce", bitstamp_nonce)
		.header("X-Auth-Timestamp", bitstamp_timestamp)
		.header("X-Auth-Version", "v2")
		//.header("Content-Type", content_type)
		//.body(payload_string)
		.build()
		.expect("\ncould not build bitstamp_request");

	let bitstamp_response = client.execute(bitstamp_request).await
		.expect("Failed to execute Bitstamp request");
	let bitstamp_response_text = bitstamp_response.text().await
		.expect("Failed to turn response into text");
	//probably dont need "bitstamp" once we transfer this to the actual function
    let v: serde_json::Value = serde_json::from_str(&bitstamp_response_text)
    .expect("Failed to parse JSON");

// Extract the bid and ask values
    let bitstamp_sell_price_bid = v["bid"].as_str().unwrap().parse::<f64>().unwrap();
    let bitstamp_buy_price_ask = v["ask"].as_str().unwrap().parse::<f64>().unwrap();
    //println!("Bid: {}, Ask: {}", bitstamp_sell_price_bid, bitstamp_buy_price_ask);
	//println!("Bitstamp:\n{:?}", bitstamp_response_text);





    
    
    
    
    
    
    
    
    
    
    
    
            //gemini calculations for buy 
                //this should equal 0.4%
                let gemini_taker_fee = 0.004;
                let fraction_of_wallet_im_using = 0.10;

                let total_spent = fraction_of_wallet_im_using*(*gemini_wallet);
                let fee_for_purchase = total_spent*gemini_taker_fee;
                let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                //new state of gemini wallet below
                *gemini_wallet -= total_spent;
                let amount_of_sol = money_going_to_sol_after_fees/gemini_buy_ask;
    
    


            //bitstamp calculations for sell
                let bitstamp_taker_fee = 0.004;
                let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                *bitstamp_wallet += money_from_sell_after_fees;
    

            //coinbase calculations for sell

                //let coinbase_taker_fee = 0.008;
                //let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
                //let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*coinbase_wallet += money_from_sell_after_fees;
    
    
    
    


            //coinbase calculations for buy - not needed in this so code commented out
            
                //let coinbase_taker_fee = 0.008;
    
                //let total_spent = 0.10*(*coinbase_wallet);
                //let fee_for_purchase = total_spent*coinbase_taker_fee;
                //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
                ////new state of coinbase wallet below
                //*coinbase_wallet -= total_spent;
                //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;
    
            //kraken calculations - for sell
                //let kraken_taker_fee = 0.0026;
                
                //let money_from_sell_before_fees = amount_of_sol * kraken_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * kraken_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell ;
                //*kraken_wallet += money_from_sell_after_fees;
    
                //let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + *bitstamp_wallet;
    
    
            //bitstamp calculations - for sell
                //let bitstamp_taker_fee = 0.004;
                //let money_from_sell_before_fees = amount_of_sol * bitstamp_sell_price_bid;
                //let fee_for_sell = money_from_sell_before_fees * bitstamp_taker_fee;
                //let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
                //*bitstamp_wallet += money_from_sell_after_fees;



            //this will count as value after
                let value_after = kraken_wallet + coinbase_wallet + *gemini_wallet + *bitstamp_wallet;
                println!("value after:\n\t{}",value_after);
    
    
                return Ok(value_after)

    }

    pub async fn s_i51_sol_1_kraken_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
        //look at m, then look at functions to figure out current price of sol at coinbase,
        //      then do .01 * coinbase_wallet - trading_fee = how much sol in usd Im sending. 
        //      then do coinbase_wallet = coinbase_wallet - (.01 * coinbase_wallet + trading_fee)
        //      then do price_of_sol / how much I'm buying   or oposite  to get how much sol im sending
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then figure out price at kraken
        //      then sell it there with the trading fee.
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then set kraken_wallet = new value of wallet.
        //      then add values of both wallets to bitstamp_wallet and gemini_wallet
        //      then compare to value_prior
        //      tehn do reward function?
        //I'll have the keys in main so it doesn't have to load everything everytime, it can just store it in RAM
        //------all stuff below this is to actually complete the request to get how much money it costs
        let now = Utc::now();
        let time_stamp = now.timestamp().to_string();
        let method = "GET";
        let request_path = "/api/v3/brokerage/best_bid_ask";
        let body = "";
        let message = format!("{}{}{}{}", &time_stamp, &method, &request_path, &body);
        type HmacSha256 = Hmac<Sha256>;
        fn sign(message: &str, coinbase_secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                    .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
        }
        let coinbase_signature = sign(&message, &coinbase_secret);

        let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
        .header("CB-ACCESS-KEY", coinbase_api_key)
        .header("CB-ACCESS-SIGN", &coinbase_signature)
        .header("CB-ACCESS-TIMESTAMP", &time_stamp)
        .build()?;
        //manages the error I described above
        //let request = match request {
        //Ok(req) => req,
        //Err(e) => {
        //eprintln!("Failed to build request: \n{}", e);
        //return Err(e);
        //}
        //};

        let response = client.execute(request).await?;
        //let response = match response {
        //    Ok(resp) => resp,
        //    Err(e) => {
        //        eprintln!("Failed to execute request: \n{}", e);
        //        return Err(e);
        //    }
        //};


        let response_text = response.text().await?;

        //added 12/29/23
        //this is the parsing
        let v: Value = serde_json::from_str(&response_text)?;
        let mut coinbase_sell_price = 0.0;
        let mut coinbase_buy_price = 0.0;

        // Access the pricebooks array
        if let Some(pricebooks) = v["pricebooks"].as_array() {
            // Iterate over each pricebook
            for pricebook in pricebooks {
                // Access the product_id, bids, and asks
                let product_id = pricebook["product_id"].as_str().unwrap_or("");
                let bids = &pricebook["bids"][0];
                let asks = &pricebook["asks"][0];
        
                // Access the price and size of the bids and asks
                coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                let bid_size = bids["size"].as_str().unwrap_or("size not found");
                coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
        
                println!("Product ID: {}", product_id);
                //println!("Best bid: {} (size: {})", bid_price, bid_size);
                //println!("Best ask: {} (size: {})", ask_price, ask_size);
            }
        }

        //manages any errors from line above
        //let response_text = match response_text {
        //    Ok(t) => t,
        //    Err(e) => {
        //        eprintln!("Failed to read response text: \n{}", e);
        //        return;
        //    }
        //};

        //prints the actual response
        //println!("list accounts response\n{:?}", &response_text);























        //---KRAKEN--//

        //basically Kraken requires a value that is always increasing to be in each request.
        //I didnt use now.timestamp().to_string()  because just in case I have 2 
        //	requests in a second I dont want to be penalized.
        //if no "now" in scope when moving file, 
        //	the code is this:
        ////returns current time. MAY NEED TO USE LOCAL TIME
        //		let now = Utc::now();
        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            //println!("Nonce:\n{}", nonce_str);
            //println!("Encoded payload:\n{}", post_data);
            //println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            //println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            //println!("Ask price: {}", kraken_buy_price_ask);
            //println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations - to buy
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            //new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations - buy
            let kraken_taker_fee = 0.0026;
            let fraction_of_wallet_im_using = 0.01;  //aka 1 percent
            let total_spent = fraction_of_wallet_im_using*(*kraken_wallet);
            let fee_for_purchase = total_spent*kraken_taker_fee;
            let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            *kraken_wallet -= money_going_to_sol_after_fees;
            let amount_of_sol = money_going_to_sol_after_fees/kraken_buy_price_ask;

            


        //coinbase calculations for sell

            let coinbase_taker_fee = 0.008;
            let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
            let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
            let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
            *coinbase_wallet += money_from_sell_after_fees;




        //this will count as value after
            let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
            println!("sol1_kraken_coinbase\tvalue after\n\t{}", value_after);

            return Ok(value_after)

     }

     pub async fn s_i52_sol_2_kraken_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
        //look at m, then look at functions to figure out current price of sol at coinbase,
        //      then do .01 * coinbase_wallet - trading_fee = how much sol in usd Im sending. 
        //      then do coinbase_wallet = coinbase_wallet - (.01 * coinbase_wallet + trading_fee)
        //      then do price_of_sol / how much I'm buying   or oposite  to get how much sol im sending
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then figure out price at kraken
        //      then sell it there with the trading fee.
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then set kraken_wallet = new value of wallet.
        //      then add values of both wallets to bitstamp_wallet and gemini_wallet
        //      then compare to value_prior
        //      tehn do reward function?
        //I'll have the keys in main so it doesn't have to load everything everytime, it can just store it in RAM
        //------all stuff below this is to actually complete the request to get how much money it costs
        let now = Utc::now();
        let time_stamp = now.timestamp().to_string();
        let method = "GET";
        let request_path = "/api/v3/brokerage/best_bid_ask";
        let body = "";
        let message = format!("{}{}{}{}", &time_stamp, &method, &request_path, &body);
        type HmacSha256 = Hmac<Sha256>;
        fn sign(message: &str, coinbase_secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                    .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
        }
        let coinbase_signature = sign(&message, &coinbase_secret);

        let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
        .header("CB-ACCESS-KEY", coinbase_api_key)
        .header("CB-ACCESS-SIGN", &coinbase_signature)
        .header("CB-ACCESS-TIMESTAMP", &time_stamp)
        .build()?;
        //manages the error I described above
        //let request = match request {
        //Ok(req) => req,
        //Err(e) => {
        //eprintln!("Failed to build request: \n{}", e);
        //return Err(e);
        //}
        //};

        let response = client.execute(request).await?;
        //let response = match response {
        //    Ok(resp) => resp,
        //    Err(e) => {
        //        eprintln!("Failed to execute request: \n{}", e);
        //        return Err(e);
        //    }
        //};


        let response_text = response.text().await?;

        //added 12/29/23
        //this is the parsing
        let v: Value = serde_json::from_str(&response_text)?;
        let mut coinbase_sell_price = 0.0;
        let mut coinbase_buy_price = 0.0;

        // Access the pricebooks array
        if let Some(pricebooks) = v["pricebooks"].as_array() {
            // Iterate over each pricebook
            for pricebook in pricebooks {
                // Access the product_id, bids, and asks
                let product_id = pricebook["product_id"].as_str().unwrap_or("");
                let bids = &pricebook["bids"][0];
                let asks = &pricebook["asks"][0];
        
                // Access the price and size of the bids and asks
                coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                let bid_size = bids["size"].as_str().unwrap_or("size not found");
                coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
        
                println!("Product ID: {}", product_id);
                //println!("Best bid: {} (size: {})", bid_price, bid_size);
                //println!("Best ask: {} (size: {})", ask_price, ask_size);
            }
        }

        //manages any errors from line above
        //let response_text = match response_text {
        //    Ok(t) => t,
        //    Err(e) => {
        //        eprintln!("Failed to read response text: \n{}", e);
        //        return;
        //    }
        //};

        //prints the actual response
        //println!("list accounts response\n{:?}", &response_text);























        //---KRAKEN--//

        //basically Kraken requires a value that is always increasing to be in each request.
        //I didnt use now.timestamp().to_string()  because just in case I have 2 
        //	requests in a second I dont want to be penalized.
        //if no "now" in scope when moving file, 
        //	the code is this:
        ////returns current time. MAY NEED TO USE LOCAL TIME
        //		let now = Utc::now();
        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            //println!("Nonce:\n{}", nonce_str);
            //println!("Encoded payload:\n{}", post_data);
            //println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            //println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            //println!("Ask price: {}", kraken_buy_price_ask);
            //println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations - to buy
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            //new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations - buy
            let kraken_taker_fee = 0.0026;
            let fraction_of_wallet_im_using = 0.02;  //aka 2 percent
            let total_spent = fraction_of_wallet_im_using*(*kraken_wallet);
            let fee_for_purchase = total_spent*kraken_taker_fee;
            let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            *kraken_wallet -= money_going_to_sol_after_fees;
            let amount_of_sol = money_going_to_sol_after_fees/kraken_buy_price_ask;

            


        //coinbase calculations for sell

            let coinbase_taker_fee = 0.008;
            let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
            let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
            let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
            *coinbase_wallet += money_from_sell_after_fees;




        //this will count as value after
            let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
            println!("sol1_kraken_coinbase\tvalue after\n\t{}", value_after);

            return Ok(value_after)

     }

     pub async fn s_i53_sol_3_kraken_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
        //look at m, then look at functions to figure out current price of sol at coinbase,
        //      then do .01 * coinbase_wallet - trading_fee = how much sol in usd Im sending. 
        //      then do coinbase_wallet = coinbase_wallet - (.01 * coinbase_wallet + trading_fee)
        //      then do price_of_sol / how much I'm buying   or oposite  to get how much sol im sending
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then figure out price at kraken
        //      then sell it there with the trading fee.
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then set kraken_wallet = new value of wallet.
        //      then add values of both wallets to bitstamp_wallet and gemini_wallet
        //      then compare to value_prior
        //      tehn do reward function?
        //I'll have the keys in main so it doesn't have to load everything everytime, it can just store it in RAM
        //------all stuff below this is to actually complete the request to get how much money it costs
        let now = Utc::now();
        let time_stamp = now.timestamp().to_string();
        let method = "GET";
        let request_path = "/api/v3/brokerage/best_bid_ask";
        let body = "";
        let message = format!("{}{}{}{}", &time_stamp, &method, &request_path, &body);
        type HmacSha256 = Hmac<Sha256>;
        fn sign(message: &str, coinbase_secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                    .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
        }
        let coinbase_signature = sign(&message, &coinbase_secret);

        let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
        .header("CB-ACCESS-KEY", coinbase_api_key)
        .header("CB-ACCESS-SIGN", &coinbase_signature)
        .header("CB-ACCESS-TIMESTAMP", &time_stamp)
        .build()?;
        //manages the error I described above
        //let request = match request {
        //Ok(req) => req,
        //Err(e) => {
        //eprintln!("Failed to build request: \n{}", e);
        //return Err(e);
        //}
        //};

        let response = client.execute(request).await?;
        //let response = match response {
        //    Ok(resp) => resp,
        //    Err(e) => {
        //        eprintln!("Failed to execute request: \n{}", e);
        //        return Err(e);
        //    }
        //};


        let response_text = response.text().await?;

        //added 12/29/23
        //this is the parsing
        let v: Value = serde_json::from_str(&response_text)?;
        let mut coinbase_sell_price = 0.0;
        let mut coinbase_buy_price = 0.0;

        // Access the pricebooks array
        if let Some(pricebooks) = v["pricebooks"].as_array() {
            // Iterate over each pricebook
            for pricebook in pricebooks {
                // Access the product_id, bids, and asks
                let product_id = pricebook["product_id"].as_str().unwrap_or("");
                let bids = &pricebook["bids"][0];
                let asks = &pricebook["asks"][0];
        
                // Access the price and size of the bids and asks
                coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                let bid_size = bids["size"].as_str().unwrap_or("size not found");
                coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
        
                println!("Product ID: {}", product_id);
                //println!("Best bid: {} (size: {})", bid_price, bid_size);
                //println!("Best ask: {} (size: {})", ask_price, ask_size);
            }
        }

        //manages any errors from line above
        //let response_text = match response_text {
        //    Ok(t) => t,
        //    Err(e) => {
        //        eprintln!("Failed to read response text: \n{}", e);
        //        return;
        //    }
        //};

        //prints the actual response
        //println!("list accounts response\n{:?}", &response_text);























        //---KRAKEN--//

        //basically Kraken requires a value that is always increasing to be in each request.
        //I didnt use now.timestamp().to_string()  because just in case I have 2 
        //	requests in a second I dont want to be penalized.
        //if no "now" in scope when moving file, 
        //	the code is this:
        ////returns current time. MAY NEED TO USE LOCAL TIME
        //		let now = Utc::now();
        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            //println!("Nonce:\n{}", nonce_str);
            //println!("Encoded payload:\n{}", post_data);
            //println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            //println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            //println!("Ask price: {}", kraken_buy_price_ask);
            //println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations - to buy
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            //new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations - buy
            let kraken_taker_fee = 0.0026;
            let fraction_of_wallet_im_using = 0.03;  //aka 3 percent
            let total_spent = fraction_of_wallet_im_using*(*kraken_wallet);
            let fee_for_purchase = total_spent*kraken_taker_fee;
            let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            *kraken_wallet -= money_going_to_sol_after_fees;
            let amount_of_sol = money_going_to_sol_after_fees/kraken_buy_price_ask;

            


        //coinbase calculations for sell

            let coinbase_taker_fee = 0.008;
            let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
            let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
            let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
            *coinbase_wallet += money_from_sell_after_fees;




        //this will count as value after
            let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
            println!("sol1_kraken_coinbase\tvalue after\n\t{}", value_after);

            return Ok(value_after)

     }

     pub async fn s_i54_sol_4_kraken_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
        //look at m, then look at functions to figure out current price of sol at coinbase,
        //      then do .01 * coinbase_wallet - trading_fee = how much sol in usd Im sending. 
        //      then do coinbase_wallet = coinbase_wallet - (.01 * coinbase_wallet + trading_fee)
        //      then do price_of_sol / how much I'm buying   or oposite  to get how much sol im sending
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then figure out price at kraken
        //      then sell it there with the trading fee.
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then set kraken_wallet = new value of wallet.
        //      then add values of both wallets to bitstamp_wallet and gemini_wallet
        //      then compare to value_prior
        //      tehn do reward function?
        //I'll have the keys in main so it doesn't have to load everything everytime, it can just store it in RAM
        //------all stuff below this is to actually complete the request to get how much money it costs
        let now = Utc::now();
        let time_stamp = now.timestamp().to_string();
        let method = "GET";
        let request_path = "/api/v3/brokerage/best_bid_ask";
        let body = "";
        let message = format!("{}{}{}{}", &time_stamp, &method, &request_path, &body);
        type HmacSha256 = Hmac<Sha256>;
        fn sign(message: &str, coinbase_secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                    .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
        }
        let coinbase_signature = sign(&message, &coinbase_secret);

        let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
        .header("CB-ACCESS-KEY", coinbase_api_key)
        .header("CB-ACCESS-SIGN", &coinbase_signature)
        .header("CB-ACCESS-TIMESTAMP", &time_stamp)
        .build()?;
        //manages the error I described above
        //let request = match request {
        //Ok(req) => req,
        //Err(e) => {
        //eprintln!("Failed to build request: \n{}", e);
        //return Err(e);
        //}
        //};

        let response = client.execute(request).await?;
        //let response = match response {
        //    Ok(resp) => resp,
        //    Err(e) => {
        //        eprintln!("Failed to execute request: \n{}", e);
        //        return Err(e);
        //    }
        //};


        let response_text = response.text().await?;

        //added 12/29/23
        //this is the parsing
        let v: Value = serde_json::from_str(&response_text)?;
        let mut coinbase_sell_price = 0.0;
        let mut coinbase_buy_price = 0.0;

        // Access the pricebooks array
        if let Some(pricebooks) = v["pricebooks"].as_array() {
            // Iterate over each pricebook
            for pricebook in pricebooks {
                // Access the product_id, bids, and asks
                let product_id = pricebook["product_id"].as_str().unwrap_or("");
                let bids = &pricebook["bids"][0];
                let asks = &pricebook["asks"][0];
        
                // Access the price and size of the bids and asks
                coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                let bid_size = bids["size"].as_str().unwrap_or("size not found");
                coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
        
                println!("Product ID: {}", product_id);
                //println!("Best bid: {} (size: {})", bid_price, bid_size);
                //println!("Best ask: {} (size: {})", ask_price, ask_size);
            }
        }

        //manages any errors from line above
        //let response_text = match response_text {
        //    Ok(t) => t,
        //    Err(e) => {
        //        eprintln!("Failed to read response text: \n{}", e);
        //        return;
        //    }
        //};

        //prints the actual response
        //println!("list accounts response\n{:?}", &response_text);























        //---KRAKEN--//

        //basically Kraken requires a value that is always increasing to be in each request.
        //I didnt use now.timestamp().to_string()  because just in case I have 2 
        //	requests in a second I dont want to be penalized.
        //if no "now" in scope when moving file, 
        //	the code is this:
        ////returns current time. MAY NEED TO USE LOCAL TIME
        //		let now = Utc::now();
        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            //println!("Nonce:\n{}", nonce_str);
            //println!("Encoded payload:\n{}", post_data);
            //println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            //println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            //println!("Ask price: {}", kraken_buy_price_ask);
            //println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations - to buy
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            //new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations - buy
            let kraken_taker_fee = 0.0026;
            let fraction_of_wallet_im_using = 0.04;  //aka 4 percent
            let total_spent = fraction_of_wallet_im_using*(*kraken_wallet);
            let fee_for_purchase = total_spent*kraken_taker_fee;
            let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            *kraken_wallet -= money_going_to_sol_after_fees;
            let amount_of_sol = money_going_to_sol_after_fees/kraken_buy_price_ask;

            


        //coinbase calculations for sell

            let coinbase_taker_fee = 0.008;
            let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
            let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
            let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
            *coinbase_wallet += money_from_sell_after_fees;




        //this will count as value after
            let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
            println!("sol1_kraken_coinbase\tvalue after\n\t{}", value_after);

            return Ok(value_after)

     }

     pub async fn s_i55_sol_5_kraken_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
        //look at m, then look at functions to figure out current price of sol at coinbase,
        //      then do .01 * coinbase_wallet - trading_fee = how much sol in usd Im sending. 
        //      then do coinbase_wallet = coinbase_wallet - (.01 * coinbase_wallet + trading_fee)
        //      then do price_of_sol / how much I'm buying   or oposite  to get how much sol im sending
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then figure out price at kraken
        //      then sell it there with the trading fee.
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then set kraken_wallet = new value of wallet.
        //      then add values of both wallets to bitstamp_wallet and gemini_wallet
        //      then compare to value_prior
        //      tehn do reward function?
        //I'll have the keys in main so it doesn't have to load everything everytime, it can just store it in RAM
        //------all stuff below this is to actually complete the request to get how much money it costs
        let now = Utc::now();
        let time_stamp = now.timestamp().to_string();
        let method = "GET";
        let request_path = "/api/v3/brokerage/best_bid_ask";
        let body = "";
        let message = format!("{}{}{}{}", &time_stamp, &method, &request_path, &body);
        type HmacSha256 = Hmac<Sha256>;
        fn sign(message: &str, coinbase_secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                    .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
        }
        let coinbase_signature = sign(&message, &coinbase_secret);

        let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
        .header("CB-ACCESS-KEY", coinbase_api_key)
        .header("CB-ACCESS-SIGN", &coinbase_signature)
        .header("CB-ACCESS-TIMESTAMP", &time_stamp)
        .build()?;
        //manages the error I described above
        //let request = match request {
        //Ok(req) => req,
        //Err(e) => {
        //eprintln!("Failed to build request: \n{}", e);
        //return Err(e);
        //}
        //};

        let response = client.execute(request).await?;
        //let response = match response {
        //    Ok(resp) => resp,
        //    Err(e) => {
        //        eprintln!("Failed to execute request: \n{}", e);
        //        return Err(e);
        //    }
        //};


        let response_text = response.text().await?;

        //added 12/29/23
        //this is the parsing
        let v: Value = serde_json::from_str(&response_text)?;
        let mut coinbase_sell_price = 0.0;
        let mut coinbase_buy_price = 0.0;

        // Access the pricebooks array
        if let Some(pricebooks) = v["pricebooks"].as_array() {
            // Iterate over each pricebook
            for pricebook in pricebooks {
                // Access the product_id, bids, and asks
                let product_id = pricebook["product_id"].as_str().unwrap_or("");
                let bids = &pricebook["bids"][0];
                let asks = &pricebook["asks"][0];
        
                // Access the price and size of the bids and asks
                coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                let bid_size = bids["size"].as_str().unwrap_or("size not found");
                coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
        
                println!("Product ID: {}", product_id);
                //println!("Best bid: {} (size: {})", bid_price, bid_size);
                //println!("Best ask: {} (size: {})", ask_price, ask_size);
            }
        }

        //manages any errors from line above
        //let response_text = match response_text {
        //    Ok(t) => t,
        //    Err(e) => {
        //        eprintln!("Failed to read response text: \n{}", e);
        //        return;
        //    }
        //};

        //prints the actual response
        //println!("list accounts response\n{:?}", &response_text);























        //---KRAKEN--//

        //basically Kraken requires a value that is always increasing to be in each request.
        //I didnt use now.timestamp().to_string()  because just in case I have 2 
        //	requests in a second I dont want to be penalized.
        //if no "now" in scope when moving file, 
        //	the code is this:
        ////returns current time. MAY NEED TO USE LOCAL TIME
        //		let now = Utc::now();
        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            //println!("Nonce:\n{}", nonce_str);
            //println!("Encoded payload:\n{}", post_data);
            //println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            //println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            //println!("Ask price: {}", kraken_buy_price_ask);
            //println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations - to buy
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            //new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations - buy
            let kraken_taker_fee = 0.0026;
            let fraction_of_wallet_im_using = 0.05;  //aka 5 percent
            let total_spent = fraction_of_wallet_im_using*(*kraken_wallet);
            let fee_for_purchase = total_spent*kraken_taker_fee;
            let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            *kraken_wallet -= money_going_to_sol_after_fees;
            let amount_of_sol = money_going_to_sol_after_fees/kraken_buy_price_ask;

            


        //coinbase calculations for sell

            let coinbase_taker_fee = 0.008;
            let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
            let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
            let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
            *coinbase_wallet += money_from_sell_after_fees;




        //this will count as value after
            let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
            println!("sol1_kraken_coinbase\tvalue after\n\t{}", value_after);

            return Ok(value_after)

     }

     pub async fn s_i56_sol_6_kraken_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
        //look at m, then look at functions to figure out current price of sol at coinbase,
        //      then do .01 * coinbase_wallet - trading_fee = how much sol in usd Im sending. 
        //      then do coinbase_wallet = coinbase_wallet - (.01 * coinbase_wallet + trading_fee)
        //      then do price_of_sol / how much I'm buying   or oposite  to get how much sol im sending
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then figure out price at kraken
        //      then sell it there with the trading fee.
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then set kraken_wallet = new value of wallet.
        //      then add values of both wallets to bitstamp_wallet and gemini_wallet
        //      then compare to value_prior
        //      tehn do reward function?
        //I'll have the keys in main so it doesn't have to load everything everytime, it can just store it in RAM
        //------all stuff below this is to actually complete the request to get how much money it costs
        let now = Utc::now();
        let time_stamp = now.timestamp().to_string();
        let method = "GET";
        let request_path = "/api/v3/brokerage/best_bid_ask";
        let body = "";
        let message = format!("{}{}{}{}", &time_stamp, &method, &request_path, &body);
        type HmacSha256 = Hmac<Sha256>;
        fn sign(message: &str, coinbase_secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                    .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
        }
        let coinbase_signature = sign(&message, &coinbase_secret);

        let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
        .header("CB-ACCESS-KEY", coinbase_api_key)
        .header("CB-ACCESS-SIGN", &coinbase_signature)
        .header("CB-ACCESS-TIMESTAMP", &time_stamp)
        .build()?;
        //manages the error I described above
        //let request = match request {
        //Ok(req) => req,
        //Err(e) => {
        //eprintln!("Failed to build request: \n{}", e);
        //return Err(e);
        //}
        //};

        let response = client.execute(request).await?;
        //let response = match response {
        //    Ok(resp) => resp,
        //    Err(e) => {
        //        eprintln!("Failed to execute request: \n{}", e);
        //        return Err(e);
        //    }
        //};


        let response_text = response.text().await?;

        //added 12/29/23
        //this is the parsing
        let v: Value = serde_json::from_str(&response_text)?;
        let mut coinbase_sell_price = 0.0;
        let mut coinbase_buy_price = 0.0;

        // Access the pricebooks array
        if let Some(pricebooks) = v["pricebooks"].as_array() {
            // Iterate over each pricebook
            for pricebook in pricebooks {
                // Access the product_id, bids, and asks
                let product_id = pricebook["product_id"].as_str().unwrap_or("");
                let bids = &pricebook["bids"][0];
                let asks = &pricebook["asks"][0];
        
                // Access the price and size of the bids and asks
                coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                let bid_size = bids["size"].as_str().unwrap_or("size not found");
                coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
        
                println!("Product ID: {}", product_id);
                //println!("Best bid: {} (size: {})", bid_price, bid_size);
                //println!("Best ask: {} (size: {})", ask_price, ask_size);
            }
        }

        //manages any errors from line above
        //let response_text = match response_text {
        //    Ok(t) => t,
        //    Err(e) => {
        //        eprintln!("Failed to read response text: \n{}", e);
        //        return;
        //    }
        //};

        //prints the actual response
        //println!("list accounts response\n{:?}", &response_text);























        //---KRAKEN--//

        //basically Kraken requires a value that is always increasing to be in each request.
        //I didnt use now.timestamp().to_string()  because just in case I have 2 
        //	requests in a second I dont want to be penalized.
        //if no "now" in scope when moving file, 
        //	the code is this:
        ////returns current time. MAY NEED TO USE LOCAL TIME
        //		let now = Utc::now();
        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            //println!("Nonce:\n{}", nonce_str);
            //println!("Encoded payload:\n{}", post_data);
            //println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            //println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            //println!("Ask price: {}", kraken_buy_price_ask);
            //println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations - to buy
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            //new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations - buy
            let kraken_taker_fee = 0.0026;
            let fraction_of_wallet_im_using = 0.06;  //aka 6 percent
            let total_spent = fraction_of_wallet_im_using*(*kraken_wallet);
            let fee_for_purchase = total_spent*kraken_taker_fee;
            let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            *kraken_wallet -= money_going_to_sol_after_fees;
            let amount_of_sol = money_going_to_sol_after_fees/kraken_buy_price_ask;

            


        //coinbase calculations for sell

            let coinbase_taker_fee = 0.008;
            let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
            let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
            let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
            *coinbase_wallet += money_from_sell_after_fees;




        //this will count as value after
            let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
            println!("sol1_kraken_coinbase\tvalue after\n\t{}", value_after);

            return Ok(value_after)

     }

     pub async fn s_i57_sol_7_kraken_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
        //look at m, then look at functions to figure out current price of sol at coinbase,
        //      then do .01 * coinbase_wallet - trading_fee = how much sol in usd Im sending. 
        //      then do coinbase_wallet = coinbase_wallet - (.01 * coinbase_wallet + trading_fee)
        //      then do price_of_sol / how much I'm buying   or oposite  to get how much sol im sending
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then figure out price at kraken
        //      then sell it there with the trading fee.
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then set kraken_wallet = new value of wallet.
        //      then add values of both wallets to bitstamp_wallet and gemini_wallet
        //      then compare to value_prior
        //      tehn do reward function?
        //I'll have the keys in main so it doesn't have to load everything everytime, it can just store it in RAM
        //------all stuff below this is to actually complete the request to get how much money it costs
        let now = Utc::now();
        let time_stamp = now.timestamp().to_string();
        let method = "GET";
        let request_path = "/api/v3/brokerage/best_bid_ask";
        let body = "";
        let message = format!("{}{}{}{}", &time_stamp, &method, &request_path, &body);
        type HmacSha256 = Hmac<Sha256>;
        fn sign(message: &str, coinbase_secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                    .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
        }
        let coinbase_signature = sign(&message, &coinbase_secret);

        let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
        .header("CB-ACCESS-KEY", coinbase_api_key)
        .header("CB-ACCESS-SIGN", &coinbase_signature)
        .header("CB-ACCESS-TIMESTAMP", &time_stamp)
        .build()?;
        //manages the error I described above
        //let request = match request {
        //Ok(req) => req,
        //Err(e) => {
        //eprintln!("Failed to build request: \n{}", e);
        //return Err(e);
        //}
        //};

        let response = client.execute(request).await?;
        //let response = match response {
        //    Ok(resp) => resp,
        //    Err(e) => {
        //        eprintln!("Failed to execute request: \n{}", e);
        //        return Err(e);
        //    }
        //};


        let response_text = response.text().await?;

        //added 12/29/23
        //this is the parsing
        let v: Value = serde_json::from_str(&response_text)?;
        let mut coinbase_sell_price = 0.0;
        let mut coinbase_buy_price = 0.0;

        // Access the pricebooks array
        if let Some(pricebooks) = v["pricebooks"].as_array() {
            // Iterate over each pricebook
            for pricebook in pricebooks {
                // Access the product_id, bids, and asks
                let product_id = pricebook["product_id"].as_str().unwrap_or("");
                let bids = &pricebook["bids"][0];
                let asks = &pricebook["asks"][0];
        
                // Access the price and size of the bids and asks
                coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                let bid_size = bids["size"].as_str().unwrap_or("size not found");
                coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
        
                println!("Product ID: {}", product_id);
                //println!("Best bid: {} (size: {})", bid_price, bid_size);
                //println!("Best ask: {} (size: {})", ask_price, ask_size);
            }
        }

        //manages any errors from line above
        //let response_text = match response_text {
        //    Ok(t) => t,
        //    Err(e) => {
        //        eprintln!("Failed to read response text: \n{}", e);
        //        return;
        //    }
        //};

        //prints the actual response
        //println!("list accounts response\n{:?}", &response_text);























        //---KRAKEN--//

        //basically Kraken requires a value that is always increasing to be in each request.
        //I didnt use now.timestamp().to_string()  because just in case I have 2 
        //	requests in a second I dont want to be penalized.
        //if no "now" in scope when moving file, 
        //	the code is this:
        ////returns current time. MAY NEED TO USE LOCAL TIME
        //		let now = Utc::now();
        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            //println!("Nonce:\n{}", nonce_str);
            //println!("Encoded payload:\n{}", post_data);
            //println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            //println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            //println!("Ask price: {}", kraken_buy_price_ask);
            //println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations - to buy
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            //new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations - buy
            let kraken_taker_fee = 0.0026;
            let fraction_of_wallet_im_using = 0.07;  //aka 7 percent
            let total_spent = fraction_of_wallet_im_using*(*kraken_wallet);
            let fee_for_purchase = total_spent*kraken_taker_fee;
            let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            *kraken_wallet -= money_going_to_sol_after_fees;
            let amount_of_sol = money_going_to_sol_after_fees/kraken_buy_price_ask;

            


        //coinbase calculations for sell

            let coinbase_taker_fee = 0.008;
            let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
            let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
            let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
            *coinbase_wallet += money_from_sell_after_fees;




        //this will count as value after
            let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
            println!("sol1_kraken_coinbase\tvalue after\n\t{}", value_after);

            return Ok(value_after)

     }

     pub async fn s_i58_sol_8_kraken_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
        //look at m, then look at functions to figure out current price of sol at coinbase,
        //      then do .01 * coinbase_wallet - trading_fee = how much sol in usd Im sending. 
        //      then do coinbase_wallet = coinbase_wallet - (.01 * coinbase_wallet + trading_fee)
        //      then do price_of_sol / how much I'm buying   or oposite  to get how much sol im sending
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then figure out price at kraken
        //      then sell it there with the trading fee.
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then set kraken_wallet = new value of wallet.
        //      then add values of both wallets to bitstamp_wallet and gemini_wallet
        //      then compare to value_prior
        //      tehn do reward function?
        //I'll have the keys in main so it doesn't have to load everything everytime, it can just store it in RAM
        //------all stuff below this is to actually complete the request to get how much money it costs
        let now = Utc::now();
        let time_stamp = now.timestamp().to_string();
        let method = "GET";
        let request_path = "/api/v3/brokerage/best_bid_ask";
        let body = "";
        let message = format!("{}{}{}{}", &time_stamp, &method, &request_path, &body);
        type HmacSha256 = Hmac<Sha256>;
        fn sign(message: &str, coinbase_secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                    .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
        }
        let coinbase_signature = sign(&message, &coinbase_secret);

        let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
        .header("CB-ACCESS-KEY", coinbase_api_key)
        .header("CB-ACCESS-SIGN", &coinbase_signature)
        .header("CB-ACCESS-TIMESTAMP", &time_stamp)
        .build()?;
        //manages the error I described above
        //let request = match request {
        //Ok(req) => req,
        //Err(e) => {
        //eprintln!("Failed to build request: \n{}", e);
        //return Err(e);
        //}
        //};

        let response = client.execute(request).await?;
        //let response = match response {
        //    Ok(resp) => resp,
        //    Err(e) => {
        //        eprintln!("Failed to execute request: \n{}", e);
        //        return Err(e);
        //    }
        //};


        let response_text = response.text().await?;

        //added 12/29/23
        //this is the parsing
        let v: Value = serde_json::from_str(&response_text)?;
        let mut coinbase_sell_price = 0.0;
        let mut coinbase_buy_price = 0.0;

        // Access the pricebooks array
        if let Some(pricebooks) = v["pricebooks"].as_array() {
            // Iterate over each pricebook
            for pricebook in pricebooks {
                // Access the product_id, bids, and asks
                let product_id = pricebook["product_id"].as_str().unwrap_or("");
                let bids = &pricebook["bids"][0];
                let asks = &pricebook["asks"][0];
        
                // Access the price and size of the bids and asks
                coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                let bid_size = bids["size"].as_str().unwrap_or("size not found");
                coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
        
                println!("Product ID: {}", product_id);
                //println!("Best bid: {} (size: {})", bid_price, bid_size);
                //println!("Best ask: {} (size: {})", ask_price, ask_size);
            }
        }

        //manages any errors from line above
        //let response_text = match response_text {
        //    Ok(t) => t,
        //    Err(e) => {
        //        eprintln!("Failed to read response text: \n{}", e);
        //        return;
        //    }
        //};

        //prints the actual response
        //println!("list accounts response\n{:?}", &response_text);























        //---KRAKEN--//

        //basically Kraken requires a value that is always increasing to be in each request.
        //I didnt use now.timestamp().to_string()  because just in case I have 2 
        //	requests in a second I dont want to be penalized.
        //if no "now" in scope when moving file, 
        //	the code is this:
        ////returns current time. MAY NEED TO USE LOCAL TIME
        //		let now = Utc::now();
        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            //println!("Nonce:\n{}", nonce_str);
            //println!("Encoded payload:\n{}", post_data);
            //println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            //println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            //println!("Ask price: {}", kraken_buy_price_ask);
            //println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations - to buy
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            //new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations - buy
            let kraken_taker_fee = 0.0026;
            let fraction_of_wallet_im_using = 0.08;  //aka 8 percent
            let total_spent = fraction_of_wallet_im_using*(*kraken_wallet);
            let fee_for_purchase = total_spent*kraken_taker_fee;
            let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            *kraken_wallet -= money_going_to_sol_after_fees;
            let amount_of_sol = money_going_to_sol_after_fees/kraken_buy_price_ask;

            


        //coinbase calculations for sell

            let coinbase_taker_fee = 0.008;
            let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
            let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
            let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
            *coinbase_wallet += money_from_sell_after_fees;




        //this will count as value after
            let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
            println!("sol1_kraken_coinbase\tvalue after\n\t{}", value_after);

            return Ok(value_after)

     }

     pub async fn s_i59_sol_9_kraken_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
        //look at m, then look at functions to figure out current price of sol at coinbase,
        //      then do .01 * coinbase_wallet - trading_fee = how much sol in usd Im sending. 
        //      then do coinbase_wallet = coinbase_wallet - (.01 * coinbase_wallet + trading_fee)
        //      then do price_of_sol / how much I'm buying   or oposite  to get how much sol im sending
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then figure out price at kraken
        //      then sell it there with the trading fee.
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then set kraken_wallet = new value of wallet.
        //      then add values of both wallets to bitstamp_wallet and gemini_wallet
        //      then compare to value_prior
        //      tehn do reward function?
        //I'll have the keys in main so it doesn't have to load everything everytime, it can just store it in RAM
        //------all stuff below this is to actually complete the request to get how much money it costs
        let now = Utc::now();
        let time_stamp = now.timestamp().to_string();
        let method = "GET";
        let request_path = "/api/v3/brokerage/best_bid_ask";
        let body = "";
        let message = format!("{}{}{}{}", &time_stamp, &method, &request_path, &body);
        type HmacSha256 = Hmac<Sha256>;
        fn sign(message: &str, coinbase_secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                    .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
        }
        let coinbase_signature = sign(&message, &coinbase_secret);

        let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
        .header("CB-ACCESS-KEY", coinbase_api_key)
        .header("CB-ACCESS-SIGN", &coinbase_signature)
        .header("CB-ACCESS-TIMESTAMP", &time_stamp)
        .build()?;
        //manages the error I described above
        //let request = match request {
        //Ok(req) => req,
        //Err(e) => {
        //eprintln!("Failed to build request: \n{}", e);
        //return Err(e);
        //}
        //};

        let response = client.execute(request).await?;
        //let response = match response {
        //    Ok(resp) => resp,
        //    Err(e) => {
        //        eprintln!("Failed to execute request: \n{}", e);
        //        return Err(e);
        //    }
        //};


        let response_text = response.text().await?;

        //added 12/29/23
        //this is the parsing
        let v: Value = serde_json::from_str(&response_text)?;
        let mut coinbase_sell_price = 0.0;
        let mut coinbase_buy_price = 0.0;

        // Access the pricebooks array
        if let Some(pricebooks) = v["pricebooks"].as_array() {
            // Iterate over each pricebook
            for pricebook in pricebooks {
                // Access the product_id, bids, and asks
                let product_id = pricebook["product_id"].as_str().unwrap_or("");
                let bids = &pricebook["bids"][0];
                let asks = &pricebook["asks"][0];
        
                // Access the price and size of the bids and asks
                coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                let bid_size = bids["size"].as_str().unwrap_or("size not found");
                coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
        
                println!("Product ID: {}", product_id);
                //println!("Best bid: {} (size: {})", bid_price, bid_size);
                //println!("Best ask: {} (size: {})", ask_price, ask_size);
            }
        }

        //manages any errors from line above
        //let response_text = match response_text {
        //    Ok(t) => t,
        //    Err(e) => {
        //        eprintln!("Failed to read response text: \n{}", e);
        //        return;
        //    }
        //};

        //prints the actual response
        //println!("list accounts response\n{:?}", &response_text);























        //---KRAKEN--//

        //basically Kraken requires a value that is always increasing to be in each request.
        //I didnt use now.timestamp().to_string()  because just in case I have 2 
        //	requests in a second I dont want to be penalized.
        //if no "now" in scope when moving file, 
        //	the code is this:
        ////returns current time. MAY NEED TO USE LOCAL TIME
        //		let now = Utc::now();
        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            //println!("Nonce:\n{}", nonce_str);
            //println!("Encoded payload:\n{}", post_data);
            //println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            //println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            //println!("Ask price: {}", kraken_buy_price_ask);
            //println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations - to buy
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            //new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations - buy
            let kraken_taker_fee = 0.0026;
            let fraction_of_wallet_im_using = 0.09;  //aka 9 percent
            let total_spent = fraction_of_wallet_im_using*(*kraken_wallet);
            let fee_for_purchase = total_spent*kraken_taker_fee;
            let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            *kraken_wallet -= money_going_to_sol_after_fees;
            let amount_of_sol = money_going_to_sol_after_fees/kraken_buy_price_ask;

            


        //coinbase calculations for sell

            let coinbase_taker_fee = 0.008;
            let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
            let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
            let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
            *coinbase_wallet += money_from_sell_after_fees;




        //this will count as value after
            let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
            println!("sol1_kraken_coinbase\tvalue after\n\t{}", value_after);

            return Ok(value_after)

     }

     pub async fn s_i60_sol_10_kraken_coinbase(value_prior: &f64, coinbase_wallet: &mut f64, kraken_wallet: &mut f64, bitstamp_wallet: &f64,
        gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client, kraken_secret: &str, kraken_api_key: &str )-> Result<f64, Box<dyn Error>> {
        //look at m, then look at functions to figure out current price of sol at coinbase,
        //      then do .01 * coinbase_wallet - trading_fee = how much sol in usd Im sending. 
        //      then do coinbase_wallet = coinbase_wallet - (.01 * coinbase_wallet + trading_fee)
        //      then do price_of_sol / how much I'm buying   or oposite  to get how much sol im sending
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then figure out price at kraken
        //      then sell it there with the trading fee.
        //      then do a 2.7 sec wait. 2.5 sec for sol transaction speed. 0.2 sec for assumed time to gather data
        //      then set kraken_wallet = new value of wallet.
        //      then add values of both wallets to bitstamp_wallet and gemini_wallet
        //      then compare to value_prior
        //      tehn do reward function?
        //I'll have the keys in main so it doesn't have to load everything everytime, it can just store it in RAM
        //------all stuff below this is to actually complete the request to get how much money it costs
        let now = Utc::now();
        let time_stamp = now.timestamp().to_string();
        let method = "GET";
        let request_path = "/api/v3/brokerage/best_bid_ask";
        let body = "";
        let message = format!("{}{}{}{}", &time_stamp, &method, &request_path, &body);
        type HmacSha256 = Hmac<Sha256>;
        fn sign(message: &str, coinbase_secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                    .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
        }
        let coinbase_signature = sign(&message, &coinbase_secret);

        let request = client.get("https://coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=SOL-USD")
        .header("CB-ACCESS-KEY", coinbase_api_key)
        .header("CB-ACCESS-SIGN", &coinbase_signature)
        .header("CB-ACCESS-TIMESTAMP", &time_stamp)
        .build()?;
        //manages the error I described above
        //let request = match request {
        //Ok(req) => req,
        //Err(e) => {
        //eprintln!("Failed to build request: \n{}", e);
        //return Err(e);
        //}
        //};

        let response = client.execute(request).await?;
        //let response = match response {
        //    Ok(resp) => resp,
        //    Err(e) => {
        //        eprintln!("Failed to execute request: \n{}", e);
        //        return Err(e);
        //    }
        //};


        let response_text = response.text().await?;

        //added 12/29/23
        //this is the parsing
        let v: Value = serde_json::from_str(&response_text)?;
        let mut coinbase_sell_price = 0.0;
        let mut coinbase_buy_price = 0.0;

        // Access the pricebooks array
        if let Some(pricebooks) = v["pricebooks"].as_array() {
            // Iterate over each pricebook
            for pricebook in pricebooks {
                // Access the product_id, bids, and asks
                let product_id = pricebook["product_id"].as_str().unwrap_or("");
                let bids = &pricebook["bids"][0];
                let asks = &pricebook["asks"][0];
        
                // Access the price and size of the bids and asks
                coinbase_sell_price = bids["price"].as_str().unwrap_or("price not found").parse::<f64>().unwrap_or(-1.0);
                let bid_size = bids["size"].as_str().unwrap_or("size not found");
                coinbase_buy_price = asks["price"].as_str().unwrap_or("ask price not found").parse::<f64>().unwrap_or(-1.0);
                let ask_size = asks["size"].as_str().unwrap_or("ask size not found");
        
                println!("Product ID: {}", product_id);
                //println!("Best bid: {} (size: {})", bid_price, bid_size);
                //println!("Best ask: {} (size: {})", ask_price, ask_size);
            }
        }

        //manages any errors from line above
        //let response_text = match response_text {
        //    Ok(t) => t,
        //    Err(e) => {
        //        eprintln!("Failed to read response text: \n{}", e);
        //        return;
        //    }
        //};

        //prints the actual response
        //println!("list accounts response\n{:?}", &response_text);























        //---KRAKEN--//

        //basically Kraken requires a value that is always increasing to be in each request.
        //I didnt use now.timestamp().to_string()  because just in case I have 2 
        //	requests in a second I dont want to be penalized.
        //if no "now" in scope when moving file, 
        //	the code is this:
        ////returns current time. MAY NEED TO USE LOCAL TIME
        //		let now = Utc::now();
        let nonce = now.timestamp_millis().to_string();
        let data = vec![
            ("nonce", &nonce),
            // Add more parameters as needed
        ];
        //let post_data: String = form_urlencoded::Serializer::new(String::new())
        //    .extend_pairs(data)
        //    .finish();
        
        let url_path = "/0/public/Ticker?pair=SOLUSD";
        //let message = format!("{}{}{}", url_path, nonce, post_data);

        fn sign_kraken(url_path: &str, nonce_str: &str, data: Vec<(&str, &String)>, secret: &str) 
        -> String {
            // Create the post data
            let post_data: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(data)
                .finish();
            //FOR DEBUGGING
            //println!("Private key:\n{}", secret);
            //println!("Nonce:\n{}", nonce_str);
            //println!("Encoded payload:\n{}", post_data);
            //println!("URI Path:\n{}", url_path);
        
            // Create the encoded string (nonce + post data) and hash it
            let encoded = format!("{}{}", nonce_str, post_data);
            let mut hasher = sha2::Sha256::new();
            hasher.update(encoded);
            let encoded_hash = hasher.finalize();
        
            // Create the message (url_path + encoded_hash as bytes)
            let mut message = url_path.as_bytes().to_vec();
            message.extend_from_slice(&encoded_hash);
        
            // Create a HMAC-SHA512 object with the base64-decoded secret
            let secret_decoded = base64::decode(secret).expect("Failed to decode secret");
            let mut mac = Hmac::<Sha512>::new_from_slice(&secret_decoded)
                .expect("HMAC can take key of any size");
        
            // Compute the HMAC of the message
            mac.update(&message);
            let result = mac.finalize();
        
            // Return the base64-encoded HMAC
            let signature = base64::encode(result.into_bytes());
            //println!("Kraken signature:\n{}", signature);
        
            signature
        }

        let kraken_signature = sign_kraken(&url_path, &nonce, data, &kraken_secret);

        //kraken asked for 3 headers: key, sign, and content type with its corresponding info
        //.body is nonce because in the Kraken code provided in cURL: 
        //https://docs.kraken.com/rest/#tag/Account-Data/operation/getAccountBalance
        //--data-urlencode "nonce=<YOUR-NONCE>"
        //		this means that nonce is added to the body of the request
        let kraken_basic_request = client.get("https://api.kraken.com/0/public/Ticker?pair=SOLUSD")
                .header("API-Key", kraken_api_key)
                .header("API-Sign", &kraken_signature)
                .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                .body(format!("nonce={}", nonce))
                .build()
                .expect("Failed to build kraken request");


        let kraken_response = client.execute(kraken_basic_request).await.expect("Failed to execute Kraken request");

        let kraken_response_text = kraken_response.text().await.expect("Failed to read response text");

        let v: Value = serde_json::from_str(&kraken_response_text)?;
        let mut kraken_buy_price_ask = 0.0;
        let mut kraken_sell_price_bid = 0.0;
        if let Some(solusd) = v["result"]["SOLUSD"].as_object() {
            // Access the ask and bid prices
            kraken_buy_price_ask = solusd["a"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
            kraken_sell_price_bid = solusd["b"][0].as_str().unwrap_or("").parse::<f64>().unwrap_or(0.0);
        
            //println!("Ask price: {}", kraken_buy_price_ask);
            //println!("Bid price: {}", kraken_sell_price_bid );
        }
        else {
            println!("didnt parse kraken correctly.");
        }

        //println!("response:\n{:?}", kraken_response_text);
        //coinbase calculations - to buy
            //let coinbase_taker_fee = 0.008;

            //let total_spent = 0.01*(*coinbase_wallet);
            //let fee_for_purchase = total_spent*coinbase_taker_fee;
            //let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            //new state of coinbase wallet below
            //*coinbase_wallet -= total_spent;
            //let amount_of_sol = money_going_to_sol_after_fees/coinbase_buy_price;

        //kraken calculations - buy
            let kraken_taker_fee = 0.0026;
            let fraction_of_wallet_im_using = 0.10;  //aka 10 percent
            let total_spent = fraction_of_wallet_im_using*(*kraken_wallet);
            let fee_for_purchase = total_spent*kraken_taker_fee;
            let money_going_to_sol_after_fees = total_spent - fee_for_purchase;
            *kraken_wallet -= money_going_to_sol_after_fees;
            let amount_of_sol = money_going_to_sol_after_fees/kraken_buy_price_ask;

            


        //coinbase calculations for sell

            let coinbase_taker_fee = 0.008;
            let money_from_sell_before_fees = amount_of_sol * coinbase_sell_price;
            let fee_for_sell = money_from_sell_before_fees * coinbase_taker_fee;
            let money_from_sell_after_fees = money_from_sell_before_fees - fee_for_sell;
            *coinbase_wallet += money_from_sell_after_fees;




        //this will count as value after
            let value_after = *kraken_wallet + *coinbase_wallet + gemini_wallet + bitstamp_wallet;
            println!("sol1_kraken_coinbase\tvalue after\n\t{}", value_after);

            return Ok(value_after)

     }







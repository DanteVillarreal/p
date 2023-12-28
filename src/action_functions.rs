use chrono::Utc;            //to get time
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

    pub async fn s_i1_sol_1_coinbase_kraken(value_prior: &f64, coinbase_wallet: &f64, kraken_wallet: &f64, bitstamp_wallet: &f64,
                                             gemini_wallet: &f64, coinbase_secret: &str, coinbase_api_key: &str, client: reqwest::Client ) -> f64 {
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
        let request_path = "api/v3/brokerage/best_bid_ask";
        let body = "";
        let message = format!("{}{}{}{}", &time_stamp, 
        &method, &request_path, &body);
        fn sign(message: &str, coinbase_secret: &str) -> String {
            let mut mac = HmacSha256::new_from_slice(&coinbase_secret.as_bytes())
                                                    .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            hex::encode(code_bytes)
        }
        let coinbase_signature = sign(&message, &coinbase_secret);

        let request = client.get("https://coinbase.com/api/v3/brokerage/accounts")
        .header("CB-ACCESS-KEY", &coinbase_api_key)
    	.header("CB-ACCESS-SIGN", &coinbase_signature)
    	.header("CB-ACCESS-TIMESTAMP", &time_stamp)
    	.build();
	//manages the error I described above
        let request = match request {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Failed to build request: \n{}", e);
                return;
            }
        };

        let response = client.execute(request).await;
        let response = match response {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("Failed to execute request: \n{}", e);
                return;
            }
        };


        let response_text = response.text().await;

        //manages any errors from line above
        let response_text = match response_text {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to read response text: \n{}", e);
                return;
            }
        };
    
        //prints the actual response
        println!("list accounts response\n{:?}", &response_text);
    }


pub mod action_functions {

    pub fn nothing() {
        //runs the action according to the index
        //i need to make this wait until the next state begins and then give it a reward?


    }
    //all my functions from this point on are going to be in this format:
    //pub fn [coin]_[percentage, floored]_[exchange to buy]_[exchange to sell] () {}
    pub fn eos_two_coinbase_kraken () -> f64 {
        //this will go to the [coinbase] api, get how much [2%] of my total usd is in it, buy [eos] from there
        //transfer it to [kraken] wallet api, and immediately sell all of it





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











}
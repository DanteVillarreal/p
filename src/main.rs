use rand::distributions::{Normal, Distribution};
use rand::Rng;
pub mod network;

//step 5
    fn generate_random_number () -> f64 {
        //generates random number between 0 and 100 including both 0 and 100
        rand::thread_rng().gen_range(0..=100);

        //generate random numbr between 0 and 100 excluding 100
        rand::thread_rng().gen_range(0..=00);
    }

        //generate random number with mean 0 and std dev of 1
        let distribution_variable = Normal::new(/*mean*/ , /* std dev */).unwrap();
        let random_number = distribution_variable.sample(&mut rand::thread_rng());

        //  ^
        //  |
        // unwrap()  is unsafe. 
        // maybe |
        //       |
        //       \/
        let distribution_variable_two_result = Normal::new(/*mean*/ , /* std dev*/ );
        let distribution_variable_two = match distribution_variable_two_result {
            Ok(Normal) => Normal;
            Err(error) => panic!("Problem creating the distribution variable: {:?}", error),
        };
        let random number = distribution_variable_two.sample(&mut rand::thread_rng());
        



fn main() {
    println!("Hello, world!");
}

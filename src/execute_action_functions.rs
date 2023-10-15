pub mod execute_action_functions {

    use crate::action_functions::{/*insert all the action functions */}
    use crate::network::NeuralNetwork;
    
    pub fn execute_function(epsilon: usize) {
        let action_index = neural_network.exploration_or_exploitation(&mut epsilon);

        // Create a vector of function pointers
        let actions: [fn(); 36] = [two_a_b, two_a_c, /* ... rest of your actions ... */];
        (actions[index_of_largest_qvalue])();
    }



}
pub mod execute_action_functions {
    /*/
    use crate::action_functions::{/*insert all the action functions */};
    //use crate::network::NeuralNetwork;
    use crate::network::network::NeuralNetwork;
    
    pub fn execute_function(epsilon: usize) {
        
        //this gives us the index of the q_value we will then use to determine which action_function to execute
        let action_index = neural_network.exploration_or_exploitation(&mut epsilon);

        // Create a vector of function pointers so we can then call
        let actions: [fn(); 36] = [two_a_b, two_a_c, /* ... rest of your actions ... */];


        //this then executes the action
        //basically:        actions[action_index] points to a function
        //(actions[aciton_index])();    calls the function that actions[action_index] points to
        //if:       action_index was 0, it would call the first function
        (actions[action_index])();
    }

    */

}
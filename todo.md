2. initialize neural network using this rule:
        There are many rule-of-thumb methods for determining the correct number of neurons to use in the hidden layers, such as the following:

        The number of hidden neurons should be between the size of the input layer and the size of the output layer.
        The number of hidden neurons should be 2/3 the size of the input layer, plus the size of the output layer.
        The number of hidden neurons should be less than twice the size of the input layer.

1.  Initialize neural network using rule-of-thumb methods.
1a. initialize input layer as 1 row
1b. initialzie bias layer as 1 column
2.  get parsed input from WebSocket APIs.
2a. normalize data. Then input data into layer1 of neural network.
{}3.  Connect rand crate to get random number functionality.
4.  Initialize network weights with random numbers that have mean 0 and standard deviation 1, then multiply each weight by sqrt(2/n).
5.  Initialize biases using values around 0.01.             ///////-----////           need to see if stochastic gradient descent also updates biases
{}6.  Implement feed-forward functionality: multiply current network layer by weight layer and add next layer’s biases.
//////////////6a. Introduce batch normalization after each layer. using z = (x-m)/s ?
{}7.  Implement Leaky ReLU activation function for each layer except for output layer.
8.  Implement Softmax activation function for output layer.
9.  Implement functionality to calculate Q-values from network output.
10. Implement epsilon-greedy strategy for exploration vs exploitation.
11. Define reward function for arbitrage task (consider using scaled profit-based reward).
12. Implement functionality to calculate target Q-values using Bellman Optimality Equation: target_q_value = reward + gamma * next_q_value.
13. Update network weights and biases using stochastic gradient descent to minimize (current_q_value - target_q_value)^2.
14. Establish decay rate of epsilon.
15. Figure out how to connect all DQN outputs to arbitrage actions.
16. add regularization techniques when establishing loss before doing the stochastic gradient Use L2 regularization: loss = (current_q_value - target_q_value)^2 + λ * sum(weights[i]^2)
16a.sum(weights[i]^2) is the sum of ALL my weights in the ENTIRE network squared.
16.5Figure out how to add experience replay. 
16.6Figure out how to give the network a batch at one time, if I should even do that.
17. Add hyperparameter tuning.
18. Training: add random number as input for trading fee schedule.
18.5Figure out how to get more than 1 output from DQN. aka buy  AND  sell. not just buy
18.6Think I figured it out: my output layer will consist of 37 neurons. I want a (nada) neuron, then I want 36 neurons each with different information:
        Buy form A, B, or C                                     3   
        Discrete percentages floored: 2, 3, 4, 5, 6, 7         *6
        Sell to: if chose A, then B or C                        *2 (i think, since you can't sell to same place)
                 if chose B, then A or C                    =  36
                 if chose C, then A or B
19. Connect DQN output to functions that buy and sell on other APIs.
20. implement ability to save a state of the neural network and load it



//////-----------------THINGS I NEED TO ADD--------------------///////
Experience Replay: DQNs typically use a technique called experience replay, where past experiences are stored and then a batch of experiences is sampled to train the network. This helps to break the correlation between experiences and stabilize training.

Target Network: In addition to the main DQN, a target network (which is a copy of the main network) is often used to calculate the target Q-values during learning. The weights of the target network are updated less frequently than those of the main network, which can help stabilize learning.

Evaluation: It’s important to periodically evaluate your DQN on a separate validation environment to check its performance.

Stopping Criteria: Define a stopping criteria for when to stop training. This could be based on the performance on a validation environment or when the change in performance is no longer significant.

Saving and Loading Models: Implement functionality to save and load your model. This allows you to stop and restart training, and also to keep the best performing models.

Logging and Monitoring: Implement logging of relevant metrics (like loss, rewards, etc.) during training. Visualizing these metrics can help understand how your DQN is learning over time.


//---------------------AFTER PROTOTYPE--------------------//

13. optimize matrix multiplication aspect of feed_forward with intel -xml or whatever it's called
14. standardize input data using historical data I have collected. 
15. optimize structs of network and bias layer by deleting row data
16. optimize matrix_add. I don't think the upper for loop is necessary. Also, need to remove the clone function. that's probably a few ms just on its own



/////----------------NEXT PROTOTYPE-----------------------//
15. maybe turn input layer to 2D layer to get batches. look this up.
15a. I think this would mean keeping network layers as 2D




//--------------------After AFTER PROTOTYPE ------------------//
15.     introduce functionality for arbitrage on the same platform itself. 
                so say someone wants to buy 1 btc at 20000
                and someone wants to sell 1 btc at 19900
                    I would:
                        I would step in and buy 2nd guy's btc for 1990
                        then sell it to the 1st guy for 20000
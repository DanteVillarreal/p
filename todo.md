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
{}4.  Initialize network weights with random numbers that have mean 0 and standard deviation 1, then multiply each weight by sqrt(2/n). n being number of neurons in previous layer
{}5.  Initialize biases using values around 0.01.             ///////-----////           need to see if stochastic gradient descent also updates biases
{}6.  Implement feed-forward functionality: multiply current network layer by weight layer and add next layer’s biases.
//////////////6a. Introduce batch normalization after each layer. using z = (x-m)/s ?
{}7.  Implement Leaky ReLU activation function for each layer except for output layer.
//////////////8.  Implement Softmax activation function for output layer.
{}9.  Implement functionality to calculate Q-values from network output.     ::::::::::The output of the networks are the q-values themselves
{}10. Implement epsilon-greedy strategy for exploration vs exploitation.
{}11. Define reward function for arbitrage task (consider using scaled profit-based reward).
{}11a.    I also want the losses to be counted exponentially. As a 50% increase in money is then worth way less
                if you just lost 50%, then if you had the OG 50% and got 50% more.
                option a:
                        start:                          100
                        gained 50% =                    150
                option b:
                        start:                          100
                        lost 50% =                      50
                        gained 50% after loss=          75
                option c:
                        start:                          100
                        lost 50% =                      50
                        gained 200% after loss =        150
                As you can see, I would have to gain 200% to get to where option a ended
                And 100% !!! just to get back to starting point

12. Implement functionality to calculate target Q-values using Bellman Optimality Equation: target_q_value = reward + gamma * next_q_value.
13. Update network weights and biases using stochastic gradient descent to minimize (current_q_value - target_q_value)^2.
{}14. Establish decay rate of epsilon.
15. Figure out how to connect all DQN outputs to arbitrage actions.
16. add regularization techniques when establishing loss before doing the stochastic gradient Use L2 regularization: loss = (current_q_value - target_q_value)^2 + λ * sum(weights[i]^2)
16a.sum(weights[i]^2) is the sum of ALL my weights in the ENTIRE network squared.
12/15/23 update: I wont actually add L2 regularization yet because it may actually lead to underfitting.
16.5Figure out how to add experience replay. implement strategy to choose which experiences get place in memory based on whether the agent lost money. if it lost, it's valuable
16.5aand if it didn't gain money, it might be valuable too. 
16.5b introduce "prioritized experience replay" as it can help
Understand Temporal-Difference (TD) Learning:

        Learn the basics of TD learning.
        Understand how TD error is calculated.
        Implement TD learning in your DQN.
        Implement Experience Replay:

        Set up a replay memory to store experiences.
        Sample experiences uniformly from the replay memory during training.
        Understand Prioritized Experience Replay (PER):

        Learn how PER modifies the sampling process to prioritize important experiences.
        Understand how the importance of an experience is measured (typically by the magnitude of its TD error).
        Implement Prioritized Experience Replay:

        Modify your replay memory to store the TD error of each experience.
        Adjust your sampling process to sample experiences based on their importance.
        Understand Stochastic Prioritization:

        Learn how stochastic prioritization maintains diversity in the sampled experiences.
        Implement Stochastic Prioritization:

        Modify your sampling process to use stochastic prioritization.
        Understand Importance Sampling:

        Learn how importance sampling is used to correct the bias introduced by prioritized sampling.
        Implement Importance Sampling:

        Adjust your learning update to use importance sampling weights.
        Test and Debug Your Implementation:

        Ensure that your DQN is learning effectively with the new modifications.
        Debug any issues that arise.

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



//////-----------------THINGS I NEED TO ADD Before I can begin--------------------///////
Experience Replay: DQNs typically use a technique called experience replay, where past experiences are stored and then a batch of experiences is sampled to train the network. This helps to break the correlation between experiences and stabilize training.

Target Network: In addition to the main DQN, a target network (which is a copy of the main network) is often used to calculate the target Q-values during learning. The weights of the target network are updated less frequently than those of the main network, which can help stabilize learning.

Evaluation: It’s important to periodically evaluate your DQN on a separate validation environment to check its performance.

Stopping Criteria: Define a stopping criteria for when to stop training. This could be based on the performance on a validation environment or when the change in performance is no longer significant.

Saving and Loading Models: Implement functionality to save and load your model. This allows you to stop and restart training, and also to keep the best performing models. Make it so that the loading of the model also executes request to obtain "previous" account balances.

Logging and Monitoring: Implement logging of relevant metrics (like loss, rewards, etc.) during training. Visualizing these metrics can help understand how your DQN is learning over time.

I need to add timers for shit so that I can then optimize it.

I need to add sandbox mode before I can even run it

Make it so after every sell order executes from the buy/sell pair, I also execute requests for all the account balances. This information will be fed into the reward function as the previous value. Then once the new trade executes and the account balances request execute, I will have the new account balances so that I can actually calculate the reward. After the reward is calculated, the new account balance will be stored as previous account balance.

Make sure that each input from the parsing program is at the last moment stored as f64. Why? I dont think I can multiply f64 by ints unless I do "as" f64. Also, change the string to zero for 1 thing and 1 for the other





//---------------------AFTER PROTOTYPE--------------------//

13. optimize matrix multiplication aspect of feed_forward with intel -xml or whatever it's called
14. standardize input data using historical data I have collected. 
15. optimize structs of network and bias layer by deleting row data
16. optimize matrix_add. I don't think the upper for loop is necessary. Also, need to remove the clone function. that's probably a few ms just on its own
17. optimize "Make it so after every sell order executes from the buy/sell pair, I also execute requests for all the account balances." so that I keep the information from the
        unchanged balances and only do 2 requests instead of 4.
18. Optimize gamma value
19. Add L2 regularization



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







Log of what I've done:
12/14/23 - Well that took quite a while. Im talking about the previous commit where it said Im going to actually do the functions themselves. In this commit I added a reward function that answers all of the things I wanted it to answer. I wanted the losses to be heavier. I wanted the losses to be scaled. This honestly is a super simple function but it was honestly much harder to come up with how it worked with the genius idea of the reciprocal than it seems. Next step is to get the Bellman Optimality Equation. I got this!
12/15/23 - This is an interlude commit. I just made a few small changes and I want to introduce that as a commit before I make a big change right after this. I got this!
12/15/23 - Made some changes to the exploration funciton to return an index as well. Then I added a new function to calculate the target_q_value. Next step is to do stochastic gradient descent. I got this!
12/15/23 - Added an update_weights function that calculates the temporal difference and of course updates the weights. It incorporates the stochastic gradient descent function that I have not made yet though. Next step is to make this. I got this!
12/15/23 - Added leaky_relu derivative, added loss function, added loss function derivative, added backpropagation, added update weights. Next step is to actually code comment it more. I got this!
12/16/23 - Added a lot of code comments and updated the backpropagation function itself. Next step is to really evaluate if its correct or not. After that is to figure out how to save and load the state or do experience replay. I got this!
12/17/23 - Added new versions of backpropagate and update_weights. The lower half of backpropagate didnt make sense before so Im going to see if this one does. I will code comment it and revise it after I eat. I got this!
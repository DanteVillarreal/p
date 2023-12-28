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

21. //added 12/27/23 -  need to make an environment that seamlessly connects with the neural network and the activation functions so that it can perform training.




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

Make it so there are functions built into 1 function so that it does the initialization, then it loops through the functions that do feed forward and backpropagation and do "x" number of loops and save it and load it and do exp. replay and what not.





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
12/17/23 - Still trying to figure out the backpropagate and update_weights fn. Might need to do it from scratch. I will watch 3B1Brown to gain a better understanding of how it should work. I will look at backpropagation but in different languages too to see how they do it. But their structs might be different so it may not look the same. Next step is to get backpropagate and update_weights fns working. I got this!
12/18/23 - COMMIT before initializationV2. This will function as an interude commit. I need to make some changes to it so that it does everything 100% correctly. I got this!
12/18/23 - I think Im overthinking it because Im remembering that this isnt your average matrix multiplication. It's actually done a little differently. Im going to leave it for now and just work on the backpropagation function and update_weights.
12/19/23 - THERE ARE SO MANY MOVING PARTS... but im the fucking best so I thank God that he gives me challanges that are worthy of me to undertake. This is just a commit to track what Ive changed. Dont think I changed anything in initialization fn. I do think I have to change it though. I think its initializing weights in the last layer, but it shouldnt be doing that. I also think its initializing biases in the input layer, but I dont think you need them there. Anyways, I will do that later because right now its not inhibiting anything and I want to focus on backpropagation and updateWeights functions. I just added a new backpropagation function. This one actually makes a good amount of sense. Next step is to actually code comment it all and then do update_weights IF it makes sense. If it doesnt make 100% sense, I wont move past it yet. I got this!
12/20/23 - I added new code comments to the code. This is an interlude commit because in the next one Im going to change up the code so I want to be able to go back to it. I got this!
12/20/23 - I think Im finally starting to crack at the backrpgopagation function. Next step is to properly exmaine it and see if I need an update_weights function and then wrok on that if I do. I got this!
12/20/23 - Updated backpropagation a little more. Im getting closer and closer to the correct backpropagation function. Next step is to finish examining the function and code commenting it. I got this!
12/20/23 - Updated backpropagation more and I think its done. Also added code comments in it. next step is to do the update_weights function. I got this!
12/20/23 - Added update_weights function. Updated name of backpropagation function. Next step is to evaluate the el_update_los_weights function. I got this!
12/21/23 - Overhalled backpropagation function because of some issues it would have in update_weights. Then looked at issues the new backpropagation function would have and fixed those. Next step is to add the update_weights function. I got this!
12/21/23 - Added Update_weights and updated backpropagation again. There was an issue with how it connected to update_weights so I think I fixed that. Next step is to look over backpropagation and update_weights and see if there are any issues. If no issues. Next step is to add save/load states. I got this!
12/21/23 - Updated update_weights and added new code comments to signify which functions are unused. Next step is to look over backpropagation and update_weights and see if there are any issues. If no issues. Next step is to add save/load states. I got this!
12/22/23 - Started Save and Load functions. Will need to THOROUGHLY REVIEW them. Next step is to review them and add code comments. I got this
12/23/23 - Added Serde to Cargo.toml
12/23/23 - Created new save and load functions. Code commented them too. Will need to see if it works next. I got this!
12/23/23 - Added Utc::chrono in Cargo.toml
12/23/23 - Interlude commit. I made tiny modifications and code commented some action and execute functions. But Im about to change initialization function.
12/23/23 - changed calculate_target_q_value
12/23/23 - changed el_backpropagation and code commented out the other variants. Next step is to review the code and then do the save and load. Then print both. I also need to make a print function. I got this!
12/23/23 - removed old and added new print functions. code commented out all of main. added code in main to initialize, print, save, load, then print. Then I changed up the main.rs, lib.rs, network.rs and it finally worked. Made many many changes. Next step is to fix any errors. I got this!
12/24/23 - added new print function because old one produced errors. Next step is to see if it works. If it does, next step is to load it then print it. I got this!
12/25/23 - Load works. Added new structs for experience replay and also a method. Will need to fix errors and code comment it next. I got this!
12/26/23 - Added save and load for experience replay. Next step is to see if it works. Maybe before I do that I have to create all the methods for the REST API. I got this!
12/27/23 - Changed reward function. Added some shit in main. Did a lot of planning on what Im going to do next. Next step is to figure out how to give value_prior as a parameter for the functions at the same time as executing the functions based on the index.
12/28/23 - updated action_functions.rs and added updated main with 0th function. Next step is to parse the text from the function. But I need to know how it outputs. So next step is to make an environment o it can run, and then run it. 
12/28/23 - added reqwest/hmac/sha2.
12/28/23 - completely updated action function. Got information just for SOL. Got it to run! Thats what we are going to use. Next step is to parse the output. I got this!
12/28/23 - added hex/tokio/dotenv
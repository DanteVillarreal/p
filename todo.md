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
20. places to optimize:
     o   in my main, find a way to make it do less checks when checking if the indices have been updated.
     o   in bitstamp and gemini to maintain uptime, do a request to just get the price and amount instead of waiting like 5 minutes for gemini or bitstamp to finally get a trade. If I did I would have to add a mutex just in case it did happen to be that it updated from the websocket client at the same time the request was updating it.
     o  not entirely sure if I need all of the messages in action_functions. I should see which ones I dont need for the functions to continue working, and remove the messages I do not need so it loads less into RAM.
     o  figure out how to use no mutex. manual checks may be faster. i think i already fixed htis.
     o  figure out how to choose "better" experience replays vs just picking random ones, aka exp replays that had unexpected outcomes
     o  remove "updated" in main because I'm not using it.
     o  remove print methods where the impl is literally only the print
     o  change update_input so it isn't .await. async functions only optimal when they aren't cpu bound. so when the function's max speed is not dependent on your CPU. So, like any time we do a REST request and are WAITING for a response. or in websocket. so in network, remove async word from function. then correct resulting errors. probably over 100. then see if network breaks.
     o  change every .unwrap_or and .unwrap to a .expect(&format!("...{}", &var)) with a message of what went wrong and a variable that helps figure out what went wrong
     o  change every .expect that doesn't have a var to have one so we can have HELLA information when we get an error and aren't guessing what the issue is.
     o  change every delay_for in action_functions.rs to delay_until
21. change all things to be warning free obviously, but also to change all initializations to be not 0, but Option<var type> instead. Except for the neurons of course.
22.
        




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
12/29/23 - successfully parsed the input. Next step is to make it so it returns the f64. I got this! 
12/29/23 - successfully parsed the kraken input as well. Next step is to put these into the formulas. Then next step after is to return f64. I got this!
12/29/23 - added url/base64 for signature functions
12/29/23 - did the calculations for the first function. Also changed the return and added a return. Next step is to do the rest of the functions. Or maybe to see if it works? I got this!
12/30/23 - completed the rest of the coinbase->kraken functions. Next step is to do gemini or bitstamp, whichever can have coins sold. I got this!
12/30/23 - worked on coinbase->bitstamp. bitstamp is buggy right now. Next step is to fix that .I got this!
12/30/23 - added uuid for bitstamp nonce
12/31/23 - if I make btc-usd lowercase, it gives me a different error output. if I make it uppercase, it just prints all of the ticker. Not sure how to fix this. next step is to fix this. I got this!
12/31/23 - fixed it because God gave me the challenge to show me I can overcome anything. Glory to God. Next step is to parse its bitch ass. I got this!
12/31/23 - parsed the input then set it as return value. Next step is to do it to the rest of the coinbase->bitstamp functions. I got this!
01/01/24 - 12:08 AM OF COURSE I WAS WORKING, you mother fucker. What would I be celebrating???? The fact that I STILL dont have version one done!?! Exactly. Lets get this shit done! So I finished coinbase->bitstamp. Next step is to see if I can do coinbase->gemini, if not, then do gemini->coinbase
01/02/24 - got the gemini->coinbase to work. Next step is to parse the output. Then next step is to put it into the variable and change the buy and sell. I got this!
01/02/24 - parsed output. Next step is to put it into the equations and do the output. I got this!
01/02/24 - put the equations and did the output. Next step is to do it for the rest of the gemini->coinbase. I got this!
01/02/24 - did the rest of the gemini->coinbase. Next step is to do gemini-> kraken. I got this!
01/03/24 - did beginning of gemini->kraken. I think everything is parsed too. Next step is to do the rest of the gemini->kraken. I got this!
01/03/24 - finished all of gemini->kraken. Next step is to do gemini->bitstamp, then gemini->coinbase. I got this!
01/03/24 - did first of gemini->bitstamp. Next step is to do all of gemini->bitstamp. I got this!
01/04/24 - finished all of gemini->bitstamp. And realized I already did gemini->coinbase. Next step is to do kraken -> coinbase, and kraken->bitstamp. I got this!
01/03/24 - did first kraken->coinbase. Next step is to do all of kraken->coinbase and at the end add the waits for transactions so its like request, buy.      2.5 sec,        request, sell.
01/04/24 - finished kraken->coinbase. Next step is to do kraken->bitstamp and then do the waits like above. I got this!
01/04/24 - did first kraken->bitstamp. Next step is to finish all of kraken->bitstamp. I got this!
01/04/24 - finished kraken->bitstamp. Next step is to see which others I need to add. Then add the waits. Then add experience replay after we made a loop thing. I got this!
01/04/24 - Added the "wait" time to each function. removed a few warnings. Next step is to connect them and then look at the fees and add whatever other fees I need to add. Also may need to remove some functions if the minimums are too large. I got this!
01/05/24 - Bitstamp removed solana again. I have to change it to AVAX after this commit. This commit code-comments out many functions because of krakens minimum solana buy. this must be changed now because now im using avax. Thats 16 THOUSAND lines of code I must change. I AM THE ONE WHO KNOCKS. WATCH WHO THE FUCK YOU TALK TO BITCH. I got this!
01/05/24 - not a real commit. Just putting thoughts here. I know what Im going to do: I will do both solana and one other coin to start out. Deposit speeds: https://support.kraken.com/hc/en-us/articles/203325283-Cryptocurrency-deposit-processing-times    . Maybe avax, maybe xlm, maybe ftm. Well see. I got this!
01/05/24 - Code-commented functions due to minimums. I will go for XLM. Why? AVAX doesnt have reliable speeds. XRP keeps appearing and disappearing from Gemini, so is unreliable. FTM isnt on coinbase. XLM has consistent transaction speeds, and they are low and is on bitstamp, coinbase, kraken, not on gemini. not mad about that. I dont like Gemini, but its part of the game so ill leave it on for solana. As a result, I need to add XLM websocket client and parsing, then do rest API client for those. I got this!
01/06/24 - there is an issue in the feed-forward OR initialization. If I give the initialization 10, 10, 2, it produces 10 columns and 13 rows. I will need to fix that. I got this!
01/07/24 - fixed the error. Made new initialization function. Will need to code comment it later. Matrix multiplication is crazy fast though. Like not even a milisecond to do the matrix multiply (if its small).
01/07/24 - So the issue is that I now have two different websites for coinbase and for kraken and for bitstamp because of SOL and XLM. So now I have to update my code to parse for the differences. Thats the next step. I also have to change coinbase because it isnt exchange anymore, its advanced api.I got this!
01/08/24 - added/updated the new functions. Updated main to handle the new functions. Next step is to fix the indices starting from the 2nd coinbase. I got this!
01/08/24 - updated functions/indices. Next step is to fix the neural_network.update_inputs function so it isnt red. I got this!
01/08/24 - updated functions so there were no more errors. Next step is to code comment action_functions so I can run the program and see if it actually works. I got this!
01/08/24 - big updates. Updated all functions and removed errors. Had to revert to december 2023 rust analyzer because it had a compiler error that wasnt actually a compiler error. Next step is to see if it is actually changing the input nodes. After that, I need to create the rest of the xlm methods. After that I need to see if experience replay is actually working. I got this!
01/09/24 - I added a way to track if all the nodes have been updated. Then I did a print statement to show the updated neurons. Next step is to create the rest of the xlm methods WITH the minimum withdraw/buy/deposite already in so I dont have to remove every function. I got this!
01/09/24 - added first xlm function in action_functions. Next step is to test it and see if it works. After that is to do the rest of all of them. I got this!
01/09/24 - finished all of xlm functions in action_functions. Next step is to check experience replay. I got this!
01/09/24 - interim commit that will just save what I have so far. I am going to change some structs and yes, add a mutex so I need to make sure I have a good save. Next step is to add the mutex. After that is to check if it works. I got this!
01/10/24 - added mutex in update_input and feed_forward. Next step is to make it check experience replay. I got this!
01/10/24 - changed networklayer struct. made new feed_forward to use cloned input layer. Added all action_functions in cycle function. Now I need to fix it. the answer is in bing.com. I got this!
01/10/24 - fixed the errors in cycle function. Next step is to finish developing the function. I got this!
01/10/24 - changed calculate_target_q_value to use feed_forward with cloned input layer
01/10/24 - updated first action_functions so they update input. Next step is to update all of them. After that is to finish cycle fn. I got this!
01/10/24 - updated all of the action_functions so they update input. Next step is to finish cycle fn. I got this!
01/10/24 - fixed some errors that happened as a result. I had a stray "mod network" in main that fucked everything in my network.rs. I couldnt import anything. Then I fixed a mutex issue that appeared when I cleared some other errors. Next step is to finish cycle fn. I got this!
01/11/24 - finished cycle fn except for exp replay part. Next step is get experience replay working. I got this!
01/11/24 - I think I finished exp replay. Next step is to put the cycle fn in main and get everything together to run it. After that is to  make it sample from it at random after its like 1000 big. I got this!
01/11/24 - Moved read_lines part of websocket client thing and then added cycle fn to main. Need to remove errors next. Then run it. I got this!
01/12/24 - removed mutexes from update_input and cycle. Added them before. removed mutex from struct. made functions in main async, changed return type of action_functions. Did a lot of shit because there were so many errors. Next step is to add the wait so that it doesnt do a cycle until it has updated each input neuron first. I got this!
01/13/24 - reverted to previous commit, but got out-of-bound error in back-propagation function. Then got error where it doesnt seem to be joining the cycle and read_lines task. Next step is to fix these. I got this!
01/14/24 - fixed out-of-bounds error with new backpropagation function. Now I have to fix the error of not joining the 2 tasks as described above. I got this!
01/14/24 - I think I figured out where my error is. I added a print statement after the neural_network.cycle function and it never was completed. The issue thus HAS to be in the cycle function. Some part of it, at least. Next step is to add print statements after everything and figure out where it is. I got this!
01/16/24 - Issue was the lack of dropping the lock. Fixed. print statements show it wasnt getting stuck anywhere in the cycle function. So I thought the issue had to be how the for loop works. However, I looked at what it connects to and it connects with the read_lines_task. These share the neural_network. so if its locked in one, it wont be able to be accessed in the other. Adding delay_for is BREAKING the code in the for loop. I dont know why because delay_for is in the action_functions as well. Maybe has something to do with locking and unlocking again.
01/17/24 - FIXED THE ISSUE. What happened was I was prematurlely locking the neural network WAYYYYYY before the neural network was being changed. So I deleted 1 line of code and it half worked. Then even with the delay in the for loop I noticed that it wasnt showing the websocket client was reading shit, so all glory to God, because I was thinking about locking and unlocking, I realized that my new delay I added was happening WHILE my neural network was locked. So, I changed its scope and BAM it finally worked. The weights dont seem to be updating with back propagation. Next step is to double check if they are, and if they arent, then modify it. I got this!
01/18/24 - figured out where the issue is. It has to Its in my back propagation funciton, most likely. I got it to update more weights, but I still havent gotten every weight updated in the first layer. Next step is to fix the back propagation funciton. I got this!
01/19/24 - redid backpropagation and update_weights functions. Fixed it. Next step is to figure out why the last inputs are not updating. I got this!
01/20/24 - fixed inputs not updating. Next step is to save neural network and then print rewards to a new file to see if its working and then run it 10000 times. I got this!
01/20/24 - added save_reward and sample from replay buffer functions. Next step is to integrate them into my program. I got this!
01/20/24 - changed NeuralNetwork struct. I realized that Ive been calculating the target_q_value incorrectly because I wasnt getting a new input layer. I need to redo it so it will do: feed forward, action and q_value, but then it will exit of new cycle function, then unlock neural_network lock. Wait 1 second, then lock it, save new input layer as next_state, then do calculate target q value, then perform back propagation hooking it up to old chosen_index and old q_value. Then update the weights. I got this!
01/21/24 - NEED TO REMOVE SOLANA FROM BITSTAMP. Will add XRP to all: gemini, bitstamp, kraken, coinbase.
01/21/24 - Separated cycle function so it can properly update weights. Next step is to introduce the i%10 part in the for loop. Then after that is to run it and see if its doing everything correctly. Then finally, next step is to do Remove solana-bitstamp functions. Then add xrp functions to all. I got this!
01/21/24 - added the i%10 part. Next step is to run it and see if its doing everything correctly. Then finally, next step is to do Remove solana-bitstamp functions. Then add xrp functions to all. I got this!
01/22/24 - in the process of fixing the modifications in main. Next step is to run it and see if its doing everything correctly. Then finally, next step is to do Remove solana-bitstamp functions. Then add xrp functions to all. I got this!
01/23/24 - in process of fixing modifications. Next step is to run it and see if its doing everything correctly. Then finally, next step is to do Remove solana-bitstamp functions. Then add xrp functions to all. I got this!
01/24/24 - finished with modifications. Next step is to run it and see if its doing everything correctly. Then finally, next step is to do Remove solana-bitstamp functions. Then add xrp functions to all. I got this!
01/24/24 - heavy modifications in main and network and added more error finding code. Neural network crashes at 4th iteration. I think the issue is my present replay buffer is fucked so I need to delete all of them. However, I still have another issue of my gradients exploding. I will try to do z-score normalization (standardization) after this commit.
01/24/24 - modified every occurence of update_input to do log transformation. instead of z-score normalization I am doing log transofmraiton because I dont know mean or std. dev for ANY of my data. Also, log transformation seems easier to implement. I dont want to add a max to my gradient becuase it seems like a crude fix for xploding weights given that I already have the learning rate at something tiny like 0.0001.
01/24/24 - massive modifications because changed to constant transformation because cant do log of a negative number. next step is to do Remove solana-bitstamp functions. Remove solana bitstamp from websocket client. Then add xrp functions to websocket client, action_functions.rs, in network.rs: choose_action_functions, in main.rs: all the new function headers in main without updated, then add all the new functions. Lowkey, might just move all the main functions, aside from read_lines to execute_action_functions so it looks cleaner. I got this!
01/25/24 - changed the websocket client. Then made changes in main and modified read_lines, then revamped all of "handle" functions and put them in execute_action_functions.rs so it can be more modular. Next step is to make the action_functions and change the choose_action_function in network.rs, then change initialization in main.rs to account for total indexes in output. I got this!
01/26/24 - added all of the xrp functions in action_functions.rs with the knew kraken fee. Then I added the kraken fee for xlm. Next step is to change the choose_action_function in network.rs, then change initialization in main.rs to account for total indexes in output. I got this!
01/27/24 - appended xrp functions to end of choose_action_functions and adjusted initialization in main.rs to account for total index in output. Next step is to run it. I got this!
01/27/24 - made final changes. This is the last commit before the first one. Weve done great so far, but jobs not over mother fucker. You aint done yet you fat piece of lard. After the 100,000 iterations, next step is to evaluate the rewards and see if its consistently over 0. If not, we need to do it again but introduce gradient clipping. I got this!
01/28/24 - made some new changes to epsilon, exploit_or_explore and save reward to further evaluate effectiveness of neural network. I got this!
01/28/24 - made further changes to exploit_or_explore function. I got this!
01/28/24 - added gradient cap and increased because its still exploding. Changed time to 15 minutes too. I got this!
01/28/24 - for panic=>file functionality. Next step is to run it. I got this!
01/29/24 - made gradient cap smaller and then added "if i > 200" to start doing exp replay. If q value still diverges, I will decrease learning rate by factor of 10. if still diverges, I will look into more methods and try those. I got this!
01/30/24 - tiny tiny commit. beginning the addition of the loop in i177_xrp_7. Next step is to finish it there and do it to all the other functions as well, then run it. I got this!
02/02/24 - changed all xrp_kraken_coinbase. I added code that protects against errors caused by corruption of some value. Next step is to do it for the rest. I got this!
02/03/24 - added conditional for reward so I can see if it ever makes money. Next step is to finish the loops for all the other functions as well. I got this!
02/03/24 - gamma=0.8;learning_rate=0.00001;EpsilonDecay=0.000001;gradientCap=0.1;AccessReplayAfter=400; Running to see if this will stop exploding q values. I got this!
02/04/24 - NOT REAL COMMIT. NEED to figure out why THE FUCK my q values are still exploding. This may take a while. I got this!
02/05/24 - NOT REAL COMMIT. I think I am going to do a Double DQN. I need to research it a little and see if this is the correct result. I want this to be the last option, once all my other options have been exhausted. I may talk to my professor about it tomorrow to see what he thinks. I got this!
02/06/24 - talked to my professor. he didnt know much about DQNs. He talked about how maybe the normalization of the inputs is the issue, but the distribution he recommended is the pareto distribution but then said this would only work if the coin was a rug pull because all the coins go to a small number of people. He got me thinking about my activation function how maybe that could play a role. I need to think about that more. Maybe the lack of exploitation is the issue. I will actually create a new exploration or exploitation function that does not worry about epsilon but instead makes its own variation of epsilon. I will try this for 20_000 iterations to see if the values are converging. If it doesnt work, "OH NO WAH WAH WAH. time to cry". PSYCH!, I will reduce the gradient cap by half. I got this!                            I AM GOING TO RUN IT FROM 0
02/07/24 - got it running to 1400 iterations just fine. I am adding iteration number to the reward to see what iteration I finally see a positive number at. I will start the next run from 1445 and load the 2nd or so last state and remove the last replay_buffer just in case. I got this!
02/07/24 - got panic at 2 AM apparently, but main didnt panic until much later. Inputs did not not seem to be updated for idk how long though. I fixed error that caused OG panic so it will log in the log file but it wont panic. the panic is not entirely necessary, but I do want to know when the errors occured. I also added better error handling in handle_all_gemini fn in execute_action_functions.rs. Next step is to run it and see what happens. I got this!                             I AM STARTING FROM 0.   I WILL REMOVE LAST FEW THOUSAND REPLAY BUFFERS 
02/07/24 - fixed infinite loop, lol.
02/07/24 - added new loop for xrp gemini_kraken. Next step is to run starting from 0. Already removed the replay buffers. I got this!
02/07/24 - retard alert! I for some reason re-initialized value_after after I initialized it so I got an error pretty fast. I have removed it from the xrp gemini kraken action_functions.              I AM STARTING network FROM 0. I WILL REMOVE REPLAY BUFFERS IF ANY. I got this!
02/08/24 - added more info on print statements saying updating input. I got this!
02/09/24 - NOT A COMMIT. restarted run and removed ALL replay buffers because I added new log errors that were once print ln and I think I got a lot of previous panics of some urls that didnt cause entire program to crash JUST the TASK SPAWN. so I am restarting the run from 0 and removing ALL replay buffers. starting from 0.     I got this!
02/09/24 - UPDATED TOKIO commit. Removed deprecated functions in main.          Starting from 0. removing ALL replay buffers. I got this!
02/09/24 - UPDATED REQWEST commit. Starting from 0. removing ALL replay buffers. I got this!
02/10/24 - NOT A COMMIT. restarting run and removing all replay buffers. I changed how re-authentication works in websocket client. God is good. I got this!
02/13/24 - updated websocket client. now updating main program. Update: changed time to update inputs for target q value from 1 second to 50 milliseconds. so 0.05 seconds. Then I added more time after calculated target q value to update inputs again. I did 200 milliseconds. Why? Becasue I want state to target q values to be as directly after the current state as possible. I am doing 50 milliseconds and not 0 because I want the inputs that have yet to be updated while action_function is going on to be updated, but thats it. I checked how long the lock takes and to lock it takes 30 microseconds. so only 0.03 milliseconds. so I think 50 milliseconds should be fine. Next step: I need to figure out a way to pause my entire neural network while my websocket client is  sleeping.      RESTARTING FROM 0 BECAUSE TARGET Q VALUE TIME DIFFERENCE.         GOD IS GOOD. I got this!
02/14/24 - HUGE CHANGES. NOT DONE YET. THIS CODE DOES NOT COMPILE. DO NOT RUN. I am in the middle of updating it so that I can loop through my websocket client if it panics and have it restart.       GOD IS GOOD. i got this!
02/15/24 - I think I got it. 15 minutes later lol. I was about to give up for today, when I felt something push me to do one more line of code. Then I finished the program WITHOUT any errors. Next step is to run it.         GOD IS GOOD. I got this!
02/15/24 - Restarting from 0. Q values diverged. I checked if 50 ms was enough for all inputs to come in. it seemed like it was because based on how long the wait was for the sell, the size of the text that happened after the 50 ms changed. I will run it again the same way. If it diverges again, I will change the the epsilon decay. I will change the rate of exploration and exploitation. I think I may have been too quick to make it exploit.     GOD IS GOOD. JESUS IS GOOD. I got this!
02/15/24 - it diverged rather quickly. Within 1000 iterations it was at 10^30. I think the issue is the value for gamma. Before, with 1 sec time interavals, I had it set at 0.8. Now, my intervals are 0.05 sec and 0.2 sec. aka 50 and 200 ms. I think the new gamma values have to reflect this change. And the answer is to decrease gamma. It isnt increase because the original decrease to gamma is partly what allowed my original DQN to go from divergence to convergence. That being said, I will decrease gamma from 0.8 to 0.7. If this does not work, I will keep decreasing it until I get to 0.1 or 0.05. Thats the limit Im setting, before I say "Ok, something ELSE is the issue here." The reason I dont think giving it more exploration is the solution is because at 1300 iteration, where exploitation happens once every 10 times, it was ALREADY diverging by an incredible margin. I dont think changing exploration_or_exploitation is the crux of the issue. The issue is gamma.           JESUS IS GOOD. I got this!
02/15/24 - NOT A COMMIT. at iteration 582, a q value reached over 1300. I realized my replay buffers were still present since last run. I will restart from 0 with REMOVED replay buffers.            JESUS IS GOOD.  I got this! Gamma equals 0.7
02/15/24 - NOT A COMMIT. at iteration 896, got q value at 10^17. Decreasing gamma by 0.1 and restarting from 0.                 JESUS IS GOOD. I got this! Gamma equals 0.6.
02/16/24 - NOT A COMMIT. 1048 iterations in at gamma of 0.6 and q values are between  +-48. at 3165, I saw 10^20
02/16/24 - NOT A COMMIT. I am changing gamma to 0.5.            JESUS IS GOOD. I got ths!
02/16/24 - NOT A COMMIT. Update. at the gamma of 0.5, at 5882 iterations, I didnt see any number below -32 and above 0. learning seems to be progressing maybe. I dont think it has truly grasped the "value" of each input yet. Im looking and I think Im going to decrease its rate of exploitation after it hits 6200. Right now its set to exploit on any number divisible by 2 or 3. I think Im going to change it to just be 2. 
02/16/24 - changed exploration_or_exploitation to be more exploration. Changed gamma to 0.5, at 6015 iterations, it doesnt seem to "grasp" the "value" of the inputs. Going to remove last 10 replay buffers and going to make it start by removing last 10 iterations and then make new p and websocket_client log_panics. Stopped at 6043 iterations at 1708120478132          JESUS IS GOOD. I got this!
02/16/24 - changed save_reward to include index chosen for when it prints the reward so I can actually see if its learning. if it is actually learning, it wont just pick the same index for exploit every time. Commit happened at iteration 6775. I will remove last 10 replay buffers and last 10 iterations, make new p and websocket_client log_panics.            JESUS IS GOOD. I got this!
02/16/24 - NOT A COMMIT. just an update. iteration 7529: q values seem to be getting closer to 0 which is good because the difference between rewards isnt the difference between -33 and 33. its more like -2 and 2.           JESUS IS GOOD. I got this! iteration 9653: q values +- 29.2 Update: iteration 10644: +-17.
02/17/24 - NOT A COMMIT. just an update. iteration: 11509: +- 27 :O
02/17/24 - changed epsilon decay in exploration_or_exploitation. made it slower. I think I want convergence to happen slower than I originally thought.         REMOVING LAST 10 REPLAY_BUFFERS and iterations. REMOVING FIRST 10_000 save states. Restarting p and websocket_client log panics. I was at 22360 and +-24.
02/17/24 - NOT A COMMIT. forgot to LOAD the previous neural network. \
Starting back from 22360, and 1708221167515
removing all new replay buffers
making new websocket_client and p log_panic
02/19/24 - q values diverged with new epsilon decay, lol. Thats fine though. I think the bigger issue may actually be the gamma. I decreased it from 0.5 to 0.4. Of course Im removing every replay buffer and restarting main.rs from 0 and not loading it.                JESUS IS GOOD.  I got this!
02/19/24 - NOT A COMMIT. just an update. gamma = 0.4, learning_rate = 0.00001; gradient cap = 0.1. iteration: 11385, +-13
02/20/24 - NOT A COMMIT. just an update. gamma = 0.4, learning_rate = 0.00001; gradient cap = 0.1. iteration: 16356, +- 35
02/20/24 - restarting from: gamma = 0.4, learning_rate = 0.00001; gradient cap = 0.1. iteration: 22,000. For some reason, my neural network glitched out after this save state. It may have been my replay buffers. I will run it with no replay buffer. p and websocket_client log panics restarted. Starting from save_state: 1708461652030 
02/21/24 - error occured. i am in the process of fixing it. i should have some new code in a few hours.         JESUS IS GOOD. I got this!
02/23/24 - HUGE CHANGES. insane amount of error fixing. had to change it to cell which caused all these other things which I then had to correct and change to std sync mutex and then this caused all these other issues so then I had to change it to arc mutex and then that called all these issues and then I had to add arc clone and then that caused issues, and then I had to clone the iterators and then put them back. and then finally after hours, by the grace of God, Jesus, almighty, I fixed all the errors.          JESUS IS GOOD. I got this!
02/23/24 - Restarting everything from 0. For some reason The previous time it crashed, it said it couldnt find the websocket file, so when I went to do "cargo build" on the websocket file it still said it couldnt find it, so I went to update it and then I restarted my comp and it fixed it. So 1. removing all replay buffers. 2. making new p and websocket log_panics. removing 30 gigs of saved states. 4. total_counter already changed to 0.                JESUS IS GOOD. I got this!
02/27/24 - Had an error occur a few days ago and the program crashed. I didnt get to it until now. I had to make the loops for xrp 143 to 150 so that if an error ever occurs there, it will easily be dealt with.      1. I am removing last 10 replay buffers.        2. I am removing last 10 saved states.  3. I am restartin from state 1708845504745       4. I am making new p and websocket log_panics.  5. removing 30 gigs of saved states.    6. total_counter changed to 26698       7. uncommenting code for continuation and commenting code for restarting from 0.               JESUS IS GOOD.  I got this!
02/28/24 - immediate error 4 iterations in. I have added new error checking if the reward is ever nan so that should help me identify the culprit. 1. I am removing any new replay buffers. 2. I am removing any new saved states. 3. i am restarting from same save state. 4. i am making new p nad websocket log_panics. 5. keeping total_counter to 26698.           JESUS IS GOOD. I got this!
02/28/24 - immediate error. this time we have more info that value_after value was NaN. it was also on index 11 again. so I added log::info statements on it. 1. made new p and websocket client log_panics. no new saved states or replay buffers were created.                JESUS IS GOOD. I got this!
02/28/24 - i added an error protecting loop to the action_functions i_113 to i_120. The issue was in 117 but I extended the protection to all of the same functions. Lets see if this stops the issue. 1. made new p and websocket client log_panics. no new replay buffers or saved states were created. Starting on same save state.          JESUS IS GOOD. I got this!
02/28/24 - changed all https://coinbase.com to https://api.coinbase.com because for some reason they changed it and didnt tell anyone. Luckily someone in the forums talked about it. 1. made new p and websocket client log_panics. no new replay buffers or saved states were created.             JESUS IS GOOD. I got this!
02/28/24 - q values started exploding suddenly. My only guess is that it has to do with the replay buffer. So we got up to 26698. Now lets surpass it.  1. removing all replay buffers. 2. code commenting code to start at a save state and uncommenting code to start from 0. 3. making new p and websocket log_panics. 4. change total counter to 0.           JESUS IS GOOD. I got this!
02/28/24 - q values exploded within 1500. not changing any of my code. I think there had to be an error somewhere. 1. Removing all replay buffers. 2. making new p and websocket log_panics. 3. restarting from 0. I will monitor the status closely.                What to change next: gamma from 0.4 to 0.3. dont begin replay buffers until 200. Maybe also add a log::info! for every iteration of the neural network.   JESUS IS GOOD. I got this!      
02/28/24 - NOT A COMMIT. just thinking. I think the issue might be that the gradient cap is actually too high. I think the issue is that its moving too quickly for it to converge quickly. I think if I decrease gradient cap from 0.1 to 0.001. Yeah, thats what Im going to do. The issue may be that there are just SO many weights, that if a minority of them happen to just be pushed slightly higher, that could mess up the whole network. The issue may be that the high gradient cap is allowing my network to jump all over the place instead of slowly converging. 
02/28/24 - changing gradient cap from 0.1 to 0.001. 1. removing all replay buffers. 2. making new p and websocket_client log panics. gamma still at 0.4. replay buffers still starting where they were last time. it was actually starting at 400, not 100 how I originally thought.            JESUS IS GOOD. I got this!
02/28/24 - added save_last_network_layer fn so it will save q values to a txt document after each iteration so I have a better understanding of how my network is evolving.             JESUS IS GOOD. I got this!
02/28/24 - shrinking gradient cap from 0.001 to 0.00001. 1. removing all replay buffers. 2. making new p and websocket_client and last_network_layer log files. gamma still at 0.4.  3. adding extra space in rewards doc for new run.          JESUS IS GOOD. I got this!
02/29/24 - got to about 2000 when a value went to -160 and then within 2000 more values exploded. shrunk gradient cap to 0.000_0001. 1. removing all replay buffers. 2. making new p and websocket_client and last_network_layer log files. gamma still set at 0.4. 3. adding extra space in rewards doc for new run.		JESUS IS GOOD. I got this!
02/29/24 - changing gamma to 0.3. 1. removing all replay buffers. making new p, websocket, last_network layer log files. 3. adding extra space in rewards doc for new run.			JESUS IS GOOD. With his help, I got this!
02/29/24 - NOT A COMMIT. at 700, gradients already at millions. changing gamma to 0.2. keeping gradient cap to 0.000_0001. 1. removing all replay buffers. 2. making new p, websocket client and last_netwokrk layerlog files. 3. adding extra space in rewards doc for new run. The more tests, the better my code gets.		JESUS IS GOOD. With his help, I got this!
02/29/24 - changed gradient cap back to 0.1. Testing it with gamma at 0.2. . if it goes wrong, I will divide learning rate by 10, then by 100. 1. removing replay buffers. 2. making new p and websocket_client and last_network_layer log files. 3. adding extra space in rewards doc for new run.
02/29/24 - decreasing learning rate by 10. made it to iteration 760, then at 761, it jumped from 0.946 to 171. idek how thats possible. I am decreasing learning rate by a factor of 10. from: 0.00001; to 0.000001. All values: gamma = 0.2, gradient cap = +-0.1. learning rate = 0.000_001.	1. making new replay buffer. 2. removing p, websocket_client, and last_network_layer log files. 3. adding extra space in rewards doc for new run.
03/02/24 - changed let mut success = true to fasle. also, theres some concerning q values like i saw one go to -500. its not that at current iteration of 21282, but still.
03/03/24 - added a bunch of error protection loops in action_functions. I may need to separate action_functions into like 3 or 4. it was actually lagging while I was trying to edit it. THIS COMMIT IS INCOMPLETE. ALLOW JESUS CHRIST TO WORK THROUGH ME so that I can finish it later.                JESUS IS GOOD. I got this!
03/05/24 - added a bunch more error protection loops in action_functions. I am NOT DONE YET. THIS IS INCOMPLETE. Next steps: 1. start from beginning and make sure all kraken, beginning OR ENDING do NOT have .ok() in them. 2. finish up on i163 after done with step 1.              JESUS WILLING, I will finish this tomorrow OR today, technically.               JESUS IS GOOD. I got this!
03/05/24 - finished error protection loops in action_functions. Next step is to double check everything.                JESUS IS GOOD. I got this!
03/05/24 - double checked everything. starting from 0.  gamma = 0.2. learning_rate = 0.000_001; gradient_cap = +-0.1.     1. removign all replay buffers. 2. making new p, websocket client and last_network layer log files. 3. adding extra space in rewards doc for new run.         JESUS WILLING... THIS IS IT. We got this!
03/05/24 - deleted some stuff in action_functions.rs. Guess I should have triple checked it. restarting from 0. gamma = 0.2. learning_rate = 0.000_001; gradient_cap = +-0.1.     1. removign all replay buffers. 2. making new p, websocket client and last_network layer log files. 3. adding extra space in rewards doc for new run.         JESUS IS GOOD. Whats the next problem? We got this!
03/05/24 - at iteration, 575, I already got q values with values in the millions.starting from 0.  gamma = 0.4. learning_rate = 0.000_001; gradient_cap = +-0.1.     1. removign all replay buffers. 2. making new p, websocket client and last_network layer log files. 3. adding extra space in rewards doc for new run.         JESUS IS GOOD. We got this!
03/06/24 - changed kraken_wallet -= money_going_to_ _after_fees to -= total_spent; starting from 0.  gamma = 0.4. learning_rate = 0.000_001; gradient_cap = +-0.1.     1. removign all replay buffers. 2. making new p, websocket client and last_network layer log files. 3. adding extra space in rewards doc for new run.         JESUS IS GOOD. We got this!
03/06/24 - added average to save_last_network_layer fn. starting from 0.  gamma = 0.5. learning_rate = 0.000_001; gradient_cap = +-0.1.     1. removign all replay buffers. 2. making new p, websocket client and last_network layer log files. 3. adding extra space in rewards doc for new run.         JESUS IS GOOD. We got this!
03/06/24 - average was at 2962 at iteration 491. I will increase gamma to 0.6. starting from 0.  gamma = 0.6. learning_rate = 0.000_001; gradient_cap = +-0.1.     1. removign all replay buffers. 2. making new p, websocket client and last_network layer log files. 3. adding extra space in rewards doc for new run.         JESUS IS GOOD. We got this!
03/06/24 - average was at 5953 at iteration 491. I will increase gamma to 0.7. starting from 0.  gamma = 0.7. learning_rate = 0.000_001; gradient_cap = +-0.1.     1. removign all replay buffers. 2. making new p, websocket client and last_network layer log files. 3. adding extra space in rewards doc for new run.         JESUS IS GOOD. We got this!
03/06/24 - added mroe robustness for geminis parsing.           JESUS IS GOOD. We got this!
MAY NEED TO STANDARDIZE IT USING STD DEV AND MIN MAX USING HISTORICAL DATA.
03/06/24 - average was at 56663 at iteration 491. 6628 at 490. Now. I will increase gamma to 0.8. starting from 0. gamma = 0.8. learning_rate = 0.000_001; gradient_cap = +-0.1.     1. removign all replay buffers. 2. making new p, websocket client and last_network layer log files. 3. adding extra space in rewards doc for new run.         JESUS IS GOOD. We got this!
03/08/24 - I was working diligently over the last couple of days figuring out this issue of standardization. I downloaded all the data. Realized, it wasnt the data I needed, then downloaded other data, realized it was the same data after a few hours, then finally downloaded the correct data: historical OHLCVT data. Then the next step was to analyze it. I put all the data in excel, then I chose graphs that matched the distribution, so most of the time I chose pareto. A few hours after I finished all the graphs, I noticed, that for the pareto distribution graph, the numbers are not sorted. The bins are instead sorted based on number of units per bin. So I had to completely restart making the graphs. So Then Today, well yesterday, I finished making all the charts. Most looked like lognorm distributions, and a few looked like normal distribution. After some careful consideration, after log transforming it, and making a histogram out of the graphs that I thought had lognormal distributions, I realized some didnt even look like normal distributions after the natural log transformation. So, I looked up how to confirm if a distribution was in fact a normal distribution, and then some excel functions poped up called anderson darling test and Kolmogor Smirnov test. I then realized excel didnt come with those tests, so I then had to find where to download them. I downlaoded the add-ins, and then they wouldnt open, so I trouble shooted that, and then I was able to open excel with the add-ins. I then tried putting in the functions that bing ai said and they werent available. I then went back to the website, and looked for similarly sounding functions. I then started trying to apply the anderson darling test, and it said that my obviously non-normally distributed data was normally distributed. I then applied it to also xrp and sol, and it said the same thing. I then applied it to my log transformed data and it again spat out numbers that indicated that it was normally distributed. It didnt sit right, so I thought okay, let me do the other test, and then once again, the add in didnt have that exact function, so I had to look it up. I then had to separate the function into 2 functions: one to find hte crit value and one to find hte probability. I then tried putting the data in and got some crit values, and then when I went to see how to do the 2nd part, bing ai said that excel cant do it. So I looked up if there was anything online that could do it, and I clicked the first link. I then tried inputting all my data, and it said it can only take 300 pieces. I then looked along, and clicked the next link. I inputted the data, and it had the option of the anderson darling test, and it popped out that it wasnt normally distributed. cool, i thought, but then it said that the p value was 0. so then I put my xrp and solana data and the same thing, p value of 0. so I looked for another link, and like God sent it from heaven, it had ALL the tests: anderson darling, Kolmogor Smirnov, and many more, AND it did them all at once. So I put my data in and it said it was normally distributed, AND there was a check box to apply a log normal distribution, so I clicked it and then it said it was normally distributed under a log transformation. AND, under the results, IT SHOWED cool graphs showing a green linear line where the data SHOULD be if it was normally OR log normally distributed, and so I went with the distribution that looked like it followed the line the best. I then wrote down what distribution to do everything, and then it was time to calculate all the mean, variance, and std. dev for price % change 24h, total trades 24h, vwap 24h, open price 24h, low price 24h, lot volume per trade, high price 24h, close price 24h, high 52 w, and low 52w. Keep in mind that none of this information was EXPLICITLY available. With the OHLCVT data, it just had that. So I utilized the 1 minute version that gave me the data for every minute. So I had to go in there and create Rust programs that would calculate all these values. And of course, I didnt do it right the first time, so I had to do it again. Okay, so now all the mean, variance, and std. dev are calculated for all the metrics and for all the coins I wanted. Then I created another file in src called standardization_functions.rs and created functions for each metric and coin recursion for standardization. Next step is to input the standardization into the execute_action_functions.rs websocket client parse, ANDDDDD make the error protection more robust in execute_action_functions.rs ANDDDDD also standardize the wallets.                        JESUS IS GOOD. We got this!
03/08/24 - completely redid handle_all_kraken to 1. have error protection loop. 2. have standardization.                JESUS IS GOOD. We got this!
03/08/24 - completely redid parse_f64 to better work with error protection loop of handle_all_coinbase. completely redid handle_all_coinbase to 1. have error protection loop. 2. have standardization.                JESUS IS GOOD. We got this!
03/08/24 - revamped handle_all_bitstamp to 1. have error protection loop. 2. have standardization.                JESUS IS GOOD. We got this!
03/08/24 - modified handle_all_gemini to have standardization.          JESUS IS GOOD. We got this!
03/08/24 - NEED TO REMOVE INPUTS FOR A1 AND B1 FOR KRAKEN. SO IN TOTAL REMOVE 6 INPUTS. THEN NEED TO REMOVE DIVISOR and scale it too.
03/09/24 - fixed ALL of the indices. including value prior, coinbase, bitstamp, kraken, and gemini wallet. had to thus delete divisor and change all the functions in action_functions.rs. I am restarting from 0. gamma = 0.99. gradient cap +-0.1. learning_rate = 0.1.              JESUS IS GOOD. We got this
Here is how I will do future iterations: I will keep learning rate the same, and first change gamma. if it doesnt succeed when it gets to 0.1, go back to 0.99 and divide learning rate by 10. then repeat until it works. 1. remove all replay buffers. 2. remove saved states. 3. rename p, websocket_client, and q value logs. 4. add extra space in rewards doc for next run.               JESUS IS GOOD. We got this!
                DONT FORGET TO CHANGE LEARNING_RATE IN MAIN TOO if you ever change it.
03/08/24 - already had to fix an error.         JESUS IS GOOD. I got this!
03/09/24 - fixed another error.                 JESUS IS GOOD. We got this!
03/09/24 - changed number of inputs in initialization. code commented out some functions that were not being used in action_functions.rs.               JESUS IS GOOD. We got this!
03/09/24 - added print statements that include time for gemini xrp and solana because in 15 minutes somehow it didnt update xrp a single time. sus.     JESUS IS GOOD. We got this!
03/09/24 - changed some heartbeat log::info! in kraken and coinbase to println!.                JESUS IS GOOD. We got this!
03/09/24 - did the same to bitstamp.
03/09/24 - when: gamma = 0.99. gradient cap +-0.1. learning_rate = 0.1., within 4 iterations, average went to 4600. new values: gamma = 0.9. gradient cap +-0.1. learning_rate = 0.1.1. remove all replay buffers. 2. remove saved states. 3. rename p, websocket_client, and q value logs. 4. add extra space in rewards doc for next run.               JESUS IS GOOD. We got this!
03/09/24 - need to change how Im calculating value_prior and value_after. I have to UNscale them before I put them into the reward function. And UNscale value_after before I return it. Have to UNscale all wallets too. Next step is also to log the gradient values each iteration and then make a histogram
        1. unscale in main                         =======DONE
        2. unscale in network.rs - NONE NECESSARY
        3. unscale in action_functions.rs          =======DONE
        4. unscale in execute_action_functions.rs NOT NECESSARY
        5. do 1 hidden layer
03/09/24 - made values returned like coinbase/bitstamp/gemini/kraken wallets back to unscaled. Also, changed hidden layers to 1. Next step is to log the gradient values each iteration and then make a histogram.		JESUS IS GOOD. With Gods strength, I got this!
03/10/24 - created save gradients fn that logs all gradients in a csv. Then I implemented it.           JESUS IS GOOD. With his strength, we got this!
03/10/24 - 
ONE
HIDDEN
LAYER
IS
BEING
USED!!!
restarting from 0. gamma = 0.99. no gradient cap. learning rate = 1. CHANGE LEARNING RATE IN MAIN ANDDDDDD NETWORK.RS      1. remove all replay buffers. 2. rename p, websocket_client, and q value logs. 3. add extra space in rewards doc for next run.            JESUS IS GOOD. With his strength, we got this!
03/10/24 - failed within a few iterations. I will try again, but this time with more information when the gradients are logged.         JESUS IS GOOD. With his strength, we got this!
03/10/24 - updated s_0 on action_functions.rs.          JESUS IS GOOD. With his strength, we got this!
03/12/24 - created save_all_weights and updated save_all_gradients.             JESUS IS GOOD. With his strength, we got this!
03/12/24 - added save_all_weights fn calls.             JESUS IS GOOD. With his strength, we got this!
03/12/24 - adding gradient cap to be 1. gamma, learning_rate, gradient cap = 1.         JESUS IS GOOD. With his strength, we got this!
03/12/24 - applying gradient cap to be +-1 for all gradients, not just last layer. gamma, learning_rate, gradient cap = 1.         JESUS IS GOOD. With his strength, we got this!
03/12/24 - Saw that many of my gradients were reaching +-1 so I will increase gradient cap to +-10 and then apply a learning rate of 0.1. so: gamma = 1.0, gradient cap = +-10, learning_rate = 0.1.            JESUS IS GOOD. With his strength, we got this!
03/13/24 - was getting an error saying that value after is None even though it is Some, so I have corrected all the action functions so this does not happen anymore.                   JESUS IS GOOD. With his strength, we got this!
03/13/24 - changed learning rate to 0.01 in network and main.rs. gamma = 0.99, gradient cap = +-10, learning rate = 0.01.               JESUS IS GOOD. With his strength, we got this!
03/13/24 - added Huber Loss function, added i in save gradients/weights and cycle part 2 fn then in their parameters. gamma = 0.99, gradient cap = +-10, learning rate = 0.01.          JESUS IS GOOD. With his strength, we got this!
03/13/24 - weird error in i99 functiton again. Adding more detailed error checking.             JESUS IS GOOD. With his strength, we got this!
03/13/24 - interlude commit.            JESUS IS GOOD. With his strength, we got this!
03/13/24 - SUPER WEIRD! was getting an insane error in i99. it was so crazy that the log statements would skip every iteration until it would hhit 250 and then it would quit. Whats crazier is that even though the funciton was the same as i100, i100 didnt have the error. So what I did is I copied i100 over to i99 and ran it and it worked. so now Im only changing just a few constants and hopefully it will work. I also had this error in i120 and it was because I removed something without realizing it. Fixed it too. Decreasing learning rate to 0.001. IN MAIN AND IN NETWORK.RS      gamma = 0.99, gradient cap = +-10, learning rate = 0.001.               JESUS IS GOOD. With his strength, we got this!
03/13/24 - decreasing learning rate to 0.0001. gamma = 0.99, gradient cap = +-10, learning rate =0.0001, delta = 1. Before: network was stable for the first 100, but it very slowly started diverging. At 491, avg was 198.               JESUS IS GOOD. With his strength, we got this!
03/14/24 - at 491, avg was -0.48. at 3434, avg was 580. Changing exploration or exploitation so that it exploits before 3000 iterations. because I had it set to exploit when %10 = 0, it never exploited as i%10 = 0 was the cue to look at replay buffer. so changed it in exploration or exploitation to be i%11 = 0.  Instead of changing gamma, gradient cap, or learning rate, I will actually change delta to be bigger. gamma = 0.99, gradient cap = +-10, learning rate = 0.0001, and delta = 2.               JESUS IS GOOD. With his strength, we got this!
03/14/24 - changed save_all_gradients/weights so that they make a new file every 1000 iterations to keep file size not too large.               JESUS IS GOOD. With his strength, we got this!
03/14/24 - at 491, avg was 4.15. at 1400, avg was 68.6. changing gradient cap to +-5. gamma = 0.99, gradient cp = +-5, learning rate = 0.0001, delta = 2.               JESUS IS GOOD. With his strength, we got this!
03/14/24 - at 491, avg was -0.5. at 1400, avg was 8.459. at 3434, avg was 148. at 7633, avg was 1855. Due to this stability, I wanted to test if it was stable enough to do a conventional epsilon greedy strategy. epsilon will take 10000 iterations to reach pure exploitation. so:
gamma = 0.99, gradient cap = +-5, learning rate = 0.0001, detla = 2,epsilon -= 0.0001;          Restarting from 0. Renaming everything so I can have new files: p, websocket_client, all_gradients, all_weights, rust_replay_buffers, last_network_layer, then adding new run on rewards.               JESUS IS GOOD. With his strength, we got this!
03/15/24 - added conditionsl from c-dqn paper. basically doing max of Huber Loss and lMSBE and taking the max as the loss_derivative. Next step is to decrease the learning_rate.        JESUS IS GOOD. With his strength, we got this!
03/16/24 - at 7633, avg was -0.82. I allowed it to reach 9000. had avg of -1.92, BUT that wasnt enough iterations for it to actually learn. So I am changing epsilon decay to -0.00002 so it will reach 0 within 50000 iterations. I will see if thats enough time. If not, I will decrease decay so it reaches it within 100,000 then 200,000 iterations. gamma = 0.99, gradient cap = +-5, learning rate = 0.0001, delta = 2, epsilon decay = -0.00002.               JESUS IS GOOD. With his strength, we got this!
03/18/24 - fixing error loop so that I get a 10 sec wait in between attempts. Redid i5 to 10. next step is to do next group until finished.             JESUS IS GOOD. With his strength, we got this!
03/18/24 - fixed up to i30. Next step is to do the next group until finished.           JESUS IS GOOD. With his strength, we got this!
03/19/24 - fixed up to i40. Next step is to do the next group until finished. REALIZED that the error was due to Gemini server maintenance! So: im deleting the last 20 replay buffers and the last 20 saved states and  restarting. Saved state: 1710680791396. iteration: 8946. gamma = 0.99, gradient cap = +-5, learning rate = 0.0001, delta = 1, epsilon decay = -0.00002.
03/19/24 - after looking at the q values, I saw that the avg was different than where it ended. I noticed that a careful check, Yes, the neural network is actually the same one I once had. Were good.         JESUS IS GOOD. With his strength, we got this!
03/19/24 - restart due to gemini server maintenance. saved state: 1710831598636 iteration: 9826 . im stopping just before 2am aka when they started the maintenance. Im changing epsilon decay as well to be in 200,000 instead of 20,000 because well have "low expectations" so lets see how it will be.              JESUS IS GOOD. With his strength, we got this! epsilon decay = 0.000005;
MAKE
SURE
TO CHANGE 
EPSILON IN MAIN
WHEN NOT
RESTARTING from 0
03/20/24 - got an error at iteration 18922 it was about i50. didnt have the response text NOR error message. This is what I did. I fixed up to i80. Im going to go ahead and rerun it from 18900 and remove the last 22 saved states and replay buffers. Tomorrow night after everything I will work to get more action functions to properly display errors and to sleep before a retry. iteration: 18900. save state: 1710953272571. gamma = 0.99, gradient cap = +-5, learning rate = 0.0001, delta = 1, epsilon = 0.9055. epsilon decay = -0.000005;             JESUS IS GOOD. With his strength, we got this!
03/21/24 - just saw that the network had q values that didnt make sense like the q value with the highest reward, aka 0, having -30 for its value. will decrease learning rate to 0.00005 most likely. I need to think about this more and look at the other c-dqn paper to see what they did. Im surprised its divergin though. it shouldnt be diverging. Maybe it only converges with a low learning rate?            JESUS IS GOOD. With his strength, we got this! 
gamma = 0.99, gradient cap = +-5, learning rate = 0.0001, delta = 1, epsilon = 0.9055. epsilon decay = -0.000005. learning_rate = 0.00001.
03/23/24 - removed lMSBE. going to restart.             JESUS IS GOOD. With his strength, we got this!
03/24/24 - decreasing gamma to 0.9 and doing a full restart WITH lMSBE. gamma = 0.9, gradient cap = +-5, learning rate = 0.00001, delta = 1, epsilon = 0.9055. epsilon decay = -0.000005. learning_rate = 0.00001.               JESUS IS GOOD. With his strength, we got this!
03/25/24 - added more robust error details for up to i90.               JESUS IS GOOD. With his strength, we got this!
03/26/24 - stopped at 21671, avg was -3.08 so continuing. iteration = 21651. saved state = 1711409060583 gamma = 0.9, gradient cap = +-5, learning rate = 0.00001, delta = 1, epsilon = 0.891745. epsilon decay = -0.000005. learning_rate = 0.00001.               JESUS IS GOOD. With his strength, we got this!
03/27/24 - updated up to i100. changed some code in main to a log error instead of a log panic. specifically when it goes to try to kill the websocket client. The reason I changed it is because in my websocket client program, I changed some code so that when a task panics, the ENTIRE program  "SHOULD"  panic. I am restarting at the same thing as before at 21651 and same saved state because what happened was the web socket client task panicked, and this program didnt detect it so it kept going. So for half the time, it didnt have new for a lot of inputs. iteration = 21651. saved state = 1711409060583 gamma = 0.9, gradient cap = +-5, learning rate = 0.00001, delta = 1, epsilon = 0.891745. epsilon decay = -0.000005. learning_rate = 0.00001.               JESUS IS GOOD. With his strength, we got this!
03/27/24 - redo since websocket client glitched out at very beginning. iteration = 21651. saved state = 1711409060583 gamma = 0.9, gradient cap = +-5, learning rate = 0.00001, delta = 1, epsilon = 0.891745. epsilon decay = -0.000005. learning_rate = 0.00001.               JESUS IS GOOD. With his strength, we got this!
03/28/24 - revamped up to i110.         JESUS IS GOOD.
03/29/24 - revamped up to i120.         JESUS IS GOOD. With his strength, we got this!
03/30/24 - restarting from iteration 60307. removing last 20 saved states and replay buffers. iteration = 60307. saved state = 1711778518447. gamma = 0.9, gradient cap += 5, learning rate = 0.00001, delta = 1, epsilon = 0.6985. epsilon decay = -0.000005.          JESUS IS GOOD. With his strength, we got this!
03/30/24 - revamped up to i130.         JESUS IS GOOD. With his strength, we got this!
03/31/24 - at 83000 iterations and still going strong. will not check q values until 100000.            JESUS IS GOOD. With his strength, we got this!
04/01/24 - at 95100 iterations. will not check q values until 100000.           JESUS IS GOOD. With his strength, we got this!
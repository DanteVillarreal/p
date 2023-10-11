2.  add funcitonality to feed_forward
3.  initialize layer1 with parsed input from websocket apis
4.  connect rand
4.5 how to initialize weights and biases
5.  epsilon greedy strategy
5a. establish function for exploration that makes random action
5b. establish function for exploitation that "looks at previous actions and does what it knows"
5c. generate random number. divide by rand-max. compare to epsilon, if rand > E, it will exploit
5d. else, exploration
5e. establish decay rate of epsilon
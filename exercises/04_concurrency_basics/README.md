# Outline
The suggested work for this week is to play with the basics of concurrency in either Go or Rust.

In both of these languages, one of the common concurrency primitives is the channel.

## Exercise

1. Create a simple Burger struct, with a item number attribute.
2. Create a waiter thread, who keeps a queue-like structure of Burger instances.  The waiter has an unlimited capacity for burgers.
3. Create a producer thread who pushes burger instances to the waiter thread. Burgers should include a unique item number. Burgers should be produced once per second. Burgers should include a unique item number. Burgers should be produced once per second.  The producer thread only has ten burgers, when they run out of burgers, print `I have no more burgers.` and exit.
4. Create a consumer thread who waits on the waiter thread, and consumes burgers.  The consumer can only eat one burger at a time.

Each thread should log its activities to stdOut, so that we can see the passage of burgers across the various threads.

NOTE: The consumer thread should wait for a burger to become available from the waiter, but should only ever 

## Sample output

Created burger
Stored burger
Consumed burger
Created burger
Created burger
Stored burger
Consumed burger
Stored burger
Consumed burger
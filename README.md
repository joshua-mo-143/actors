# Actors Example
This is an example of how you can implement an actor in Rust, with only Tokio. It extends the model explained [here](https://ryhl.io/blog/actors-with-tokio/) where you can read more about actors.

## What are actors?
Actors are in short, flexible processes or background tasks within a distributed system that typically use a mailbox-based system to communicate with either a system or other actors.
Actors will normally present as messaging channels that have a given capacity.They are normally used in distributed systems to share workload or alternatively, 
you can use them as flexible background tasks within a regular web service that can be stopped, started and has graceful shutdown.

Actors may also be handled by processes called supervisors which can handle the rebooting of processes if they die.

## How do actors work in Rust?
Actors can be described as having two parts: the actor itself which holds a Receiver, and a handle to the actor which holds the Sender part of the actor.
When you spin up a new Actor handle, a channel should be created with the receiver being run on a new async thread that simply awaits messages and processes them.
With Tokio we can simply use what are called MPSC ("Multi Producer Single Consumer") channels, meaning many senders but 1 receiver.
This is quite useful as we can implement Clone for our Actor handle and then simply clone our Sender to another Actor handle if we want to have multiple send sources (see source code).

With regards to manual shutdown of actors, we can simply drop the receiver and the sender will close forever, allowing us to then drop the Actor handle and create a new one.

If we want our actor to change what task they're doing, we can send a kill message to force the receiver to drop and change whatever they're doing, then allow the supervisor to restart it.

## Why should I use actors?
Actors are non-blocking, fault-tolerant processes that operate through channels. They can handle shutdown by themselves, one actor can do multiple things depending on the message you send to it and are very lightweight.

## When should I not use actors?
When either:
- Your model is not concurrent.
- No mutable state (as in functional programming)
- You don't need async (ie you use mostly synchronous communication)
# MergeTree
A library for representing a tree data structure.

The tree is "mergable" with other instances of trees.

For example, multiple threads, each with their own instance of a tree, and which are exploring the state-space of a game board, can periodically merge their results and uptake the latest from the other threads.

Ideas (not all will be implemented; these are just thoughts):
- Perhaps one atomic "master" tree that threads will periodically merge into and then copy from
- A shared hashmap of state values, so if a thread is exploring one node it can check if other threads are performing similar work
- Threads can globally "reserve" nodes to explore to guarantee other threads are not wasting time exploring the same node.
- How to prevent two threads from doing the same work? Two possibilities:
1. They have different heurstics for deciding which node to explore
1. They update some global registry of where they are looking, and "check out" a certain node for exploration.

Challenges:
- Make sure the threads are not wasting time duplicating the work of other threads


multiple instances of trees that are totally isolated
but every X iterations, the state of a tree is sync'd to a master instance
(perhaps a hashtable of node state -> vec![node_data, ...] where tree instances push to the vec and another worker merges them all together)

when does a tree instance sync/push to the master?
consider: four instances are running, growing, getting edited.
if no one has a lock on the tree, anyone can take it.
if you try to take it and someone is using already, you just keep working.
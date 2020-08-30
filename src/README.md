# MergeTree
A library for representing a tree data structure.

The tree is "mergable" with other instances of trees.

For example, multiple threads, each with their own instance of a tree, and which are exploring the state-space of a game board, can periodically merge their results and uptake the latest from the other threads.

Ideas (not all will be implemented; these are just thoughts):
- Perhaps one atomic "master" tree that threads will periodically merge into and then copy from
- A shared hashmap of state values, so if a thread is exploring one node it can check if other threads are performing similar work
- Threads can globally "reserve" nodes to explore to guarantee other threads are not wasting time exploring the same node.

Challenges:
- Make sure the threads are not wasting time duplicating the work of other threads
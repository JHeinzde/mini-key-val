# Mini Key-Value store

This is a purely educational project and should not be used in production anywhere.
It features 3 operations insert/update, read and delete. Technically it is basically
a rust HashMap with a web-frontend and built in raft replication. 

The goal is to stay as minimal as possible with the implementation but to integrate
all features needed for raft replication of state. 

The code needs to be built with rust nightly as the main dependency is 
the web-framework rocket. 

Tests can be run with ```cargo test``` and you build it by running ```cargo build```.


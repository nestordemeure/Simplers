# Simple(x) Global Optimization

Experimentations with the [Simple(x)](https://github.com/chrisstroemel/Simple) global optimization algorithm.

Our goal is to implement the main algorithm in Rust and improve it if possible.

**This is a work in progress.**

## Implemented Improvements

- The user can define the search space as an hypercube

## Potential improvement

Do not hesitate to ask for improvements if needed.

- Let the user sugest some points to speed-up the search (will require the ability to check wether a point is in a simplex or a triangularisation algorithm).

- Let the user request several points to explore in parallel.

## TODO

Add tags to the github page.

Experiment with the main exploration/exploitation algorithm.

## Project submission

Submit project to [crates.io](https://crates.io/).

Once properly usable, the project could be submited to [www.arewelearningyet.com/metaheuristics](http://www.arewelearningyet.com/metaheuristics/).

We could offer to integrate the project into the [argmin](https://docs.rs/argmin/0.2.4/argmin/) optimization framework (to make the algorithm more accesible, future-proof and easier to compare with the state of the art).

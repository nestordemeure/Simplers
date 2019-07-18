# Simple(x) Global Optimization

A rust port of the [Simple(x)](https://github.com/chrisstroemel/Simple) global optimization librarie.

*This port is still a work in progress*, the current version is roughly on par with the original but lets you define your hyperparameter as an hypercube instead of a simplex.

Our goal is to deal with edge cases and provide a nicer interface on top of the algorithm.

## TODO

Let the user sugest some points to speed-up the search (will require the ability to check wether a point is in a simplex or a triangularisation algorithm).

Let the user save and load the search (?).

Use the builder patter to make it easier to define a problem:

## Useful references

The interface of the python [BayesianOptimization](https://github.com/fmfn/BayesianOptimization) package would be a good inspiration.

Once properly usable, the project could be submited to [www.arewelearningyet.com/metaheuristics](http://www.arewelearningyet.com/metaheuristics/).

We could offer to integrate the project into the [argmin](https://docs.rs/argmin/0.2.4/argmin/) optimization framework (to make the algorithm more accesible, future-proof and easier to compare with the state of the art).

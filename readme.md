# Simple(x) Global Optimization

A rust port of the [Simple(x)](https://github.com/chrisstroemel/Simple) global optimization librarie.

*This port is still a work in progress*, the current version is roughly on par with the original but lets you define your hyperparameter as an hypercube instead of a simplex.

Our goal is to deal with edge cases and provide a nicer interface on top of the algorithm.

## TODO

Let the user sugest some points to speed-up the search (will require the ability to check wether a point is in a simplex or a triangularisation algorithm).

Let the user save and load the search.

The interface of the python [BayesianOptimization](https://github.com/fmfn/BayesianOptimization) package would be a good inspiration.

## Potential algorithmic improvements

Rescale parameter of various amplitudes and types such as categorical variables (all converted to 0..1 and back) to avoid giving an artificial importance to a parameter.

Deal with points out of the search hypercube by clipping on strict limits (we need a proper Parameter object).

Test alternative exploration/exploitation algorithm such as UCB and UCB-tuned.

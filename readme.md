# Simple(x) Global Optimization

A rust port of the [Simple(x)](https://github.com/chrisstroemel/Simple) global optimization librarie.

Trying to reproduce the algorithm and to experiment with it.

## TODO

Write an implementation that can deal with parameter of various amplitudes and types (all converted to 0..1 and back to avoid giving an artificial importance to a parameter).

Deal with points out of the search hypercube by clipping (if needed, we need a proper Parameter object).

Let the user sugest some points to speed-up the search (will require the ability to check wether a point is in a simplex).

Let the user save and load the search.

The interface of the python [BayesianOptimization](https://github.com/fmfn/BayesianOptimization) package would be a good inspiration.

Test alternative exploration/exploitation algorithm such as UCB.
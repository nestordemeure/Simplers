# Simple(x) Global Optimization

A rust port of the [Simple(x)](https://github.com/chrisstroemel/Simple) global optimization librarie.

*This port is still a work in progress*, the current version is roughly on par with the original but lets you define your hyperparameter as an hypercube instead of a simplex.

Our goal is to deal with edge cases and provide a nicer interface on top of the algorithm.

## TODO

Let the user sugest some points to speed-up the search (will require the ability to check wether a point is in a simplex or a triangularisation algorithm).

Let the user save and load the search (?).

Use the builder patter to make it easier to define a problem:

let optimizer = optimizer(f, parameters)
   .set_exploration_preference(0.15) // define algorithm parameter (there is a default value)
   .probe_point([0.5, 1.6]) // define an additional point for exploration
   .initialize() // build the optimizer by running the function on all points that are needed

// opt implemens iter, each iteration returns a (coordinate,value) that has just been tested
opt.run_n_iter()

opt.best_value()

// in the absence of a fucntion, we could just rely on the user suggesting points... maybe

## Potential algorithmic improvements

Rescale parameter of various amplitudes (all converted to 0..1 and back) to avoid giving an artificial importance to a parameter.

Deal with points out of the search hypercube by clipping on strict limits (we need a proper Parameter object).

Test alternative exploration/exploitation algorithm such as UCB and UCB-tuned.

## Useful references

The interface of the python [BayesianOptimization](https://github.com/fmfn/BayesianOptimization) package would be a good inspiration.

Once properly usable, the project could be submited to [www.arewelearningyet.com/metaheuristics](http://www.arewelearningyet.com/metaheuristics/).

We could offer to integrate the project into the [argmin](https://docs.rs/argmin/0.2.4/argmin/) optimization framework (to make the algorithm more accesible, future-proof and easier to compare with the state of the art).

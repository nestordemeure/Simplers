# Simple(x) Global Optimization

A Rust implementation of the [Simple(x)](https://github.com/chrisstroemel/Simple) global optimization algorithm.

The strengths of this algorithm compared to Gaussian optimization would be the ability to deal with a large number of sample and high dimension gracefully.

**This is a work in progress.**

## Divergences to the original algorithm

- The user can define the search space as an hypercube (which is then mapped to a simplex using [this](https://math.stackexchange.com/a/385071/495073) method).

- The `exploration_preference` (float) parameter has been replaced by an `exploration_depth` (unsigned integer) parameter with a different but, hopefully, clearer semantic.
It represents how many split deep the algorithm can search before requiring higher-level exploration (0 meaning grid-search like exploration, 5 being a good default and large values (10+) being very exploitation/greedy focusses).

## Potential improvement

Do not hesitate to ask for improvements if needed. The list of things that could be done but will probably be left undone unless requested include :

- Let the user sugest some points to speed-up the search (will require the ability to check wether a point is in a simplex or a triangularization algorithm).

- Let the user request several points to explore in parallel.

- Let the user perform the search ofline.

- Define the algorithm on categorial variables and exponential spaces.

- Let the user modify the `exploration_depth` parameter while the function is running (to increase it over time à la simmulated annealing).

## TODO

Turn the algorithm into a library instead of an application.

Implement the iterator trait on the algorithm to give full control over the stopping criteria to the user.

Conducts tests using functions from [argmin_testfunctions](https://crates.io/crates/argmin_testfunctions).

## Target

Submit project to [crates.io](https://crates.io/).

Once properly usable, the project could be submited to [www.arewelearningyet.com/metaheuristics](http://www.arewelearningyet.com/metaheuristics/).

We could offer to integrate the project into the [argmin](https://docs.rs/argmin/0.2.4/argmin/) optimization framework (to make the algorithm more accesible, future-proof and easier to compare with the state of the art).

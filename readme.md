# Simple(x) Global Optimization

Experimentations with the [Simple(x)](https://github.com/chrisstroemel/Simple) global optimization algorithm.

The strengths of this algorithm compared to Gaussian optimization would be the ability to deal with a large number of sample and high dimenssion gracefully.

Our goal is to implement the main algorithm in Rust and improve it if possible.

**This is a work in progress.**

## Implemented Improvements

- The user can define the search space as an hypercube.

- The `exploration_preference` (float) parameter has been replaced by an `exploration_depth` (unsigned integer) parameter with a different but, hopefully, clearer semantic.

## Potential improvement

Do not hesitate to ask for improvements if needed. The list of things that could be done but will probably be left undone unless requested include :

- Let the user sugest some points to speed-up the search (will require the ability to check wether a point is in a simplex or a triangularisation algorithm).

- Let the user request several points to explore in parallel.

- Define the algorithm on categorial variables.

- Let the user indicate that the space should be explored in an exponential or quadratic fashion.

## TODO

Add tags to the github page.

Turn the algorithm into a library instead of an application.

Implement the iterator trait on the algorithm.

Update difference properly for the simplex type.

## Target

Submit project to [crates.io](https://crates.io/).

Once properly usable, the project could be submited to [www.arewelearningyet.com/metaheuristics](http://www.arewelearningyet.com/metaheuristics/).

We could offer to integrate the project into the [argmin](https://docs.rs/argmin/0.2.4/argmin/) optimization framework (to make the algorithm more accesible, future-proof and easier to compare with the state of the art).

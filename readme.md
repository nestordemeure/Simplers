# Simple(x) Global Optimization

A Rust implementation of the [Simple(x)](https://github.com/chrisstroemel/Simple) global optimization algorithm.

This algorithm, which should not be confused with the [simplex algorithm](https://en.wikipedia.org/wiki/Simplex_algorithm), is closest to [bayesian optimization](https://en.wikipedia.org/wiki/Bayesian_optimization).
Its strengths compared to bayesian optimization would be the ability to deal with a large number of sample and high dimension gracefully.

## Usage

**TODO**

## Divergences to the reference implementation

- The user can define the search space as an hypercube (which is then mapped to a simplex using [this](https://math.stackexchange.com/a/385071/495073) method).

- The `exploration_preference` (float) parameter has been replaced by an `exploration_depth` (unsigned integer) parameter with a different but, hopefully, clearer semantic.
It represents how many split deep the algorithm can search before requiring higher-level exploration (0 meaning grid-search like exploration, 5 being a good default and large values (10+) being very exploitation/greedy focusses).

- There are two implementations of the main loop, a fully integrated algorithm bit also an iterator based algorithm which gives the user full control on the stopping criteria.

## Potential future developements

Do not hesitate to ask for improvements if needed. The list of things that could be done but will probably be left undone unless requested includes :

- Let the user suggest some points to speed-up the search (will require the ability to check wether a point is in a simplex or a triangularization algorithm).

- Let the user explore several points in parallel.

- Let the user perform the search offline.

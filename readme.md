# Simple(x) Global Optimization

A Rust implementation of the [Simple(x)](https://github.com/chrisstroemel/Simple) black-box global optimization algorithm.

This algorithm (which should not be confused with the [simplex algorithm](https://en.wikipedia.org/wiki/Simplex_algorithm)) is closest to [bayesian optimization](https://en.wikipedia.org/wiki/Bayesian_optimization).
Its strengths, compared to bayesian optimization, are the ability to deal with a large number of sample and high dimension efficiently.

## Usage

There are two ways to use the algorithm, either use one of the `Optimizer::minimize` / `Optimizer::maximize` functions :

```rust
let f = |v:&[f64]| v[0] + v[1] * v[2];
let input_interval = vec![(-10., 10.), (-20., 20.), (0, 5.)];
let nb_iterations = 100;

let (max_value, coordinates) = Optimizer::maximize(&f, &input_interval, nb_iterations);
println!("max value: {} found in [{}, {}, {}]", max_value, coordinates[0], coordinates[1], coordinates[2]);
```

Or use an iterator if you want to set `exploration_depth` to an exotic value or to have fine grained control on the stopping criteria :

```rust
let f = |v:&[f64]| v[0] * v[1];
let input_interval = vec![(-10., 10.), (-20., 20.)];
let should_minimize = true;

// sets `exploration_depth` to be greedy
// runs the search for 30 iterations
// then waits until we find a point good enough
// finally stores the best value so far
let (min_value, coordinates) = Optimizer::new(&f, &input_interval, should_minimize)
                                       .set_exploration_depth(10)
                                       .skip(30)
                                       .skip_while(|(value,coordinates)| value > 1. )
                                       .next().unwrap();

println!("min value: {} found in [{}, {}]", min_value, coordinates[0], coordinates[1]);
```

## Divergences from the reference implementation

- The user defines the search space as an hypercube (which is then mapped to a simplex using [this](https://math.stackexchange.com/a/385071/495073) method).

- The `exploration_preference` (float) parameter has been replaced by an `exploration_depth` (unsigned integer) parameter.
It represents how many split deep the algorithm can search before requiring higher-level exploration (0 meaning grid-search like exploration, 5 being a good default and large values (10+) being very exploitation/greedy focusses).

- There are two ways to call the algorithm, either by calling a single function or via an iterator which gives the user full control on the stopping criteria.

## Potential future developements

Do not hesitate to ask for improvements.
The list of things that could be done but will probably be left undone unless requested includes :

- Add (optional) [serde](https://serde.rs/) support to save searches.

- Add an option to introduce noise in the [point generation](https://github.com/nestordemeure/Simplers/blob/master/src/point.rs#L35) (?).

- Let the user suggest some points to speed-up the search (will require the ability to check wether a point is in a simplex or a triangularization algorithm).

- Let the user explore several points in parallel.

- Let the user perform the search offline.

- We could offer to integrate the project into the [argmin](https://docs.rs/argmin/0.2.4/argmin/) optimization framework (to make the algorithm more accesible, future-proof and easier to compare with the state of the art).

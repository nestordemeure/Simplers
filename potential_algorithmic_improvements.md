# Potential algorithmic improvements

Reflexion on ways to improve / experiment with the algorithm.

## Initialization

If we have a mapping from the unit simplex to the unit hypercube and from the unit hypercube to the target hypercube, we can take an hypercube as parameters while doing the search in the simplex space.

This would be the easiest way to deal with an input hypercube without putting additional stress on the user.

A [Mapping](https://math.stackexchange.com/a/385071/495073) from hypercube (hi) to simplex (si) :

```
max_h = max_i(hi)
sum_h = sum_i(hi)
si = hi * (max_h / sum_h)
```

And the reverse mapping :

```
max_h = sum_s = sum_i(si)
max_s = max_h² / sum_h => sum_h / max_h = max_h / max_s = sum_s / max_s
hi = si * (sum_s / max_s)
```

(I have tested the other mapping from the link and it degraded results)

## Exploration-exploitation

I am unhappy with the current formula for two reasons :

- it has a parameter
- it uses the difference between the best and worst points so far which implies that we need to update our scores

One could imagine using a classical gaussian process + bayesian optimization formula instead (using the corners as the only data point to give a score to the center point).

## Tree structure

One could organize the simplex as a tree structure (each branch having d+1 sons, one per corner).
The search for the next simplex to expand would then become a monte-carlo tree search (selecting the next branch to be explored with UCB, thompson sampling or any similar algorithm).

One limitation is that, before having the ability to choose a branch properly, one would choose at random d+1 times (we may avoid that by choosing the most promising expansion, the one with the best expected value, first).

Keeping the idea of monte carlo tree search in mind, one could also build a kd-tree instead of a simplex (doing only one binary partition of the space at each level).
The partitions could be done along a random dimension or along the dimensions in order or along the largest dimension.
The point on which the partition is done should probably not be the center of the partitions to avoid grid search type of problems (points aligned on the same value per dimension reducing the among of information gained per search).
Random point plus cutting along the largest dimension seem like a good compromise.

A limitation is that, if things are flat along the first dimension, we might do twice the work because sibling branches do not communicate with each others.

In a way, while the simplex algorithm uses linear interpolation, the kd-tree algorithm uses rectangular interpolation (we lose interpolation power but improve scaling across number of dimenssions).

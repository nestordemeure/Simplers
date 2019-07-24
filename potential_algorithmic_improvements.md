# Potential algorithmic improvements

## Initialization

The easiest way to initialize the algorithm is to take a simplex of d+1 points as parameters.

One could also take a minimum of d+1 points and triangulize them in order to build the original simplex.

Finally, one could take intervals (open or closed) and build one or more simplex to contain them (if all intervals are open on at least one side, then you can just build a simplex containing the hypercube).

### Mapping

If we have a mapping from the unit simplex to the unit hypercube and from the unit hypercube to the target hypercube, we can take an hypercube as parameters while doing the search in the simplex space.

This would be the easiest way to deal with an input hypercube without putting additional stress on the user.

A reverse mapping would let the user suggest points but is not needed otherwise.

A [Mapping](https://math.stackexchange.com/a/385071/495073) from hypercube (hi) to simplex (si) :

max_h = max_i(hi)
sum_h = sum_i(hi)
si = hi * (max_h / sum_h)

And the reverse mapping :

max_h = sum_s = sum_i(si)
max_s = max_h² / sum_h => sum_h / max_h = max_h / max_s = sum_s / max_s
hi = si * (sum_s / max_s)

(I have tested the other mapping from the link and it degraded results)

## Exploration-exploitation

The current formula is ad-hoc but we have access to the number of visit to a node (approximated with a logarithm of its fraction of the total volume) and its approximated value.
If we add a variance, we can then use UCB, expected improvement and UCB-tuned in order to do our search (the last two having the advantages of not relying on a parameter).

## Tree structure

One could organize the simplex as a tree structure (each branch having d+1 sons, one per corner).
The search for the next simplex to expand would then become a monte-carlo tree search (selecting the next branch to be explored with UCB, thompson sampling or any similar algorithm).

One limitation is that, before having the ability to choose a branch properly, one would choose at random d+1 times (we may avoid that by choosing the most prosing expansion, the one with the best expected value, first).

Keeping the idea of monte carlo tree search in mind, one could also build a kd-tree instead of a simplex (doing only one binary partition of the space at each level).
The partitions could be done along a random dimension or along the dimensions in order or along the largest dimension.
The point on which the partition is done should probably not be the center of the partitions to avoid grid search type of problems (points aligned on the same value per dimension reducing the among of information gained per search).
Random point plus cutting along the largest dimension seem like a good compromise.

With this framework, one can easily add categorial variables (as one hot encoding or as a set of more than two possible branches).

A limitation is that, if things are flat along the first dimension, we might do twice the work because sibling branches do not communicate with each others.

In a way, while the simplex algorithm uses linear interpolation, the kd-tree algorithm uses rectangular interpolation (we lose interpolation power but gain scaling across number of dimenssions).

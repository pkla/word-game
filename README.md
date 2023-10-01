# Wordle Solver

This repository contains a Wordle solver written in Rust. The solver is designed to guess the correct word in a maximum of five tries using an optimization algorithm to maximize the expected reduction in possible words after each guess. It evolves the potential set of letters for each position in the word based on previous guesses and the feedback provided for those guesses.

## How to Use

To run the program, use the following syntax, providing the correct word as the first argument and initial guesses as subsequent arguments:

```bash
cargo run <word> <guess_1> <guess_2> ...
```

### Example:

```bash
cargo run probe crane
```

## Problem Formulation

Define:

- $ W $ as the set of all possible words.
- $ L $ as a list of sets, each element $ L_i $ representing the possible letters at the $ i $-th position in the word.
- $ G $ as the subset of words in $ W $ satisfying the constraints in $ L $, i.e., $ G = \{ w \in W : \forall i, w_i \in L_i \} $, where $ w_i $ is the $ i $-th letter of the word $ w $.
- $ N $ as the cardinality of $ G $, $ N = |G| $.
- $ g $ as a potential guess.

Now define the evolution operator, $ \text{evolve}(L, w, g) $, as the new constraints $ L' $ obtained after making a guess $ g $ assuming $ w $ as the correct word. Specifically, for each $ i $, $ L'_i $ is updated based on comparing $ g_i $ and $ w_i $.

Let $ R_g(w) $ be the resulting set of words after evolving the constraints by guessing $ g $ and assuming $ w $ is the correct word:

$$
R_g(w) = \{ v \in W : \forall i, v_i \in \text{evolve}(L, w, g)_i \}
$$

The expected number of words remaining in $ G $ after making a guess $ g $ is:

$$
E(g) = \frac{1}{N} \sum_{w \in G} |R_g(w)|
$$

Where $ |R_g(w)| $ is the cardinality of the set $ R_g(w) $.

The objective is to find the guess $ g $ that maximizes the expected reduction in the number of possible words:

$$
g^* = \arg \max_{g \in W} (N - E(g))
$$

Where $ g^* $ is the best guess, which is the one that is expected to minimize the expected number of words remaining in $ G $.


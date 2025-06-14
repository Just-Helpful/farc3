<!-- cargo-rdme start -->

# Constraint Satisfaction Solving

This crate is focused on a semi-generic approach to solving Constraint Satisfaction Problems,\
with the possibility to optimise based on the specific implementation of Constraints.

The primary exports of this crate are:

- [`System`] for generic constraint solving
- the [`Assignment`] trait for assigning values to variables
- the [`Constraint`] trait for constraining the values variables can take
- the [`Heuristic`] trait for deciding search order for constraint solving

[`System`]: https://docs.rs/farc3-csp/latest/farc3_csp/system/struct.System.html
[`Assignment`]: https://docs.rs/farc3-csp/latest/farc3_csp/assignment/trait.Assignment.html
[`Constraint`]: https://docs.rs/farc3-csp/latest/farc3_csp/constraint/trait.Constraint.html
[`Heuristic`]: https://docs.rs/farc3-csp/latest/farc3_csp/heuristics/trait.Heuristic.html

There's also some common variants of `Constraints`:

- [`DiscreteConstraint`] that covers most forms of discrete constraints
- [`MineConstraint`] that can be used for minesweeper mine solving

[`DiscreteConstraint`]: https://docs.rs/farc3-csp/latest/farc3_csp/systems/generic/constraint/struct.DiscreteConstraint.html
[`MineConstraint`]: https://docs.rs/farc3-csp/latest/farc3_csp/systems/mines/constraint/struct.MineConstraint.html

<!-- cargo-rdme end -->

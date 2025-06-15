# Farc3 Constraint Solving

<!-- cargo-rdme start -->

[![Github homepage](https://img.shields.io/badge/github-Just--Helpful%2Ffarc3-brightgreend?style=for-the-badge&logo=github)](https://github.com/Just-Helpful/farc3)
![Package version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FJust-Helpful%2Ffarc3-csp%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=%24.package.version&prefix=v&style=for-the-badge&logo=rust&label=crates.io&color=%23FF642D)

A semi-generic approach to solving Constraint Satisfaction Problems,\
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

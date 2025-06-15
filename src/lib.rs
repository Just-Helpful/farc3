//! [![Github homepage](https://raw.githubusercontent.com/Just-Helpful/Farc3/refs/heads/main/.github/badges/github.svg)
//! ![Package version](https://raw.githubusercontent.com/Just-Helpful/Farc3/refs/heads/main/.github/badges/version.svg)
//! ![Coverage report](https://raw.githubusercontent.com/Just-Helpful/Farc3/refs/heads/main/.github/badges/coverage.svg)
//!
//! A semi-generic approach to solving Constraint Satisfaction Problems,\
//! with the possibility to optimise based on the specific implementation of Constraints.
//!
//! The primary exports of this crate are:
//!
//! - [`System`] for generic constraint solving
//! - the [`Assignment`] trait for assigning values to variables
//! - the [`Constraint`] trait for constraining the values variables can take
//! - the [`Heuristic`] trait for deciding search order for constraint solving
//!
//! [`System`]: crate::system::System
//! [`Assignment`]: crate::assignment::Assignment
//! [`Constraint`]: crate::constraint::Constraint
//! [`Heuristic`]: crate::heuristics::Heuristic
//!
//! There's also some common variants of `Constraints`:
//!
//! - [`DiscreteConstraint`] that covers most forms of discrete constraints
//! - [`MineConstraint`] that can be used for minesweeper mine solving
//!
//! [`DiscreteConstraint`]: crate::systems::generic::constraint::DiscreteConstraint
//! [`MineConstraint`]: crate::systems::mines::constraint::MineConstraint
#![warn(missing_docs)]

pub mod assignment;
pub mod constraint;
pub mod heuristics;
pub mod system;
pub mod systems;
mod utils;

pub mod prelude {
  //! Common imports to `farc3-csp`
  pub use super::{
    assignment::Assignment,
    constraint::Constraint,
    heuristics::Heuristic,
    system::{System, SystemIter},
    systems::prelude::*,
  };
}

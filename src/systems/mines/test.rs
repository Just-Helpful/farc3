/// Unit testing generic [`Assignments`]
///
/// [`Assignments`]: crate::systems::generic::assignment::GenericAssignment
mod assignments {
  use super::super::assignment::MineAssignment;
  use crate::prelude::Assignment;
  use std::collections::HashMap;

  #[test]
  fn construction() {
    MineAssignment::new([0, 2], [1, 3]);
  }

  #[test]
  fn iteration() {
    let assign = MineAssignment::from_iter([(0, true), (1, false), (2, true)]);
    assert_eq!(
      HashMap::from_iter(assign),
      HashMap::from([(0, true), (1, false), (2, true)])
    );
  }

  #[test]
  fn intersection() {
    let assign0 = MineAssignment::from_iter([(0, true), (1, false), (2, true)]);
    let assign1 = MineAssignment::from_iter([/*      */ (1, false), (2, false), (3, true)]);
    let assign = assign0.intersection(assign1);

    assert_eq!(
      HashMap::from_iter(assign),
      HashMap::from([
        /* 0 not common to both assignments */
        (1, false),
        /* 2 is contradictory */
        /* 3 not common to both assignments */
      ])
    );
  }

  #[test]
  fn union() {
    let assign0 = MineAssignment::from_iter([(0, true), (1, false), (2, true)]);
    let assign1 = MineAssignment::from_iter([/*      */ (1, false), (2, false), (3, true)]);
    let assign = assign0.union(assign1);

    assert_eq!(
      HashMap::from_iter(assign),
      HashMap::from([
        (0, true),
        (1, false),
        /* 2 is contradictory */
        (3, true)
      ])
    );
  }
}

/// Unit testing generic [`Constraints`]
///
/// [`Constraints`]: crate::systems::generic::assignment::GenericConstraint
mod constraints {
  use super::super::constraint::MineConstraint;
  use crate::prelude::Constraint;
  use std::collections::{HashMap, HashSet};

  #[test]
  fn construction() {
    MineConstraint::new([0, 1, 2], 0); // decided, no mines
    MineConstraint::new([0, 1, 2], 2); // undecided
    MineConstraint::new([0, 1, 2], 3); // decided, all mines
  }

  #[test]
  fn size() {
    let cons = MineConstraint::new([0, 1, 2], 0);
    assert_eq!(cons.size(), 1); // {}

    let cons = MineConstraint::new([0, 1, 2], 2);
    assert_eq!(cons.size(), 3); // {0, 1}, {1, 2}, {2, 0}

    let cons = MineConstraint::new([0, 1, 2], 3);
    assert_eq!(cons.size(), 1);
  }

  #[test]
  fn variables() {
    let cons = MineConstraint::new([0, 1, 2], 2);
    assert_eq!(
      HashSet::from_iter(cons.variables()),
      HashSet::from([0, 1, 2])
    );
  }

  #[test]
  fn decompositions() {
    let cons = MineConstraint::new([0, 1, 2], 2);
    let vars = HashSet::from([0, 1, 2]);

    for decomp in cons.decompositions() {
      assert_eq!(decomp.size(), 1);
      let vars0 = decomp.variables().collect();
      assert!(vars.is_superset(&vars0));
    }
  }

  #[test]
  fn pop_solution() {
    let mut cons = MineConstraint::new([0, 1, 2], 0);

    let sltn = cons.pop_solution().unwrap();
    assert_eq!(
      HashMap::from_iter(sltn),
      HashMap::from([(0, false), (1, false), (2, false)])
    );

    assert_eq!(cons.size(), 1);
    assert_eq!(HashSet::from_iter(cons.variables()), HashSet::from([]));
  }

  /// Mine constraint reduction works when one constraint is a subset of another
  #[test]
  fn reduce_subset() {
    let mut cons0 = MineConstraint::new([0, 1, 2], 2);
    let cons1 = MineConstraint::new([0, 1], 1);

    cons0.reduce(&cons1).unwrap();
    assert_eq!(cons0.size(), 1);
    assert_eq!(HashSet::from_iter(cons0.variables()), HashSet::from([2]));

    let sltn = cons0.pop_solution().unwrap();
    assert_eq!(HashMap::from_iter(sltn), HashMap::from([(2, true)]));

    assert_eq!(cons0.size(), 1);
    assert_eq!(HashSet::from_iter(cons0.variables()), HashSet::new());
  }

  /// Mine constraint reduction works when one constraint is decided
  #[test]
  fn reduce_decided() {
    let mut cons0 = MineConstraint::new([0, 1, 2], 2);
    let cons1 = MineConstraint::new([0, 1, 3], 3);

    cons0.reduce(&cons1).unwrap();
    assert_eq!(cons0.size(), 1);
    assert_eq!(HashSet::from_iter(cons0.variables()), HashSet::from([2]));

    let sltn = cons0.pop_solution().unwrap();
    assert_eq!(HashMap::from_iter(sltn), HashMap::from([(2, false)]));

    assert_eq!(cons0.size(), 1);
    assert_eq!(HashSet::from_iter(cons0.variables()), HashSet::new());
  }
}

/// Testing generic constraint compatability with [`System`] solving
///
/// [`System`]: crate::system::System
mod solver {
  use std::collections::HashMap;

  use crate::prelude::MineConstraint;
  use crate::prelude::System;

  #[test]
  fn unresolvable() {
    let cons = MineConstraint::new([0, 1], 1);

    let mut sys = System::from_iter([cons]);
    sys.minimise().unwrap();
    assert_eq!(sys.len(), 1);

    let sltn = sys.pop_solution().unwrap();
    assert_eq!(HashMap::from_iter(sltn), HashMap::from([]));
    assert_eq!(sys.len(), 1);
  }

  #[test]
  fn trivial() {
    let cons = MineConstraint::new([0, 1], 0);

    let mut sys = System::from_iter([cons]);
    sys.minimise().unwrap();
    assert_eq!(sys.len(), 1);

    let sltn = sys.pop_solution().unwrap();
    assert_eq!(
      HashMap::from_iter(sltn),
      HashMap::from([(0, false), (1, false)])
    );
    assert_eq!(sys.len(), 0);
  }

  /// The solver can fully solve a system of constraints
  #[test]
  fn full_minimise() {
    // I don't think a fully minimisable system
    // is actually constructable from 2 constraints,
    // where none of them are already decided.
    let cons0 = MineConstraint::new([0, 1, 2], 2);
    let cons1 = MineConstraint::new([1, 2], 1);
    let cons2 = MineConstraint::new([0, 1], 1);

    let mut sys = System::from_iter([cons0, cons1, cons2]);
    sys.minimise().unwrap();
    assert_eq!(sys.len(), 3);

    let sltn = sys.pop_solution().unwrap();
    assert_eq!(
      HashMap::from_iter(sltn),
      HashMap::from([(0, true), (1, false), (2, true)])
    );
    assert_eq!(sys.len(), 0);
  }

  /// The solver can pick out the fully resolved part of a system
  #[test]
  fn partial_minimise() {
    let cons0 = MineConstraint::new([0, 1, 2], 2);
    let cons1 = MineConstraint::new([1, 2], 1);

    let mut sys = System::from_iter([cons0, cons1]);
    sys.minimise().unwrap();
    assert_eq!(sys.len(), 2);

    let sltn = sys.pop_solution().unwrap();
    assert_eq!(HashMap::from_iter(sltn), HashMap::from([(0, true)]));

    // we have remaining undecided constraints
    assert_eq!(sys.len(), 1);
    assert_eq!(Vec::from_iter(sys), vec![MineConstraint::new([1, 2], 1)]);
  }
}

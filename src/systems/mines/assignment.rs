//! Assignments for mines in a minesweeper tiles

use std::collections::hash_set;
use std::hash::Hash;

use crate::assignment::Assignment;
use crate::utils::NewHashSet;

/// An assignment of safe / mine tiles in a minesweeper game.\
/// This keeps track of which tiles are safe and which tiles are mines.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MineAssignment<V: Hash + Eq> {
  safe_tiles: NewHashSet<V>,
  mine_tiles: NewHashSet<V>,
}

impl<V: Hash + Eq> FromIterator<(V, bool)> for MineAssignment<V> {
  fn from_iter<T: IntoIterator<Item = (V, bool)>>(iter: T) -> Self {
    let mut safe_tiles = NewHashSet::default();
    let mut mine_tiles = NewHashSet::default();
    for (tile, mine) in iter {
      if mine {
        mine_tiles.insert(tile);
      } else {
        safe_tiles.insert(tile);
      }
    }

    Self {
      safe_tiles,
      mine_tiles,
    }
  }
}
impl<V: Hash + Eq, const N: usize> From<[(V, bool); N]> for MineAssignment<V> {
  fn from(value: [(V, bool); N]) -> Self {
    Self::from_iter(value)
  }
}

impl<V: Hash + Eq> MineAssignment<V> {
  /// Constructs a new mine assignment from the tiles that are safe and those that are mines.
  pub fn new(
    safe_tiles: impl IntoIterator<Item = V>,
    mine_tiles: impl IntoIterator<Item = V>,
  ) -> Self {
    Self {
      safe_tiles: NewHashSet::from_iter(safe_tiles),
      mine_tiles: NewHashSet::from_iter(mine_tiles),
    }
  }

  /// Constructs a mine assignment where all tiles are known to be safe.
  pub fn all_safe(safe_tiles: impl Into<NewHashSet<V>>) -> Self {
    Self {
      safe_tiles: safe_tiles.into(),
      mine_tiles: NewHashSet::default(),
    }
  }

  /// Constructs a mine assignment where all tiles are known to be mines.
  pub fn all_mine(mine_tiles: impl Into<NewHashSet<V>>) -> Self {
    Self {
      safe_tiles: NewHashSet::default(),
      mine_tiles: mine_tiles.into(),
    }
  }
}

impl<V: Hash + Eq> Assignment for MineAssignment<V> {
  fn intersection(mut self, other: Self) -> Self {
    self
      .safe_tiles
      .retain(|tile| other.safe_tiles.contains(tile));
    self
      .mine_tiles
      .retain(|tile| other.mine_tiles.contains(tile));
    self
  }

  fn union(mut self, other: Self) -> Self {
    // calculate union of mine and safe tiles
    for tile in other.safe_tiles {
      self.safe_tiles.insert(tile);
    }
    for tile in other.mine_tiles {
      self.mine_tiles.insert(tile);
    }

    // remove conflicting assignments
    self.safe_tiles.retain(|tile| {
      let shared = self.mine_tiles.contains(tile);
      if shared {
        self.mine_tiles.remove(tile);
      }
      !shared
    });

    self
  }
}

impl<V: Hash + Eq> IntoIterator for MineAssignment<V> {
  type Item = (V, bool);
  type IntoIter = IntoIter<V>;
  fn into_iter(self) -> Self::IntoIter {
    IntoIter {
      safe_tiles: self.safe_tiles.into_iter(),
      mine_tiles: self.mine_tiles.into_iter(),
    }
  }
}

/// An iterator for tiles assigned by a [`MineAssignment`].\
/// This'll yield:
/// - `(tile, true)` if a tile is a mine
/// - `(tile, false)` if a tile is safe
pub struct IntoIter<V> {
  safe_tiles: hash_set::IntoIter<V>,
  mine_tiles: hash_set::IntoIter<V>,
}

impl<V> Iterator for IntoIter<V> {
  type Item = (V, bool);
  fn next(&mut self) -> Option<Self::Item> {
    if let Some(tile) = self.safe_tiles.next() {
      return Some((tile, false));
    }
    if let Some(tile) = self.mine_tiles.next() {
      return Some((tile, true));
    }
    None
  }
}

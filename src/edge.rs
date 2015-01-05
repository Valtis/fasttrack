#[derive(Eq, PartialEq, Show, Copy)]
pub struct Edge {
  pub node: uint,
  pub distance: uint
}

impl Edge {
  pub fn new(n: uint, d: uint) -> Edge {
    Edge {node: n, distance: d }
  }
}

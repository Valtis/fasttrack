#[derive(Eq, PartialEq, Show, Copy)]
pub struct Edge {
  pub node: usize,
  pub distance: usize
}

impl Edge {
  pub fn new(n: usize, d: usize) -> Edge {
    Edge {node: n, distance: d }
  }
}

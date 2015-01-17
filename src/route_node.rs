
#[derive(Show, Copy, Encodable, Decodable, PartialEq, Eq)]
pub struct RouteNode {
  pub from: usize,
  pub to: usize,
  pub weight: usize,
}

impl RouteNode {
  pub fn new(f: usize, t: usize, w: usize) -> RouteNode {
    RouteNode{ from: f, to: t, weight: w }
  }
}

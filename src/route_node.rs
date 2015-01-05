
#[derive(Show, Copy, Encodable, Decodable, PartialEq, Eq)]
pub struct RouteNode {
  pub from: uint,
  pub to: uint,
  pub weight: uint,
}

impl RouteNode {
  pub fn new(f: uint, t: uint, w: uint) -> RouteNode {
    RouteNode{ from: f, to: t, weight: w }
  }
}

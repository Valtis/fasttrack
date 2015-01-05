use serialize::json;

#[derive(Show, Copy, Encodable, Decodable, PartialEq, Eq)]
pub struct RouteNode {
  pub from: i32,
  pub to: i32,
  pub weight: i32,
}

impl RouteNode {
  pub fn new(f: i32, t: i32, w: i32) -> RouteNode {
    RouteNode{ from: f, to: t, weight: w }
  }
}

#[derive(Show, Encodable, Decodable)]
pub struct Routes {
  pub nodes: Vec<RouteNode>,
}

impl Routes {
  pub fn decode(data: &str) -> Routes {
    Routes{nodes: json::decode(data).unwrap() }
  }
}

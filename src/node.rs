use serialize::json;

#[derive(Show, Copy, Encodable, Decodable)]
pub struct RouteNode {
  from: i32,
  to: i32,
  weight: i32,
}

impl RouteNode {
  pub fn new() -> RouteNode {
    RouteNode{ from: 0, to: 0, weight: 0 }
  }
}

#[derive(Show, Encodable, Decodable)]
pub struct Routes {
  nodes: Vec<RouteNode>,
}

impl Routes {
  pub fn deserialize(data: &str) -> Routes {
    json::decode(data).unwrap()
  }
}

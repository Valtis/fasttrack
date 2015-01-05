use std::collections::HashMap;

use route_node::RouteNode;


#[derive(Eq, PartialEq, Show)]
pub struct Edge {
  pub to: i32,
  pub weight: i32
}

impl Edge {
  pub fn new(t: i32, w: i32) -> Edge {
    Edge {to: t, weight: w }
  }
}

// returns a hashmap where the from-value serves as a key, and value is a
// vector containing connected nodes and distances to them
pub fn construct(routes: Vec<RouteNode>) -> HashMap<i32, Vec<Edge>>  {
  let mut list:HashMap<i32, Vec<Edge>> = HashMap::new();

  for node in routes.iter() {
    let key:i32 = node.from;
    if (!list.contains_key(&key)) {
      list.insert(key, Vec::new());
    }

    list.get_mut(&key).unwrap().push(Edge::new(node.to, node.weight));
  }

  list
}

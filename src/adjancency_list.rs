use route_node::RouteNode;


#[derive(Eq, PartialEq, Show, Copy)]
pub struct Edge {
  pub to: uint,
  pub weight: uint
}

impl Edge {
  pub fn new(t: uint, w: uint) -> Edge {
    Edge {to: t, weight: w }
  }
}

// returns a vector containing vector of edges that are reachable from given node
pub fn construct(routes: Vec<RouteNode>) -> Vec<Vec<Edge>> {

  // there probably is a smarter way of doing this
  // also, if there are only few nodes, we end up with vector containing  tons
  // of empty vectors; hashmap might be a smarter choice in that case
  let mut largest_node:uint = 0;

  for node in routes.iter() {
    if node.from > largest_node {
      largest_node = node.from;
    }

    if node.to > largest_node {
      largest_node = node.to;
    }
  }

  let mut list:Vec<Vec<Edge>> = range(0, largest_node + 1).map(|_| vec![]).collect();

  for node in routes.iter() {
    list[node.from].push(Edge::new(node.to, node.weight));
  }

  list
}

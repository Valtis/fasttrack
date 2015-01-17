use route_node::RouteNode;
use edge::Edge;

// returns a vector containing vector of edges that are reachable from given node
pub fn construct(routes: Vec<RouteNode>) -> Vec<Vec<Edge>> {

  // there probably is a smarter way of doing this
  // basically we need to ensure that the adjanency list has a
  // size of largest node + 1 so that we do not get out of bounds errors when
  // using the list to check node's neighbours.

  let mut largest_node:usize = 0;

  for node in routes.iter() {
    if node.from > largest_node {
      largest_node = node.from;
    }

    if node.to > largest_node {
      largest_node = node.to;
    }
  }

  // initialize the list with empty vectors. largest node + 1 is so that indexing
  // the vector with [largest_node] is allowed.
  let mut list:Vec<Vec<Edge>> = range(0, largest_node + 1).map(|_| vec![]).collect();

  for node in routes.iter() {
    list[node.from].push(Edge::new(node.to, node.weight));
  }

  list
}

use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::uint;


use edge::Edge;


// helper function, so that we don't need to create struct at call site needlessly.
pub fn calculate_path(from: uint, to: uint, adj_list: &Vec<Vec<Edge>>) -> Option<Vec<uint>> {
  let mut solver = Dijkstra::new(from, to, adj_list);
  solver.calculate_path()
}



// Dijkstra's algorithm
struct Dijkstra<'a> {
  from: uint,
  to: uint,
  distances:Vec<uint>,
  came_from:Vec<uint>,
  visited_nodes:HashSet<uint>,
  queue: BinaryHeap<Edge>,
  adj_list: &'a Vec<Vec<Edge>>,

}

impl<'a> Dijkstra<'a> {

  fn new(_from: uint, _to: uint, _adj_list: &'a Vec<Vec<Edge>>) -> Dijkstra {
    let mut _distances:Vec<uint> = range(0, _adj_list.len()).map(|_| uint::MAX).collect();
    let _came_from:Vec<uint> = range(0, _adj_list.len()).map(|_| uint::MAX).collect();
    _distances[_from] = 0;

    let mut _queue = BinaryHeap::new();
    _queue.push(Edge::new(_from, 0));

    Dijkstra{from: _from, to: _to, distances: _distances, came_from: _came_from,
      visited_nodes: HashSet::new(), queue: _queue, adj_list: _adj_list }
  }

  fn calculate_path(&mut self) -> Option<Vec<uint>> {
    // lowest distance from start node to any given node

    loop {
      match self.queue.pop() {
        Some(current_node) => {
          // reached target node, stop
          if current_node.node == self.to {
            break;
          }

          self.handle_neighbours(&current_node);

        },
        None => return None // empty queue => no path to target
      }
    }

    Some(self.construct_path())
  }

  fn handle_neighbours(&mut self, current_node: &Edge) {
    // node has already been visited, skip
    if self.visited_nodes.contains(&current_node.node) {
      return;
    }

    for edge in self.adj_list[current_node.node].iter() {
      if !self.handle_edge(current_node, edge) {
        continue;
      }
    }

    self.visited_nodes.insert(current_node.node);
  }

  fn handle_edge(&mut self, current_node: &Edge, edge: &Edge) -> bool {
    // node has been visited, skip
    if self.visited_nodes.contains(&edge.node) {
      return false;
    }

    let distance = self.distances[current_node.node] + edge.distance;

    if distance < self.distances[edge.node] {
      self.came_from[edge.node] = current_node.node;
      self.distances[edge.node] = distance;
      self.queue.push(Edge::new(edge.node, distance));
    }

    return true;
  }

  fn construct_path(&self) -> Vec<uint> {
    let mut path = vec![];

    let mut current_node:uint = self.to;

    loop {
      path.push(current_node);

      if current_node == self.from {
        break;
      }

      current_node = self.came_from[current_node];
    }

    path.reverse();
    path
  }
}







// binary heap is max-heap, so we need to implement & reverse (partial)ord to
// make it min-heap

impl PartialOrd for Edge {
  fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
    other.distance.partial_cmp(&self.distance)
  }
}

impl Ord for Edge {
  fn cmp(&self, other: &Edge) -> Ordering {

    other.distance.cmp(&self.distance)
  }
}

use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::usize;


use edge::Edge;


// helper function, so that we don't need to create struct at call site needlessly.
pub fn calculate_path(from: usize, to: usize, adj_list: &Vec<Vec<Edge>>) -> Option<Vec<usize>> {
  let mut solver = Dijkstra::new(from, to, adj_list);
  solver.calculate_path()
}



// Dijkstra's algorithm
struct Dijkstra<'a> {
  from: usize,
  to: usize,
  distances:Vec<usize>,
  came_from:Vec<usize>,
  visited_nodes:HashSet<usize>,
  queue: BinaryHeap<Edge>,
  adj_list: &'a Vec<Vec<Edge>>,

}

impl<'a> Dijkstra<'a> {

  // initializes the state for path finding.
  fn new(_from: usize, _to: usize, _adj_list: &'a Vec<Vec<Edge>>) -> Dijkstra {
    let mut _distances:Vec<usize> = range(0, _adj_list.len()).map(|_| usize::MAX).collect();
    let _came_from:Vec<usize> = range(0, _adj_list.len()).map(|_| usize::MAX).collect();

    _distances[_from] = 0;

    let mut _queue = BinaryHeap::new();
    _queue.push(Edge::new(_from, 0));

    Dijkstra{from: _from, to: _to, distances: _distances, came_from: _came_from,
      visited_nodes: HashSet::new(), queue: _queue, adj_list: _adj_list }
  }

  fn calculate_path(&mut self) -> Option<Vec<usize>> {

    loop {
      match self.queue.pop() {
        Some(current_node) => {
          // reached target node, stop
          if current_node.node == self.to {
            break;
          }

          // handle unvisited neighbours, and if path through them is shorted,
          // update the value and push to queue
          self.handle_neighbours(&current_node);

        },
        None => return None // empty queue => no path to target
      }
    }

    Some(self.construct_path())
  }

  fn handle_neighbours(&mut self, current_node: &Edge) {

    // old values are not removed from queue when updating with shorted distance
    // (as far as I can tell, impossible to do with rust binary heap implementation;
    // likely would need custom implementation) so we need to check if the
    // node actually has already been handled, and skip it if so
    if self.visited_nodes.contains(&current_node.node) {
      return;
    }

    for edge in self.adj_list[current_node.node].iter() {
      self.handle_edge(current_node, edge)
    }

    self.visited_nodes.insert(current_node.node);
  }

  fn handle_edge(&mut self, current_node: &Edge, edge: &Edge) {
    // node has been visited, skip
    if self.visited_nodes.contains(&edge.node) {
      return;
    }

    let distance = self.distances[current_node.node] + edge.distance;

    if distance < self.distances[edge.node] {
      self.came_from[edge.node] = current_node.node;
      self.distances[edge.node] = distance;
      self.queue.push(Edge::new(edge.node, distance));
    }
  }

  fn construct_path(&self) -> Vec<usize> {
    let mut path = vec![];

    let mut current_node:usize = self.to;

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

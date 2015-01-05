use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::uint;


use edge::Edge;


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


// Dijkstra's algorithm
pub fn calculate_path(from: uint, to: uint, adj_list: &Vec<Vec<Edge>>) -> Option<Vec<uint>> {

  // lowest distance from start node to any given node
  let mut distances:Vec<uint> = range(0, adj_list.len()).map(|_| uint::MAX).collect();
  let mut came_from:Vec<uint> = range(0, adj_list.len()).map(|_| uint::MAX).collect();

  distances[from] = 0;

  let mut visited_nodes: HashSet<uint> = HashSet::new();
  let mut queue:BinaryHeap<Edge> = BinaryHeap::new();

  queue.push(Edge::new(from, 0));

  loop {
    match queue.pop() {
      Some(current_node) => {
        // reached target node, stop
        if current_node.node == to {
          break;
        }

        // node has already been visited, skip
        if visited_nodes.contains(&current_node.node) {
          continue;
        }

        for edge in adj_list[current_node.node].iter() {
          // node has been visited, skip
          if visited_nodes.contains(&edge.node) {
            continue;
          }

          let distance = distances[current_node.node] + edge.distance;

          if distance < distances[edge.node] {
            came_from[edge.node] = current_node.node;
            distances[edge.node] = distance;
            queue.push(Edge::new(edge.node, distance));
          }
        }


        visited_nodes.insert(current_node.node);
      },
      None => return None
    }



  }
  let mut path = vec![];

  let mut current_node:uint = to;

  loop {
    path.push(current_node);

    if current_node == from {
      break;
    }

    current_node = came_from[current_node];
  }

  path.reverse();

  println!("Path cost: {}", distances[to]);

  println!("Path: {}", path);

  Some(path)
}

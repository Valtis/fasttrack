use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::uint;


use adjancency_list::Edge;

#[derive(PartialEq, Eq, Show, Copy)]
pub struct QueueNode {
  node: uint,
  distance: uint,
}

// binary heap is max-heap, so we need to implement & reverse (partial)ord to
// make it min-heap

impl PartialOrd for QueueNode {
  fn partial_cmp(&self, other: &QueueNode) -> Option<Ordering> {
    other.distance.partial_cmp(&self.distance)
  }
}

impl Ord for QueueNode {
  fn cmp(&self, other: &QueueNode) -> Ordering {

    other.distance.cmp(&self.distance)
  }
}

impl QueueNode {
  pub fn new(n: uint, d: uint) -> QueueNode {
    QueueNode{node: n, distance: d}
  }
}

// Dijkstra's algorithm
pub fn calculate_path(from: uint, to: uint, adj_list: &Vec<Vec<Edge>>) {

  // lowest distance from start node to any given node
  let mut distances:Vec<uint> = range(0, adj_list.len()).map(|_| uint::MAX).collect();

  distances[from] = 0;

  let mut visited_nodes: HashSet<uint> = HashSet::new();
  let mut queue:BinaryHeap<QueueNode> = BinaryHeap::new();

  queue.push(QueueNode::new(from, 0));

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
          if visited_nodes.contains(&edge.to) {
            continue;
          }

          let distance = distances[current_node.node] + edge.weight;

          if distance < distances[edge.to] {
            distances[edge.to] = distance;
            queue.push(QueueNode::new(edge.to, distance));
          }
        }


        visited_nodes.insert(current_node.node);
      },
      None => break,
    }



  }

  println!("Path cost: {}", distances[to]);
}

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::i32;


use adjancency_list::Edge;

#[derive(PartialEq, Eq, Show)]
pub struct QueueNode {
  node: i32,
  distance: i32,
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
  pub fn new(n: i32, d: i32) -> QueueNode {
    QueueNode{node: n, distance: d}
  }
}

// Dijkstra's algorithm
pub fn calculate_path(from: i32, to: i32, adj_list: HashMap<i32, Vec<Edge>>) {
  // lowest distance from start node to given node; node - distance pairs
  let mut distances:HashMap<i32, i32> =  HashMap::new();

  for (key, _) in adj_list.iter() {
    distances.insert(*key, i32::MAX);
  }

  let mut visited_nodes: HashSet<i32> = HashSet::new();
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

        match adj_list.get(&current_node.node) {
          Some(neighbours) => {
            for edge in neighbours.iter() {
              // node has been visited, skip
              if visited_nodes.contains(&edge.to) {
                continue;
              }
              let distance = *distances.get(&current_node.node).unwrap() + edge.weight;

              if distance < *distances.get(&edge.to).unwrap() {
                distances.insert(edge.to, distance);
                queue.push(QueueNode::new(edge.to, distance));
              }
            }
          },
          None => { /* do nothing*/}
        }



        visited_nodes.insert(current_node.node);
      },
      None => break,
    }



  }

  println!("Path cost: {}", distances[to]);
}

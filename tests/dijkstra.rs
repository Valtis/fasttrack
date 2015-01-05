extern crate fasttrack;
use fasttrack::dijkstra::QueueNode;

use std::collections::BinaryHeap;


#[test]
fn queue_node_ordering_works_correctly() {

  let mut priority_queue = create_queue();

  assert_eq!(QueueNode::new(4, 10), priority_queue.pop().unwrap());
  assert_eq!(QueueNode::new(9, 12), priority_queue.pop().unwrap());
  assert_eq!(QueueNode::new(6, 15), priority_queue.pop().unwrap());
  assert_eq!(QueueNode::new(5, 20), priority_queue.pop().unwrap());
  assert_eq!(QueueNode::new(7, 32), priority_queue.pop().unwrap());
}

fn create_queue() -> BinaryHeap<QueueNode> {
  let mut priority_queue:BinaryHeap<QueueNode> = BinaryHeap::new();
  priority_queue.push(QueueNode::new(5, 20));
  priority_queue.push(QueueNode::new(6, 15));
  priority_queue.push(QueueNode::new(7, 32));
  priority_queue.push(QueueNode::new(9, 12));
  priority_queue.push(QueueNode::new(4, 10));

  priority_queue
}

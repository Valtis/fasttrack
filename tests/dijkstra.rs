extern crate fasttrack;
use fasttrack::dijkstra::calculate_path;

use fasttrack::edge::Edge;

use std::collections::BinaryHeap;

#[test]
fn queue_node_ordering_works_correctly() {

  let mut priority_queue = create_queue();

  assert_eq!(Edge::new(4, 10), priority_queue.pop().unwrap());
  assert_eq!(Edge::new(9, 12), priority_queue.pop().unwrap());
  assert_eq!(Edge::new(6, 15), priority_queue.pop().unwrap());
  assert_eq!(Edge::new(5, 20), priority_queue.pop().unwrap());
  assert_eq!(Edge::new(7, 32), priority_queue.pop().unwrap());
}

fn create_queue() -> BinaryHeap<Edge> {
  let mut priority_queue:BinaryHeap<Edge> = BinaryHeap::new();
  priority_queue.push(Edge::new(5, 20));
  priority_queue.push(Edge::new(6, 15));
  priority_queue.push(Edge::new(7, 32));
  priority_queue.push(Edge::new(9, 12));
  priority_queue.push(Edge::new(4, 10));

  priority_queue
}


#[test]
fn if_no_path_exists_none_is_returned() {
  let path = calculate_path(0, 12, &create_adjancency_list());
  assert_eq!(None, path);
}


#[test]
fn path_is_computed_correctly_to_neighboring_node() {
  let path = calculate_path(2, 3, &create_adjancency_list()).unwrap();
  assert_eq!(vec![2, 3], path);
}

#[test]
fn path_is_computed_correctly_to_self() {
  let path = calculate_path(2, 2, &create_adjancency_list()).unwrap();
  assert_eq!(vec![2], path);
}


#[test]
fn path_is_computed_correctly_to_distant_node() {
  let path = calculate_path(0, 10, &create_adjancency_list()).unwrap();
  assert_eq!(vec![0, 2, 3, 4, 5, 9, 10], path);
}

#[test]
fn path_is_computed_correctly_to_node_with_one_way_transition() {
  let path = calculate_path(12, 6, &create_adjancency_list()).unwrap();
  assert_eq!(vec![12, 5, 4, 3, 2, 6], path);
}


#[test]
fn path_from_10_to_1_is_correct() {
  let path = calculate_path(10, 1, &create_adjancency_list()).unwrap();
  assert_eq!(vec![10, 1], path);
}

#[test]
fn path_from_10_to_7_is_correct() {
  let path = calculate_path(10, 7, &create_adjancency_list()).unwrap();
  assert_eq!(vec![10, 9, 5, 4, 8, 7], path);
}

#[test]
fn path_from_10_to_6_is_correct() {
  let path = calculate_path(10, 6, &create_adjancency_list()).unwrap();
  assert_eq!(vec![10, 9, 5, 4, 3, 2, 6], path);
}

/*  Graph

       10            30      -> 20      -> 5
  0 -------- 1 --------- 10---------9-------- 11
  |                          <-4    |
  |4                                | 7
  |    6          3            8    |
  2----------3-----------4----------5-------- 12
  |                      |             <- 1
  |3                     | 5
  |                      |
  6----------7-----------8
       12         4


*/
fn create_adjancency_list() -> Vec<Vec<Edge>>  {
  let mut adj_list:Vec<Vec<Edge>> = Vec::new();

  // neighbours for nodes:
  // 0
  let mut neighbours = Vec::new();
  neighbours.push(Edge::new(1,  10));
  neighbours.push(Edge::new(2,  4));
  adj_list.push(neighbours);

  // 1
  neighbours = Vec::new();
  neighbours.push(Edge::new(0,  10));
  neighbours.push(Edge::new(10,  30));
  adj_list.push(neighbours);

  // 2
  neighbours = Vec::new();
  neighbours.push(Edge::new(0,  4));
  neighbours.push(Edge::new(3,  6));
  neighbours.push(Edge::new(6,  3));
  adj_list.push(neighbours);

  // 3
  neighbours = Vec::new();
  neighbours.push(Edge::new(2,  6));
  neighbours.push(Edge::new(4,  3));
  adj_list.push(neighbours);

  // 4
  neighbours = Vec::new();
  neighbours.push(Edge::new(3,  3));
  neighbours.push(Edge::new(5,  8));
  neighbours.push(Edge::new(8,  5));
  adj_list.push(neighbours);

  // 5
  neighbours = Vec::new();
  neighbours.push(Edge::new(4,  8));
  neighbours.push(Edge::new(9,  7));
  adj_list.push(neighbours);

  // 6
  neighbours = Vec::new();
  neighbours.push(Edge::new(2,  3));
  neighbours.push(Edge::new(7,  12));
  adj_list.push(neighbours);

  // 7
  neighbours = Vec::new();
  neighbours.push(Edge::new(6, 12));
  neighbours.push(Edge::new(8,  4));
  adj_list.push(neighbours);

  // 8
  neighbours = Vec::new();
  neighbours.push(Edge::new(4,  5));
  neighbours.push(Edge::new(7,  4));
  adj_list.push(neighbours);

  // 9
  neighbours = Vec::new();
  neighbours.push(Edge::new(5,  7));
  neighbours.push(Edge::new(10,  4));
  neighbours.push(Edge::new(11,  5));
  adj_list.push(neighbours);


  // 10
  neighbours = Vec::new();
  neighbours.push(Edge::new(1,  30));
  neighbours.push(Edge::new(9,  20));
  adj_list.push(neighbours);

  // 11
  neighbours = Vec::new();
  adj_list.push(neighbours);

  // 12
  neighbours = Vec::new();
  neighbours.push(Edge::new(5,  1));
  adj_list.push(neighbours);

  adj_list
}























//

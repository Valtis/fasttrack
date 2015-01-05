extern crate fasttrack;
extern crate serialize;

use std::io::File;
use std::str::from_utf8;
use serialize::json;
use std::collections::HashMap;
use std::collections::BinaryHeap;



use fasttrack::route_node::RouteNode;
use fasttrack::adjancency_list;

fn main() {
/*
  let routes = load_data();
  let adj_list = adjancency_list::construct(routes);

  calculate_path(123, 345 , adj_list);*/


}

fn load_data() -> Vec<RouteNode> {
  match File::open(&Path::new("graph.json")).read_to_end() {
    Ok(content_vec) => {
      match from_utf8(content_vec.as_slice()) {
        Ok(data) => return json::decode(data).unwrap(),
        Err(err) => panic!("Could not convert byte data into utf8 string: {}", err),
      }
    }
    Err(err) => panic!("IO error: {}", err),
  }
}
/*
struct DijkstraQueueNode {
  node: i32,
  distance: i32,
}

// Dijkstra's algorithm
fn calculate_path(from: i32, to: i32, adj_list: HashMap<i32, HashMap<i32, i32>>) {
  // lowest distance from start node to given node; node - distance pairs
  let mut distances =  Vec::from_elem(adj_list.len(), std::i32::MAX);

  /*for (key, _) in adj_list.iter() {
    lowest_distance.insert(*key, std::i32::MAX);
  }

  let mut visited_nodes: HashSet<i32> = HashSet::new();
  let mut queue:RingBuf<i32> = RingBuf::new();

  queue.push_back(from);


  loop {
    if queue.len() == 0 {
      break;
    }

    let current_node = queue.pop_front();
    if current_node == to {
      break;
    }

    for edge in adj_list.get(current_node).iter() {
      if visited_nodes.contains_key(edge.to) {
        continue;
      }

      let distance = lowest_distance.get(current_node) + edge.weight;


      if !queue.contains(edge.to) {

      }
    }


    visited_nodes.insert(current_node);
  }*/

}*/

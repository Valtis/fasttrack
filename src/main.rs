extern crate fasttrack;
extern crate serialize;

use std::io::File;
use std::str::from_utf8;
use serialize::json;


use fasttrack::route_node::RouteNode;
use fasttrack::adjancency_list;
use fasttrack::dijkstra;

fn main() {

  let routes = load_data();
  let adj_list = adjancency_list::construct(routes);

  println!("{}", dijkstra::calculate_path(201, 8775 , &adj_list));

  println!("{}", dijkstra::calculate_path(23, 23877 , &adj_list));

  println!("{}", dijkstra::calculate_path(0, 49900 , &adj_list));

  println!("{}", dijkstra::calculate_path(7896, 38949 , &adj_list));

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

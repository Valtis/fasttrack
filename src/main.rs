extern crate fasttrack;
extern crate serialize;

use std::io::File;
use std::str::from_utf8;
use serialize::json;

use fasttrack::route_node::RouteNode;
use fasttrack::adjancency_list;
use fasttrack::dijkstra;
use fasttrack::journey::Journey;


fn main() {

  let routes = load_graph();
  let adj_list = adjancency_list::construct(routes);

  let mut journeys = load_journeys();

  // paths that have already a route will be recalculated
  // but that's fine (in my opinion), as those are merely for example anyway
  journeys = journeys.iter().map(
    |journey| Journey::new(journey.from, journey.to,
          dijkstra::calculate_path(journey.from, journey.to, &adj_list))
    ).collect();


  println!("{}", json::as_pretty_json(&journeys));
}

fn load_graph() -> Vec<RouteNode> {
  json::decode(read_content("graph.json").as_slice()).unwrap()
}

fn load_journeys() -> Vec<Journey> {
  json::decode(read_content("journeys.json").as_slice()).unwrap()
}

fn read_content(file: &str) -> String {
  match File::open(&Path::new(file)).read_to_end() {
    Ok(byte_vec) => from_utf8(byte_vec.as_slice()).unwrap().to_string(),
    Err(err) => panic!("IO error: {}", err),
  }
}

extern crate fasttrack;

use fasttrack::route;


#[test]
fn decode_initializes_datastructures_correctly() {
  let data = route::Routes::decode(test_data());
  assert_eq!(3, data.nodes.len());
  assert_eq!(route::RouteNode::new(0, 7142, 3), data.nodes[0]);
  assert_eq!(route::RouteNode::new(0, 0, 18), data.nodes[1]);
  assert_eq!(route::RouteNode::new(0, 1, 14), data.nodes[2]);
}

fn test_data() -> &'static str {

  return "[
  { \"from\": 0, \"to\": 7142, \"weight\": 3 },
  { \"from\": 0, \"to\": 0, \"weight\": 18 },
  { \"from\": 0, \"to\": 1, \"weight\": 14 }
  ]";

}

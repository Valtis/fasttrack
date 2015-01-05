extern crate fasttrack;
extern crate serialize;

use fasttrack::route_node::RouteNode;
use serialize::json;


#[test]
fn data_is_decoded_correctly() {
  let data:Vec<RouteNode> = json::decode(test_data()).unwrap();
  assert_eq!(3, data.len());
  assert_eq!(RouteNode::new(0, 7142, 3), data[0]);
  assert_eq!(RouteNode::new(0, 0, 18), data[1]);
  assert_eq!(RouteNode::new(0, 1, 14), data[2]);
}

fn test_data() -> &'static str {

  return "[
  { \"from\": 0, \"to\": 7142, \"weight\": 3 },
  { \"from\": 0, \"to\": 0, \"weight\": 18 },
  { \"from\": 0, \"to\": 1, \"weight\": 14 }
  ]";

}

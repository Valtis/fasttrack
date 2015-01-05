extern crate fasttrack;

use fasttrack::adjancency_list;
use fasttrack::adjancency_list::Edge;
use fasttrack::route_node::RouteNode;


#[test]
fn adjancency_list_is_built_correctly() {
  let mut routes:Vec<RouteNode> = Vec::new();

  routes.push(RouteNode{from: 0, to: 0, weight: 15});
  routes.push(RouteNode{from: 0, to: 1, weight: 20});
  routes.push(RouteNode{from: 1, to: 0, weight: 12});
  routes.push(RouteNode{from: 1, to: 2, weight: 25});
  routes.push(RouteNode{from: 2, to: 1, weight: 13});

  let adj_list = adjancency_list::construct(routes);

  assert_eq!(3, adj_list.len());

  assert_eq!(2, adj_list.get(&0).unwrap().len());
  assert_eq!(vec![Edge::new(0, 15), Edge::new(1, 20)], *adj_list.get(&0).unwrap());

  assert_eq!(2, adj_list.get(&1).unwrap().len());
  assert_eq!(vec![Edge::new(0, 12), Edge::new(2, 25)], *adj_list.get(&1).unwrap());

  assert_eq!(1, adj_list.get(&2).unwrap().len());
  assert_eq!(vec![Edge::new(1, 13)], *adj_list.get(&2).unwrap());


}

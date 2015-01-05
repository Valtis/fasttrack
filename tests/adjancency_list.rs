extern crate fasttrack;

use fasttrack::adjancency_list;
use fasttrack::route;

#[test]
fn adjancency_list_is_built_correctly() {
  let mut routes = route::Routes { nodes: vec![] };

  routes.nodes.push(route::RouteNode{from: 0, to: 0, weight: 15});
  routes.nodes.push(route::RouteNode{from: 0, to: 1, weight: 20});
  routes.nodes.push(route::RouteNode{from: 1, to: 0, weight: 12});
  routes.nodes.push(route::RouteNode{from: 1, to: 2, weight: 25});
  routes.nodes.push(route::RouteNode{from: 2, to: 1, weight: 13});

  let adj_list = adjancency_list::construct(routes);

  assert_eq!(3, adj_list.len());

  assert_eq!(2, adj_list.get(&0).unwrap().len());
  assert_eq!(15, *adj_list.get(&0).unwrap().get(&0).unwrap());
  assert_eq!(20, *adj_list.get(&0).unwrap().get(&1).unwrap());

  assert_eq!(2, adj_list.get(&1).unwrap().len());
  assert_eq!(12, *adj_list.get(&1).unwrap().get(&0).unwrap());
  assert_eq!(25, *adj_list.get(&1).unwrap().get(&2).unwrap());

  assert_eq!(1, adj_list.get(&2).unwrap().len());
  assert_eq!(13, *adj_list.get(&2).unwrap().get(&1).unwrap());
}

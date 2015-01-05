use std::collections::HashMap;

use route::Routes;

// returns a hashmap where the from-value serves as a key, and value is a hashmap
// containing to-value as keys and weight-values as values
pub fn construct(routes: Routes) -> HashMap<i32, HashMap<i32, i32>>  {
  let mut list:HashMap<i32, HashMap<i32, i32>> = HashMap::new();

  for node in routes.nodes.iter() {
    let key:i32 = node.from;
    if (!list.contains_key(&key)) {
      list.insert(key, HashMap::new());
    }

    list.get_mut(&key).unwrap().insert(node.to, node.weight);
  }

  list
}

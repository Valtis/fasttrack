extern crate fasttrack;
extern crate serialize;
use fasttrack::node;
use serialize::json;

use std::io::File;
use std::str::from_utf8;

fn main() {

  let data = load_data();


}

fn load_data() -> node::Routes {
  match File::open(&Path::new("graph.json")).read_to_end() {
    Ok(content_vec) => {
      match from_utf8(content_vec.as_slice()) {
        Ok(data) => return node::Routes::decode(data),
        Err(err) => panic!("Could not convert byte data into utf8 string: {}", err),
      }
    }
    Err(err) => panic!("IO error: {}", err),
  }

}


fn compute_routes(data: &str) {


}

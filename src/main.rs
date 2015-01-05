extern crate fasttrack;
extern crate serialize;
use fasttrack::route;
use serialize::json;

use std::io::File;
use std::str::from_utf8;

fn main() {

  let routes = load_data();
  build_adjancency_list(routes);



}

fn load_data() -> route::Routes {
  match File::open(&Path::new("graph.json")).read_to_end() {
    Ok(content_vec) => {
      match from_utf8(content_vec.as_slice()) {
        Ok(data) => return route::Routes::decode(data),
        Err(err) => panic!("Could not convert byte data into utf8 string: {}", err),
      }
    }
    Err(err) => panic!("IO error: {}", err),
  }

}

fn build_adjancency_list(data: route::Routes) {

}



fn compute_routes() {


}

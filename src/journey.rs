
#[derive(Show, Encodable, Decodable)]
pub struct Journey {
  pub from: uint,
  pub to: uint,
  pub route: Option<Vec<uint>>,
}

impl Journey {
  pub fn new(f: uint, t: uint, r: Option<Vec<uint>>) -> Journey {
    Journey{from: f, to: t, route: r }
  }
}

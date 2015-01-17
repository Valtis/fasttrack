
#[derive(Show, Encodable, Decodable)]
pub struct Journey {
  pub from: usize,
  pub to: usize,
  pub route: Option<Vec<usize>>,
}

impl Journey {
  pub fn new(f: usize, t: usize, r: Option<Vec<usize>>) -> Journey {
    Journey{from: f, to: t, route: r }
  }
}

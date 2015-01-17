// as of writing this (2015/1/4), compiler & traits seem to be undergoing changes
// and old_orphan_check feature is required for now to make things compile.
// The compiler suggested fix (Encodable -> RustcEncodable) itself causes compiler
// errors due to missing trait (not implemented yet?).
// This likely will cause issues with later compiler versions.
#![feature(old_orphan_check)]
extern crate serialize;



pub mod route_node;
pub mod adjancency_list;
pub mod dijkstra;
pub mod edge;
pub mod journey;

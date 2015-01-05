// as of writing this (2015/1/4), json crate seems to be undergoing changes so
// stick with the old orphan check for now. This should be fixed\changed once
// things stabilize
#![feature(old_orphan_check)]
extern crate serialize;



pub mod route_node;
pub mod adjancency_list;

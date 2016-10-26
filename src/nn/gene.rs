use std::collections::HashMap;

#[allow(dead_code)]
pub struct Node {
	bias: f64,
}

#[allow(dead_code)]
pub struct Link {
	weight: f64,
}

#[allow(dead_code)]
pub struct Net {
	nodes: HashMap<u32, Node>,
	links: HashMap<(u32, u32), Link>,
}